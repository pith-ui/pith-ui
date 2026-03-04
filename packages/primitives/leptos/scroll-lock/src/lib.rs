//! Leptos port of Radix Scroll Lock.
//!
//! This is an internal utility, not intended for public usage.
//!
//! Simplified body scroll lock: sets `overflow: hidden` on body while mounted.
//! React uses `react-remove-scroll` which supports shards (allowing scroll on specific elements
//! like the content) and pinch-zoom. This simplified version just hides body overflow.

mod scroll_lock;

pub use scroll_lock::*;
