use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use pith_virtual_core::{
    ScrollCommand, ScrollDirection, ScrollToOptions, VirtualItem, Virtualizer,
};
use send_wrapper::SendWrapper;
use web_sys::{
    Element, HtmlElement, ResizeObserver,
    wasm_bindgen::{JsCast, closure::Closure},
};

use crate::measure::{index_from_element, measure_element, observe_item};
use crate::scroll_fns::{element_max_scroll_offset, element_scroll};

/// Handle returned by [`use_virtualizer`].
///
/// Follows the React adapter pattern: the view reads from the core
/// **imperatively** via `get_virtual_items()` / `get_total_size()`, and a
/// lightweight version counter signal triggers re-renders when state changes.
///
/// If `measure` mode is enabled (the default), the handle automatically
/// measures item elements that have the `data-index` attribute set. Place
/// `node_ref=virtualizer.container_ref()` on the element that directly
/// contains your virtual items.
#[derive(Clone)]
pub struct VirtualizerHandle {
    pub(crate) core: Arc<Mutex<SendWrapper<Virtualizer>>>,
    pub(crate) scroll_element: Arc<Mutex<Option<SendWrapper<Element>>>>,
    pub(crate) item_observer: Arc<Mutex<Option<SendWrapper<ResizeObserver>>>>,
    pub(crate) elements_cache: Arc<Mutex<SendWrapper<HashMap<usize, HtmlElement>>>>,
    pub(crate) horizontal: bool,

    /// Version counter — bumped on every state change to trigger reactive
    /// re-renders in closures that call `track()` or any `get_*` method.
    pub(crate) version: RwSignal<u64>,

    /// NodeRef for the item container element. When set, the handle
    /// automatically measures children with `data-index` attributes
    /// after each render cycle via `requestAnimationFrame`.
    pub(crate) container_ref: AnyNodeRef,
}

impl VirtualizerHandle {
    // -- Reactive output methods --

    /// Get the current virtual items (subscribes to changes).
    pub fn get_virtual_items(&self) -> Vec<VirtualItem> {
        self.track();
        self.core.lock().unwrap().get_virtual_items()
    }

    /// Get the total scrollable content size in pixels (subscribes to changes).
    pub fn get_total_size(&self) -> f64 {
        self.track();
        self.core.lock().unwrap().get_total_size()
    }

    /// Whether the virtualizer is currently scrolling (subscribes to changes).
    pub fn is_scrolling(&self) -> bool {
        self.track();
        self.core.lock().unwrap().is_scrolling()
    }

    /// Current scroll direction (subscribes to changes).
    pub fn scroll_direction(&self) -> Option<ScrollDirection> {
        self.track();
        self.core.lock().unwrap().scroll_direction()
    }

    /// Subscribe to the version counter so the calling reactive context
    /// re-runs on any virtualizer state change.
    pub fn track(&self) {
        self.version.track();
    }

    // -- Container ref for automatic measurement --

    /// Returns a `NodeRef` to place on the element that directly contains
    /// your virtual items. The virtualizer will automatically measure
    /// children that have the `data-index` attribute.
    ///
    /// ```rust,ignore
    /// <div node_ref=virtualizer.container_ref()>
    ///     {move || virtualizer.get_virtual_items().into_iter().map(|item| {
    ///         view! { <div data-index=item.index>...</div> }
    ///     }).collect_view()}
    /// </div>
    /// ```
    pub fn container_ref(&self) -> AnyNodeRef {
        self.container_ref
    }

    // -- Imperative scroll methods --

    /// Scroll to a specific item index with default alignment and behavior.
    pub fn scroll_to_index(&self, index: usize, opts: ScrollToOptions) {
        let max = self.get_max_scroll_offset();
        let now = self.now();
        let cmd = self
            .core
            .lock()
            .unwrap()
            .scroll_to_index(index, opts, max, now);
        if let Some(cmd) = cmd {
            self.execute_scroll_command(&cmd);
            self.schedule_reconciliation();
        }
        self.notify();
    }

    /// Scroll to a specific pixel offset.
    pub fn scroll_to_offset(&self, offset: f64, opts: ScrollToOptions) {
        let max = self.get_max_scroll_offset();
        let now = self.now();
        let cmd = self
            .core
            .lock()
            .unwrap()
            .scroll_to_offset(offset, opts, max, now);
        self.execute_scroll_command(&cmd);
        self.schedule_reconciliation();
        self.notify();
    }

    /// Scroll by a relative delta.
    pub fn scroll_by(&self, delta: f64, opts: ScrollToOptions) {
        let now = self.now();
        let cmd = self.core.lock().unwrap().scroll_by(delta, opts, now);
        self.execute_scroll_command(&cmd);
        self.schedule_reconciliation();
        self.notify();
    }

    /// Clear all measurement caches and re-layout.
    pub fn measure(&self) {
        self.core.lock().unwrap().measure();
        self.notify();
    }

