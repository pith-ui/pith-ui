use std::fmt::{Display, Formatter};

use crate::support::compose_refs::use_composed_refs;
use crate::support::presence::Presence;
use crate::support::primitive::{Primitive, compose_callbacks, data_attr, prop_or};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};
use crate::support::use_previous::use_previous;
use crate::support::use_size::use_size;
use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedState {
    False,
    True,
    Indeterminate,
}

impl Display for CheckedState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CheckedState::False => "false",
                CheckedState::True => "true",
                CheckedState::Indeterminate => "indeterminate",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
struct CheckboxContextValue {
    state: Signal<CheckedState>,
    disabled: Signal<bool>,
}

#[component]
pub fn Checkbox(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] default_checked: MaybeProp<CheckedState>,
    #[prop(into, optional)] on_checked_change: Option<Callback<CheckedState>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] value: MaybeProp<String>,
    /// The `id` of a `<form>` element to associate the checkbox with. Allows the checkbox
    /// to participate in a form even when it is not a descendant of that form.
    #[prop(into, optional)]
    form: MaybeProp<String>,
    #[prop(into, optional)] on_keydown: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let name = Signal::derive(move || name.get());
    let required = prop_or(required, false);
    let disabled = prop_or(disabled, false);
    let value = prop_or(value, "on".into());

    let button_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, button_ref]);

    let is_form_control = Signal::derive(move || {
        if form.get().is_some() {
            return true;
        }
        button_ref
            .get()
            .and_then(|button| {
                let el: &web_sys::Element = button.unchecked_ref();
                el.closest("form").ok()
            })
            .flatten()
            .is_some()
    });
    let (checked, set_checked) = use_controllable_state(UseControllableStateParams {
        prop: checked,
        on_change: on_checked_change.map(|on_checked_change| {
            Callback::new(move |value: Option<CheckedState>| {
                if let Some(value) = value {
                    on_checked_change.run(value);
                }
            })
        }),
        default_prop: default_checked,
    });
    let checked = Signal::derive(move || checked.get().unwrap_or(CheckedState::False));

    let initial_checked_state = RwSignal::new(checked.get_untracked());
    let handle_reset: SendWrapper<Closure<dyn Fn(web_sys::Event)>> =
        SendWrapper::new(Closure::new(move |_| {
            set_checked.run(Some(initial_checked_state.get_untracked()));
        }));
    let handle_reset = StoredValue::new(handle_reset);

    Effect::new(move |_| {
        if let Some(form) = button_ref
            .get()
            .and_then(|button| {
                let el: &web_sys::Element = button.unchecked_ref();
                el.closest("form").ok()
            })
            .flatten()
        {
            handle_reset.with_value(|closure| {
                form.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                    .expect("Reset event listener should be added.");
            });
        }
    });

    Owner::on_cleanup(move || {
        if let Some(form) = button_ref
            .get()
            .and_then(|button| {
                let el: &web_sys::Element = button.unchecked_ref();
                el.closest("form").ok()
            })
            .flatten()
        {
            handle_reset.with_value(|closure| {
                form.remove_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                    .expect("Reset event listener should be removed.");
            });
        }
    });

    let context_value = CheckboxContextValue {
        state: checked,
        disabled,
    };

    view! {
        <Provider value=context_value>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_refs
                    attr:r#type="button"
                    attr:role="checkbox"
                    attr:aria-checked=move || match checked.get() {
                        CheckedState::True => "true",
                        CheckedState::False => "false",
                        CheckedState::Indeterminate => "mixed",
                    }
                    attr:aria-required=move || match required.get() {
                        true => "true",
                        false => "false",
                    }
                    attr:data-state=move || get_state(checked.get())
                    attr:data-disabled=data_attr(disabled)
                    attr:disabled=data_attr(disabled)
                    attr:value=move || value.get()
                    on:keydown=compose_callbacks(on_keydown, Some(Callback::new(move |event: ev::KeyboardEvent| {
                        // According to WAI ARIA, checkboxes don't activate on enter keypress.
                        if event.key() == "Enter" {
                            event.prevent_default();
                        }
                    })), None)
                    on:click=compose_callbacks(on_click, Some(Callback::new(move |event: ev::MouseEvent| {
                        if !disabled.get() {
                            set_checked.run(Some(match checked.get() {
                                CheckedState::False => CheckedState::True,
                                CheckedState::True => CheckedState::False,
                                CheckedState::Indeterminate => CheckedState::True
                            }));

                            if is_form_control.get() {
                                // If checkbox is in a form, stop propagation from the button, so that we only propagate
                                // one click event (from the input). We propagate changes from an input so that native
                                // form validation works and form events reflect checkbox updates.
                                event.stop_propagation();
                            }
                        }
                    })), None)
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
            <Show when=move || is_form_control.get()>
                <BubbleInput
                    attr:name=move || name.get()
                    control_ref=button_ref
                    bubbles=Signal::derive(|| true)
                    value=value
                    checked=checked
                    required=required
                    disabled=disabled
                    form=Signal::derive(move || form.get())
                />
            </Show>
        </Provider>
    }
}

