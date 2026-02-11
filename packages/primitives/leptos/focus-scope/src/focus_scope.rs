use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use leptos::{attribute_interceptor::AttributeInterceptor, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use once_cell::sync::Lazy;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::Primitive;
use send_wrapper::SendWrapper;
use web_sys::{
    CustomEvent, CustomEventInit, Event, FocusEvent, KeyboardEvent, MutationObserver,
    MutationObserverInit, MutationRecord, NodeFilter,
    wasm_bindgen::{JsCast, closure::Closure},
};

const AUTOFOCUS_ON_MOUNT: &str = "focusScope.autoFocusOnMount";
const AUTOFOCUS_ON_UNMOUNT: &str = "focusScope.autoFocusOnUnmount";

type FocusEventClosure = Arc<SendWrapper<Closure<dyn Fn(FocusEvent)>>>;

#[component]
pub fn FocusScope(
    /// When `true`, tabbing from last item will focus first tabbable and shift+tab from first item will focus last tababble. Defaults to `false`.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    /// When `true`, focus cannot escape the focus scope via keyboard, pointer, or a programmatic focus. Defaults to `false`.
    #[prop(into, optional)]
    trapped: MaybeProp<bool>,
    #[prop(into, optional)] on_mount_auto_focus: Option<Callback<Event>>,
    // TODO: hopefully remove the double option
    #[prop(into, optional)] on_unmount_auto_focus: Option<Option<Callback<Event>>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let r#loop = Signal::derive(move || r#loop.get().unwrap_or(false));
    let trapped = Signal::derive(move || trapped.get().unwrap_or(false));

    let container_ref: AnyNodeRef = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, container_ref]);
    let last_focused_element: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> =
        RwSignal::new(None);
    let focus_scope = RwSignal::new(FocusScopeAPI::new());

    let handle_focus_in: FocusEventClosure =
        Arc::new(SendWrapper::new(Closure::new(move |event: FocusEvent| {
            if focus_scope.get_untracked().paused() {
                return;
            }

            if let Some(container) = container_ref.get_untracked() {
                let container: &web_sys::HtmlElement = container.unchecked_ref();
                let target = event
                    .target()
                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>());

                if container.contains(target.as_ref().map(|e| e.unchecked_ref())) {
                    last_focused_element.set(target.map(SendWrapper::new));
                } else {
                    focus(
                        last_focused_element.get_untracked().as_deref().cloned(),
                        Some(FocusOptions { select: true }),
                    );
                }
            }
        })));

    let handle_focus_out: FocusEventClosure =
        Arc::new(SendWrapper::new(Closure::new(move |event: FocusEvent| {
            if focus_scope.get_untracked().paused() {
                return;
            }

            if let Some(container) = container_ref.get_untracked() {
                let container: &web_sys::HtmlElement = container.unchecked_ref();
                let related_target = event
                    .related_target()
                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>());

                // A `focusout` event with a `None` `related_target` will happen in at least two cases:
                //
                // 1. When the user switches app/tabs/windows/the browser itself loses focus.
                // 2. In Google Chrome, when the focused element is removed from the DOM.
                //
                // We let the browser do its thing here because:
                //
                // 1. The browser already keeps a memory of what's focused for when the page gets refocused.
                // 2. In Google Chrome, if we try to focus the deleted focused element (as per below), it
                //    throws the CPU to 100%, so we avoid doing anything for this reason here too.
                if related_target.is_none() {
                    return;
                }

                // If the focus has moved to an actual legitimate element (`related_target != None`)
                // that is outside the container, we move focus to the last valid focused element inside.
                if !container.contains(related_target.as_ref().map(|e| e.unchecked_ref())) {
                    focus(
                        last_focused_element.get_untracked().as_deref().cloned(),
                        Some(FocusOptions { select: true }),
                    );
                }
            }
        })));

    let mutation_observer: StoredValue<SendWrapper<RefCell<Option<MutationObserver>>>> =
        StoredValue::new(SendWrapper::new(RefCell::new(None)));

    type TrappedCleanupFn = Box<dyn Fn()>;
    let trapped_cleanup: StoredValue<SendWrapper<RefCell<Option<TrappedCleanupFn>>>> =
        StoredValue::new(SendWrapper::new(RefCell::new(None)));

    // Takes care of trapping focus if focus is moved outside programmatically for example.
    // Mirrors the React useEffect with [trapped, container, focusScope.paused] deps:
    // cleans up on re-run (e.g. when trapped goes from true → false) and on unmount.
    Effect::new(move |_| {
        // Clean up previous effect run (equivalent to React useEffect cleanup on deps change)
        trapped_cleanup.with_value(|f| {
            if let Some(cleanup) = f.borrow_mut().take() {
                cleanup();
            }
        });

        if trapped.get() {
            let hi = handle_focus_in.clone();
            let ho = handle_focus_out.clone();

            document()
                .add_event_listener_with_callback("focusin", (*hi).as_ref().unchecked_ref())
                .expect("Focus in event listener should be added.");
            document()
                .add_event_listener_with_callback("focusout", (*ho).as_ref().unchecked_ref())
                .expect("Focus out event listener should be added.");

            // When the focused element gets removed from the DOM, browsers move focus
            // back to the document.body. In this case, we move focus to the container
            // to keep focus trapped correctly.
            if let Some(container) = container_ref.get() {
                let container: web_sys::HtmlElement = container.unchecked_into();

                let handle_mutations: Closure<dyn Fn(Vec<MutationRecord>)> =
                    Closure::new(move |mutations: Vec<MutationRecord>| {
                        let focused_element = document()
                            .active_element()
                            .map(|element| element.unchecked_into::<web_sys::HtmlElement>());
                        if focused_element != document().body() {
                            return;
                        }

                        for mutation in mutations {
                            if mutation.removed_nodes().length() > 0 {
                                focus(
                                    container_ref
                                        .get_untracked()
                                        .map(|el| el.unchecked_into::<web_sys::HtmlElement>()),
                                    None,
                                );
                            }
                        }
                    });

                let new_observer =
                    MutationObserver::new(handle_mutations.into_js_value().unchecked_ref())
                        .expect("Mutation observer should be created.");

                let init = MutationObserverInit::new();
                init.set_child_list(true);
                init.set_subtree(true);

                new_observer
                    .observe_with_options(&container, &init)
                    .expect("Mutation observer should observe target.");

                mutation_observer.with_value(|obs| {
                    obs.borrow_mut().replace(new_observer);
                });
            }

            // Store cleanup for this effect run
            let cleanup_hi = hi;
            let cleanup_ho = ho;
            trapped_cleanup.with_value(|f| {
                f.borrow_mut().replace(Box::new(move || {
                    document()
                        .remove_event_listener_with_callback(
                            "focusin",
                            (*cleanup_hi).as_ref().unchecked_ref(),
                        )
                        .expect("Focus in event listener should be removed.");
                    document()
                        .remove_event_listener_with_callback(
                            "focusout",
                            (*cleanup_ho).as_ref().unchecked_ref(),
                        )
                        .expect("Focus out event listener should be removed.");

                    mutation_observer.with_value(|obs| {
                        if let Some(observer) = obs.borrow_mut().take() {
                            observer.disconnect();
                        }
                    });
                }));
            });
        }
    });

    type AutoFocusEndFn = Box<dyn Fn()>;
    let auto_focus_end: StoredValue<SendWrapper<RefCell<Option<AutoFocusEndFn>>>> =
        StoredValue::new(SendWrapper::new(RefCell::new(None)));

    Effect::new(move |_| {
        auto_focus_end.with_value(|end| {
            if let Some(on_mount_auto_focus_cleanup) = end.borrow_mut().take() {
                on_mount_auto_focus_cleanup();
            }
        });

        if let Some(container) = container_ref.get() {
            let container: web_sys::HtmlElement = container.unchecked_into();

            {
                let mut focus_scope_stack = FOCUS_SCOPE_STACK
                    .lock()
                    .expect("Focus scope stack mutex should lock.");
                focus_scope_stack.add(focus_scope.get());
            }

            let previously_focused_element = document()
                .active_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>());
            let has_focused_candidate = container.contains(
                previously_focused_element
                    .as_ref()
                    .map(|element| element.unchecked_ref()),
            );

            if !has_focused_candidate {
                let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
                    if let Some(on_mount_auto_focus) = on_mount_auto_focus {
                        on_mount_auto_focus.run(event);
                    }
                });

                let init = CustomEventInit::new();
                init.set_bubbles(false);
                init.set_cancelable(true);

                let mount_event = CustomEvent::new_with_event_init_dict(AUTOFOCUS_ON_MOUNT, &init)
                    .expect("Auto focus on mount event should be instantiated.");

                container
                    .add_event_listener_with_callback(
                        AUTOFOCUS_ON_MOUNT,
                        closure.as_ref().unchecked_ref(),
                    )
                    .expect("Auto focus on mount event listener should be added.");
                container
                    .dispatch_event(&mount_event)
                    .expect("Auto focus on mount event should be dispatched.");

                if !mount_event.default_prevented() {
                    focus_first(
                        remove_links(get_tabbable_candidates(&container)),
                        Some(FocusOptions { select: true }),
                    );
                    if document().active_element().as_ref() == previously_focused_element.as_deref()
                    {
                        focus(Some(container.clone()), None);
                    }
                }

                let container_clone = container.clone();
                auto_focus_end.with_value(|end| {
                    end.borrow_mut().replace(Box::new(move || {
                        container_clone
                            .remove_event_listener_with_callback(
                                AUTOFOCUS_ON_MOUNT,
                                closure.as_ref().unchecked_ref(),
                            )
                            .expect("Auto focus on mount event listener should be removed.");

                        let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
                            if let Some(on_unmount_auto_focus) = on_unmount_auto_focus.flatten() {
                                on_unmount_auto_focus.run(event);
                            }
                        });

                        let init = CustomEventInit::new();
                        init.set_bubbles(false);
                        init.set_cancelable(true);

                        let unmount_event =
                            CustomEvent::new_with_event_init_dict(AUTOFOCUS_ON_UNMOUNT, &init)
                                .expect("Auto focus on unmount event should be instantiated.");

                        container_clone
                            .add_event_listener_with_callback(
                                AUTOFOCUS_ON_UNMOUNT,
                                closure.as_ref().unchecked_ref(),
                            )
                            .expect("Auto focus on unmount event listener should be added.");
                        container_clone
                            .dispatch_event(&unmount_event)
                            .expect("Auto focus on unmount event should be dispatched.");

                        if !unmount_event.default_prevented() {
                            focus(
                                previously_focused_element.clone().or(document().body()),
                                Some(FocusOptions { select: true }),
                            );
                        }

                        container_clone
                            .remove_event_listener_with_callback(
                                AUTOFOCUS_ON_UNMOUNT,
                                closure.as_ref().unchecked_ref(),
                            )
                            .expect("Auto focus on unmount event listener should be removed.");

                        {
                            let mut focus_scope_stack = FOCUS_SCOPE_STACK
                                .lock()
                                .expect("Focus scope stack mutex should lock.");
                            focus_scope_stack.remove(&focus_scope.get_untracked());
                        }
                    }));
                });
            }
        }
    });

    on_cleanup(move || {
        trapped_cleanup.with_value(|f| {
            if let Some(cleanup) = f.borrow_mut().take() {
                cleanup();
            }
        });

        auto_focus_end.with_value(|end| {
            if let Some(auto_focus_cleanup) = end.borrow_mut().take() {
                auto_focus_cleanup();
            }
        });
    });

    // Takes care of looping focus (when tabbing whilst at the edges).
    let handle_key_down = move |event: KeyboardEvent| {
        let r#loop = r#loop.get_untracked();

        if !r#loop && !trapped.get_untracked() {
            return;
        }
        if focus_scope.get_untracked().paused() {
            return;
        }

        let is_tab_key =
            event.key() == "Tab" && !event.alt_key() && !event.ctrl_key() && !event.meta_key();
        let focused_element = document()
            .active_element()
            .map(|element| element.unchecked_into::<web_sys::HtmlElement>());

        if is_tab_key && let Some(focused_element) = focused_element {
            let container = event
                .current_target()
                .expect("Event should have current target.")
                .unchecked_into::<web_sys::HtmlElement>();
            let (first, last) = get_tabbable_edges(&container);
            let has_tabbable_elements_inside = first.is_some() && last.is_some();

            if !has_tabbable_elements_inside {
                if focused_element == container {
                    event.prevent_default();
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if !event.shift_key()
                    && &focused_element == last.as_ref().expect("Last option checked above.")
                {
                    event.prevent_default();

                    if r#loop {
                        focus(first, Some(FocusOptions { select: true }));
                    }
                } else if event.shift_key()
                    && &focused_element == first.as_ref().expect("First option checked above.")
                {
                    event.prevent_default();

                    if r#loop {
                        focus(last, Some(FocusOptions { select: true }));
                    }
                }
            }
        }
    };

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                attr:tabindex="-1"
                on:keydown=move |event: web_sys::KeyboardEvent| handle_key_down(event)
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

