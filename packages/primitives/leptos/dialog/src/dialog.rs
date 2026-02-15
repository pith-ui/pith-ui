use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_focus_guards::use_focus_guards;
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_id::use_id;
use radix_leptos_portal::Portal;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/* -------------------------------------------------------------------------------------------------
 * Dialog
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct DialogContextValue {
    trigger_ref: AnyNodeRef,
    content_ref: AnyNodeRef,
    content_id: ReadSignal<String>,
    title_id: ReadSignal<String>,
    description_id: ReadSignal<String>,
    open: Signal<bool>,
    on_open_change: Callback<bool>,
    on_open_toggle: Callback<()>,
    modal: Signal<bool>,
}

#[component]
pub fn Dialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let trigger_ref = AnyNodeRef::new();
    let content_ref = AnyNodeRef::new();

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.map(|on_open_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_open_change.run(value);
                }
            })
        }),
    });
    let open = Signal::derive(move || open_signal.get().unwrap_or(false));
    let modal = Signal::derive(move || modal.get().unwrap_or(true));

    let content_id = use_id(None);
    let title_id = use_id(None);
    let description_id = use_id(None);

    let context = DialogContextValue {
        trigger_ref,
        content_ref,
        content_id,
        title_id,
        description_id,
        open,
        on_open_change: Callback::new(move |value: bool| {
            set_open.run(Some(value));
        }),
        on_open_toggle: Callback::new(move |_| {
            set_open.run(Some(!open.get()));
        }),
        modal,
    };

    view! {
        <Provider value=context>
            {children.with_value(|children| children())}
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    let composed_trigger_ref = use_composed_refs(vec![node_ref, context.trigger_ref]);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_trigger_ref
                attr:r#type="button"
                attr:aria-haspopup="dialog"
                attr:aria-expanded=move || context.open.get().to_string()
                attr:aria-controls=move || context.content_id.get()
                attr:data-state=move || get_state(context.open.get())
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        context.on_open_toggle.run(());
                    })),
                    None,
                )
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogPortal
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct PortalContextValue {
    force_mount: Signal<bool>,
}

#[component]
pub fn DialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let force_mount_signal = Signal::derive(move || force_mount.get().unwrap_or(false));

    let portal_context = PortalContextValue {
        force_mount: force_mount_signal,
    };

    // React wraps each child individually in <Presence><Portal>, allowing each child
    // to observe its own animation events. We cannot map over children the same way,
    // so we always render the Portal and let each child (DialogOverlay, DialogContent)
    // handle mount/unmount via their own Presence wrappers. This avoids the portal-level
    // Presence (which has no node_ref to observe) from prematurely unmounting children
    // before their exit animations complete.
    view! {
        <Provider value=portal_context>
            <Portal container=container container_ref=container_ref as_child=true>
                {children.with_value(|children| children())}
            </Portal>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogOverlay
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogOverlay(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let portal_context = use_context::<PortalContextValue>();

    let force_mount = Signal::derive(move || {
        force_mount
            .get()
            .or_else(|| portal_context.as_ref().map(|pc| pc.force_mount.get()))
            .unwrap_or(false)
    });

    let present = Signal::derive(move || force_mount.get() || context.open.get());
    let is_modal = context.modal;

    let presence_ref = AnyNodeRef::new();

    view! {
        <Show when=move || is_modal.get()>
            <Presence present=present node_ref=presence_ref>
                <DialogOverlayImpl
                    as_child=as_child
                    node_ref=node_ref
                    presence_ref=presence_ref
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </DialogOverlayImpl>
            </Presence>
        </Show>
    }
}

#[component]
fn DialogOverlayImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, presence_ref]);

    // Body scroll lock: set overflow hidden on body while overlay is mounted
    use_body_scroll_lock();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:data-state=move || get_state(context.open.get())
                attr:style="pointer-events: auto;"
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogContent
 * -----------------------------------------------------------------------------------------------*/

/// Shared callback props for dialog content variants, stored to avoid ownership issues
/// when forwarding `Option<Callback<...>>` through multiple component layers.
#[derive(Clone, Copy)]
struct ContentCallbacks {
    on_open_auto_focus: StoredValue<Option<Callback<web_sys::Event>>>,
    on_close_auto_focus: StoredValue<Option<Callback<web_sys::Event>>>,
    on_escape_key_down: StoredValue<Option<Callback<web_sys::KeyboardEvent>>>,
    on_pointer_down_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_focus_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
    on_interact_outside: StoredValue<Option<Callback<web_sys::CustomEvent>>>,
}

