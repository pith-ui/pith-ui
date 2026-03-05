use std::marker::PhantomData;

use crate::collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot, use_collection,
};
use crate::compose_refs::use_composed_refs;
use crate::dismissable_layer::DismissableLayerBranch;
use crate::portal::Portal;
use crate::presence::Presence;
use crate::primitive::Primitive;
use crate::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::visually_hidden::VisuallyHidden;
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/* -------------------------------------------------------------------------------------------------
 * Constants
 * -----------------------------------------------------------------------------------------------*/

const VIEWPORT_DEFAULT_HOTKEY: &[&str] = &["F8"];
const VIEWPORT_PAUSE: &str = "toast.viewportPause";
const VIEWPORT_RESUME: &str = "toast.viewportResume";

const ITEM_DATA_PHANTOM: PhantomData<()> = PhantomData;

type GetCollectionItems = StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<()>>>>>;

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl SwipeDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            SwipeDirection::Up => "up",
            SwipeDirection::Down => "down",
            SwipeDirection::Left => "left",
            SwipeDirection::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ToastType {
    #[default]
    Foreground,
    Background,
}

#[derive(Clone, Debug)]
pub struct SwipeEvent {
    pub current_target: Option<SwipeEventTarget>,
    pub delta: (f64, f64),
}

#[derive(Clone, Debug)]
pub struct SwipeEventTarget(SendWrapper<web_sys::HtmlElement>);

impl SwipeEventTarget {
    fn new(el: web_sys::HtmlElement) -> Self {
        Self(SendWrapper::new(el))
    }

    fn set_attribute(&self, name: &str, value: &str) -> Result<(), wasm_bindgen::JsValue> {
        self.0.set_attribute(name, value)
    }