#[derive(Clone, Debug, Default)]
struct FocusOptions {
    pub select: bool,
}

/// Attempts focusing the first element in a list of candidates.
/// Stops when focus has actually moved.
fn focus_first(candidates: Vec<web_sys::HtmlElement>, options: Option<FocusOptions>) {
    let previously_focused_element = document().active_element();

    for candidate in candidates {
        focus(Some(candidate), options.clone());
        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

/// Returns the first and last tabbable elements inside a container.
fn get_tabbable_edges(
    container: &web_sys::HtmlElement,
) -> (Option<web_sys::HtmlElement>, Option<web_sys::HtmlElement>) {
    let candidates = get_tabbable_candidates(container);

    let mut reverse_candidates = candidates.clone();
    reverse_candidates.reverse();

    let first = find_visible(candidates, container);
    let last = find_visible(reverse_candidates, container);

    (first, last)
}

/// Returns a list of potential tabbable candidates.
///
/// NOTE: This is only a close approximation. For example it doesn't take into account cases like when
/// elements are not visible. This cannot be worked out easily by just reading a property, but rather
/// necessitate runtime knowledge (computed styles, etc). We deal with these cases separately.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker
/// Credit: https://github.com/discord/focus-layers/blob/master/src/util/wrapFocus.tsx#L1
fn get_tabbable_candidates(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes: Vec<web_sys::HtmlElement> = vec![];

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>() {
                if html_element.hidden() {
                    // NodeFilter.FILTER_SKIP
                    return 3;
                }

                if let Some(input_element) = node.dyn_ref::<web_sys::HtmlInputElement>()
                    && (input_element.disabled() || input_element.type_() == "hidden")
                {
                    // NodeFilter.FILTER_SKIP
                    return 3;
                }

                if html_element.tab_index() >= 0 {
                    // NodeFilter.FILTER_ACCEPT
                    return 1;
                }
            }

            // NodeFilter.FILTER_SKIP
            3
        });

    let node_filter = NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    let walker = document()
        // 0x01 is NodeFilter.SHOW_ELEMENT
        .create_tree_walker_with_what_to_show_and_filter(container, 0x1, Some(&node_filter))
        .expect("Tree walker should be created.");

    while let Some(node) = walker
        .next_node()
        .expect("Tree walker should return a next node.")
    {
        let node: web_sys::Node = node;
        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
            nodes.push(element);
        }
    }

    // We do not take into account the order of nodes with positive `tabindex` as it
    // hinders accessibility to have tab order different from visual order.
    nodes
}

