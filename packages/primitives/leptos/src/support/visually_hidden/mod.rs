//! Visually hidden content for screen readers.
//!
//! Renders content that is visually hidden but remains accessible to
//! screen readers and other assistive technology. Uses the CSS clip
//! technique to hide the element without removing it from the
//! accessibility tree.

use crate::support::primitive::Primitive;
use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// Visually hidden component.
///
/// Renders as a `<span>` with CSS styles that hide it visually while
/// keeping it in the accessibility tree. Commonly used for accessible
/// labels and descriptions.
#[component]
pub fn VisuallyHidden(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref

            // See: https://github.com/twbs/bootstrap/blob/main/scss/mixins/_visually-hidden.scss
            style:position="absolute"
            style:border="0px"
            style:width="1px"
            style:height="1px"
            style:padding="0px"
            style:margin="-1px"
            style:overflow="hidden"
            style:clip="rect(0, 0, 0, 0)"
            style:white-space="nowrap"
            style:word-wrap="normal"
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
