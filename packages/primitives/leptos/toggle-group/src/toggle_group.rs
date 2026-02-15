use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupType
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ToggleGroupType {
    Single,
    Multiple,
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroup
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ToggleGroupValueContextValue {
    r#type: ToggleGroupType,
    value: Signal<Vec<String>>,
    on_item_activate: Callback<String>,
    on_item_deactivate: Callback<String>,
}

#[derive(Clone, Debug)]
struct ToggleGroupContextValue {
    roving_focus: Signal<bool>,
    disabled: Signal<bool>,
}

#[component]
pub fn ToggleGroup(
    /// Whether the group is single or multiple selection.
    r#type: ToggleGroupType,
    /// The controlled value of the pressed items.
    #[prop(into, optional)]
    value: MaybeProp<Vec<String>>,
    /// The default value of the pressed items when uncontrolled.
    #[prop(into, optional)]
    default_value: MaybeProp<Vec<String>>,
    /// Callback when the value changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<Vec<String>>>,
    /// Whether the group is disabled from user interaction.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// Whether the group should maintain roving focus of its buttons.
    #[prop(into, optional)]
    roving_focus: MaybeProp<bool>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (current_value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<Vec<String>>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });
    let current_value = Signal::derive(move || current_value.get().unwrap_or_default());

    let on_item_activate = match r#type {
        ToggleGroupType::Single => Callback::new(move |item_value: String| {
            set_value.run(Some(vec![item_value]));
        }),
        ToggleGroupType::Multiple => Callback::new(move |item_value: String| {
            let mut values = current_value.get();
            values.push(item_value);
            set_value.run(Some(values));
        }),
    };

    let on_item_deactivate = match r#type {
        ToggleGroupType::Single => Callback::new(move |_: String| {
            set_value.run(Some(vec![]));
        }),
        ToggleGroupType::Multiple => Callback::new(move |item_value: String| {
            let values = current_value
                .get()
                .into_iter()
                .filter(|v| *v != item_value)
                .collect();
            set_value.run(Some(values));
        }),
    };

    let value_context = ToggleGroupValueContextValue {
        r#type,
        value: current_value,
        on_item_activate,
        on_item_deactivate,
    };

    let disabled_signal = Signal::derive(move || disabled.get().unwrap_or(false));
    let roving_focus_signal = Signal::derive(move || roving_focus.get().unwrap_or(true));

    let group_context = ToggleGroupContextValue {
        roving_focus: roving_focus_signal,
        disabled: disabled_signal,
    };

    let direction = use_direction(dir);

    view! {
        <Provider value=value_context>
            <Provider value=group_context>
                <ToggleGroupImpl
                    roving_focus=roving_focus_signal
                    orientation=orientation
                    direction=direction
                    r#loop=Signal::derive(move || r#loop.get().unwrap_or(true))
                    as_child=as_child
                    node_ref=node_ref
                >
                    {children.with_value(|children| children())}
                </ToggleGroupImpl>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ToggleGroupImpl(
    #[prop(into)] roving_focus: Signal<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into)] direction: Signal<Direction>,
    #[prop(into)] r#loop: Signal<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        {move || {
            if roving_focus.get() {
                view! {
                    <RovingFocusGroup
                        as_child=true
                        orientation=orientation
                        dir=direction
                        r#loop=r#loop
                    >
                        <Primitive
                            element=html::div
                            as_child=as_child
                            node_ref=node_ref
                            attr:role="group"
                            attr:dir=move || direction.get().to_string()
                            attr:data-orientation=move || orientation.get().map(|o| o.to_string())
                        >
                            {children.with_value(|children| children())}
                        </Primitive>
                    </RovingFocusGroup>
                }
                .into_any()
            } else {
                view! {
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=node_ref
                        attr:role="group"
                        attr:dir=move || direction.get().to_string()
                        attr:data-orientation=move || orientation.get().map(|o| o.to_string())
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                }
                .into_any()
            }
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToggleGroupItem(
    /// A string value for the toggle group item. All items within a toggle group should use a unique value.
    #[prop(into)]
    value: Signal<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let value_context = expect_context::<ToggleGroupValueContextValue>();
    let context = expect_context::<ToggleGroupContextValue>();

    let pressed = Signal::derive(move || value_context.value.get().contains(&value.get()));
    let disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));

    view! {
        {move || {
            if context.roving_focus.get() {
                view! {
                    <RovingFocusGroupItem
                        as_child=true
                        focusable=Signal::derive(move || !disabled.get())
                        active=pressed
                    >
                        <ToggleGroupItemImpl
                            value=value
                            pressed=pressed
                            disabled=disabled
                            on_click=on_click
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </ToggleGroupItemImpl>
                    </RovingFocusGroupItem>
                }
                .into_any()
            } else {
                view! {
                    <ToggleGroupItemImpl
                        value=value
                        pressed=pressed
                        disabled=disabled
                        on_click=on_click
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ToggleGroupItemImpl>
                }
                .into_any()
            }
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupItemImpl
 * -----------------------------------------------------------------------------------------------*/

/// Renders the toggle button directly instead of wrapping the Toggle primitive.
///
/// In React, ToggleGroupItemImpl wraps the Toggle component and overrides `aria-pressed`
/// to `undefined` for single mode (using `role="radio"` + `aria-checked` instead).
/// React's prop spread allows setting `aria-pressed: undefined` to remove the attribute.
///
/// In Leptos, we can't easily override attributes set internally by another component.
/// Instead, we render the button directly with the correct ARIA attributes per mode:
/// - Single: `role="radio"` + `aria-checked` (no `aria-pressed`)
/// - Multiple: `aria-pressed` (standard toggle behavior)
#[component]
fn ToggleGroupItemImpl(
    #[prop(into)] value: Signal<String>,
    #[prop(into)] pressed: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into, optional)] on_click: Option<Option<Callback<ev::MouseEvent>>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let value_context = expect_context::<ToggleGroupValueContextValue>();
    let is_single = value_context.r#type == ToggleGroupType::Single;

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:role=is_single.then_some("radio")
                attr:aria-checked=move || is_single.then(|| pressed.get().to_string())
                attr:aria-pressed=move || (!is_single).then(|| pressed.get().to_string())
                attr:data-state=move || match pressed.get() {
                    true => "on",
                    false => "off",
                }
                attr:data-disabled=move || disabled.get().then_some("")
                attr:disabled=move || disabled.get().then_some("")
                on:click=compose_callbacks(
                    on_click.flatten(),
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        if !disabled.get() {
                            if !pressed.get() {
                                value_context.on_item_activate.run(value.get_untracked());
                            } else {
                                value_context.on_item_deactivate.run(value.get_untracked());
                            }
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
