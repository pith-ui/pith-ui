use crate::support::direction::{Direction, use_direction};
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, data_attr, prop_or, prop_or_default,
};
use crate::support::roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupType (kept for convenience wrapper)
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ToggleGroupType {
    Single,
    Multiple,
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupMode trait + implementations
 * -----------------------------------------------------------------------------------------------*/

pub trait ToggleGroupMode: Send + 'static {
    type Value: Clone + PartialEq + Send + Sync + 'static;

    fn default_value() -> Self::Value;
    fn to_vec(value: &Self::Value) -> Vec<String>;
    fn on_activate(current: &Self::Value, item: &str) -> Self::Value;
    fn on_deactivate(current: &Self::Value, item: &str) -> Self::Value;
    fn toggle_group_type() -> ToggleGroupType;
}

pub struct Single;
pub struct Multiple;

impl ToggleGroupMode for Single {
    type Value = String;

    fn default_value() -> String {
        String::new()
    }

    fn to_vec(value: &String) -> Vec<String> {
        if value.is_empty() {
            vec![]
        } else {
            vec![value.clone()]
        }
    }

    fn on_activate(_current: &String, item: &str) -> String {
        item.to_string()
    }

    fn on_deactivate(_current: &String, _item: &str) -> String {
        String::new()
    }

    fn toggle_group_type() -> ToggleGroupType {
        ToggleGroupType::Single
    }
}

impl ToggleGroupMode for Multiple {
    type Value = Vec<String>;

    fn default_value() -> Vec<String> {
        vec![]
    }

    fn to_vec(value: &Vec<String>) -> Vec<String> {
        value.clone()
    }

    fn on_activate(current: &Vec<String>, item: &str) -> Vec<String> {
        let mut v = current.clone();
        v.push(item.to_string());
        v
    }

    fn on_deactivate(current: &Vec<String>, item: &str) -> Vec<String> {
        current.iter().filter(|v| v.as_str() != item).cloned().collect()
    }

    fn toggle_group_type() -> ToggleGroupType {
        ToggleGroupType::Multiple
    }
}

/* -------------------------------------------------------------------------------------------------
 * Context types (unchanged — internal, uses Vec<String>)
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

/* -------------------------------------------------------------------------------------------------
 * Generic core function
 * -----------------------------------------------------------------------------------------------*/

