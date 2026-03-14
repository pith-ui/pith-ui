//! Hooks for applying internal CSS properties across component boundaries.
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
//! Use `use_internal_styles`, `use_forced_styles`, or `use_internal_styles_effect`
//! when the `node_ref` is passed across a **component boundary** — i.e., the ref
//! is handed to another Leptos component (like `<CollapsibleContent>`,
//! `<PopperContent>`, `<MenuContent>`) rather than directly to a native element.
//! The `style:` directive syntax requires `ElementExt`, which is only implemented
//! for DOM elements, not for component prop builders.
//!
//! These hooks work by creating a local ref, composing it with the caller's ref,
//! and running a reactive Effect that calls `el.style.setProperty()` after mount.
//! This has runtime cost (extra ref + Effect allocation), so prefer `style:`
//! directives when possible.
//!
//! # Choosing between `use_internal_styles` and `use_forced_styles`
//!
//! React Radix uses two different style merge orders depending on whether the
//! internal styles are **defaults** or **derived state**:
//!
//! - **Defaults** (`{internal, ...props.style}` — user wins): Simple styles like
//!   `outline: none` or `pointer-events: none` that provide sensible defaults.
//!   Users may override these. Use [`use_internal_styles`].
//!
//! - **Derived state** (`{...props.style, internal}` — internal wins): CSS custom
//!   property aliases that must always reflect current popper/positioning state
//!   (e.g., `--radix-popover-content-available-width: var(--radix-popper-available-width)`).
//!   These are output values, not configuration. Use [`use_forced_styles`].
//!
//! # Summary
//!
//! | Target element              | Overridable?   | Approach                  |
//! |-----------------------------|----------------|---------------------------|
//! | `<div>`, `<Primitive>`, etc | N/A            | `style:prop="value"`      |
//! | `<SomeComponent>`           | Yes (default)  | `use_internal_styles()`   |
//! | `<SomeComponent>`           | No (derived)   | `use_forced_styles()`     |

use crate::support::compose_refs::use_composed_refs;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

/// Apply static CSS properties as **overridable defaults** via
/// `style.setProperty()` on the DOM node. Properties already set by the user
/// (via `attr:style`) are skipped so that user styles win.
///
/// Use this for simple internal defaults like `outline: none` or
/// `pointer-events: none` that users may legitimately want to override.
///
/// Returns a composed ref that should be passed as `node_ref` to the element.
pub fn use_internal_styles(
    node_ref: AnyNodeRef,
    properties: &[(&'static str, &'static str)],
) -> AnyNodeRef {
    let properties: Vec<(&'static str, &'static str)> = properties.to_vec();
    use_internal_styles_effect(node_ref, move |style| {
        for (name, value) in &properties {
            // Only set if the user hasn't already provided this property.
            if style.get_property_value(name).unwrap_or_default().is_empty() {
                let _ = style.set_property(name, value);
            }
        }
    })
}

/// Apply static CSS properties as **non-overridable state** via
/// `style.setProperty()` on the DOM node. These always overwrite any
/// user-provided values for the same properties.
///
/// Use this for CSS custom property aliases that must reflect current
/// positioning/layout state (e.g., popper-derived `--radix-*-available-width`).
///
/// Returns a composed ref that should be passed as `node_ref` to the element.
pub fn use_forced_styles(
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
