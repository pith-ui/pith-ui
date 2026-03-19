//! Visual or semantic separator between content sections.
//!
//! Renders a `<div>` with `role="separator"` by default, or `role="none"`
//! when used purely as a visual decoration.
//!
//! Implements the [WAI-ARIA Separator role](https://www.w3.org/TR/wai-aria-1.2/#separator).
//!
//! # Anatomy
//!
//! ```text
//! <Separator />
//! ```
//!
//! # Features
//!
//! - Horizontal or vertical orientation
//! - Decorative mode removes the element from the accessibility tree
//!
//! # Data Attributes
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-orientation` | `horizontal`, `vertical` |

use std::fmt::{Display, Formatter};

use crate::support::primitive::{Primitive, prop_or_default};
use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;

/// The orientation of a separator.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    /// Horizontal separator (default).
    #[default]
    Horizontal,
    /// Vertical separator.
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
        )
    }
}

/// Separator component.
///
/// Renders as a `<div>` with `role="separator"` (or `role="none"` when
/// `decorative` is true). Supports horizontal and vertical orientations.
#[component]
pub fn Separator(
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    /// When true, renders with `role="none"` instead of `role="separator"`,
    /// removing the element from the accessibility tree.
    #[prop(into, optional)]
    decorative: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let orientation = prop_or_default(orientation);
    let decorative = prop_or_default(decorative);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=move || orientation.get().to_string()
            attr:aria-orientation=move || {
                if decorative.get() {
                    None
                } else {
                    match orientation.get() {
                        Orientation::Vertical => Some("vertical".to_owned()),
                        Orientation::Horizontal => None,
                    }
                }
            }
            attr:role=move || if decorative.get() { "none" } else { "separator" }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}
