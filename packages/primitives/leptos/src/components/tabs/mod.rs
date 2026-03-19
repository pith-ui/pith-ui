//! Tabbed interface for switching between panels of content.
//!
//! A set of layered content sections (tab panels) displayed one at a time,
//! with tabs for switching between them. Supports automatic and manual
//! activation, keyboard navigation with roving focus, and RTL direction.
//!
//! Implements the [WAI-ARIA Tabs pattern](https://www.w3.org/WAI/ARIA/apd/patterns/tabs/).
//!
//! # Anatomy
//!
//! ```text
//! <Tabs>
//!     <TabsList>
//!         <TabsTrigger />
//!         <TabsTrigger />
//!     </TabsList>
//!     <TabsContent />
//!     <TabsContent />
//! </Tabs>
//! ```
//!
//! # Features
//!
//! - Controlled and uncontrolled active tab state
//! - Automatic activation on focus or manual activation on click/Enter
//! - Roving focus with arrow key navigation
//! - Horizontal and vertical orientation
//! - RTL support
//!
//! # Keyboard Interactions
//!
//! | Key | Action |
//! |-----|--------|
//! | Tab | Moves focus into the tab list, then to the active panel |
//! | ArrowRight | Focuses next tab (horizontal) |
//! | ArrowLeft | Focuses previous tab (horizontal) |
//! | ArrowDown | Focuses next tab (vertical) |
//! | ArrowUp | Focuses previous tab (vertical) |
//! | Home | Focuses first tab |
//! | End | Focuses last tab |
//! | Space / Enter | Activates the focused tab (manual mode) |
//!
//! # Data Attributes
//!
//! **TabsTrigger, TabsContent:**
//!
//! | Attribute | Values |
//! |-----------|--------|
//! | `data-state` | `active`, `inactive` |
//! | `data-disabled` | Present when disabled (trigger only) |
//! | `data-orientation` | `horizontal`, `vertical` |

use crate::support::compose_refs::use_composed_refs;
use crate::support::direction::{Direction, use_direction};
use crate::support::id::use_id;
use crate::support::presence::Presence;
use crate::support::primitive::{
    Primitive, adapt_callback, compose_callbacks, data_attr, prop_or, prop_or_default,
};
use crate::support::roving_focus::{RovingFocusGroup, RovingFocusGroupItem};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

// Re-export Orientation from roving-focus so consumers don't need a separate import.
pub use crate::support::roving_focus::Orientation;

/// Controls when tabs activate: on focus or on explicit interaction.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ActivationMode {
    /// Tab activates when it receives focus (default).
    #[default]
    Automatic,
    /// Tab activates only on click or Enter/Space.
    Manual,
}

#[derive(Clone)]
struct TabsContextValue {
    base_id: ReadSignal<String>,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
    orientation: Signal<Orientation>,
    dir: Signal<Direction>,
    activation_mode: Signal<ActivationMode>,
}