/// Generic core logic for ToggleGroup, parameterized by mode.
///
/// Handles `use_controllable_state` with mode-appropriate value types,
/// converts to `Vec<String>` internally for the context, and renders `ToggleGroupImpl`.
#[allow(clippy::too_many_arguments)]
fn toggle_group_core<M: ToggleGroupMode>(
    value: MaybeProp<M::Value>,
    default_value: MaybeProp<M::Value>,
    on_value_change: Option<Callback<M::Value>>,
    disabled: MaybeProp<bool>,
    roving_focus: MaybeProp<bool>,
    r#loop: MaybeProp<bool>,
    orientation: MaybeProp<Orientation>,
    dir: MaybeProp<Direction>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (current_value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: adapt_callback(on_value_change),
    });
    let current_value =
        Signal::derive(move || current_value.get().unwrap_or_else(M::default_value));

    // Convert mode-specific value to Vec<String> for the context
    let value_as_vec = Signal::derive(move || M::to_vec(&current_value.get()));

    let on_item_activate = Callback::new(move |item_value: String| {
        let new_val = M::on_activate(&current_value.get(), &item_value);
        set_value.run(Some(new_val));
    });

    let on_item_deactivate = Callback::new(move |item_value: String| {
        let new_val = M::on_deactivate(&current_value.get(), &item_value);
        set_value.run(Some(new_val));
    });

    let value_context = ToggleGroupValueContextValue {
        r#type: M::toggle_group_type(),
        value: value_as_vec,
        on_item_activate,
        on_item_deactivate,
    };

    let disabled_signal = prop_or_default(disabled);
    let roving_focus_signal = prop_or(roving_focus, true);

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
                    r#loop=prop_or(r#loop, true)
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
 * ToggleGroupSingle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToggleGroupSingle(
    /// The controlled value of the pressed item.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The default value of the pressed item when uncontrolled.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Callback when the value changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
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
    toggle_group_core::<Single>(
        value,
        default_value,
        on_value_change,
        disabled,
        roving_focus,
        r#loop,
        orientation,
        dir,
        as_child,
        node_ref,
        children,
    )
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroupMultiple
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToggleGroupMultiple(
    /// The controlled values of the pressed items.
    #[prop(into, optional)]
    value: MaybeProp<Vec<String>>,
    /// The default values of the pressed items when uncontrolled.
    #[prop(into, optional)]
    default_value: MaybeProp<Vec<String>>,
    /// Callback when the values change.
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
    toggle_group_core::<Multiple>(
        value,
        default_value,
        on_value_change,
        disabled,
        roving_focus,
        r#loop,
        orientation,
        dir,
        as_child,
        node_ref,
        children,
    )
}

/* -------------------------------------------------------------------------------------------------
 * ToggleGroup (convenience wrapper for React API parity)
 * -----------------------------------------------------------------------------------------------*/

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
    match r#type {
        ToggleGroupType::Single => {
            // Adapt Vec<String> props to String for the single-mode core.
            let single_value: MaybeProp<String> = Signal::derive(move || {
                value.get().and_then(|v| v.into_iter().next())
            })
            .into();
            let single_default: MaybeProp<String> = Signal::derive(move || {
                default_value.get().and_then(|v| v.into_iter().next())
            })
            .into();
            let single_cb = on_value_change.map(|cb| {
                Callback::new(move |v: String| {
                    cb.run(if v.is_empty() { vec![] } else { vec![v] });
                })
            });

            toggle_group_core::<Single>(
                single_value,
                single_default,
                single_cb,
                disabled,
                roving_focus,
                r#loop,
                orientation,
                dir,
                as_child,
                node_ref,
                children,
            )
            .into_any()
        }
        ToggleGroupType::Multiple => toggle_group_core::<Multiple>(
            value,
            default_value,
            on_value_change,
            disabled,
            roving_focus,
            r#loop,
            orientation,
            dir,
            as_child,
            node_ref,
            children,
        )
        .into_any(),
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
                attr:data-disabled=data_attr(disabled)
                attr:disabled=data_attr(disabled)
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

#[cfg(test)]
mod tests {
    use super::*;

    // ── Single mode (via trait) ─────────────────────────────

    #[test]
    fn single_activate_replaces() {
        assert_eq!(Single::on_activate(&"a".into(), "b"), "b");
    }

    #[test]
    fn single_deactivate_clears() {
        assert_eq!(Single::on_deactivate(&"a".into(), "a"), "");
    }

    #[test]
    fn single_to_vec_empty() {
        assert_eq!(Single::to_vec(&String::new()), Vec::<String>::new());
    }

    #[test]
    fn single_to_vec_value() {
        assert_eq!(Single::to_vec(&"a".into()), vec!["a"]);
    }

    // ── Multiple mode (via trait) ───────────────────────────

    #[test]
    fn multiple_activate_appends() {
        assert_eq!(
            Multiple::on_activate(&vec!["a".into()], "b"),
            vec!["a", "b"]
        );
    }

    #[test]
    fn multiple_activate_from_empty() {
        assert_eq!(Multiple::on_activate(&vec![], "a"), vec!["a"]);
    }

    #[test]
    fn multiple_deactivate_removes() {
        assert_eq!(
            Multiple::on_deactivate(&vec!["a".into(), "b".into(), "c".into()], "b"),
            vec!["a", "c"]
        );
    }

    #[test]
    fn multiple_deactivate_nonexistent() {
        assert_eq!(
            Multiple::on_deactivate(&vec!["a".into(), "b".into()], "z"),
            vec!["a", "b"]
        );
    }
}