/// Shared content options that aren't callbacks, stored to thread through layers.
#[derive(Clone, Copy)]
struct ContentOptions {
    /// The ARIA role for the content element. Defaults to `"dialog"`.
    /// AlertDialog overrides this to `"alertdialog"`.
    role: StoredValue<String>,
}

#[component]
pub fn DialogContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    /// The ARIA role for the content element. Defaults to `"dialog"`.
    /// AlertDialog overrides this to `"alertdialog"`.
    #[prop(into, optional)]
    role: Option<String>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let portal_context = use_context::<PortalContextValue>();

    let force_mount = Signal::derive(move || {
        force_mount
            .get()
            .or_else(|| portal_context.as_ref().map(|pc| pc.force_mount.get()))
            .unwrap_or(false)
    });

    let present = Signal::derive(move || force_mount.get() || context.open.get());
    let is_modal = context.modal;

    let callbacks = ContentCallbacks {
        on_open_auto_focus: StoredValue::new(on_open_auto_focus),
        on_close_auto_focus: StoredValue::new(on_close_auto_focus),
        on_escape_key_down: StoredValue::new(on_escape_key_down),
        on_pointer_down_outside: StoredValue::new(on_pointer_down_outside),
        on_focus_outside: StoredValue::new(on_focus_outside),
        on_interact_outside: StoredValue::new(on_interact_outside),
    };

    let options = ContentOptions {
        role: StoredValue::new(role.unwrap_or_else(|| "dialog".to_string())),
    };

    let presence_ref = AnyNodeRef::new();

    view! {
        <Presence present=present node_ref=presence_ref>
            <Show
                when=move || is_modal.get()
                fallback=move || view! {
                    <DialogContentNonModal
                        as_child=as_child
                        node_ref=node_ref
                        presence_ref=presence_ref
                        callbacks=callbacks
                        options=options
                    >
                        {children.with_value(|children| children.as_ref().map(|children| children()))}
                    </DialogContentNonModal>
                }
            >
                <DialogContentModal
                    as_child=as_child
                    node_ref=node_ref
                    presence_ref=presence_ref
                    callbacks=callbacks
                    options=options
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </DialogContentModal>
            </Show>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogContentModal
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn DialogContentModal(
    callbacks: ContentCallbacks,
    options: ContentOptions,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![
        node_ref,
        context.content_ref,
        content_ref,
        presence_ref,
    ]);

    // aria-hide everything except the content (better supported equivalent to setting aria-modal).
    // Deferred to requestAnimationFrame to emulate React's useEffect timing (post-layout):
    // FocusScope auto-focus runs first (as a synchronous Effect, like React's useLayoutEffect),
    // moving focus into the dialog content before hide_others sets aria-hidden on outside elements.
    // Without this deferral, hide_others would run while focus is still on the trigger, causing
    // a "Blocked aria-hidden on an element because its descendant retained focus" browser warning.
    //
    // The hidden_elements signal is managed at this level so on_close_auto_focus can remove
    // aria-hidden BEFORE focusing the trigger (preventing the same warning on close).
    let hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>> = RwSignal::new(Vec::new());

    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            let content: web_sys::HtmlElement = content.unchecked_into();
            let cb = Closure::once_into_js(move || {
                hide_others(&content, hidden_elements);
            });
            web_sys::window()
                .expect("Window should exist.")
                .request_animation_frame(cb.unchecked_ref())
                .ok();
        }
    });

    on_cleanup(move || {
        unhide_others(hidden_elements);
    });

    let on_close_auto_focus = Callback::new(move |event: web_sys::Event| {
        callbacks.on_close_auto_focus.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        // Remove aria-hidden from outside elements BEFORE focusing the trigger.
        // Without this, the trigger (inside an aria-hidden ancestor) would receive focus,
        // causing a "Blocked aria-hidden on a focused element's ancestor" browser warning.
        unhide_others(hidden_elements);
        event.prevent_default();
        if let Some(trigger) = context.trigger_ref.get_untracked() {
            let trigger: &web_sys::HtmlElement = trigger.unchecked_ref();
            trigger.focus().ok();
        }
    });

    let on_pointer_down_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_pointer_down_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        let original_event = js_sys::Reflect::get(&event.detail(), &"originalEvent".into())
            .ok()
            .and_then(|v| v.dyn_into::<web_sys::PointerEvent>().ok());
        if let Some(original_event) = original_event {
            let ctrl_left_click = original_event.button() == 0 && original_event.ctrl_key();
            let is_right_click = original_event.button() == 2 || ctrl_left_click;
            if is_right_click {
                event.prevent_default();
            }
        }
    });

    let on_focus_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_focus_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        // When focus is trapped, a `focusout` event may still happen.
        // We make sure we don't trigger our `onDismiss` in such case.
        event.prevent_default();
    });

    let on_open_auto_focus = callbacks
        .on_open_auto_focus
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_escape_key_down = callbacks
        .on_escape_key_down
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_interact_outside = callbacks
        .on_interact_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));

    view! {
        <DialogContentImpl
            as_child=as_child
            node_ref=composed_refs
            role=options.role.get_value().clone()
            trap_focus=context.open
            disable_outside_pointer_events=true
            on_open_auto_focus=on_open_auto_focus
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_interact_outside=on_interact_outside
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </DialogContentImpl>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogContentNonModal
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn DialogContentNonModal(
    callbacks: ContentCallbacks,
    options: ContentOptions,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let composed_refs = use_composed_refs(vec![node_ref, presence_ref]);

    let has_interacted_outside = RwSignal::new(false);
    let has_pointer_down_outside = RwSignal::new(false);

    let on_close_auto_focus = Callback::new(move |event: web_sys::Event| {
        callbacks.on_close_auto_focus.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });

        if !event.default_prevented() {
            if !has_interacted_outside.get_untracked()
                && let Some(trigger) = context.trigger_ref.get_untracked()
            {
                let trigger: &web_sys::HtmlElement = trigger.unchecked_ref();
                trigger.focus().ok();
            }
            // Always prevent auto focus because we either focus manually or want user agent focus
            event.prevent_default();
        }

        has_interacted_outside.set(false);
        has_pointer_down_outside.set(false);
    });

    let on_interact_outside = Callback::new(move |event: web_sys::CustomEvent| {
        callbacks.on_interact_outside.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });

        if !event.default_prevented() {
            has_interacted_outside.set(true);
            let original_event_type =
                js_sys::Reflect::get(&event.detail(), &"originalEvent".into())
                    .ok()
                    .and_then(|v| v.dyn_into::<web_sys::Event>().ok())
                    .map(|e| e.type_());

            if original_event_type.as_deref() == Some("pointerdown") {
                has_pointer_down_outside.set(true);
            }
        }

        // Prevent dismissing when clicking the trigger.
        // As the trigger is already setup to close, without doing so would
        // cause it to close and immediately open.
        if let Some(target) = event.target() {
            let target: web_sys::Node = target.unchecked_into();
            if let Some(trigger) = context.trigger_ref.get_untracked() {
                let trigger: &web_sys::Node = trigger.unchecked_ref();
                if trigger.contains(Some(&target)) {
                    event.prevent_default();
                }
            }
        }

        // On Safari if the trigger is inside a container with tabIndex={0}, when clicked
        // we will get the pointer down outside event on the trigger, but then a subsequent
        // focus outside event on the container, we ignore any focus outside event when we've
        // already had a pointer down outside event.
        let original_event_type = js_sys::Reflect::get(&event.detail(), &"originalEvent".into())
            .ok()
            .and_then(|v| v.dyn_into::<web_sys::Event>().ok())
            .map(|e| e.type_());
        if original_event_type.as_deref() == Some("focusin")
            && has_pointer_down_outside.get_untracked()
        {
            event.prevent_default();
        }
    });

    let on_open_auto_focus = callbacks
        .on_open_auto_focus
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_escape_key_down = callbacks
        .on_escape_key_down
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_pointer_down_outside = callbacks
        .on_pointer_down_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));
    let on_focus_outside = callbacks
        .on_focus_outside
        .get_value()
        .unwrap_or(Callback::new(|_| {}));

    view! {
        <DialogContentImpl
            as_child=as_child
            node_ref=composed_refs
            role=options.role.get_value().clone()
            trap_focus=false
            disable_outside_pointer_events=false
            on_open_auto_focus=on_open_auto_focus
            on_close_auto_focus=on_close_auto_focus
            on_escape_key_down=on_escape_key_down
            on_pointer_down_outside=on_pointer_down_outside
            on_focus_outside=on_focus_outside
            on_interact_outside=on_interact_outside
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </DialogContentImpl>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn DialogContentImpl(
    #[prop(into, optional)] role: Option<String>,
    #[prop(into, optional)] trap_focus: MaybeProp<bool>,
    #[prop(into, optional)] disable_outside_pointer_events: MaybeProp<bool>,
    #[prop(into)] on_open_auto_focus: Callback<web_sys::Event>,
    #[prop(into)] on_close_auto_focus: Callback<web_sys::Event>,
    #[prop(into)] on_escape_key_down: Callback<web_sys::KeyboardEvent>,
    #[prop(into)] on_pointer_down_outside: Callback<web_sys::CustomEvent>,
    #[prop(into)] on_focus_outside: Callback<web_sys::CustomEvent>,
    #[prop(into)] on_interact_outside: Callback<web_sys::CustomEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();
    let role = StoredValue::new(role.unwrap_or_else(|| "dialog".to_string()));

    // Make sure the whole tree has focus guards as our `Dialog` will be
    // the last element in the DOM (because of the `Portal`)
    use_focus_guards();

    let trapped = Signal::derive(move || trap_focus.get().unwrap_or(false));
    let disable_outside =
        Signal::derive(move || disable_outside_pointer_events.get().unwrap_or(false));

    view! {
        <FocusScope
            as_child=true
            r#loop=true
            trapped=trapped
            on_mount_auto_focus=on_open_auto_focus
            on_unmount_auto_focus=Some(on_close_auto_focus)
        >
            <DismissableLayer
                as_child=as_child
                node_ref=node_ref
                attr:role=move || role.get_value()
                attr:id=move || context.content_id.get()
                attr:aria-describedby=move || context.description_id.get()
                attr:aria-labelledby=move || context.title_id.get()
                attr:data-state=move || get_state(context.open.get())
                disable_outside_pointer_events=disable_outside
                on_escape_key_down=on_escape_key_down
                on_pointer_down_outside=on_pointer_down_outside
                on_focus_outside=on_focus_outside
                on_interact_outside=on_interact_outside
                on_dismiss=Callback::new(move |_| {
                    context.on_open_change.run(false);
                })
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </DismissableLayer>
        </FocusScope>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogTitle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::h2
                as_child=as_child
                node_ref=node_ref
                attr:id=move || context.title_id.get()
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogDescription
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::p
                as_child=as_child
                node_ref=node_ref
                attr:id=move || context.description_id.get()
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * DialogClose
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn DialogClose(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<DialogContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        context.on_open_change.run(false);
                    })),
                    None,
                )
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