#[component]
pub fn CheckboxIndicator(
    /// Used to force mounting when more control is needed. Useful when controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let force_mount = prop_or(force_mount, false);

    let context = expect_context::<CheckboxContextValue>();

    let present = Signal::derive(move || {
        force_mount.get()
            || context.state.get() == CheckedState::Indeterminate
            || context.state.get() == CheckedState::True
    });

    let children = StoredValue::new(children);

    view! {
        <Presence present=present node_ref=node_ref>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || get_state(context.state.get())
                attr:data-disabled=data_attr(context.disabled)
                attr:style="pointer-events: none;"
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Presence>
    }
}

#[component]
fn BubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<CheckedState>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] form: Signal<Option<String>>,
) -> impl IntoView {
    let node_ref: NodeRef<html::Input> = NodeRef::new();
    let prev_checked = use_previous(checked);
    let control_size = use_size(control_ref);

    // Bubble checked change to parents
    Effect::new(move |_| {
        if let Some(input) = node_ref.get() {
            if prev_checked.get() != checked.get() {
                let init = web_sys::EventInit::new();
                init.set_bubbles(bubbles.get());

                let event = web_sys::Event::new_with_event_init_dict("click", &init)
                    .expect("Click event should be instantiated.");

                input.set_indeterminate(is_indeterminiate(checked.get()));
                input.set_checked(match checked.get() {
                    CheckedState::False => false,
                    CheckedState::True => true,
                    CheckedState::Indeterminate => false,
                });

                input
                    .dispatch_event(&event)
                    .expect("Click event should be dispatched.");
            }
        }
    });

    view! {
        <input
            node_ref=node_ref
            type="checkbox"
            aria-hidden="true"
            checked=move || (match checked.get() {
                CheckedState::False => false,
                CheckedState::True => true,
                CheckedState::Indeterminate => false,
            }).then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            value=move || value.get()
            form=move || form.get()
            tabindex="-1"
            // We transform because the input is absolutely positioned, but we have
            // rendered it **after** the button. This pulls it back to sit on top
            // of the button.
            style:transform="translateX(-100%)"
            style:width=move || control_size.get().map(|size| format!("{}px", size.width))
            style:height=move || control_size.get().map(|size| format!("{}px", size.height))
            style:position="absolute"
            style:pointer-events="none"
            style:opacity="0"
            style:margin="0px"
        />
    }
}

fn is_indeterminiate(checked: CheckedState) -> bool {
    checked == CheckedState::Indeterminate
}

fn get_state(checked: CheckedState) -> String {
    (match checked {
        CheckedState::True => "checked",
        CheckedState::False => "unchecked",
        CheckedState::Indeterminate => "indeterminate",
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_indeterminate_true_for_indeterminate() {
        assert!(is_indeterminiate(CheckedState::Indeterminate));
    }

    #[test]
    fn is_indeterminate_false_for_true() {
        assert!(!is_indeterminiate(CheckedState::True));
    }

    #[test]
    fn is_indeterminate_false_for_false() {
        assert!(!is_indeterminiate(CheckedState::False));
    }

    #[test]
    fn get_state_checked() {
        assert_eq!(get_state(CheckedState::True), "checked");
    }

    #[test]
    fn get_state_unchecked() {
        assert_eq!(get_state(CheckedState::False), "unchecked");
    }

    #[test]
    fn get_state_indeterminate() {
        assert_eq!(get_state(CheckedState::Indeterminate), "indeterminate");
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", CheckedState::True), "true");
        assert_eq!(format!("{}", CheckedState::False), "false");
        assert_eq!(format!("{}", CheckedState::Indeterminate), "indeterminate");
    }
}
