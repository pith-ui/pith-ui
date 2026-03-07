use super::*;

/* -------------------------------------------------------------------------------------------------
 * FormField
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into, optional)] server_invalid: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let id = use_id(None);

    let field_name = name.clone();
    let server_invalid_signal = prop_or_default(server_invalid);

    let field_context = FormFieldContextValue {
        id: id.get_untracked(),
        name: field_name.clone(),
        server_invalid: server_invalid_signal,
    };

    let validity_name = field_name.clone();
    let validity = Memo::new(move |_| validation_context.get_field_validity(&validity_name));
    let valid_attr =
        Memo::new(move |_| get_valid_attribute(&validity.get(), server_invalid_signal.get()));
    let invalid_attr =
        Memo::new(move |_| get_invalid_attribute(&validity.get(), server_invalid_signal.get()));

    let children = StoredValue::new(children);

    view! {
        <Provider value=field_context>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-valid=move || valid_attr.get()
                attr:data-invalid=move || invalid_attr.get()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormLabel(
    #[prop(into, optional)] html_for: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let field_context = expect_context::<FormFieldContextValue>();

    let html_for = html_for.unwrap_or_else(|| field_context.id.clone());
    let field_name = field_context.name.clone();
    let server_invalid = field_context.server_invalid;

    let validity = Memo::new(move |_| validation_context.get_field_validity(&field_name));
    let valid_attr = Memo::new(move |_| get_valid_attribute(&validity.get(), server_invalid.get()));
    let invalid_attr =
        Memo::new(move |_| get_invalid_attribute(&validity.get(), server_invalid.get()));

    let children = StoredValue::new(children);

    view! {
        <crate::label::Label
            attr:r#for=html_for
            as_child=as_child
            node_ref=node_ref
            attr:data-valid=move || valid_attr.get()
            attr:data-invalid=move || invalid_attr.get()
        >
            {children.with_value(|children| children())}
        </crate::label::Label>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormControl
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormControl(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] on_invalid: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_change: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let field_context = expect_context::<FormFieldContextValue>();
    let aria_description_context = expect_context::<AriaDescriptionContextValue>();

    let control_name = name.unwrap_or_else(|| field_context.name.clone());
    let control_id = id.unwrap_or_else(|| field_context.id.clone());
    let server_invalid = field_context.server_invalid;

    let internal_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, internal_ref]);

    // Derived validity for rendering
    let validity_name = control_name.clone();
    let validity_ctx = validation_context.clone();
    let validity = Memo::new(move |_| validity_ctx.get_field_validity(&validity_name));
    let valid_attr = Memo::new(move |_| get_valid_attribute(&validity.get(), server_invalid.get()));
    let invalid_attr =
        Memo::new(move |_| get_invalid_attribute(&validity.get(), server_invalid.get()));

    let desc_name = control_name.clone();
    let aria_describedby =
        Memo::new(move |_| aria_description_context.get_field_description(&desc_name));

    let aria_invalid_attr = Memo::new(move |_| {
        if server_invalid.get() {
            Some("true")
        } else {
            None
        }
    });

    // Set up native `change` event listener for validation.
    // We use the native `change` event (not Leptos `on:change` which fires on input)
    // to validate only when the user finishes changing the value, not on every keystroke.
    let change_name = control_name.clone();
    let change_validation_context = validation_context.clone();
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
        {
            let name = change_name.clone();
            let ctx = change_validation_context.clone();
            let control_clone = control_el.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                update_control_validity(&control_clone, &name, &ctx);
            });

            control_el
                .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .ok();

            let control_cleanup = SendWrapper::new(control_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                control_cleanup
                    .remove_event_listener_with_callback(
                        "change",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    // Set up form `reset` event listener to clear validation
    let reset_name = control_name.clone();
    let reset_validation_context = validation_context.clone();
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
            && let Some(form) = control_el.form()
        {
            let name = reset_name.clone();
            let ctx = reset_validation_context.clone();
            let control_clone = control_el.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                control_clone.set_custom_validity("");
                ctx.clear_field_validation(&name);
            });

            form.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                .ok();

            let form_cleanup = SendWrapper::new(form);
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                form_cleanup
                    .remove_event_listener_with_callback(
                        "reset",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    // Focus first invalid control when fields are set as invalid by server
    Effect::new(move |_| {
        if server_invalid.get()
            && let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
            && let Some(form) = control_el.closest("form").ok().flatten()
        {
            let form: web_sys::HtmlFormElement = form.unchecked_into();
            if let Some(first_invalid) = get_first_invalid_control(&form) {
                let control_html: &web_sys::HtmlElement = control_el.as_ref();
                if first_invalid == *control_html {
                    first_invalid.focus().ok();
                }
            }
        }
    });

    // Set up native `invalid` event listener for validation.
    // We use `addEventListener` directly because `on:invalid` on a component (VoidPrimitive)
    // may not reliably forward to the underlying <input> element through component layers.
    let invalid_name = control_name.clone();
    let invalid_validation_context = validation_context.clone();
    let on_invalid_prop = on_invalid;
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
        {
            let name = invalid_name.clone();
            let ctx = invalid_validation_context.clone();
            let control_clone = control_el.clone();
            let on_invalid_prop = on_invalid_prop;

            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Some(cb) = &on_invalid_prop {
                    cb.run(event.clone());
                }

                // Always update validity — don't check defaultPrevented here because
                // the Form's capture-phase invalid listener already calls preventDefault
                // to suppress browser tooltips. React doesn't check defaultPrevented either.
                update_control_validity(&control_clone, &name, &ctx);
            });

            control_el
                .add_event_listener_with_callback("invalid", closure.as_ref().unchecked_ref())
                .ok();

            let control_cleanup = SendWrapper::new(control_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                control_cleanup
                    .remove_event_listener_with_callback(
                        "invalid",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    // Set up native `input` event listener to reset validity when user changes value.
    // React's `onChange` is actually the native `input` event (fires on every keystroke).
    let reset_validity_name = control_name.clone();
    let reset_validity_ctx = validation_context.clone();
    let on_change_prop = on_change;
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
        {
            let name = reset_validity_name.clone();
            let ctx = reset_validity_ctx.clone();
            let control_clone = control_el.clone();
            let on_change_prop = on_change_prop;

            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Some(cb) = &on_change_prop {
                    cb.run(event);
                }

                control_clone.set_custom_validity("");
                ctx.clear_field_validation(&name);
            });

            control_el
                .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
                .ok();

            let control_cleanup = SendWrapper::new(control_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                control_cleanup
                    .remove_event_listener_with_callback(
                        "input",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let children = StoredValue::new(children);

    view! {
        <VoidPrimitive
            element=html::input
            as_child=as_child
            node_ref=composed_ref
            attr:data-valid=move || valid_attr.get()
            attr:data-invalid=move || invalid_attr.get()
            attr:aria-invalid=move || aria_invalid_attr.get()
            attr:aria-describedby=move || aria_describedby.get()
            attr:title=""
            attr:id=control_id.clone()
            attr:name=control_name.clone()
        >
            {children.with_value(|children| children.as_ref().map(|c| c()))}
        </VoidPrimitive>
    }
}