/// Root tabs component.
///
/// Renders as a `<div>`. Manages active tab state and provides context
/// for [`TabsList`], [`TabsTrigger`], and [`TabsContent`].
#[component]
pub fn Tabs(
    /// The controlled value of the active tab.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The default active tab value (uncontrolled).
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Callback when the active tab changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// The orientation of the tabs. Determines arrow key navigation direction.
    #[prop(into, optional)]
    orientation: MaybeProp<Orientation>,
    /// The reading direction.
    #[prop(into, optional)]
    dir: MaybeProp<Direction>,
    /// Whether tabs activate automatically on focus or manually on click/Enter.
    #[prop(into, optional)]
    activation_mode: MaybeProp<ActivationMode>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let direction = use_direction(dir);
    let orientation = prop_or(orientation, Orientation::Horizontal);
    let activation_mode = prop_or_default(activation_mode);
    let base_id = use_id(None);

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: adapt_callback(on_value_change),
    });

    let on_value_change_cb = Callback::new(move |value: String| {
        set_value.run(Some(value));
    });

    let context = TabsContextValue {
        base_id,
        value: value_signal,
        on_value_change: on_value_change_cb,
        orientation,
        dir: direction,
        activation_mode,
    };

    view! {
        <Provider value=context>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:dir=move || direction.get().to_string()
                attr:data-orientation=move || orientation.get().to_string()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

/// Container for tab triggers with roving focus.
///
/// Renders as a `<div>` with `role="tablist"`. Wraps triggers in a
/// [`RovingFocusGroup`] for arrow key navigation. Must be a descendant
/// of [`Tabs`].
#[component]
pub fn TabsList(
    /// Whether keyboard navigation loops around. Default true.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<TabsContextValue>();

    let orientation = context.orientation;

    view! {
        <RovingFocusGroup
            as_child=true
            orientation=Signal::derive(move || Some(orientation.get()))
            dir=context.dir
            r#loop=prop_or(r#loop, true)
        >
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:role="tablist"
                attr:aria-orientation=move || orientation.get().to_string()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </RovingFocusGroup>
    }
}

/// Button that activates its associated tab panel.
///
/// Renders as a `<button>` with `role="tab"` and `aria-selected`.
/// Must be a descendant of [`TabsList`].
#[component]
pub fn TabsTrigger(
    /// A unique value identifying this tab.
    #[prop(into)]
    value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_mouse_down: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<TabsContextValue>();
    let trigger_value = StoredValue::new(value);
    let disabled = prop_or_default(disabled);

    let trigger_id =
        Signal::derive(move || make_trigger_id(&context.base_id.get(), &trigger_value.get_value()));
    let content_id =
        Signal::derive(move || make_content_id(&context.base_id.get(), &trigger_value.get_value()));
    let is_selected = Signal::derive(move || {
        context
            .value
            .get()
            .is_some_and(|v| v == trigger_value.get_value())
    });

    // Compose the user's on_focus with automatic activation into a single handler.
    // This is passed through RovingFocusGroupItem's on_focus prop so that only ONE
    // on:focus listener is registered on the DOM element, avoiding duplicate event
    // handler closures that can lead to WASM "closure invoked after being dropped"
    // panics when multiple focus listeners interact with the reactive system.
    let composed_on_focus: Callback<ev::FocusEvent> = Callback::new(compose_callbacks(
        on_focus,
        Some(Callback::new(move |_: ev::FocusEvent| {
            // Handle "automatic" activation if necessary:
            // activate tab following focus.
            let is_automatic_activation = context.activation_mode.get() != ActivationMode::Manual;
            if !is_selected.get() && !disabled.get() && is_automatic_activation {
                // Defer the value change to a macrotask (setTimeout 0) to avoid
                // re-entrant signal updates in WASM. In React, setState is batched;
                // in Leptos, signal updates are synchronous. When RovingFocusGroup's
                // keydown handler calls focus_first() which triggers this focus handler,
                // the synchronous signal update can cause reactive effects while the
                // keydown closure is still on the call stack.
                let on_value_change = context.on_value_change;
                let value = trigger_value.get_value();
                let window = web_sys::window().expect("Window should exist.");
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        Closure::once_into_js(move || {
                            on_value_change.run(value);
                        })
                        .unchecked_ref(),
                        0,
                    )
                    .expect("setTimeout should succeed.");
            }
        })),
        None,
    ));

    view! {
        <RovingFocusGroupItem
            as_child=true
            focusable=Signal::derive(move || !disabled.get())
            active=is_selected
            on_focus=composed_on_focus
        >
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:role="tab"
                attr:aria-selected=move || is_selected.get().to_string()
                attr:aria-controls=move || content_id.get()
                attr:data-state=move || if is_selected.get() { "active" } else { "inactive" }
                attr:data-disabled=data_attr(disabled)
                attr:disabled=data_attr(disabled)
                attr:id=move || trigger_id.get()
                on:mousedown=compose_callbacks(on_mouse_down, Some(Callback::new(move |event: ev::MouseEvent| {
                    // Only call handler if it's the left button (mousedown gets triggered by all mouse buttons)
                    // but not when the control key is pressed (avoiding MacOS right click).
                    if !disabled.get() && event.button() == 0 && !event.ctrl_key() {
                        context.on_value_change.run(trigger_value.get_value());
                    } else {
                        // Prevent focus to avoid accidental activation.
                        event.prevent_default();
                    }
                })), None)
                on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                    if [" ", "Enter"].contains(&event.key().as_str()) {
                        context.on_value_change.run(trigger_value.get_value());
                    }
                })), None)
            >
                {children.with_value(|children| children())}
            </Primitive>
        </RovingFocusGroupItem>
    }
}