/// Returns the first visible element in a list.
/// NOTE: Only checks visibility up to the `container`.
fn find_visible(
    elements: Vec<web_sys::HtmlElement>,
    container: &web_sys::HtmlElement,
) -> Option<web_sys::HtmlElement> {
    elements.into_iter().find(|element| {
        !is_hidden(
            element,
            Some(IsHiddenOptions {
                up_to: Some(container),
            }),
        )
    })
}

#[derive(Debug, Default, Clone)]
struct IsHiddenOptions<'a> {
    pub up_to: Option<&'a web_sys::HtmlElement>,
}

fn is_hidden(node: &web_sys::HtmlElement, options: Option<IsHiddenOptions>) -> bool {
    let options = options.unwrap_or_default();

    let window = web_sys::window().expect("Window should exist.");

    if window
        .get_computed_style(node)
        .expect("Element is valid.")
        .expect("Element should have computed style.")
        .get_property_value("visibility")
        .expect("Computed style should have visibility.")
        == "hidden"
    {
        return true;
    }

    let mut node: Option<web_sys::Element> = Some(node.deref().clone());
    while let Some(n) = node.as_ref() {
        // We stop at `upTo` (excluding it).
        if let Some(up_to) = options.up_to.as_ref() {
            let up_to_element: &web_sys::Element = up_to;
            if n == up_to_element {
                return false;
            }
        }

        if window
            .get_computed_style(n)
            .expect("Element is valid.")
            .expect("Element should have computed style.")
            .get_property_value("display")
            .expect("Computed style should have display.")
            == "none"
        {
            return true;
        }

        node = n.parent_element();
    }

    false
}

