use std::collections::HashMap;

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

use crate::use_state_machine::use_state_machine;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MachineState {
    Mounted,
    UnmountSuspended,
    Unmounted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum MachineEvent {
    Mount,
    AnimationOut,
    AnimationEnd,
    Unmount,
}

/// Presence component that controls mount/unmount of children based on the `present` signal,
/// with support for CSS animation-based exit transitions.
///
/// The caller must pass a `node_ref` that is also applied to the child element.
/// Presence observes animation events on that ref to delay unmount during exit animations.
#[component]
pub fn Presence(
    #[prop(into)] present: Signal<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let is_present = use_presence(present, node_ref);

    view! {
        <Show when=move || is_present.get()>
            {children.with_value(|children| children())}
        </Show>
    }
}

fn use_presence(present: Signal<bool>, node_ref: AnyNodeRef) -> Signal<bool> {
    let styles: RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>> = RwSignal::new(None);
    let prev_present = RwSignal::new(present.get_untracked());
    let prev_animation_name = RwSignal::new("none".to_string());
    let initial_state = match present.get_untracked() {
        true => MachineState::Mounted,
        false => MachineState::Unmounted,
    };
    let (state, send) = use_state_machine(
        initial_state,
        HashMap::from([
            (
                MachineState::Mounted,
                HashMap::from([
                    (MachineEvent::Unmount, MachineState::Unmounted),
                    (MachineEvent::AnimationOut, MachineState::UnmountSuspended),
                ]),
            ),
            (
                MachineState::UnmountSuspended,
                HashMap::from([
                    (MachineEvent::Mount, MachineState::Mounted),
                    (MachineEvent::AnimationEnd, MachineState::Unmounted),
                ]),
            ),
            (
                MachineState::Unmounted,
                HashMap::from([(MachineEvent::Mount, MachineState::Mounted)]),
            ),
        ]),
    );

    Effect::new(move |_| {
        let current_animation_name = get_animation_name(styles.get().as_deref());
        prev_animation_name.set(match state.get() {
            MachineState::Mounted => current_animation_name,
            _ => "none".into(),
        });
    });

    Effect::new(move |_| {
        let styles = styles.get();
        let was_present = prev_present.get();
        let is_present = present.get();
        let has_present_changed = was_present != is_present;

        if has_present_changed {
            let prev_animation_name = prev_animation_name.get();
            let current_animation_name = get_animation_name(styles.as_deref());

            if is_present {
                send.run(MachineEvent::Mount);
            } else if current_animation_name == "none"
                || styles
                    .as_deref()
                    .and_then(|styles| styles.get_property_value("display").ok())
                    == Some("none".into())
            {
                // If there is no exit animation or the element is hidden, animations won't run so we unmount instantly.
                send.run(MachineEvent::Unmount);
            } else {
                // When `present` changes to `false`, we check changes to animation-name to
                // determine whether an animation has started. We chose this approach (reading
                // computed styles) because there is no `animationrun` event and `animationstart`
                // fires after `animation-delay` has expired which would be too late.
                let is_animating = prev_animation_name != current_animation_name;

                if was_present && is_animating {
                    send.run(MachineEvent::AnimationOut);
                } else {
                    send.run(MachineEvent::Unmount);
                }
            }

            prev_present.set(is_present);
        }
    });

    // Triggering an ANIMATION_OUT during an ANIMATION_IN will fire an `animationcancel`
    // event for ANIMATION_IN after we have entered `unmountSuspended` state. So, we
    // make sure we only trigger ANIMATION_END for the currently active animation.
    let handle_animation_end: SendWrapper<Closure<dyn Fn(web_sys::AnimationEvent)>> =
        SendWrapper::new(Closure::new(move |event: web_sys::AnimationEvent| {
            let current_animation_name = get_animation_name(styles.get_untracked().as_deref());
            let is_current_animation = current_animation_name.contains(&event.animation_name());
            if is_current_animation
                && event.target().as_ref()
                    == node_ref
                        .get_untracked()
                        .as_ref()
                        .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
            {
                send.run(MachineEvent::AnimationEnd);
            }
        }));
    let handle_animation_end = StoredValue::new(handle_animation_end);

    let handle_animation_start: SendWrapper<Closure<dyn Fn(web_sys::AnimationEvent)>> =
        SendWrapper::new(Closure::new(move |event: web_sys::AnimationEvent| {
            if event.target().as_ref()
                == node_ref
                    .get_untracked()
                    .as_ref()
                    .map(|node| node.unchecked_ref::<web_sys::EventTarget>())
            {
                // If animation occurred, store its name as the previous animation.
                prev_animation_name.set(get_animation_name(styles.get_untracked().as_deref()));
            }
        }));
    let handle_animation_start = StoredValue::new(handle_animation_start);

    Effect::new(move |_| {
        if let Some(node) = node_ref.get() {
            let node: &web_sys::EventTarget = node.unchecked_ref();
            handle_animation_start.with_value(|closure| {
                node.add_event_listener_with_callback(
                    "animationstart",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Animation start event listener should be added.");
            });
            handle_animation_end.with_value(|closure| {
                node.add_event_listener_with_callback(
                    "animationcancel",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Animation cancel event listener should be added.");
                node.add_event_listener_with_callback(
                    "animationend",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Animation end event listener should be added.");
            });
        } else {
            // Transition to the unmounted state if the node is removed prematurely.
            // We avoid doing so during cleanup as the node may change but still exist.
            send.run(MachineEvent::AnimationEnd);
        }
    });

    Effect::new(move |_| {
        if let Some(node) = node_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();
            let window = web_sys::window().expect("Window should exist.");
            styles.set(
                window
                    .get_computed_style(node)
                    .expect("Element is valid.")
                    .map(SendWrapper::new),
            );
        }
    });

    Owner::on_cleanup(move || {
        if let Some(node) = node_ref.get_untracked() {
            let node: &web_sys::EventTarget = node.unchecked_ref();
            handle_animation_start.with_value(|closure| {
                node.remove_event_listener_with_callback(
                    "animationstart",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Animation start event listener should be removed.");
            });
            handle_animation_end.with_value(|closure| {
                node.remove_event_listener_with_callback(
                    "animationcancel",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Animation cancel event listener should be removed.");
                node.remove_event_listener_with_callback(
                    "animationend",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("Animation end event listener should be removed.");
            });
        }
    });

    Signal::derive(move || {
        [MachineState::Mounted, MachineState::UnmountSuspended].contains(&state.get())
    })
}

fn get_animation_name(styles: Option<&web_sys::CssStyleDeclaration>) -> String {
    styles
        .and_then(|styles| styles.get_property_value("animation-name").ok())
        .unwrap_or("none".into())
}
