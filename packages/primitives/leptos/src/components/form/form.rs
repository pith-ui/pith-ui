use super::*;

/* -------------------------------------------------------------------------------------------------
 * Form
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Form(
    #[prop(into, optional)] on_clear_server_errors: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = ValidationContextValue {
        validity_map: RwSignal::new(HashMap::new()),
        custom_matcher_entries_map: StoredValue::new(HashMap::new()),
        custom_errors_map: RwSignal::new(HashMap::new()),
    };
    let aria_description_context = AriaDescriptionContextValue {
        message_ids_map: RwSignal::new(HashMap::new()),
    };

    let internal_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, internal_ref]);

    // The `invalid` event does NOT bubble, so `on:invalid` on a <form> element won't catch
    // invalid events from child inputs. React works around this via event delegation with capture.
    // We use a capture-phase event listener to intercept invalid events from all controls.
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(form_el) = node.dyn_ref::<web_sys::HtmlFormElement>()
        {
            let form_clone = form_el.clone();
            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                let target = event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());

                if let Some(target) = target
                    && let Some(first_invalid) = get_first_invalid_control(&form_clone)
                    && first_invalid == target
                {
                    first_invalid.focus().ok();
                }

                // Prevent default browser UI for form validation (built-in tooltips).
                event.prevent_default();
            });

            // Use capture phase (third arg = true) since `invalid` doesn't bubble.
            form_el
                .add_event_listener_with_callback_and_bool(
                    "invalid",
                    closure.as_ref().unchecked_ref(),
                    true,
                )
                .ok();

            let form_cleanup = SendWrapper::new(form_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                form_cleanup
                    .remove_event_listener_with_callback_and_bool(
                        "invalid",
                        closure_cleanup.as_ref().unchecked_ref(),
                        true,
                    )
                    .ok();
            });
        }
    });

    let on_clear = on_clear_server_errors;
    let children = StoredValue::new(children);

    view! {
        <Provider value=validation_context>
            <Provider value=aria_description_context>
                <Primitive
                    element=html::form
                    as_child=as_child
                    node_ref=composed_ref
                    on:submit=move |_event: web_sys::SubmitEvent| {
                        if let Some(on_clear) = &on_clear {
                            on_clear.run(());
                        }
                    }
                    on:reset=move |_event: web_sys::Event| {
                        if let Some(on_clear) = &on_clear {
                            on_clear.run(());
                        }
                    }
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </Provider>
        </Provider>
    }
}
