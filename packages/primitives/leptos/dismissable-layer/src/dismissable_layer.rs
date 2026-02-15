use std::sync::Mutex;

use leptos::{attribute_interceptor::AttributeInterceptor, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use once_cell::sync::Lazy;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_primitive::Primitive;
use radix_leptos_use_escape_keydown::use_escape_keydown;
use send_wrapper::SendWrapper;
use web_sys::{
    AddEventListenerOptions, CustomEvent, CustomEventInit,
    wasm_bindgen::{JsCast, JsValue, closure::Closure},
};

const CONTEXT_UPDATE: &str = "dismissableLayer.update";
const POINTER_DOWN_OUTSIDE: &str = "dismissableLayer.pointerDownOutside";
const FOCUS_OUTSIDE: &str = "dismissableLayer.focusOutside";

pub type PointerDownOutsideEvent = CustomEvent;
pub type FocusOutsideEvent = CustomEvent;

// -- Global layer context ---------------------------------------------------

static DISMISSABLE_LAYER_CONTEXT: Lazy<Mutex<DismissableLayerContextValue>> =
    Lazy::new(|| Mutex::new(DismissableLayerContextValue::new()));

static ORIGINAL_BODY_POINTER_EVENTS: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

#[derive(Debug)]
struct DismissableLayerContextValue {
    layers: Vec<SendWrapper<web_sys::HtmlElement>>,
    layers_with_outside_pointer_events_disabled: Vec<SendWrapper<web_sys::HtmlElement>>,
    branches: Vec<SendWrapper<web_sys::HtmlElement>>,
}

impl DismissableLayerContextValue {
    fn new() -> Self {
        Self {
            layers: Vec::new(),
            layers_with_outside_pointer_events_disabled: Vec::new(),
            branches: Vec::new(),
        }
    }

    fn add_layer(&mut self, node: &web_sys::HtmlElement) {
        if !self.layers.iter().any(|l| **l == *node) {
            self.layers.push(SendWrapper::new(node.clone()));
        }
    }

    fn remove_layer(&mut self, node: &web_sys::HtmlElement) {
        self.layers.retain(|l| **l != *node);
    }

    fn add_layer_with_outside_pointer_events_disabled(&mut self, node: &web_sys::HtmlElement) {
        if !self
            .layers_with_outside_pointer_events_disabled
            .iter()
            .any(|l| **l == *node)
        {
            self.layers_with_outside_pointer_events_disabled
                .push(SendWrapper::new(node.clone()));
        }
    }

    fn remove_layer_with_outside_pointer_events_disabled(&mut self, node: &web_sys::HtmlElement) {
        self.layers_with_outside_pointer_events_disabled
            .retain(|l| **l != *node);
    }

    fn add_branch(&mut self, node: &web_sys::HtmlElement) {
        if !self.branches.iter().any(|b| **b == *node) {
            self.branches.push(SendWrapper::new(node.clone()));
        }
    }

    fn remove_branch(&mut self, node: &web_sys::HtmlElement) {
        self.branches.retain(|b| **b != *node);
    }

    fn layer_index(&self, node: &web_sys::HtmlElement) -> Option<usize> {
        self.layers.iter().position(|l| **l == *node)
    }

    fn layers_count(&self) -> usize {
        self.layers.len()
    }

    fn layers_with_outside_pointer_events_disabled_count(&self) -> usize {
        self.layers_with_outside_pointer_events_disabled.len()
    }

    fn highest_layer_with_outside_pointer_events_disabled_index(&self) -> Option<usize> {
        let highest = self.layers_with_outside_pointer_events_disabled.last()?;
        self.layers.iter().position(|l| **l == **highest)
    }

    fn branches_contain(&self, target: &web_sys::Node) -> bool {
        self.branches.iter().any(|b| b.contains(Some(target)))
    }

    /// Check if any layer registered AFTER `node` contains the target.
    ///
    /// This is needed because Leptos doesn't have React's Portal event propagation.
    /// In React, synthetic events from portaled children bubble through the React tree,
    /// so a parent DismissableLayer's capture handler fires for events inside portaled
    /// child layers. In Leptos/DOM, capture handlers only fire on DOM ancestors. Portaled
    /// layers are siblings in the DOM, so we must explicitly check if the event target is
    /// inside a higher (child) layer to avoid false "outside" detections.
    ///
    /// Only layers with a higher index are checked — these are layers registered after
    /// the current one, which in nested scenarios correspond to child layers. This ensures
    /// clicking inside a parent layer still correctly triggers "outside" for child layers.
    fn higher_layers_contain(&self, node: &web_sys::HtmlElement, target: &web_sys::Node) -> bool {
        let my_index = self.layer_index(node).unwrap_or(0);
        self.layers
            .iter()
            .skip(my_index + 1)
            .any(|l| l.contains(Some(target)))
    }
}

// -- Update broadcasting ----------------------------------------------------

fn dispatch_update() {
    let event = CustomEvent::new(CONTEXT_UPDATE).expect("CustomEvent should be instantiated.");
    document().dispatch_event(&event).ok();
}

// -- use_pointer_down_outside -----------------------------------------------

struct PointerDownOutsideReturn {
    on_pointer_down_capture: Box<dyn Fn()>,
}

fn use_pointer_down_outside(
    on_pointer_down_outside: Option<Callback<PointerDownOutsideEvent>>,
    owner_document: web_sys::Document,
) -> PointerDownOutsideReturn {
    type ClickClosure = Closure<dyn Fn(web_sys::Event)>;

    let is_pointer_inside_tree = StoredValue::new(false);
    let handle_click_ref: StoredValue<SendWrapper<Option<ClickClosure>>> =
        StoredValue::new(SendWrapper::new(None));

    let owner_doc = SendWrapper::new(owner_document.clone());
    let owner_doc_for_cleanup = SendWrapper::new(owner_document);

    type PointerDownClosure = Closure<dyn Fn(web_sys::PointerEvent)>;
    let handle_pointer_down: StoredValue<Option<SendWrapper<PointerDownClosure>>> =
        StoredValue::new(None);

    let setup_owner_doc = owner_doc.clone();
    let cleanup_owner_doc = owner_doc_for_cleanup.clone();

    Effect::new(move |_| {
        let owner_doc = setup_owner_doc.clone();
        let owner_doc2 = setup_owner_doc.clone();

        let closure: PointerDownClosure = Closure::new(move |event: web_sys::PointerEvent| {
            let has_target = event.target().is_some();
            let is_inside = is_pointer_inside_tree.get_value();

            if has_target && !is_inside {
                let event_detail = event.clone();

                let dispatch_fn = {
                    let event_detail = event_detail.clone();
                    let on_pointer_down_outside = on_pointer_down_outside;
                    move || {
                        handle_and_dispatch_custom_event(
                            POINTER_DOWN_OUTSIDE,
                            on_pointer_down_outside,
                            &event_detail,
                        );
                    }
                };

                // On touch devices, wait for the click event.
                if event.pointer_type() == "touch" {
                    // Remove any previous click handler
                    handle_click_ref.with_value(|prev| {
                        if let Some(prev_closure) = prev.as_ref() {
                            owner_doc
                                .remove_event_listener_with_callback(
                                    "click",
                                    prev_closure.as_ref().unchecked_ref(),
                                )
                                .ok();
                        }
                    });

                    let click_closure: Closure<dyn Fn(web_sys::Event)> =
                        Closure::new(move |_event: web_sys::Event| {
                            dispatch_fn();
                        });

                    let options = AddEventListenerOptions::new();
                    options.set_once(true);

                    owner_doc
                        .add_event_listener_with_callback_and_add_event_listener_options(
                            "click",
                            click_closure.as_ref().unchecked_ref(),
                            &options,
                        )
                        .expect("Click event listener should be added.");

                    handle_click_ref.set_value(SendWrapper::new(Some(click_closure)));
                } else {
                    dispatch_fn();
                }
            } else {
                // Remove the click listener if pointer was inside (cancellation).
                handle_click_ref.with_value(|prev| {
                    if let Some(prev_closure) = prev.as_ref() {
                        owner_doc
                            .remove_event_listener_with_callback(
                                "click",
                                prev_closure.as_ref().unchecked_ref(),
                            )
                            .ok();
                    }
                });
            }
            is_pointer_inside_tree.set_value(false);
        });

        // Delay listener registration to avoid catching the mount event.
        let closure_ref: &JsValue = closure.as_ref();
        let closure_ref_js: web_sys::js_sys::Function = closure_ref.clone().unchecked_into();
        let owner_doc_for_timeout = owner_doc2.clone();
        let timeout_closure: Closure<dyn Fn()> = Closure::new(move || {
            owner_doc_for_timeout
                .add_event_listener_with_callback("pointerdown", &closure_ref_js)
                .expect("Pointer down event listener should be added.");
        });
        let window = web_sys::window().expect("Window should exist.");
        let timer_id = window
            .set_timeout_with_callback(timeout_closure.as_ref().unchecked_ref())
            .expect("setTimeout should succeed.");

        // Store the closure so we can remove the listener on cleanup
        handle_pointer_down.set_value(Some(SendWrapper::new(closure)));

        // Prevent timeout_closure from being dropped while the timer is pending
        // by leaking it (it will only fire once).
        timeout_closure.forget();

        // Store timer_id for cleanup
        // We can't easily return from Effect, so we use on_cleanup below
        let _ = timer_id;
    });

    on_cleanup(move || {
        let window = web_sys::window().expect("Window should exist.");
        // Note: we can't easily clear the timer here since we don't have the ID stored accessibly.
        // But the handler will be removed, so even if the timeout fires, the listener add will
        // reference a removed closure.

        handle_pointer_down.with_value(|closure_opt| {
            if let Some(closure) = closure_opt.as_ref() {
                cleanup_owner_doc
                    .remove_event_listener_with_callback(
                        "pointerdown",
                        (**closure).as_ref().unchecked_ref(),
                    )
                    .ok();
            }
        });

        handle_click_ref.with_value(|prev| {
            if let Some(prev_closure) = prev.as_ref() {
                cleanup_owner_doc
                    .remove_event_listener_with_callback(
                        "click",
                        prev_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            }
        });

        let _ = window;
    });

    PointerDownOutsideReturn {
        on_pointer_down_capture: Box::new(move || {
            is_pointer_inside_tree.set_value(true);
        }),
    }
}

// -- use_focus_outside ------------------------------------------------------

struct FocusOutsideReturn {
    on_focus_capture: Box<dyn Fn()>,
    on_blur_capture: Box<dyn Fn()>,
}

fn use_focus_outside(
    on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    owner_document: web_sys::Document,
) -> FocusOutsideReturn {
    let is_focus_inside_tree = StoredValue::new(false);

    let owner_doc = SendWrapper::new(owner_document.clone());
    let owner_doc_for_cleanup = SendWrapper::new(owner_document);

    type FocusInClosure = Closure<dyn Fn(web_sys::FocusEvent)>;
    let handle_focus: StoredValue<Option<SendWrapper<FocusInClosure>>> = StoredValue::new(None);

    let setup_owner_doc = owner_doc.clone();

    Effect::new(move |_| {
        let closure: FocusInClosure = Closure::new(move |event: web_sys::FocusEvent| {
            let has_target = event.target().is_some();
            let is_inside = is_focus_inside_tree.get_value();

            if has_target && !is_inside {
                handle_and_dispatch_custom_event(FOCUS_OUTSIDE, on_focus_outside, &event);
            }
        });

        setup_owner_doc
            .add_event_listener_with_callback("focusin", closure.as_ref().unchecked_ref())
            .expect("Focusin event listener should be added.");

        handle_focus.set_value(Some(SendWrapper::new(closure)));
    });

    on_cleanup(move || {
        handle_focus.with_value(|closure_opt| {
            if let Some(closure) = closure_opt.as_ref() {
                owner_doc_for_cleanup
                    .remove_event_listener_with_callback(
                        "focusin",
                        (**closure).as_ref().unchecked_ref(),
                    )
                    .ok();
            }
        });
    });

    FocusOutsideReturn {
        on_focus_capture: Box::new(move || {
            is_focus_inside_tree.set_value(true);
        }),
        on_blur_capture: Box::new(move || {
            is_focus_inside_tree.set_value(false);
        }),
    }
}

// -- handle_and_dispatch_custom_event ---------------------------------------

fn handle_and_dispatch_custom_event<E: AsRef<web_sys::Event>>(
    name: &str,
    handler: Option<Callback<CustomEvent>>,
    original_event: &E,
) {
    let original_event = original_event.as_ref();
    let target = match original_event.target() {
        Some(t) => t,
        None => return,
    };

    let init = CustomEventInit::new();
    init.set_bubbles(false);
    init.set_cancelable(true);
    init.set_detail(&JsValue::from(original_event));

    let event = CustomEvent::new_with_event_init_dict(name, &init)
        .expect("CustomEvent should be instantiated.");

    if let Some(handler) = handler {
        // Register one-shot listener, then dispatch
        let handler_closure: Closure<dyn Fn(CustomEvent)> =
            Closure::new(move |evt: CustomEvent| {
                handler.run(evt);
            });

        let options = AddEventListenerOptions::new();
        options.set_once(true);

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                name,
                handler_closure.as_ref().unchecked_ref(),
                &options,
            )
            .expect("Event handler should be added.");

        target.dispatch_event(&event).ok();

        // Remove just in case it wasn't called (shouldn't happen with once: true, but be safe)
        target
            .remove_event_listener_with_callback(name, handler_closure.as_ref().unchecked_ref())
            .ok();
    } else {
        target.dispatch_event(&event).ok();
    }
}

type StoredCleanupFn = StoredValue<SendWrapper<std::cell::RefCell<Option<Box<dyn Fn()>>>>>;

fn new_stored_cleanup() -> StoredCleanupFn {
    StoredValue::new(SendWrapper::new(std::cell::RefCell::new(None)))
}

// -- DismissableLayer component ---------------------------------------------

#[component]
pub fn DismissableLayer(
    /// When `true`, hover/focus/click interactions will be disabled on elements outside
    /// the `DismissableLayer`. Users will need to click twice on outside elements to
    /// interact with them: once to close the `DismissableLayer`, and again to trigger the element.
    #[prop(into, optional)]
    disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<PointerDownOutsideEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<FocusOutsideEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<CustomEvent>>,
    #[prop(into, optional)] on_dismiss: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let disable_outside_pointer_events =
        Signal::derive(move || disable_outside_pointer_events.get().unwrap_or(false));

    let container_ref: AnyNodeRef = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, container_ref]);

    // Force re-render signal, bumped whenever the global context updates
    let force_update = RwSignal::new(0u64);

    // Listen for context update events
    let update_closure: SendWrapper<Closure<dyn Fn(web_sys::Event)>> =
        SendWrapper::new(Closure::new(move |_event: web_sys::Event| {
            force_update.update(|v| *v += 1);
        }));

    // Add update listener on mount
    document()
        .add_event_listener_with_callback(CONTEXT_UPDATE, update_closure.as_ref().unchecked_ref())
        .expect("Context update event listener should be added.");

    on_cleanup({
        let update_closure = update_closure;
        move || {
            document()
                .remove_event_listener_with_callback(
                    CONTEXT_UPDATE,
                    update_closure.as_ref().unchecked_ref(),
                )
                .ok();
        }
    });

    let owner_document = document();

    // Derived computations that depend on force_update and container_ref
    let is_body_pointer_events_disabled = Signal::derive(move || {
        let _ = force_update.get();
        let ctx = DISMISSABLE_LAYER_CONTEXT
            .lock()
            .expect("Context mutex should lock.");
        ctx.layers_with_outside_pointer_events_disabled_count() > 0
    });

    let is_pointer_events_enabled = Signal::derive(move || {
        let _ = force_update.get();
        let ctx = DISMISSABLE_LAYER_CONTEXT
            .lock()
            .expect("Context mutex should lock.");

        let highest_idx = ctx
            .highest_layer_with_outside_pointer_events_disabled_index()
            .map(|i| i as i32)
            .unwrap_or(-1);

        let index = container_ref
            .get()
            .and_then(|node| {
                let node: web_sys::HtmlElement = node.unchecked_into();
                ctx.layer_index(&node).map(|i| i as i32)
            })
            .unwrap_or(-1);

        index >= highest_idx
    });

    // Pointer down outside detection
    let pointer_down_outside = use_pointer_down_outside(
        Some(Callback::new(move |event: PointerDownOutsideEvent| {
            let _ = force_update.get_untracked();

            let target = event
                .detail()
                .dyn_into::<web_sys::Event>()
                .ok()
                .and_then(|e| e.target())
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok());

            let node = container_ref
                .get_untracked()
                .map(|n| -> web_sys::HtmlElement { n.unchecked_into() });

            let (is_pointer_down_on_branch, is_pointer_down_in_higher_layer) = target
                .as_ref()
                .map(|t| {
                    let ctx = DISMISSABLE_LAYER_CONTEXT
                        .lock()
                        .expect("Context mutex should lock.");
                    let in_branch = ctx.branches_contain(t);
                    let in_higher_layer = node
                        .as_ref()
                        .map(|n| ctx.higher_layers_contain(n, t))
                        .unwrap_or(false);
                    (in_branch, in_higher_layer)
                })
                .unwrap_or((false, false));

            if !is_pointer_events_enabled.get_untracked()
                || is_pointer_down_on_branch
                || is_pointer_down_in_higher_layer
            {
                return;
            }

            if let Some(handler) = on_pointer_down_outside {
                handler.run(event.clone());
            }
            if let Some(handler) = on_interact_outside {
                handler.run(event.clone());
            }
            if !event.default_prevented()
                && let Some(on_dismiss) = on_dismiss
            {
                on_dismiss.run(());
            }
        })),
        owner_document.clone(),
    );

    // Focus outside detection
    let focus_outside = use_focus_outside(
        Some(Callback::new(move |event: FocusOutsideEvent| {
            let target = event
                .detail()
                .dyn_into::<web_sys::Event>()
                .ok()
                .and_then(|e| e.target())
                .and_then(|t| t.dyn_into::<web_sys::Node>().ok());

            let node = container_ref
                .get_untracked()
                .map(|n| -> web_sys::HtmlElement { n.unchecked_into() });

            let (is_focus_in_branch, is_focus_in_higher_layer) = target
                .as_ref()
                .map(|t| {
                    let ctx = DISMISSABLE_LAYER_CONTEXT
                        .lock()
                        .expect("Context mutex should lock.");
                    let in_branch = ctx.branches_contain(t);
                    let in_higher_layer = node
                        .as_ref()
                        .map(|n| ctx.higher_layers_contain(n, t))
                        .unwrap_or(false);
                    (in_branch, in_higher_layer)
                })
                .unwrap_or((false, false));

            if is_focus_in_branch || is_focus_in_higher_layer {
                return;
            }

            if let Some(handler) = on_focus_outside {
                handler.run(event.clone());
            }
            if let Some(handler) = on_interact_outside {
                handler.run(event.clone());
            }
            if !event.default_prevented()
                && let Some(on_dismiss) = on_dismiss
            {
                on_dismiss.run(());
            }
        })),
        owner_document.clone(),
    );

    // Escape key handling
    use_escape_keydown(
        Some(Callback::new(move |event: web_sys::KeyboardEvent| {
            let _ = force_update.get_untracked();

            let is_highest_layer = container_ref.get_untracked().is_some_and(|node| {
                let node: web_sys::HtmlElement = node.unchecked_into();
                let ctx = DISMISSABLE_LAYER_CONTEXT
                    .lock()
                    .expect("Context mutex should lock.");
                ctx.layer_index(&node)
                    .map(|idx| idx == ctx.layers_count() - 1)
                    .unwrap_or(false)
            });

            if !is_highest_layer {
                return;
            }

            // If a text-editing element inside this layer has focus, move focus to
            // the container instead of dismissing. This provides the expected
            // "two escapes" UX: first Escape leaves the input, second dismisses.
            if let Some(active) = document().active_element()
                && is_text_input(&active)
                && let Some(container) = container_ref.get_untracked()
            {
                let container_node: &web_sys::Node = container.unchecked_ref();
                if container_node.contains(Some(&active)) {
                    let container_el: &web_sys::HtmlElement = container.unchecked_ref();
                    if container_el.get_attribute("tabindex").is_none() {
                        container_el.set_attribute("tabindex", "-1").ok();
                    }
                    container_el.focus().ok();
                    return;
                }
            }

            if let Some(handler) = on_escape_key_down {
                handler.run(event.clone());
            }
            if !event.default_prevented()
                && let Some(on_dismiss) = on_dismiss
            {
                event.prevent_default();
                on_dismiss.run(());
            }
        })),
        Some(owner_document.clone()),
    );

    // Effect 1: Register layer in context, manage body pointer-events
    let owner_doc_for_effect = SendWrapper::new(owner_document.clone());
    let layer_effect_cleanup: StoredCleanupFn = new_stored_cleanup();

    Effect::new(move |_| {
        // Clean up previous run
        layer_effect_cleanup.with_value(|f| {
            if let Some(cleanup) = f.borrow_mut().take() {
                cleanup();
            }
        });

        let disable = disable_outside_pointer_events.get();

        if let Some(node) = container_ref.get() {
            let node: web_sys::HtmlElement = node.unchecked_into();
            let owner_doc = owner_doc_for_effect.clone();

            {
                let mut ctx = DISMISSABLE_LAYER_CONTEXT
                    .lock()
                    .expect("Context mutex should lock.");

                if disable {
                    if ctx.layers_with_outside_pointer_events_disabled_count() == 0 {
                        let mut orig = ORIGINAL_BODY_POINTER_EVENTS
                            .lock()
                            .expect("Original body pointer events mutex should lock.");
                        *orig = owner_doc
                            .body()
                            .map(|b| {
                                b.style()
                                    .get_property_value("pointer-events")
                                    .unwrap_or_default()
                            })
                            .unwrap_or_default();
                        if let Some(body) = owner_doc.body() {
                            body.style()
                                .set_property("pointer-events", "none")
                                .expect("Body pointer-events should be set.");
                        }
                    }
                    ctx.add_layer_with_outside_pointer_events_disabled(&node);
                }
                ctx.add_layer(&node);
            }

            dispatch_update();

            let cleanup_doc = owner_doc.clone();
            layer_effect_cleanup.with_value(|f| {
                f.borrow_mut().replace(Box::new(move || {
                    if disable {
                        let ctx = DISMISSABLE_LAYER_CONTEXT
                            .lock()
                            .expect("Context mutex should lock.");
                        if ctx.layers_with_outside_pointer_events_disabled_count() == 1 {
                            let orig = ORIGINAL_BODY_POINTER_EVENTS
                                .lock()
                                .expect("Original body pointer events mutex should lock.");
                            if let Some(body) = cleanup_doc.body() {
                                body.style()
                                    .set_property("pointer-events", &orig)
                                    .expect("Body pointer-events should be restored.");
                            }
                        }
                    }
                }));
            });
        }
    });

    // Effect 2: Cleanup-only — removes from all context sets on unmount.
    // Kept separate so that changes to disable_outside_pointer_events don't
    // remove and re-add the layer (which would change stacking order).
    let unmount_cleanup: StoredCleanupFn = new_stored_cleanup();

    Effect::new(move |_| {
        if let Some(node) = container_ref.get() {
            let node: web_sys::HtmlElement = node.unchecked_into();
            unmount_cleanup.with_value(|f| {
                f.borrow_mut().replace(Box::new(move || {
                    let mut ctx = DISMISSABLE_LAYER_CONTEXT
                        .lock()
                        .expect("Context mutex should lock.");
                    ctx.remove_layer(&node);
                    ctx.remove_layer_with_outside_pointer_events_disabled(&node);
                    dispatch_update();
                }));
            });
        }
    });

    on_cleanup(move || {
        layer_effect_cleanup.with_value(|f| {
            if let Some(cleanup) = f.borrow_mut().take() {
                cleanup();
            }
        });
        unmount_cleanup.with_value(|f| {
            if let Some(cleanup) = f.borrow_mut().take() {
                cleanup();
            }
        });
    });

    // Capture-phase event handlers: we need to attach them manually because
    // Leptos `on:` bindings don't support capture phase.
    type EventClosure = SendWrapper<Closure<dyn Fn(web_sys::Event)>>;
    type CaptureClosures =
        StoredValue<SendWrapper<std::cell::RefCell<Vec<(&'static str, EventClosure)>>>>;
    let capture_closures: CaptureClosures =
        StoredValue::new(SendWrapper::new(std::cell::RefCell::new(Vec::new())));

    let pointer_down_capture: StoredValue<SendWrapper<Box<dyn Fn()>>> = StoredValue::new(
        SendWrapper::new(pointer_down_outside.on_pointer_down_capture),
    );
    let focus_capture: StoredValue<SendWrapper<Box<dyn Fn()>>> =
        StoredValue::new(SendWrapper::new(focus_outside.on_focus_capture));
    let blur_capture: StoredValue<SendWrapper<Box<dyn Fn()>>> =
        StoredValue::new(SendWrapper::new(focus_outside.on_blur_capture));

    Effect::new(move |_| {
        // Clean up previous capture listeners
        capture_closures.with_value(|closures| {
            closures.borrow_mut().clear();
        });

        if let Some(node) = container_ref.get() {
            let node: web_sys::EventTarget = node.unchecked_into();
            let mut new_closures: Vec<(&'static str, EventClosure)> = Vec::new();

            // pointerdown capture
            let pdc_closure: Closure<dyn Fn(web_sys::Event)> =
                Closure::new(move |_event: web_sys::Event| {
                    pointer_down_capture.with_value(|f| f());
                });
            let options = AddEventListenerOptions::new();
            options.set_capture(true);
            node.add_event_listener_with_callback_and_add_event_listener_options(
                "pointerdown",
                pdc_closure.as_ref().unchecked_ref(),
                &options,
            )
            .expect("Pointer down capture listener should be added.");
            new_closures.push(("pointerdown", SendWrapper::new(pdc_closure)));

            // focus capture
            let fc_closure: Closure<dyn Fn(web_sys::Event)> =
                Closure::new(move |_event: web_sys::Event| {
                    focus_capture.with_value(|f| f());
                });
            let options = AddEventListenerOptions::new();
            options.set_capture(true);
            node.add_event_listener_with_callback_and_add_event_listener_options(
                "focusin",
                fc_closure.as_ref().unchecked_ref(),
                &options,
            )
            .expect("Focus capture listener should be added.");
            new_closures.push(("focusin", SendWrapper::new(fc_closure)));

            // blur capture
            let bc_closure: Closure<dyn Fn(web_sys::Event)> =
                Closure::new(move |_event: web_sys::Event| {
                    blur_capture.with_value(|f| f());
                });
            let options = AddEventListenerOptions::new();
            options.set_capture(true);
            node.add_event_listener_with_callback_and_add_event_listener_options(
                "blur",
                bc_closure.as_ref().unchecked_ref(),
                &options,
            )
            .expect("Blur capture listener should be added.");
            new_closures.push(("blur", SendWrapper::new(bc_closure)));

            capture_closures.with_value(|closures| {
                *closures.borrow_mut() = new_closures;
            });
        }
    });

    on_cleanup(move || {
        if let Some(node) = container_ref.get_untracked() {
            let node: web_sys::EventTarget = node.unchecked_into();
            capture_closures.with_value(|closures| {
                let closures = closures.borrow();
                for (event_name, closure) in closures.iter() {
                    let options = web_sys::EventListenerOptions::new();
                    options.set_capture(true);
                    node.remove_event_listener_with_callback_and_event_listener_options(
                        event_name,
                        closure.as_ref().unchecked_ref(),
                        &options,
                    )
                    .ok();
                }
            });
        }
    });

    let pointer_events_style = Memo::new(move |_| {
        let is_disabled = is_body_pointer_events_disabled.get();
        let is_enabled = is_pointer_events_enabled.get();
        if is_disabled {
            if is_enabled {
                "auto".to_string()
            } else {
                "none".to_string()
            }
        } else {
            String::new()
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_refs
                style:pointer-events=move || pointer_events_style.get()
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

// -- DismissableLayerBranch component ---------------------------------------

#[component]
pub fn DismissableLayerBranch(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let container_ref: AnyNodeRef = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, container_ref]);

    Effect::new(move |_| {
        if let Some(node) = container_ref.get() {
            let node: web_sys::HtmlElement = node.unchecked_into();
            let mut ctx = DISMISSABLE_LAYER_CONTEXT
                .lock()
                .expect("Context mutex should lock.");
            ctx.add_branch(&node);
        }
    });

    on_cleanup(move || {
        if let Some(node) = container_ref.get_untracked() {
            let node: web_sys::HtmlElement = node.unchecked_into();
            let mut ctx = DISMISSABLE_LAYER_CONTEXT
                .lock()
                .expect("Context mutex should lock.");
            ctx.remove_branch(&node);
        }
    });

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_refs
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

/// Returns `true` if the element is a text-editing element (text input, textarea, or
/// contenteditable). Used to implement the "two escapes" UX pattern: when such an element
/// has focus inside a dismissable layer, the first Escape moves focus to the container
/// and the second Escape dismisses the layer.
fn is_text_input(element: &web_sys::Element) -> bool {
    let tag = element.tag_name();
    if tag.eq_ignore_ascii_case("TEXTAREA") {
        return true;
    }
    if tag.eq_ignore_ascii_case("INPUT") {
        let input_type = element
            .get_attribute("type")
            .unwrap_or_else(|| "text".to_string())
            .to_ascii_lowercase();
        return matches!(
            input_type.as_str(),
            "text"
                | "email"
                | "number"
                | "password"
                | "search"
                | "tel"
                | "url"
                | "date"
                | "datetime-local"
                | "month"
                | "time"
                | "week"
        );
    }
    element
        .get_attribute("contenteditable")
        .is_some_and(|v| v == "true" || v.is_empty())
}
