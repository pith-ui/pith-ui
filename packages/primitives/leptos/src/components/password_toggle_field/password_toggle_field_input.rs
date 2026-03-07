use super::*;

#[allow(clippy::unused_unit)]
#[component]
pub fn PasswordToggleFieldInput(
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let _children = children;
    let context = expect_context::<PasswordToggleFieldContextValue>();

    let auto_complete_value =
        Signal::derive(move || auto_complete.get().unwrap_or_default().as_str());

    // Sync user-provided id prop into context
    Effect::new(move |_| {
        context.sync_input_id.run(id.get());
    });

    // Form reset/submit listener — reset visibility to false
    let set_visible = context.set_visible;
    let input_ref = context.input_ref;
    Effect::new(move |_| {
        let Some(node) = input_ref.get() else {
            return;
        };
        let input: &web_sys::HtmlInputElement = node.unchecked_ref();
        let Some(form) = input.form() else {
            return;
        };

        let set_visible = set_visible;
        let reset_closure = SendWrapper::new(Closure::<dyn Fn(web_sys::Event)>::new(
            move |event: web_sys::Event| {
                if !event.default_prevented() {
                    set_visible.run(Some(false));
                }
            },
        ));
        let submit_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            set_visible.run(Some(false));
        }));

        let form = SendWrapper::new(form);
        form.add_event_listener_with_callback("reset", reset_closure.as_ref().unchecked_ref())
            .expect("Reset event listener should be added.");
        form.add_event_listener_with_callback("submit", submit_closure.as_ref().unchecked_ref())
            .expect("Submit event listener should be added.");

        Owner::on_cleanup(move || {
            form.remove_event_listener_with_callback(
                "reset",
                reset_closure.as_ref().unchecked_ref(),
            )
            .expect("Reset event listener should be removed.");
            form.remove_event_listener_with_callback(
                "submit",
                submit_closure.as_ref().unchecked_ref(),
            )
            .expect("Submit event listener should be removed.");
        });
    });

    let composed_ref = use_composed_refs(vec![node_ref, context.input_ref]);

    let focus_state = context.focus_state;

    let resolved_id = Signal::derive(move || id.get().unwrap_or_else(|| context.input_id.get()));

    view! {
        <AttributeInterceptor let:attrs>
            <VoidPrimitive
                element=html::input
                as_child=as_child
                node_ref=composed_ref
                attr:id=move || resolved_id.get()
                attr:autocapitalize="off"
                attr:autocomplete=move || auto_complete_value.get()
                attr:spellcheck="false"
                attr:r#type=move || if context.visible.get() { "text" } else { "password" }
                on:blur=compose_callbacks(
                    on_blur,
                    Some(Callback::new(move |event: ev::FocusEvent| {
                        let target: web_sys::HtmlInputElement = event
                            .current_target()
                            .expect("Event should have current target")
                            .unchecked_into();
                        focus_state.update_value(|state| {
                            state.selection_start = target.selection_start().ok().flatten();
                            state.selection_end = target.selection_end().ok().flatten();
                        });
                    })),
                    None,
                )
                {..attrs}
            >
                {()}
            </VoidPrimitive>
        </AttributeInterceptor>
    }
}
