//! Leptos port of the `aria-hidden` library used by Radix.
//!
//! This is an internal utility, not intended for public usage.
//!
//! Simplified `aria-hidden` implementation: sets `aria-hidden="true"` on body's direct children
//! that don't contain the target element, and restores on cleanup.
//! React uses the `aria-hidden` npm package which walks the tree more thoroughly.
//! This simplified version only hides body's direct children.

mod aria_hidden;

pub use aria_hidden::*;