fn is_selectable_input(element: &web_sys::Element) -> bool {
    web_sys::HtmlInputElement::instanceof(element)
}

fn focus(element: Option<web_sys::HtmlElement>, options: Option<FocusOptions>) {
    let options = options.unwrap_or_default();

    if let Some(element) = element {
        let previously_focused_element = document().active_element();

        // NOTE: We prevent scrolling on focus, to minimize jarring transitions for users.
        // TODO: web_sys does not support passing options. JS: element.focus({ preventScroll: true })
        element.focus().expect("Element should be focused.");

        // Only select if its not the same element, it supports selection and we need to select.
        let el: &web_sys::Element = &element;
        if Some(el) != previously_focused_element.as_ref()
            && is_selectable_input(el)
            && options.select
        {
            element
                .unchecked_into::<web_sys::HtmlInputElement>()
                .select();
        }
    }
}

fn remove_links(items: Vec<web_sys::HtmlElement>) -> Vec<web_sys::HtmlElement> {
    items
        .into_iter()
        .filter(|item| item.tag_name() != "A")
        .collect()
}

static FOCUS_SCOPE_STACK: Lazy<Mutex<FocusScopeStack>> =
    Lazy::new(|| Mutex::new(FocusScopeStack::new()));

#[derive(Clone, Debug)]
struct FocusScopeAPI {
    id: u64,
    paused: Arc<AtomicBool>,
}

