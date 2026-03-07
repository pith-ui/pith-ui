use crate::support::compose_refs::use_composed_refs;
use crate::support::primitive::{Primitive, compose_callbacks, data_attr, prop_or};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::use_previous::use_previous;
use crate::support::use_size::use_size;
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug)]
struct SwitchContextValue {
    checked: Signal<bool>,
    disabled: Signal<bool>,
}

#[component]
pub fn Switch(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: Option<Callback<bool>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    /// The `id` of a `<form>` element to associate the switch with. Allows the switch
    /// to participate in a form even when it is not a descendant of that form.
    #[prop(into, optional)]
    form: MaybeProp<String>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let name = Signal::derive(move || name.get());
    let required = prop_or(required, false);
    let disabled = prop_or(disabled, false);
    let value = prop_or(value, "on".into());

    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        if form.get().is_some() {
            return true;
        }
        button_ref
            .get()
            .and_then(|button| {
                let el: &web_sys::Element = button.unchecked_ref();
                el.closest("form").ok()
            })
            .flatten()
            .is_some()
    });
    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        on_change: on_checked_change.map(|on_checked_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_checked_change.run(value);
                }
            })
        }),
        default_prop: default_checked,
    });
    let checked = Signal::derive(move || checked.get().unwrap_or(false));

    let context_value = SwitchContextValue { checked, disabled };

    view! {
        <Provider value=context_value>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_refs
                    attr:r#type="button"
                    attr:role="switch"
                    attr:aria-checked=move || match checked.get() {
                        true => "true",
                        false => "false",
                    }
                    attr:aria-required=move || match required.get() {
                        true => "true",
                        false => "false",
                    }
                    attr:data-state=move || get_state(checked.get())
                    attr:data-disabled=data_attr(disabled)
                    attr:disabled=data_attr(disabled)
                    attr:value=move || value.get()
                    on:click=compose_callbacks(on_click, Some(Callback::new(move |event: ev::MouseEvent| {
                        if !disabled.get() {
                            set_checked.run(Some(!checked.get()));

                            if is_form_control.get() {
                                // If switch is in a form, stop propagation from the button, so that we only propagate
                                // one click event (from the input). We propagate changes from an input so that native
                                // form validation works and form events reflect switch updates.
                                event.stop_propagation();
                            }
                        }
                    })), None)
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    attr:name=move || name.get()
                    control_ref=button_ref
                    bubbles=Signal::derive(|| true)
                    value=value
                    checked=checked
                    required=required
                    disabled=disabled
                    form=Signal::derive(move || form.get())
                />
            </Show>
        </Provider>
    }
}

#[component]
pub fn SwitchThumb(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SwitchContextValue>();

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-state=move || get_state(context.checked.get())
            attr:data-disabled=data_attr(context.disabled)
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] form: Signal<Option<String>>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    // Bubble checked change to parents
    Effect::new(move |_| {
        if let Some(input) = node_ref.get() {
            if prev_checked.get() != checked.get() {
                let init = web_sys::EventInit::new();
                init.set_bubbles(bubbles.get());

                let event = web_sys::Event::new_with_event_init_dict("click", &init)
                    .expect("Click event should be instantiated.");

                input.set_checked(checked.get());

                input
                    .dispatch_event(&event)
                    .expect("Click event should be dispatched.");
            }
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            aria-hidden="true"
            checked=move || checked.get().then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            value=move || value.get()
            form=move || form.get()
            tabindex="-1"
            // We transform because the input is absolutely positioned, but we have
            // rendered it **after** the button. This pulls it back to sit on top
            // of the button.
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
        />
    }
}

fn get_state(checked: bool) -> String {
    (match checked {
        true => "checked",
        false => "unchecked",
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_state_checked() {
        assert_eq!(get_state(true), "checked");
    }

    #[test]
    fn get_state_unchecked() {
        assert_eq!(get_state(false), "unchecked");
    }
}