fn get_state(open: bool) -> &'static str {
    match open {
        true => "open",
        false => "closed",
    }
}

/* -------------------------------------------------------------------------------------------------
 * use_body_scroll_lock
 * Simplified body scroll lock: sets `overflow: hidden` on body while mounted.
 * React uses `react-remove-scroll` which supports shards (allowing scroll on specific elements
 * like the content) and pinch-zoom. This simplified version just hides body overflow.
 * -----------------------------------------------------------------------------------------------*/

fn use_body_scroll_lock() {
    let original_overflow: RwSignal<Option<String>> = RwSignal::new(None);

    Effect::new(move |_| {
        let body = document().body().expect("Document should have body.");
        let style = body.style();
        let current = style.get_property_value("overflow").unwrap_or_default();
        original_overflow.set(Some(current));
        style
            .set_property("overflow", "hidden")
            .expect("Style should be set.");
    });

    on_cleanup(move || {
        if let Some(original) = original_overflow.get_untracked()
            && let Some(body) = document().body()
        {
            body.style()
                .set_property("overflow", &original)
                .expect("Style should be set.");
        }
    });
}

/* -------------------------------------------------------------------------------------------------
 * hide_others
 * Simplified `aria-hidden` implementation: sets `aria-hidden="true"` on body's direct children
 * that don't contain the dialog content element, and restores on cleanup.
 * React uses the `aria-hidden` library (`hideOthers`) which walks the tree more thoroughly.
 * This simplified version only hides body's direct children.
 * -----------------------------------------------------------------------------------------------*/

/// Sets `aria-hidden="true"` on body's direct children that don't contain the dialog content,
/// storing the affected elements in the provided signal for later cleanup.
fn hide_others(
    content: &web_sys::HtmlElement,
    hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>>,
) {
    let body = document().body().expect("Document should have body.");
    let children = body.children();
    let mut hidden = Vec::new();

    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            // Skip elements that contain the content or are already hidden
            let contains_content = child.contains(Some(content));
            let already_hidden = child
                .get_attribute("aria-hidden")
                .is_some_and(|v| v == "true");
            let is_script = child.tag_name().eq_ignore_ascii_case("SCRIPT");

            if !contains_content && !already_hidden && !is_script {
                child
                    .set_attribute("aria-hidden", "true")
                    .expect("Attribute should be set.");
                hidden.push(SendWrapper::new(child));
            }
        }
    }

    hidden_elements.set(hidden);
}

/// Removes `aria-hidden` from all elements previously hidden by `hide_others`.
fn unhide_others(hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>>) {
    for element in hidden_elements.get_untracked() {
        element
            .remove_attribute("aria-hidden")
            .expect("Attribute should be removed.");
    }
    hidden_elements.set(Vec::new());
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}