/// Content panel associated with a tab trigger.
///
/// Renders as a `<div>` with `role="tabpanel"`. Shown when its matching
/// trigger is active, hidden otherwise. Must be a descendant of [`Tabs`].
#[component]
pub fn TabsContent(
    /// A unique value matching the corresponding TabsTrigger.
    #[prop(into)]
    value: String,
    /// Force mount the content even when inactive (for animation control).
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<TabsContextValue>();
    let content_value = StoredValue::new(value);

    let trigger_id =
        Signal::derive(move || make_trigger_id(&context.base_id.get(), &content_value.get_value()));
    let content_id =
        Signal::derive(move || make_content_id(&context.base_id.get(), &content_value.get_value()));
    let is_selected = Signal::derive(move || {
        context
            .value
            .get()
            .is_some_and(|v| v == content_value.get_value())
    });

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || is_selected.get());

    let presence_ref = AnyNodeRef::new();

    view! {
        <Presence present=present node_ref=presence_ref>
            <TabsContentImpl
                trigger_id=trigger_id
                content_id=content_id
                is_selected=is_selected
                orientation=context.orientation
                as_child=as_child
                node_ref=node_ref
                presence_ref=presence_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </TabsContentImpl>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * TabsContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn TabsContentImpl(
    trigger_id: Signal<String>,
    content_id: Signal<String>,
    is_selected: Signal<bool>,
    orientation: Signal<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let composed_ref = use_composed_refs(vec![node_ref, presence_ref]);

    // Prevent animation on initial mount: if the tab is already selected when it first mounts,
    // suppress the entry animation by setting animation-duration to 0s for one frame.
    let is_mount_animation_prevented = RwSignal::new(is_selected.get_untracked());

    let raf_closure: SendWrapper<Closure<dyn Fn()>> = SendWrapper::new(Closure::new(move || {
        is_mount_animation_prevented.set(false);
    }));
    let raf_closure = StoredValue::new(raf_closure);

    // Schedule rAF on mount (client-only — window is unavailable during SSR)
    let raf_id: RwSignal<Option<i32>> = RwSignal::new(None);
    if let Some(window) = web_sys::window() {
        raf_closure.with_value(|closure| {
            if let Ok(id) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
                raf_id.set(Some(id));
            }
        });
    }

    // Cancel the rAF when the component unmounts to prevent invoking a dropped
    // Closure. When a tab change causes this content to unmount via Presence,
    // the StoredValue (and its Closure) is dropped. Without cancellation, the
    // browser would fire the rAF callback on the dropped Closure, causing a
    // WASM "closure invoked after being dropped" panic.
    Owner::on_cleanup(move || {
        if let Some(id) = raf_id.get_untracked() {
            if let Some(window) = web_sys::window() {
                window.cancel_animation_frame(id).ok();
            }
        }
    });

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_ref
            style:animation-duration=move || is_mount_animation_prevented.get().then_some("0s")
            attr:data-state=move || if is_selected.get() { "active" } else { "inactive" }
            attr:data-orientation=move || orientation.get().to_string()
            attr:role="tabpanel"
            attr:aria-labelledby=move || trigger_id.get()
            attr:id=move || content_id.get()
            attr:hidden=move || (!is_selected.get()).then_some("")
            attr:tabindex="0"
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn make_trigger_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-trigger-{value}")
}

fn make_content_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-content-{value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_id_format() {
        assert_eq!(make_trigger_id("tabs-1", "tab1"), "tabs-1-trigger-tab1");
    }

    #[test]
    fn content_id_format() {
        assert_eq!(make_content_id("tabs-1", "tab1"), "tabs-1-content-tab1");
    }

    #[test]
    fn trigger_id_empty_values() {
        assert_eq!(make_trigger_id("", ""), "-trigger-");
    }

    #[test]
    fn content_id_empty_values() {
        assert_eq!(make_content_id("", ""), "-content-");
    }
}
