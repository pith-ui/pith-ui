//! Leptos port of [Radix Primitives](https://www.radix-ui.com/primitives).
//!
//! A set of accessible, unstyled UI components for building high-quality design systems and web apps.
//!
//! See [`@radix-ui/react-primitives`](https://www.radix-ui.com/primitives) for the original library.

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

// ── Re-export support modules at crate level ──
pub use support::announce;
pub use support::aria_hidden;
pub use support::arrow;
pub use support::collection;
pub use support::compose_refs;
pub use support::direction;
pub use support::dismissable_layer;
pub use support::focus_guards;
pub use support::focus_scope;
pub use support::id;
pub use support::popper;
pub use support::portal;
pub use support::presence;
pub use support::primitive;
pub use support::roving_focus;
pub use support::scroll_lock;
pub use support::use_controllable_state;
pub use support::use_escape_keydown;
pub use support::use_previous;
pub use support::use_rect;
pub use support::use_size;
pub use support::visually_hidden;

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
#[cfg(feature = "checkbox")]
pub use components::checkbox;
#[cfg(feature = "collapsible")]
pub use components::collapsible;
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
