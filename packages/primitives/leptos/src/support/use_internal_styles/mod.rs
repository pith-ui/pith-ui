//! Hooks for applying internal CSS properties that must not be clobbered by user styles.
//!
//! # When to use `style:` directives instead
//!
//! Leptos's native `style:` directives (e.g., `style:outline="none"`) call
//! `el.style.setProperty()` under the hood and are the preferred way to set
//! internal styles. They are zero-overhead (no extra refs, no Effects) and
//! declarative. Use them whenever the styles are applied to a **native HTML
//! element** — either a raw `<div>`, `<span>`, etc., or a `<Primitive>` component
//! (which renders a native element).
//!
//! For reactive styles that should be conditionally set or removed, use a signal:
//! ```text
//! style:animation-duration=move || condition.get().then_some("0s")
//! ```
//!
//! # When to use these hooks
//!
//! Use `use_internal_styles` or `use_internal_styles_effect` when the `node_ref`
//! is passed across a **component boundary** — i.e., the ref is handed to another
//! Leptos component (like `<CollapsibleContent>`, `<PopperContent>`,
//! `<MenuContent>`) rather than directly to a native element. The `style:`
//! directive syntax requires `ElementExt`, which is only implemented for DOM
//! elements, not for component prop builders.
//!
//! These hooks work by creating a local ref, composing it with the caller's ref,
//! and running a reactive Effect that calls `el.style.setProperty()` after mount.
//! This has runtime cost (extra ref + Effect allocation), so prefer `style:`
//! directives when possible.
//!
//! # Summary
//!
//! | Target element              | Approach                  |
//! |-----------------------------|---------------------------|
//! | `<div>`, `<Primitive>`, etc | `style:prop="value"`      |
//! | `<SomeComponent>`           | `use_internal_styles()`   |

use crate::support::compose_refs::use_composed_refs;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

/// Apply static CSS properties via `style.setProperty()` on the DOM node,
/// avoiding conflicts with user-provided `attr:style` which overwrites
/// all inline styles via `setAttribute`.
///
/// Returns a composed ref that should be passed as `node_ref` to the element.
pub fn use_internal_styles(
    node_ref: AnyNodeRef,
    properties: &[(&'static str, &'static str)],
) -> AnyNodeRef {
    let properties: Vec<(&'static str, &'static str)> = properties.to_vec();
    use_internal_styles_effect(node_ref, move |style| {
        for (name, value) in &properties {
            let _ = style.set_property(name, value);
        }
    })
}

/// Apply reactive/conditional CSS properties via `style.setProperty()` on
/// the DOM node. The closure re-runs whenever any signal it reads changes.
///
/// Returns a composed ref that should be passed as `node_ref` to the element.
pub fn use_internal_styles_effect(
    node_ref: AnyNodeRef,
    apply: impl Fn(&web_sys::CssStyleDeclaration) + Send + Sync + 'static,
) -> AnyNodeRef {
    let local_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, local_ref]);

    Effect::new(move |_| {
        if let Some(node) = local_ref.get() {
            let el: &web_sys::HtmlElement = node.unchecked_ref();
            apply(&el.style());
        }
    });

    composed_ref
}
