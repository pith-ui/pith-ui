//! Text label for form controls.
//!
//! Renders a `<label>` element with built-in double-click text selection
//! prevention when the label wraps interactive controls.
//!
//! # Anatomy
//!
//! ```text
//! <Label />
//! ```
//!
//! # Features
//!
//! - Native `<label>` semantics (use `attr:r#for` for association)
//! - Prevents text selection on double-click when wrapping controls

use crate::support::primitive::Primitive;
use leptos::{ev::MouseEvent, html, prelude::*};
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;

/// Label component for form controls.
///
/// Renders a native `<label>` element. Prevents accidental text selection
/// when double-clicking on labels that wrap interactive elements.
#[component]
pub fn Label(
    #[prop(into, optional)] on_mouse_down: MaybeCallback<MouseEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::label
            as_child=as_child
            node_ref=node_ref
            on:mousedown=move |event: MouseEvent| {
                // Only prevent text selection if clicking inside the label itself.
                let target = event_target::<web_sys::Element>(&event);
                if target
                    .closest("button, input, select, textarea")
                    .expect("Element should be able to query closest.")
                    .is_some()
                {
                    return;
                }

                on_mouse_down.run(event.clone());

                // Prevent text selection when double clicking label.
                if !event.default_prevented() && event.detail() > 1 {
                    event.prevent_default();
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