    /// Register a DOM element for size measurement.
    ///
    /// Most consumers should use `container_ref()` for automatic measurement
    /// instead of calling this directly.
    pub fn measure_element(&self, node: Option<&HtmlElement>) {
        let Some(node) = node else {
            let mut cache = self.elements_cache.lock().unwrap();
            cache.retain(|_, el| el.is_connected());
            return;
        };

        let index_attr = self.core.lock().unwrap().options().index_attribute.clone();
        let horizontal = self.horizontal;

        let Some(index) = index_from_element(node.unchecked_ref(), &index_attr) else {
            return;
        };

        let key = (self.core.lock().unwrap().options().get_item_key)(index);

        self.ensure_item_observer();

        let mut cache = self.elements_cache.lock().unwrap();
        let prev = cache.get(&key);

        if prev != Some(node) {
            if let Some(prev_el) = prev
                && let Some(obs) = self.item_observer.lock().unwrap().as_ref()
            {
                obs.unobserve(prev_el.unchecked_ref());
            }
            if let Some(obs) = self.item_observer.lock().unwrap().as_ref() {
                observe_item(obs, node.unchecked_ref());
            }
            cache.insert(key, node.clone());
        }
        drop(cache);

        // Sync-measure when idle or during programmatic scroll.
        let is_scrolling = self.core.lock().unwrap().is_scrolling();
        let has_pending = self.core.lock().unwrap().has_pending_scroll();
        let should_measure = self
            .core
            .lock()
            .unwrap()
            .should_measure_during_scroll(index);

        if (!is_scrolling || has_pending) && should_measure {
            let size = measure_element(node, None, horizontal);
            let result = self.core.lock().unwrap().resize_item(index, size);
            if let Some(correction) = result.scroll_correction {
                self.execute_scroll_command(&correction);
            }
            if result.should_notify {
                self.notify();
            }
        }
    }

    // -- Internal helpers --

    /// Bump the version counter to trigger reactive re-renders.
    pub(crate) fn notify(&self) {
        self.version.update(|v| *v = v.wrapping_add(1));
    }

    /// Scan the container for elements with `data-index` and measure them.
    /// Called from a `requestAnimationFrame` callback.
    pub(crate) fn measure_container_children(&self) {
        let Some(container_node) = self.container_ref.get_untracked() else {
            return;
        };
        let Ok(container_el) = container_node.dyn_into::<Element>() else {
            return;
        };

        let index_attr = self.core.lock().unwrap().options().index_attribute.clone();
        let selector = format!("[{index_attr}]");

        let Ok(nodes) = container_el.query_selector_all(&selector) else {
            return;
        };

        for i in 0..nodes.length() {
            if let Some(node) = nodes.item(i)
                && let Ok(html_el) = node.dyn_into::<HtmlElement>()
            {
                self.measure_element(Some(&html_el));
            }
        }
    }

    pub(crate) fn execute_scroll_command(&self, cmd: &ScrollCommand) {
        if cmd.behavior.is_none() && cmd.adjustments.is_none() {
            return;
        }
        if let Some(el) = self.scroll_element.lock().unwrap().as_ref() {
            element_scroll(
                el,
                cmd.offset,
                self.horizontal,
                cmd.behavior,
                cmd.adjustments,
            );
        }
    }

    fn get_max_scroll_offset(&self) -> f64 {
        self.scroll_element
            .lock()
            .unwrap()
            .as_ref()
            .map(|el| element_max_scroll_offset(el, self.horizontal))
            .unwrap_or(0.0)
    }

    fn now(&self) -> f64 {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0)
    }

    pub(crate) fn ensure_item_observer(&self) {
        if self.item_observer.lock().unwrap().is_some() {
            return;
        }

        let core = self.core.clone();
        let handle_clone = self.clone();
        let horizontal = self.horizontal;
        let index_attr = self.core.lock().unwrap().options().index_attribute.clone();

        let observer = crate::measure::create_item_observer(move |element, entry| {
            let Some(index) = index_from_element(element.unchecked_ref(), &index_attr) else {
                return;
            };

            let should = core.lock().unwrap().should_measure_during_scroll(index);
            if !should {
                return;
            }

            let size = measure_element(element, Some(entry), horizontal);
            let result = core.lock().unwrap().resize_item(index, size);
            if let Some(correction) = result.scroll_correction {
                handle_clone.execute_scroll_command(&correction);
            }
            if result.should_notify {
                handle_clone.notify();
            }
        });

        *self.item_observer.lock().unwrap() = Some(SendWrapper::new(observer));
    }

    pub(crate) fn schedule_reconciliation(&self) {
        if !self.core.lock().unwrap().has_pending_scroll() {
            return;
        }

        let handle = self.clone();
        let closure: Closure<dyn FnMut()> = Closure::once(move || {
            handle.run_reconciliation();
        });

        if let Some(window) = web_sys::window() {
            window
                .request_animation_frame(closure.as_ref().unchecked_ref())
                .ok();
        }
        closure.forget();
    }

    fn run_reconciliation(&self) {
        if !self.core.lock().unwrap().has_pending_scroll() {
            return;
        }

        let now = self.now();
        let max = self.get_max_scroll_offset();
        let result = self.core.lock().unwrap().reconcile_scroll(now, max);

        match result {
            Some(cmd) => {
                self.execute_scroll_command(&cmd);
                self.schedule_reconciliation();
            }
            None => {
                self.notify();
            }
        }
    }

    /// Disconnect all observers and clean up.
    pub(crate) fn cleanup(&self) {
        if let Some(obs) = self.item_observer.lock().unwrap().as_ref() {
            obs.disconnect();
        }
        *self.item_observer.lock().unwrap() = None;
        self.elements_cache.lock().unwrap().clear();
    }
}
