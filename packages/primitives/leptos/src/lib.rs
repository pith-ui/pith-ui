//! Leptos port of [Radix Primitives](https://www.radix-ui.com/primitives).
//!
//! A set of accessible, unstyled UI components for building high-quality
//! design systems and web apps in [Leptos](https://leptos.dev).
//!
//! # Feature Flags
//!
//! Each component is behind a Cargo feature flag matching its name.
//! Enable only the components you need:
//!
//! ```toml
//! [dependencies]
//! cardo-ui = { version = "0.0.2", features = ["checkbox", "dialog", "tabs"] }
//! ```
//!
//! # Public Support Modules
//!
//! These modules are always available (no feature flag required):
//!
//! - [`arrow`] — SVG arrow for floating UI components
//! - [`direction`] — RTL/LTR direction context
//! - [`portal`] — Render children into a different DOM node
//! - [`primitive`] — Base rendering primitive with `as_child` support
//! - [`visually_hidden`] — Screen-reader-only content
//!
//! # Internal Support Modules
//!
//! Building blocks used internally by components. Exposed behind the
//! `internals` feature for stories, testing, and advanced use cases.
//! Not part of the stable public API.

// Pre-existing style lints across component modules — not worth fixing individually during
// the mega-crate consolidation. These should be cleaned up incrementally.
#![allow(clippy::collapsible_if)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

mod components;
mod internal;
mod support;

#[cfg(feature = "colors")]
pub mod colors;

#[cfg(feature = "icons")]
pub mod icons;

// ── Re-export public support modules at crate level ──
pub use support::arrow;
pub use support::class;
pub use support::direction;
pub use support::portal;
pub use support::primitive;
pub use support::visually_hidden;

// ── Re-export internal support modules behind `internals` feature ──
// These are building blocks used by components internally. Not part of the
// public consumer API, but exposed for stories, testing, and power users.
#[cfg(feature = "internals")]
pub use support::announce;
#[cfg(feature = "internals")]
pub use support::aria_hidden;
#[cfg(feature = "internals")]
pub use support::collection;
#[cfg(feature = "internals")]
pub use support::compose_refs;
#[cfg(feature = "internals")]
pub use support::dismissable_layer;
#[cfg(feature = "internals")]
pub use support::focus_guards;
#[cfg(feature = "internals")]
pub use support::focus_scope;
#[cfg(feature = "internals")]
pub use support::id;
#[cfg(feature = "internals")]
pub use support::popper;
#[cfg(feature = "internals")]
pub use support::presence;
#[cfg(feature = "internals")]
pub use support::roving_focus;
#[cfg(feature = "internals")]
pub use support::scroll_lock;
#[cfg(feature = "internals")]
pub use support::use_controllable_state;
#[cfg(feature = "internals")]
pub use support::use_escape_keydown;
#[cfg(feature = "internals")]
pub use support::use_previous;
#[cfg(feature = "internals")]
pub use support::use_rect;
#[cfg(feature = "internals")]
pub use support::use_size;

// ── Re-export feature-gated components at crate level ──
#[cfg(feature = "accessible-icon")]
pub use components::accessible_icon;
#[cfg(feature = "accordion")]
pub use components::accordion;
#[cfg(feature = "alert-dialog")]
pub use components::alert_dialog;
#[cfg(feature = "aspect-ratio")]
pub use components::aspect_ratio;
#[cfg(feature = "avatar")]
pub use components::avatar;
#[cfg(feature = "calendar")]
pub use components::calendar;
#[cfg(feature = "checkbox")]
pub use components::checkbox;
#[cfg(feature = "collapsible")]
pub use components::collapsible;
#[cfg(feature = "combobox")]
pub use components::combobox;
#[cfg(feature = "context-menu")]
pub use components::context_menu;
#[cfg(feature = "dialog")]
pub use components::dialog;
#[cfg(feature = "dropdown-menu")]
pub use components::dropdown_menu;
#[cfg(feature = "form")]
pub use components::form;
#[cfg(feature = "hover-card")]
pub use components::hover_card;
#[cfg(feature = "label")]
pub use components::label;
#[cfg(feature = "menu")]
pub use components::menu;
#[cfg(feature = "menubar")]
pub use components::menubar;
#[cfg(feature = "navigation-menu")]
pub use components::navigation_menu;
#[cfg(feature = "one-time-password-field")]
pub use components::one_time_password_field;
#[cfg(feature = "password-toggle-field")]
pub use components::password_toggle_field;
#[cfg(feature = "popover")]
pub use components::popover;
#[cfg(feature = "progress")]
pub use components::progress;
#[cfg(feature = "radio-group")]
pub use components::radio_group;
#[cfg(feature = "scroll-area")]
pub use components::scroll_area;
#[cfg(feature = "select")]
pub use components::select;
#[cfg(feature = "separator")]
pub use components::separator;
#[cfg(feature = "slider")]
pub use components::slider;
#[cfg(feature = "switch")]
pub use components::switch;
#[cfg(feature = "tabs")]
pub use components::tabs;
#[cfg(feature = "toast")]
pub use components::toast;
#[cfg(feature = "toggle")]
pub use components::toggle;
#[cfg(feature = "toggle-group")]
pub use components::toggle_group;
#[cfg(feature = "toolbar")]
pub use components::toolbar;
#[cfg(feature = "tooltip")]
pub use components::tooltip;
