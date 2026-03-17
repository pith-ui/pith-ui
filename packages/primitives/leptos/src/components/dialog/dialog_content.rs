use super::*;

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

    let force_mount = resolve_force_mount(force_mount);

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
        // Only apply default focus-to-trigger behavior if the consumer hasn't
        // already called preventDefault() (matches React's composeEventHandlers).
        if !event.default_prevented() {
            event.prevent_default();
            if let Some(trigger) = context.trigger_ref.get_untracked() {
                let trigger: &web_sys::HtmlElement = trigger.unchecked_ref();
                trigger.focus().ok();
            }
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

    // Dev-time warnings for missing accessibility labels (matches React's
    // TitleWarning / DescriptionWarning in development mode).
    if cfg!(debug_assertions) {
        let title_id = context.title_id;
        let description_id = context.description_id;
        let content_name = role.get_value().clone();

        Effect::new(move |_| {
            let tid = title_id.get();
            if !tid.is_empty() && document().get_element_by_id(&tid).is_none() {
                web_sys::console::error_1(
                    &format!(
                        "`DialogContent` (role=\"{}\") requires a `DialogTitle` for the component \
                         to be accessible for screen reader users.\n\n\
                         If you want to hide the `DialogTitle`, you can wrap it with the \
                         VisuallyHidden component.\n\n\
                         For more information, see https://radix-ui.com/primitives/docs/components/dialog",
                        content_name
                    )
                    .into(),
                );
            }
        });

        Effect::new(move |_| {
            let did = description_id.get();
            if !did.is_empty() && document().get_element_by_id(&did).is_none() {
                web_sys::console::warn_1(
                    &"Warning: Missing `Description` or `aria-describedby={undefined}` for DialogContent."
                        .into(),
                );
            }
        });
    }

    let trapped = prop_or_default(trap_focus);
    let disable_outside = prop_or_default(disable_outside_pointer_events);

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
                attr:data-state=move || open_closed_state(context.open.get())
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
