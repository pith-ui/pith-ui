//! Leptos port of [Radix Primitives](https://www.radix-ui.com/primitives).
//!
//! A set of accessible, unstyled UI components for building high-quality design systems and web apps.
//!
//! See [`@radix-ui/react-primitives`](https://www.radix-ui.com/primitives) for the original library.

// Pre-existing style lints across component modules — not worth fixing individually during
// the mega-crate consolidation. These should be cleaned up incrementally.
#![allow(clippy::collapsible_if)]
#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

// ── Always-compiled internal utilities ──
pub mod announce;
pub mod aria_hidden;
pub mod arrow;
pub mod collection;
pub mod compose_refs;
pub mod direction;
pub mod dismissable_layer;
pub mod focus_guards;
pub mod focus_scope;
pub mod id;
pub mod popper;
pub mod portal;
pub mod presence;
pub mod primitive;
pub mod roving_focus;
pub mod scroll_lock;
pub mod use_controllable_state;
pub mod use_escape_keydown;
pub mod use_previous;
pub mod use_rect;
pub mod use_size;
pub mod visually_hidden;

// ── Feature-gated user-facing components ──
#[cfg(feature = "accessible-icon")]
pub mod accessible_icon;
#[cfg(feature = "accordion")]
pub mod accordion;
#[cfg(feature = "alert-dialog")]
pub mod alert_dialog;
#[cfg(feature = "aspect-ratio")]
pub mod aspect_ratio;
#[cfg(feature = "avatar")]
pub mod avatar;
#[cfg(feature = "checkbox")]
pub mod checkbox;
#[cfg(feature = "collapsible")]
pub mod collapsible;
#[cfg(feature = "context-menu")]
pub mod context_menu;
#[cfg(feature = "dialog")]
pub mod dialog;
#[cfg(feature = "dropdown-menu")]
pub mod dropdown_menu;
#[cfg(feature = "form")]
pub mod form;
#[cfg(feature = "hover-card")]
pub mod hover_card;
#[cfg(feature = "label")]
pub mod label;
#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "menubar")]
pub mod menubar;
#[cfg(feature = "navigation-menu")]
pub mod navigation_menu;
#[cfg(feature = "one-time-password-field")]
pub mod one_time_password_field;
#[cfg(feature = "password-toggle-field")]
pub mod password_toggle_field;
#[cfg(feature = "popover")]
pub mod popover;
#[cfg(feature = "progress")]
pub mod progress;
#[cfg(feature = "radio-group")]
pub mod radio_group;
#[cfg(feature = "scroll-area")]
pub mod scroll_area;
#[cfg(feature = "select")]
pub mod select;
#[cfg(feature = "separator")]
pub mod separator;
#[cfg(feature = "slider")]
pub mod slider;
#[cfg(feature = "switch")]
pub mod switch;
#[cfg(feature = "tabs")]
pub mod tabs;
#[cfg(feature = "toast")]
pub mod toast;
#[cfg(feature = "toggle")]
pub mod toggle;
#[cfg(feature = "toggle-group")]
pub mod toggle_group;
#[cfg(feature = "toolbar")]
pub mod toolbar;
#[cfg(feature = "tooltip")]
pub mod tooltip;