    fn style(&self) -> web_sys::CssStyleDeclaration {
        self.0.style()
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastProvider
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ToastProviderContextValue {
    // Used by ToastAnnounce for screen reader announcements.
    #[allow(dead_code)]
    label: StoredValue<String>,
    duration: Signal<i32>,
    swipe_direction: Signal<SwipeDirection>,
    swipe_threshold: Signal<f64>,
    toast_count: ReadSignal<i32>,
    set_toast_count: WriteSignal<i32>,
    viewport: ReadSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_viewport_change: WriteSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    is_focused_toast_escape_key_down_ref: StoredValue<bool>,
    is_close_paused_ref: StoredValue<bool>,
}

#[component]
pub fn ToastProvider(
    /// An author-localized label for each toast. Used to help screen reader users
    /// associate the interruption with a toast.
    #[prop(into, optional, default = "Notification".to_string())]
    label: String,
    /// Time in milliseconds that each toast should remain visible for.
    #[prop(into, optional, default = 5000.into())]
    duration: Signal<i32>,
    /// Direction of pointer swipe that should close the toast.
    #[prop(into, optional, default = Signal::derive(|| SwipeDirection::Right))]
    swipe_direction: Signal<SwipeDirection>,
    /// Distance in pixels that the swipe must pass before a close is triggered.
    #[prop(into, optional, default = 50.0.into())]
    swipe_threshold: Signal<f64>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (viewport, on_viewport_change) = signal(None::<SendWrapper<web_sys::HtmlElement>>);
    let (toast_count, set_toast_count) = signal(0i32);

    let context_value = ToastProviderContextValue {
        label: StoredValue::new(label),
        duration,
        swipe_direction,
        swipe_threshold,
        toast_count,
        set_toast_count,
        viewport,
        on_viewport_change,
        is_focused_toast_escape_key_down_ref: StoredValue::new(false),
        is_close_paused_ref: StoredValue::new(false),
    };

    view! {
        <Provider value=context_value>
            <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                {children.with_value(|children| children())}
            </CollectionProvider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastViewport(
    /// The keys to use as the keyboard shortcut that will move focus to the toast viewport.
    #[prop(into, optional)]
    hotkey: Option<Vec<String>>,
    /// An author-localized label for the toast viewport to provide context for screen reader users
    /// when navigating page landmarks. The available `{hotkey}` placeholder will be replaced for you.
    #[prop(into, optional)]
    label: Option<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ToastProviderContextValue>();
    let get_items = StoredValue::new(use_collection::<()>());

    let hotkey: Vec<String> = hotkey.unwrap_or_else(|| {
        VIEWPORT_DEFAULT_HOTKEY
            .iter()
            .map(|s| s.to_string())
            .collect()
    });
    let label = label.unwrap_or_else(|| "Notifications ({hotkey})".to_string());

    let hotkey_label = hotkey
        .iter()
        .map(|k| k.replace("Key", "").replace("Digit", ""))
        .collect::<Vec<_>>()
        .join("+");
    let aria_label = label.replace("{hotkey}", &hotkey_label);
    let aria_label = StoredValue::new(aria_label);

    let wrapper_ref = AnyNodeRef::new();
    let head_focus_proxy_ref = AnyNodeRef::new();
    let tail_focus_proxy_ref = AnyNodeRef::new();
    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, viewport_ref]);

    let has_toasts = Signal::derive(move || context.toast_count.get() > 0);

    // F8 hotkey to focus the viewport
    let hotkey_stored = StoredValue::new(hotkey);
    Effect::new(move |_| {
        let hotkey = hotkey_stored.get_value();
        if hotkey.is_empty() {
            return;
        }
        let callback: Closure<dyn Fn(web_sys::KeyboardEvent)> =
            Closure::new(move |event: web_sys::KeyboardEvent| {
                let hotkey = hotkey_stored.get_value();
                let is_hotkey_pressed = !hotkey.is_empty()
                    && hotkey.iter().all(|key| {
                        event.code() == *key
                            || (key == "altKey" && event.alt_key())
                            || (key == "ctrlKey" && event.ctrl_key())
                            || (key == "metaKey" && event.meta_key())
                            || (key == "shiftKey" && event.shift_key())
                    });
                if is_hotkey_pressed && let Some(el) = viewport_ref.get_untracked() {
                    let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                    let _ = el.focus();
                }
            });

        let _ = document()
            .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

        let callback_ref: SendWrapper<Closure<dyn Fn(web_sys::KeyboardEvent)>> =
            SendWrapper::new(callback);
        on_cleanup(move || {
            let _ = document().remove_event_listener_with_callback(
                "keydown",
                callback_ref.as_ref().unchecked_ref(),
            );
        });
    });

    // Register viewport element with provider
    Effect::new(move |_| {
        if let Some(el) = viewport_ref.get() {
            let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
            context.on_viewport_change.set(Some(SendWrapper::new(el)));
        }
    });

    // Pause/resume on hover/focus
    Effect::new(move |_| {
        let has_toasts = has_toasts.get();
        let wrapper_el = wrapper_ref.get();
        let viewport_el = viewport_ref.get();

        if !has_toasts {
            return;
        }
        let Some(wrapper) = wrapper_el else { return };
        let Some(viewport) = viewport_el else { return };

        let wrapper: web_sys::HtmlElement = (*wrapper).clone().unchecked_into();
        let viewport: web_sys::HtmlElement = (*viewport).clone().unchecked_into();

        let viewport_for_pause = SendWrapper::new(viewport.clone());
        let handle_pause: Closure<dyn Fn()> = Closure::new(move || {
            if !context.is_close_paused_ref.get_value() {
                let pause_event = web_sys::CustomEvent::new(VIEWPORT_PAUSE)
                    .expect("CustomEvent should be created");
                let _ = viewport_for_pause.dispatch_event(&pause_event);
                context.is_close_paused_ref.set_value(true);
            }
        });

        let viewport_for_resume = SendWrapper::new(viewport.clone());
        let _handle_resume: Closure<dyn Fn()> = Closure::new(move || {
            if context.is_close_paused_ref.get_value() {
                let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                    .expect("CustomEvent should be created");
                let _ = viewport_for_resume.dispatch_event(&resume_event);
                context.is_close_paused_ref.set_value(false);
            }
        });

        let wrapper_for_focusout = SendWrapper::new(wrapper.clone());
        let handle_focus_out_resume: Closure<dyn Fn(web_sys::FocusEvent)> =
            Closure::new(move |event: web_sys::FocusEvent| {
                let is_focus_moving_outside = event
                    .related_target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
                    .map(|node| !wrapper_for_focusout.contains(Some(&node)))
                    .unwrap_or(true);
                if is_focus_moving_outside
                    && context.is_close_paused_ref.get_value()
                    && let Some(viewport) = context.viewport.get_untracked()
                {
                    let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                        .expect("CustomEvent should be created");
                    let _ = viewport.dispatch_event(&resume_event);
                    context.is_close_paused_ref.set_value(false);
                }
            });

        let wrapper_for_pointerleave = SendWrapper::new(wrapper.clone());
        let handle_pointer_leave_resume: Closure<dyn Fn()> = Closure::new(move || {
            let is_focus_inside = document()
                .active_element()
                .map(|el| {
                    let node: &web_sys::Node = el.unchecked_ref();
                    wrapper_for_pointerleave.contains(Some(node))
                })
                .unwrap_or(false);
            if !is_focus_inside
                && context.is_close_paused_ref.get_value()
                && let Some(viewport) = context.viewport.get_untracked()
            {
                let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                    .expect("CustomEvent should be created");
                let _ = viewport.dispatch_event(&resume_event);
                context.is_close_paused_ref.set_value(false);
            }
        });

        let handle_window_blur: Closure<dyn Fn()> = Closure::new(move || {
            if !context.is_close_paused_ref.get_value()
                && let Some(viewport) = context.viewport.get_untracked()
            {
                let pause_event = web_sys::CustomEvent::new(VIEWPORT_PAUSE)
                    .expect("CustomEvent should be created");
                let _ = viewport.dispatch_event(&pause_event);
                context.is_close_paused_ref.set_value(true);
            }
        });

        let handle_window_focus: Closure<dyn Fn()> = Closure::new(move || {
            if context.is_close_paused_ref.get_value()
                && let Some(viewport) = context.viewport.get_untracked()
            {
                let resume_event = web_sys::CustomEvent::new(VIEWPORT_RESUME)
                    .expect("CustomEvent should be created");
                let _ = viewport.dispatch_event(&resume_event);
                context.is_close_paused_ref.set_value(false);
            }
        });

        let _ = wrapper
            .add_event_listener_with_callback("focusin", handle_pause.as_ref().unchecked_ref());
        let _ = wrapper.add_event_listener_with_callback(
            "focusout",
            handle_focus_out_resume.as_ref().unchecked_ref(),
        );
        let _ = wrapper
            .add_event_listener_with_callback("pointermove", handle_pause.as_ref().unchecked_ref());
        let _ = wrapper.add_event_listener_with_callback(
            "pointerleave",
            handle_pointer_leave_resume.as_ref().unchecked_ref(),
        );
        let window = web_sys::window().expect("Window should exist.");
        let _ = window
            .add_event_listener_with_callback("blur", handle_window_blur.as_ref().unchecked_ref());
        let _ = window.add_event_listener_with_callback(
            "focus",
            handle_window_focus.as_ref().unchecked_ref(),
        );

        let wrapper_cleanup = SendWrapper::new(wrapper);
        let handle_pause = SendWrapper::new(handle_pause);
        let handle_focus_out_resume = SendWrapper::new(handle_focus_out_resume);
        let handle_pointer_leave_resume = SendWrapper::new(handle_pointer_leave_resume);
        let handle_window_blur = SendWrapper::new(handle_window_blur);
        let handle_window_focus = SendWrapper::new(handle_window_focus);

        on_cleanup(move || {
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "focusin",
                handle_pause.as_ref().unchecked_ref(),
            );
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "focusout",
                handle_focus_out_resume.as_ref().unchecked_ref(),
            );
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "pointermove",
                handle_pause.as_ref().unchecked_ref(),
            );
            let _ = wrapper_cleanup.remove_event_listener_with_callback(
                "pointerleave",
                handle_pointer_leave_resume.as_ref().unchecked_ref(),
            );
            if let Some(window) = web_sys::window() {
                let _ = window.remove_event_listener_with_callback(
                    "blur",
                    handle_window_blur.as_ref().unchecked_ref(),
                );
                let _ = window.remove_event_listener_with_callback(
                    "focus",
                    handle_window_focus.as_ref().unchecked_ref(),
                );
            }
        });
    });

    // Custom tab management (reverse order, most-recent-first)

    Effect::new(move |_| {
        let Some(viewport) = viewport_ref.get() else {
            return;
        };
        let viewport: web_sys::HtmlElement = (*viewport).clone().unchecked_into();

        let callback: Closure<dyn Fn(web_sys::KeyboardEvent)> =
            Closure::new(move |event: web_sys::KeyboardEvent| {
                let is_meta_key = event.alt_key() || event.ctrl_key() || event.meta_key();
                let is_tab_key = event.key() == "Tab" && !is_meta_key;

                if !is_tab_key {
                    return;
                }

                let focused_element = document().active_element();
                let is_tabbing_backwards = event.shift_key();

                let target = event.target();
                let target_is_viewport = target
                    .as_ref()
                    .and_then(|t| t.dyn_ref::<web_sys::HtmlElement>())
                    .map(|t| {
                        viewport_ref
                            .get_untracked()
                            .map(|v| {
                                let v: &web_sys::HtmlElement = (*v).unchecked_ref();
                                t == v
                            })
                            .unwrap_or(false)
                    })
                    .unwrap_or(false);

                if target_is_viewport && is_tabbing_backwards {
                    if let Some(el) = head_focus_proxy_ref.get_untracked() {
                        let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        let _ = el.focus();
                    }
                    return;
                }

                let tabbing_direction = if is_tabbing_backwards {
                    "backwards"
                } else {
                    "forwards"
                };
                let sorted_candidates = compute_sorted_tabbable(get_items, tabbing_direction);
                let index = focused_element.and_then(|focused| {
                    sorted_candidates.iter().position(|candidate| {
                        let c: &web_sys::HtmlElement = candidate;
                        let f: &web_sys::Element = &focused;
                        c.unchecked_ref::<web_sys::Element>() == f
                    })
                });

                // Match React behavior: when focused element is not found
                // (JS findIndex returns -1, slice(-1+1) = slice(0) = full array),
                // try the full candidate list instead of skipping.
                let start = index.map(|i| i + 1).unwrap_or(0);
                let remaining = &sorted_candidates[start..];
                if focus_first_html(remaining) {
                    event.prevent_default();
                } else if is_tabbing_backwards {
                    if let Some(el) = head_focus_proxy_ref.get_untracked() {
                        let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        let _ = el.focus();
                    }
                } else if let Some(el) = tail_focus_proxy_ref.get_untracked() {
                    let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                    let _ = el.focus();
                }
            });

        let _ =
            viewport.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

        let viewport_cleanup = SendWrapper::new(viewport);
        let callback = SendWrapper::new(callback);
        on_cleanup(move || {
            let _ = viewport_cleanup
                .remove_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());
        });
    });

    view! {
        <DismissableLayerBranch
            node_ref=wrapper_ref
            attr:role="region"
            attr:aria-label=aria_label.get_value()
            attr:tabindex="-1"
            style:pointer-events=move || {
                if has_toasts.get() { None } else { Some("none") }
            }
        >
            <Show when=move || has_toasts.get()>
                <FocusProxy
                    node_ref=head_focus_proxy_ref
                    on_focus_from_outside_viewport=Callback::new(move |_: ()| {
                        let candidates = compute_sorted_tabbable(get_items, "forwards");
                        focus_first_html(&candidates);
                    })
                />
            </Show>

            <CollectionSlot item_data_type=ITEM_DATA_PHANTOM node_ref=composed_refs>
                <Primitive
                    element=html::ol
                    as_child=as_child
                    node_ref=composed_refs
                    attr:tabindex="-1"
                >
                    {children.with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </CollectionSlot>

            <Show when=move || has_toasts.get()>
                <FocusProxy
                    node_ref=tail_focus_proxy_ref
                    on_focus_from_outside_viewport=Callback::new(move |_: ()| {
                        let candidates = compute_sorted_tabbable(get_items, "backwards");
                        focus_first_html(&candidates);
                    })
                />
            </Show>
        </DismissableLayerBranch>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FocusProxy
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FocusProxy(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    on_focus_from_outside_viewport: Callback<()>,
) -> impl IntoView {
    let context = expect_context::<ToastProviderContextValue>();

    view! {
        <VisuallyHidden
            node_ref=node_ref
            attr:tabindex="0"
            style:position="fixed"
            on:focus=move |event: web_sys::FocusEvent| {
                let prev_focused_element = event
                    .related_target()
                    .and_then(|t| t.dyn_into::<web_sys::Node>().ok());
                let is_focus_from_outside_viewport = context
                    .viewport
                    .get_untracked()
                    .map(|viewport| {
                        let viewport_el: &web_sys::HtmlElement = &viewport;
                        !viewport_el.contains(prev_focused_element.as_ref())
                    })
                    .unwrap_or(true);
                if is_focus_from_outside_viewport {
                    on_focus_from_outside_viewport.run(());
                }
            }
        >
            {""}
        </VisuallyHidden>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Toast (Root) — merged with ToastImpl
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct ToastInteractiveContextValue {
    on_close: Callback<()>,
}

#[component]
pub fn Toast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    /// Used to force mounting when more control is needed. Useful when
    /// controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: Option<bool>,
    /// The type of toast. `Foreground` toasts are announced as `assertive`,
    /// `Background` toasts as `polite`.
    #[prop(into, optional)]
    r#type: MaybeProp<ToastType>,
    /// Time in milliseconds that toast should remain visible for. Overrides value
    /// given to `ToastProvider`.
    #[prop(into, optional)]
    duration: MaybeProp<i32>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pause: Option<Callback<()>>,
    #[prop(into, optional)] on_resume: Option<Callback<()>>,
    #[prop(into, optional)] on_swipe_start: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_move: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_cancel: Option<Callback<SwipeEvent>>,
    #[prop(into, optional)] on_swipe_end: Option<Callback<SwipeEvent>>,
    /// Explicit class forwarding — `attr:class` on `<Toast>` cannot cross the Portal
    /// boundary (Portal uses `mount_to` which creates a separate rendering context).
    /// Use `attr:class` as usual; this prop is for internal forwarding to the `<li>`.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: match default_open.get_untracked() {
            Some(_) => default_open,
            None => MaybeProp::from(Some(true)),
        },
        on_change: on_open_change.map(|on_open_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_open_change.run(value);
                }
            })
        }),
    });
    let is_open = Signal::derive(move || open_signal.get().unwrap_or(true));

    let force_mount = force_mount.unwrap_or(false);
    let toast_type = Signal::derive(move || r#type.get().unwrap_or_default());

    let context = expect_context::<ToastProviderContextValue>();
    let toast_node_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, toast_node_ref]);
    let pointer_start_ref: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);
    let swipe_delta_ref: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);
    let duration_val =
        Signal::derive(move || duration.get().unwrap_or_else(|| context.duration.get()));
    let close_timer_start_time_ref: StoredValue<f64> = StoredValue::new(0.0);
    let close_timer_remaining_time_ref: StoredValue<f64> =
        StoredValue::new(duration_val.get_untracked() as f64);
    let close_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let handle_close = Callback::new(move |_: ()| {
        let is_focus_in_toast = toast_node_ref
            .get_untracked()
            .and_then(|node| {
                let node: &web_sys::HtmlElement = (*node).unchecked_ref();
                document()
                    .active_element()
                    .map(|active| node.contains(Some(active.unchecked_ref())))
            })
            .unwrap_or(false);

        if is_focus_in_toast && let Some(viewport) = context.viewport.get_untracked() {
            let _ = viewport.focus();
        }
        set_open.run(Some(false));
    });

    let start_timer = Callback::new(move |dur: f64| {
        if dur <= 0.0 || dur == f64::INFINITY {
            return;
        }
        clear_timeout(close_timer_ref);
        close_timer_start_time_ref.set_value(js_sys::Date::now());
        let timeout_id = set_timeout(
            move || {
                handle_close.run(());
            },
            dur as i32,
        );
        close_timer_ref.set_value(Some(timeout_id));
    });

    // Listen for viewport pause/resume events
    Effect::new(move |_| {
        let Some(viewport) = context.viewport.get() else {
            return;
        };
        let viewport_el: web_sys::HtmlElement = (*viewport).clone();

        let handle_resume_fn: Closure<dyn Fn()> = Closure::new(move || {
            start_timer.run(close_timer_remaining_time_ref.get_value());
            if let Some(cb) = on_resume {
                cb.run(());
            }
        });

        let handle_pause_fn: Closure<dyn Fn()> = Closure::new(move || {
            let elapsed_time = js_sys::Date::now() - close_timer_start_time_ref.get_value();
            close_timer_remaining_time_ref
                .set_value(close_timer_remaining_time_ref.get_value() - elapsed_time);
            clear_timeout(close_timer_ref);
            if let Some(cb) = on_pause {
                cb.run(());
            }
        });

        let _ = viewport_el.add_event_listener_with_callback(
            VIEWPORT_PAUSE,
            handle_pause_fn.as_ref().unchecked_ref(),
        );
        let _ = viewport_el.add_event_listener_with_callback(
            VIEWPORT_RESUME,
            handle_resume_fn.as_ref().unchecked_ref(),
        );

        let viewport_cleanup = SendWrapper::new(viewport_el);
        let handle_pause_fn = SendWrapper::new(handle_pause_fn);
        let handle_resume_fn = SendWrapper::new(handle_resume_fn);
        on_cleanup(move || {
            let _ = viewport_cleanup.remove_event_listener_with_callback(
                VIEWPORT_PAUSE,
                handle_pause_fn.as_ref().unchecked_ref(),
            );
            let _ = viewport_cleanup.remove_event_listener_with_callback(
                VIEWPORT_RESUME,
                handle_resume_fn.as_ref().unchecked_ref(),
            );
        });
    });

    // Start timer when toast opens or duration changes
    Effect::new(move |_| {
        let is_open_val = is_open.get();
        let dur = duration_val.get() as f64;
        if is_open_val && !context.is_close_paused_ref.get_value() {
            close_timer_remaining_time_ref.set_value(dur);
            start_timer.run(dur);
        }
    });

    // Track toast count
    Effect::new(move |_| {
        context.set_toast_count.update(|c| *c += 1);
        on_cleanup(move || {
            context.set_toast_count.update(|c| *c -= 1);
        });
    });

    on_cleanup(move || {
        clear_timeout(close_timer_ref);
    });

    // Announce text content
    let announce_text = RwSignal::new(Vec::<String>::new());
    Effect::new(move |_| {
        // Track open state to re-read text when content changes
        let _ = is_open.get();
        if let Some(node) = toast_node_ref.get() {
            let node: &web_sys::HtmlElement = (*node).unchecked_ref();
            announce_text.set(get_announce_text_content(node));
        }
    });

    let swipe_direction = context.swipe_direction;
    let swipe_threshold = context.swipe_threshold;

    let has_viewport = Signal::derive(move || context.viewport.get().is_some());

    // Compose swipe handlers with data-attribute / CSS variable management
    let on_swipe_start_composed = on_swipe_start;
    let on_swipe_move_composed = on_swipe_move;
    let on_swipe_cancel_composed = on_swipe_cancel;
    let on_swipe_end_composed = on_swipe_end;
    let on_escape_key_down_stored = on_escape_key_down;

    view! {
        <Presence present=Signal::derive(move || force_mount || is_open.get())>
            <Show when=move || has_viewport.get()>
                <Provider value=ToastInteractiveContextValue { on_close: handle_close }>
                    <Portal
                        as_child=true
                        container=Signal::derive(move || {
                            context.viewport.get().map(|v| {
                                let el: web_sys::Element = (*v).clone().unchecked_into();
                                SendWrapper::new(el)
                            })
                        })
                    >
                        <Provider value=context>
                            <Provider value=ToastInteractiveContextValue { on_close: handle_close }>
                                <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=()>
                                    <Primitive
                                        element=html::li
                                        as_child=as_child
                                        node_ref=composed_refs
                                        attr:class=move || class.get().unwrap_or_default()
                                        attr:role="status"
                                        attr:aria-live=move || {
                                            match toast_type.get() {
                                                ToastType::Foreground => "assertive",
                                                ToastType::Background => "polite",
                                            }
                                        }
                                        attr:aria-atomic="true"
                                        attr:tabindex="0"
                                        attr:data-state=move || if is_open.get() { "open" } else { "closed" }
                                        attr:data-swipe-direction=move || swipe_direction.get().as_str()
                                        style:user-select="none"
                                        style:touch-action="none"
                                        on:keydown=move |event: web_sys::KeyboardEvent| {
                                            if event.key() != "Escape" {
                                                return;
                                            }
                                            if let Some(cb) = on_escape_key_down_stored {
                                                cb.run(event.clone());
                                            }
                                            if !event.default_prevented() {
                                                context.is_focused_toast_escape_key_down_ref.set_value(true);
                                                handle_close.run(());
                                            }
                                        }
                                        on:pointerdown=move |event: web_sys::PointerEvent| {
                                            if event.button() != 0 {
                                                return;
                                            }
                                            pointer_start_ref.set_value(Some((event.client_x() as f64, event.client_y() as f64)));
                                        }
                                        on:pointermove=move |event: web_sys::PointerEvent| {
                                            let Some(start) = pointer_start_ref.get_value() else {
                                                return;
                                            };
                                            let x = event.client_x() as f64 - start.0;
                                            let y = event.client_y() as f64 - start.1;
                                            let has_swipe_move_started = swipe_delta_ref.get_value().is_some();
                                            let direction = swipe_direction.get_untracked();
                                            let is_horizontal_swipe = matches!(direction, SwipeDirection::Left | SwipeDirection::Right);
                                            let clamp_fn: fn(f64, f64) -> f64 = match direction {
                                                SwipeDirection::Left | SwipeDirection::Up => f64::min,
                                                SwipeDirection::Right | SwipeDirection::Down => f64::max,
                                            };
                                            let clamped_x = if is_horizontal_swipe { clamp_fn(0.0, x) } else { 0.0 };
                                            let clamped_y = if !is_horizontal_swipe { clamp_fn(0.0, y) } else { 0.0 };
                                            let move_start_buffer = if event.pointer_type() == "touch" { 10.0 } else { 2.0 };
                                            let delta = (clamped_x, clamped_y);
                                            let current_target = event.current_target()
                                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                .map(SwipeEventTarget::new);

                                            if has_swipe_move_started {
                                                swipe_delta_ref.set_value(Some(delta));
                                                if let Some(ref ct) = current_target {
                                                    let _ = ct.set_attribute("data-swipe", "move");
                                                    let style = ct.style();
                                                    let _ = style.set_property("--radix-toast-swipe-move-x", &format!("{}px", delta.0));
                                                    let _ = style.set_property("--radix-toast-swipe-move-y", &format!("{}px", delta.1));
                                                }
                                                if let Some(cb) = on_swipe_move_composed {
                                                    cb.run(SwipeEvent { current_target, delta });
                                                }
                                            } else if is_delta_in_direction(delta, direction, move_start_buffer) {
                                                swipe_delta_ref.set_value(Some(delta));
                                                if let Some(ref ct) = current_target {
                                                    let _ = ct.set_attribute("data-swipe", "start");
                                                }
                                                if let Some(cb) = on_swipe_start_composed {
                                                    cb.run(SwipeEvent { current_target, delta });
                                                }
                                                let _ = event.target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                    .map(|t| t.set_pointer_capture(event.pointer_id()));
                                            } else if x.abs() > move_start_buffer || y.abs() > move_start_buffer {
                                                pointer_start_ref.set_value(None);
                                            }
                                        }
                                        on:pointerup=move |event: web_sys::PointerEvent| {
                                            let delta = swipe_delta_ref.get_value();
                                            if let Some(target) = event.target()
                                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                && target.has_pointer_capture(event.pointer_id()) {
                                                    let _ = target.release_pointer_capture(event.pointer_id());
                                                }
                                            swipe_delta_ref.set_value(None);
                                            pointer_start_ref.set_value(None);
                                            if let Some(delta) = delta {
                                                let direction = swipe_direction.get_untracked();
                                                let threshold = swipe_threshold.get_untracked();
                                                let current_target = event.current_target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                    .map(SwipeEventTarget::new);
                                                if is_delta_in_direction(delta, direction, threshold) {
                                                    if let Some(ref ct) = current_target {
                                                        let _ = ct.set_attribute("data-swipe", "end");
                                                        let style = ct.style();
                                                        let _ = style.remove_property("--radix-toast-swipe-move-x");
                                                        let _ = style.remove_property("--radix-toast-swipe-move-y");
                                                        let _ = style.set_property("--radix-toast-swipe-end-x", &format!("{}px", delta.0));
                                                        let _ = style.set_property("--radix-toast-swipe-end-y", &format!("{}px", delta.1));
                                                    }
                                                    if let Some(cb) = on_swipe_end_composed {
                                                        cb.run(SwipeEvent { current_target, delta });
                                                    }
                                                    set_open.run(Some(false));
                                                } else {
                                                    if let Some(ref ct) = current_target {
                                                        let _ = ct.set_attribute("data-swipe", "cancel");
                                                        let style = ct.style();
                                                        let _ = style.remove_property("--radix-toast-swipe-move-x");
                                                        let _ = style.remove_property("--radix-toast-swipe-move-y");
                                                        let _ = style.remove_property("--radix-toast-swipe-end-x");
                                                        let _ = style.remove_property("--radix-toast-swipe-end-y");
                                                    }
                                                    if let Some(cb) = on_swipe_cancel_composed {
                                                        cb.run(SwipeEvent { current_target, delta });
                                                    }
                                                }
                                                // Prevent click event from triggering on items within the toast
                                                if let Some(toast) = event.current_target()
                                                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
                                                {
                                                    let prevent_click: Closure<dyn FnMut(web_sys::Event)> =
                                                        Closure::once(move |event: web_sys::Event| {
                                                            event.prevent_default();
                                                        });
                                                    let mut options = web_sys::AddEventListenerOptions::new();
                                                    options.set_once(true);
                                                    let _ = toast.add_event_listener_with_callback_and_add_event_listener_options(
                                                        "click",
                                                        prevent_click.as_ref().unchecked_ref(),
                                                        &options,
                                                    );
                                                    prevent_click.forget();
                                                }
                                            }
                                        }
                                    >
                                        {children.with_value(|children| children())}
                                    </Primitive>
                                </CollectionItemSlot>
                            </Provider>
                        </Provider>
                    </Portal>
                </Provider>
            </Show>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastTitle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastTitle(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastDescription
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastDescription(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive element=html::div as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastAction
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastAction(
    /// A short description for an alternate way to carry out the action. For screen reader users
    /// who will not be able to navigate to the button easily/quickly.
    #[prop(into)]
    alt_text: String,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let alt_text = StoredValue::new(alt_text);

    view! {
        <ToastAnnounceExclude alt_text=alt_text.get_value()>
            <ToastClose node_ref=node_ref as_child=as_child>
                {children.with_value(|children| children())}
            </ToastClose>
        </ToastAnnounceExclude>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastClose
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToastClose(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let interactive_context = expect_context::<ToastInteractiveContextValue>();

    view! {
        <ToastAnnounceExclude>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                on:click=move |_: web_sys::MouseEvent| {
                    interactive_context.on_close.run(());
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </ToastAnnounceExclude>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToastAnnounceExclude
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ToastAnnounceExclude(
    #[prop(into, optional)] alt_text: Option<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let alt_text = StoredValue::new(alt_text);

    // React always uses asChild here — the data attributes are merged onto the child
    // element rather than rendering a wrapper <div>.
    view! {
        <Primitive
            element=html::div
            as_child=true
            attr:data-radix-toast-announce-exclude=""
            attr:data-radix-toast-announce-alt=move || alt_text.get_value()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utility functions
 * -----------------------------------------------------------------------------------------------*/

fn get_announce_text_content(container: &web_sys::HtmlElement) -> Vec<String> {
    let mut text_content = Vec::new();
    let child_nodes = container.child_nodes();

    for i in 0..child_nodes.length() {
        if let Some(node) = child_nodes.item(i) {
            if node.node_type() == web_sys::Node::TEXT_NODE {
                if let Some(text) = node.text_content()
                    && !text.is_empty()
                {
                    text_content.push(text);
                }
            } else if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
                let is_hidden = element
                    .get_attribute("aria-hidden")
                    .is_some_and(|v| v == "true")
                    || element.hidden()
                    || element
                        .style()
                        .get_property_value("display")
                        .ok()
                        .is_some_and(|v| v == "none");

                let is_excluded = element
                    .get_attribute("data-radix-toast-announce-exclude")
                    .is_some();

                if !is_hidden {
                    if is_excluded {
                        if let Some(alt_text) =
                            element.get_attribute("data-radix-toast-announce-alt")
                            && !alt_text.is_empty()
                        {
                            text_content.push(alt_text);
                        }
                    } else {
                        text_content.extend(get_announce_text_content(&element));
                    }
                }
            }
        }
    }

    text_content
}

fn compute_sorted_tabbable(
    get_items: GetCollectionItems,
    tabbing_direction: &str,
) -> Vec<SendWrapper<web_sys::HtmlElement>> {
    let toast_items = get_items.with_value(|f| f());
    let tabbable_candidates: Vec<Vec<SendWrapper<web_sys::HtmlElement>>> = toast_items
        .iter()
        .map(|toast_item| {
            let Some(node) = toast_item.r#ref.get_untracked() else {
                return vec![];
            };
            let toast_node: web_sys::HtmlElement = (*node).clone().unchecked_into();

            let mut candidates = vec![SendWrapper::new(toast_node.clone())];
            candidates.extend(
                get_tabbable_candidates(&toast_node)
                    .into_iter()
                    .map(SendWrapper::new),
            );

            if tabbing_direction == "backwards" {
                candidates.reverse();
            }
            candidates
        })
        .collect();

    let ordered: Vec<Vec<SendWrapper<web_sys::HtmlElement>>> = if tabbing_direction == "forwards" {
        tabbable_candidates.into_iter().rev().collect()
    } else {
        tabbable_candidates
    };

    ordered.into_iter().flatten().collect()
}

fn is_delta_in_direction(delta: (f64, f64), direction: SwipeDirection, threshold: f64) -> bool {
    let delta_x = delta.0.abs();
    let delta_y = delta.1.abs();
    let is_delta_x = delta_x > delta_y;
    match direction {
        SwipeDirection::Left | SwipeDirection::Right => is_delta_x && delta_x > threshold,
        SwipeDirection::Up | SwipeDirection::Down => !is_delta_x && delta_y > threshold,
    }
}

/// Returns a list of potential tabbable candidates.
fn get_tabbable_candidates(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes = Vec::new();

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>() {
                if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>()
                    && input.type_() == "hidden"
                {
                    return 3; // FILTER_SKIP
                }
                if html_element.hidden() {
                    return 3; // FILTER_SKIP
                }
                if html_element.tab_index() >= 0 {
                    return 1; // FILTER_ACCEPT
                }
            }
            3 // FILTER_SKIP
        });

    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    if let Ok(walker) = document().create_tree_walker_with_what_to_show_and_filter(
        container,
        0x1,
        Some(&node_filter),
    ) {
        while let Ok(Some(node)) = walker.next_node() {
            if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
                nodes.push(element);
            }
        }
    }

    drop(accept_node_closure);
    nodes
}

fn focus_first_html(candidates: &[SendWrapper<web_sys::HtmlElement>]) -> bool {
    let previously_focused = document().active_element();
    for candidate in candidates {
        let c: &web_sys::HtmlElement = candidate;
        if previously_focused
            .as_ref()
            .is_some_and(|f| c.unchecked_ref::<web_sys::Element>() == f)
        {
            return true;
        }
        let _ = c.focus();
        if document()
            .active_element()
            .is_some_and(|active| previously_focused.as_ref().is_none_or(|f| active != *f))
        {
            return true;
        }
    }
    false
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

fn set_timeout(f: impl Fn() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── SwipeDirection ──────────────────────────────────────

    #[test]
    fn swipe_direction_as_str() {
        assert_eq!(SwipeDirection::Up.as_str(), "up");
        assert_eq!(SwipeDirection::Down.as_str(), "down");
        assert_eq!(SwipeDirection::Left.as_str(), "left");
        assert_eq!(SwipeDirection::Right.as_str(), "right");
    }

    #[test]
    fn swipe_direction_default_is_right() {
        assert_eq!(SwipeDirection::default(), SwipeDirection::Right);
    }

    // ── is_delta_in_direction ───────────────────────────────

    #[test]
    fn horizontal_swipe_right_above_threshold() {
        assert!(is_delta_in_direction(
            (20.0, 5.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn horizontal_swipe_left_above_threshold() {
        // Negative x delta, but abs is used — direction variant only checks axis
        assert!(is_delta_in_direction(
            (-20.0, 5.0),
            SwipeDirection::Left,
            10.0
        ));
    }

    #[test]
    fn horizontal_swipe_below_threshold() {
        assert!(!is_delta_in_direction(
            (5.0, 2.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn horizontal_swipe_at_threshold_not_exceeded() {
        // delta_x == threshold, but we need > threshold
        assert!(!is_delta_in_direction(
            (10.0, 2.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn vertical_swipe_down_above_threshold() {
        assert!(is_delta_in_direction(
            (5.0, 20.0),
            SwipeDirection::Down,
            10.0
        ));
    }

    #[test]
    fn vertical_swipe_up_above_threshold() {
        assert!(is_delta_in_direction(
            (5.0, -20.0),
            SwipeDirection::Up,
            10.0
        ));
    }

    #[test]
    fn vertical_swipe_below_threshold() {
        assert!(!is_delta_in_direction((2.0, 5.0), SwipeDirection::Up, 10.0));
    }

    #[test]
    fn diagonal_equal_is_not_horizontal() {
        // delta_x == delta_y → is_delta_x is false → horizontal direction fails
        assert!(!is_delta_in_direction(
            (15.0, 15.0),
            SwipeDirection::Right,
            10.0
        ));
    }

    #[test]
    fn diagonal_equal_is_not_vertical() {
        // delta_x == delta_y → is_delta_x is false, but !is_delta_x is true
        // however delta_y must > threshold
        assert!(is_delta_in_direction(
            (15.0, 15.0),
            SwipeDirection::Down,
            10.0
        ));
    }

    #[test]
    fn wrong_axis_rejects() {
        // Mostly vertical movement, but asking about horizontal direction
        assert!(!is_delta_in_direction(
            (2.0, 50.0),
            SwipeDirection::Right,
            10.0
        ));
        // Mostly horizontal movement, but asking about vertical direction
        assert!(!is_delta_in_direction(
            (50.0, 2.0),
            SwipeDirection::Down,
            10.0
        ));
    }

    #[test]
    fn zero_delta() {
        assert!(!is_delta_in_direction(
            (0.0, 0.0),
            SwipeDirection::Right,
            0.0
        ));
        assert!(!is_delta_in_direction((0.0, 0.0), SwipeDirection::Up, 0.0));
    }
}