impl FocusScopeAPI {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            paused: Arc::new(AtomicBool::new(false)),
        }
    }

    fn paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    fn pause(&mut self) {
        self.paused.store(true, Ordering::Relaxed)
    }

    fn resume(&mut self) {
        self.paused.store(false, Ordering::Relaxed);
    }
}

impl PartialEq for FocusScopeAPI {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A stack of focus scopes, with the active one at the top.
#[derive(Clone, Debug, PartialEq)]
struct FocusScopeStack {
    stack: Vec<FocusScopeAPI>,
}

impl FocusScopeStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn add(&mut self, focus_scope: FocusScopeAPI) {
        // Pause the currently active focus scope (at the top of the stack).
        if let Some(active_focus_scope) = self.stack.first_mut()
            && focus_scope != *active_focus_scope
        {
            active_focus_scope.pause();
        }

        // Remove in case it already exists (because we'll re-add it at the top of the stack).
        self.remove_without_resume(&focus_scope);
        self.stack.insert(0, focus_scope);

        // This is not in the React implementation, but without the unit tests could never pass.
        if let Some(first_focus_scope) = self.stack.first_mut() {
            first_focus_scope.resume();
        }
    }

    fn remove(&mut self, focus_scope: &FocusScopeAPI) {
        self.remove_without_resume(focus_scope);

        if let Some(first_focus_scope) = self.stack.first_mut() {
            first_focus_scope.resume();
        }
    }

    fn remove_without_resume(&mut self, focus_scope: &FocusScopeAPI) {
        let index = self.stack.iter().position(|f| f == focus_scope);

        if let Some(index) = index {
            self.stack.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_scope_api() {
        let mut a = FocusScopeAPI::new();
        let mut b = a.clone();

        assert!(!a.paused());
        assert!(!b.paused());

        a.pause();
        assert!(a.paused());
        assert!(b.paused());

        a.resume();
        assert!(!a.paused());
        assert!(!b.paused());

        b.pause();
        assert!(a.paused());
        assert!(b.paused());

        b.resume();
        assert!(!a.paused());
        assert!(!b.paused());
    }

    #[test]
    fn test_focus_scope_stack() {
        let mut stack = FocusScopeStack::new();

        let a = FocusScopeAPI::new();
        let b = FocusScopeAPI::new();
        let c = FocusScopeAPI::new();

        stack.add(a.clone());
        assert_eq!(vec![a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());

        stack.add(b.clone());
        assert_eq!(vec![b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());

        stack.add(c.clone());
        assert_eq!(vec![c.clone(), b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());
        assert!(stack.stack[2].paused());

        stack.add(b.clone());
        assert_eq!(vec![b.clone(), c.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());
        assert!(stack.stack[2].paused());

        stack.remove(&c);
        assert_eq!(vec![b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());

        stack.remove(&c);
        assert_eq!(vec![b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());

        stack.remove(&b);
        assert_eq!(vec![a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());

        stack.remove(&a);
        assert!(stack.stack.is_empty());
    }
}
