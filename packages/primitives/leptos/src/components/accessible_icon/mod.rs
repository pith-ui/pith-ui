//! Makes icons accessible to screen readers.
//!
//! Wraps an icon element with `aria-hidden="true"` and renders a visually
//! hidden label alongside it so screen readers announce the icon's purpose.
//!
//! # Anatomy
//!
//! ```text
//! <AccessibleIcon>
//!     {icon}
//! </AccessibleIcon>
//! ```
//!
//! # Features
//!
//! - Hides the icon from assistive technology
//! - Provides a visually hidden text label for screen readers

use crate::support::visually_hidden::VisuallyHidden;
use leptos::{
    attr::{aria_hidden, custom::custom_attribute},
    prelude::*,
};

/// Accessible icon wrapper.
///
/// Adds `aria-hidden="true"` and `focusable="false"` to child elements
/// and renders a [`VisuallyHidden`] label for screen readers.
#[component]
pub fn AccessibleIcon(
    /// The accessible label for the icon. This label will be visually hidden but announced to screen reader users, similar to `alt` text for `img` tags.
    #[prop(into)]
    label: Signal<String>,
    children: TypedChildren<impl IntoView + 'static>,
) -> impl IntoView {
    let label = Signal::derive(move || label.get());
    view! {
        <>
            {children
                .into_inner()()
                // Accessibility
                .add_any_attr(aria_hidden("true"))
                // See: https://allyjs.io/tutorials/focusing-in-svg.html#making-svg-elements-focusable
                .add_any_attr(custom_attribute("focusable", "false"))}
            <VisuallyHidden>{label.get()}</VisuallyHidden>
        </>
    }
}
