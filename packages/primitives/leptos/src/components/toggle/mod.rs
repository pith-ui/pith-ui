use crate::support::primitive::{Primitive, compose_callbacks, data_attr, prop_or_default};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{attribute_interceptor::AttributeInterceptor, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

/* -------------------------------------------------------------------------------------------------
 * Toggle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Toggle(
    /// The controlled state of the toggle.
    #[prop(into, optional)]
    pressed: MaybeProp<bool>,
    /// The state of the toggle when initially rendered. Use `default_pressed` if you do not need to control the state of the toggle. Defaults to `false`.
    #[prop(into, optional)]
    default_pressed: MaybeProp<bool>,
    /// The callback that fires when the state of the toggle changes.
    #[prop(into, optional)]
    on_pressed_change: Option<Callback<bool>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let disabled = prop_or_default(disabled);

    let (pressed, set_pressed) = use_controllable_state(UseControllableStateParams {
        prop: pressed,
        on_change: on_pressed_change.map(|on_pressed_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_pressed_change.run(value);
                }
            })
        }),
        default_prop: default_pressed,
    });
    let pressed = Signal::derive(move || pressed.get().unwrap_or(false));

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:aria-pressed=move || pressed.get().to_string()
                attr:data-state=move || match pressed.get() {
                    true => "on",
                    false => "off",
                }
                attr:data-disabled=data_attr(disabled)
                attr:disabled=data_attr(disabled)
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        if !disabled.get() {
                            set_pressed.run(Some(!pressed.get()));
                        }
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
