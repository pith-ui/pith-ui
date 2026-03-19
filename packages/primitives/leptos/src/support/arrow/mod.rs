//! SVG arrow element for floating UI components.
//!
//! Renders an SVG arrow (caret/triangle) used by popper-positioned
//! components like popover, tooltip, and hover card to visually connect
//! the floating content to its anchor.

use crate::support::primitive::Primitive;
use leptos::{
    attr::{NextAttribute, custom::custom_attribute},
    prelude::*,
    svg,
};
use leptos_node_ref::AnyNodeRef;

/// SVG arrow component.
///
/// Renders a triangular SVG shape. Typically used inside popper-positioned
/// content as a visual connector to the anchor element.
#[component]
pub fn Arrow(
    #[prop(into, optional, default=10.0.into())] width: MaybeProp<f64>,
    #[prop(into, optional, default=5.0.into())] height: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let attrs = custom_attribute("viewBox", "0 0 30 10")
        .add_any_attr(custom_attribute("preserveAspectRatio", "none"));

    view! {
        <Primitive
            element=svg::svg
            as_child=as_child
            attr:width=move || width.get().unwrap_or(10.0)
            attr:height=move || height.get().unwrap_or(5.0)
            node_ref={node_ref}
            {..attrs}
        >
            <Show
                when=move || as_child.get().unwrap_or_default()
                fallback=move || {
                    view! {
                        <polygon points="0,0 30,0 15,10" />
                    }
                }
            >
                {children.with_value(|maybe_children| maybe_children.as_ref().map(|child_fn| child_fn()))}
            </Show>
        </Primitive>
    }
}
