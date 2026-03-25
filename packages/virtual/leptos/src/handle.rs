use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use leptos::prelude::*;
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
/// Unlike the previous signal-based design, this handle follows the React
/// adapter pattern: the view reads from the core **imperatively** via
/// `get_virtual_items()` / `get_total_size()`, and a lightweight version
/// counter signal triggers re-renders when state changes. This avoids the
/// batching/lifecycle issues that arise when `Vec<VirtualItem>` is stored
/// in a signal.
#[derive(Clone)]
pub struct VirtualizerHandle {
    pub(crate) core: Arc<Mutex<SendWrapper<Virtualizer>>>,
    pub(crate) scroll_element: Arc<Mutex<Option<SendWrapper<Element>>>>,
    pub(crate) item_observer: Arc<Mutex<Option<SendWrapper<ResizeObserver>>>>,
    pub(crate) elements_cache: Arc<Mutex<SendWrapper<HashMap<usize, HtmlElement>>>>,
    pub(crate) horizontal: bool,

    /// Monotonic version counter. Bumped on every state change (scroll,
    /// resize, measurement). Reactive closures that call `track()` will
    /// re-run when this changes.
    pub(crate) version: RwSignal<u64>,
}

impl VirtualizerHandle {
    /// Subscribe to changes and get the current virtual items.
    ///
    /// Call this inside a reactive closure (e.g. inside `view!` or
    /// `move || { ... }`). It subscribes to the version counter so the
    /// closure re-runs when the virtualizer state changes.
    pub fn get_virtual_items(&self) -> Vec<VirtualItem> {
        self.track();
        self.core.lock().unwrap().get_virtual_items()
    }

    /// Subscribe to changes and get the total scrollable content size.
    pub fn get_total_size(&self) -> f64 {
        self.track();
        self.core.lock().unwrap().get_total_size()
    }

    /// Subscribe to changes and get the scrolling state.
    pub fn is_scrolling(&self) -> bool {
        self.track();
        self.core.lock().unwrap().is_scrolling()
    }

    /// Subscribe to changes and get the scroll direction.
    pub fn scroll_direction(&self) -> Option<ScrollDirection> {
        self.track();
        self.core.lock().unwrap().scroll_direction()
    }

    /// Subscribe to the version counter so the calling reactive context
    /// re-runs on any virtualizer state change.
    pub fn track(&self) {
        self.version.track();
    }

    /// The `data-index` attribute name.
    pub fn index_attribute(&self) -> String {
        self.core.lock().unwrap().options().index_attribute.clone()
    }

    // -- Imperative scroll methods --

    /// Scroll to a specific item index.
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
    /// Each rendered virtual item should call this with its DOM element.
    /// The element **must** have the `data-index` attribute set.
    ///
    /// Pass `None` to trigger cleanup of disconnected elements.
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
