use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use radix_leptos_separator::Separator;
use radix_leptos_toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupType};
use web_sys::wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * ToolbarContext
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ToolbarContextValue {
    orientation: Orientation,
    dir: Signal<Direction>,
}

/* -------------------------------------------------------------------------------------------------
 * Toolbar
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Toolbar(
    #[prop(optional)] orientation: Option<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let orientation = orientation.unwrap_or(Orientation::Horizontal);
    let direction = use_direction(dir);

    let context = ToolbarContextValue {
        orientation,
        dir: direction,
    };

    view! {
        <Provider value=context>
            <RovingFocusGroup
                as_child=true
                orientation=orientation
                dir=direction
                r#loop=Signal::derive(move || r#loop.get().unwrap_or(true))
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    attr:role="toolbar"
                    attr:aria-orientation=orientation.to_string()
                    attr:dir=move || direction.get().to_string()
                    attr:data-orientation=orientation.to_string()
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </RovingFocusGroup>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToolbarSeparator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToolbarSeparator(
    #[prop(into, optional)] decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ToolbarContextValue>();
    let perpendicular_orientation = match context.orientation {
        Orientation::Horizontal => Orientation::Vertical,
        Orientation::Vertical => Orientation::Horizontal,
    };

    // Map from roving_focus::Orientation to separator::Orientation
    let separator_orientation = match perpendicular_orientation {
        Orientation::Horizontal => radix_leptos_separator::Orientation::Horizontal,
        Orientation::Vertical => radix_leptos_separator::Orientation::Vertical,
    };

    view! {
        <Separator
            orientation=separator_orientation
            decorative=decorative
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Separator>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToolbarButton
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToolbarButton(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <RovingFocusGroupItem
            as_child=true
            focusable=Signal::derive(move || !disabled.get().unwrap_or(false))
        >
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:disabled=move || disabled.get().unwrap_or(false).then_some("")
            >
                {children.with_value(|children| children())}
            </Primitive>
        </RovingFocusGroupItem>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToolbarLink
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToolbarLink(
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <RovingFocusGroupItem as_child=true focusable=true>
            <Primitive
                element=html::a
                as_child=as_child
                node_ref=node_ref
                on:keydown=compose_callbacks(
                    on_key_down,
                    Some(Callback::new(|event: ev::KeyboardEvent| {
                        if event.key() == " "
                            && let Some(target) = event.current_target()
                            && let Ok(el) = target.dyn_into::<web_sys::HtmlElement>()
                        {
                            el.click();
                        }
                    })),
                    None,
                )
            >
                {children.with_value(|children| children())}
            </Primitive>
        </RovingFocusGroupItem>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToolbarToggleGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToolbarToggleGroup(
    r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<ToolbarContextValue>();
    let orientation = context.orientation;

    view! {
        <ToggleGroup
            r#type=r#type
            value=value
            default_value=default_value
            on_value_change=Callback::new(move |value: Vec<String>| {
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(value);
                }
            })
            disabled=disabled
            roving_focus=false
            orientation=orientation
            dir=context.dir
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=orientation.to_string()
        >
            {children.with_value(|children| children())}
        </ToggleGroup>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ToolbarToggleItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ToolbarToggleItem(
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <ToolbarButton as_child=true disabled=disabled>
            <ToggleGroupItem
                value=value
                disabled=disabled
                on_click=Callback::new(move |event: ev::MouseEvent| {
                    if let Some(on_click) = on_click {
                        on_click.run(event);
                    }
                })
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children())}
            </ToggleGroupItem>
        </ToolbarButton>
    }
}
