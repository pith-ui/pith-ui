//! Layout primitive for maintaining a fixed aspect ratio.
//!
//! Wraps content in a container that enforces a width-to-height ratio
//! using the CSS padding-bottom technique. Useful for responsive images,
//! videos, and maps.
//!
//! # Anatomy
//!
//! ```text
//! <AspectRatio />
//! ```
//!
//! # Features
//!
//! - Reactive ratio via signal
//! - Content fills the ratio-constrained area

use crate::support::primitive::Primitive;
use leptos::{attribute_interceptor::AttributeInterceptor, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// Aspect ratio container.
///
/// Renders an outer wrapper `<div>` with a padding-bottom ratio constraint
/// and an inner `<div>` that fills the space. The inner element is
/// replaceable via `as_child`.
#[component]
pub fn AspectRatio(
    #[prop(into, optional, default = 1.0.into())] ratio: Signal<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <div
                // Ensures inner element is contained
                style:position="relative"
                // Ensures padding bottom trick maths works
                style:width="100%"
                style:padding-bottom=move || format!("{}%", 100.0 / ratio.get())
                data-radix-aspect-ratio-wrapper=""
            >
                <Primitive
                    {..attrs}
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    // Ensures children expand in ratio
                    style:position="absolute"
                    style:top="0px"
                    style:right="0px"
                    style:bottom="0px"
                    style:left="0px"
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </div>
        </AttributeInterceptor>
    }
}
