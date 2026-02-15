use std::sync::Arc;

use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_roving_focus::{Orientation, RovingFocusGroup, RovingFocusGroupItem};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

use crate::radio::{RadioBubbleInput, RadioContextValue, RadioIndicator, get_state};

const ARROW_KEYS: [&str; 4] = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

/* -------------------------------------------------------------------------------------------------
 * RadioGroup
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct RadioGroupContextValue {
    name: Signal<Option<String>>,
    required: Signal<bool>,
    disabled: Signal<bool>,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
}

#[component]
pub fn RadioGroup(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let required = Signal::derive(move || required.get().unwrap_or(false));
    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let loop_signal = Signal::derive(move || r#loop.get().unwrap_or(true));

    let direction = use_direction(dir);

    let (current_value, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });

    let value_signal = Signal::derive(move || current_value.get());

    let on_value_change_callback = Callback::new(move |value: String| {
        set_value.run(Some(value));
    });

    let context_value = RadioGroupContextValue {
        name: Signal::derive(move || name.get()),
        required,
        disabled,
        value: value_signal,
        on_value_change: on_value_change_callback,
    };

    view! {
        <Provider value=context_value>
            <RovingFocusGroup
                as_child=true
                orientation=orientation
                dir=direction
                r#loop=loop_signal
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    attr:role="radiogroup"
                    attr:aria-required=move || required.get().then_some("true")
                    attr:aria-orientation=move || orientation.get().map(|o| o.to_string())
                    attr:data-disabled=move || disabled.get().then_some("")
                    attr:dir=move || direction.get().to_string()
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </RovingFocusGroup>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * RadioGroupItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn RadioGroupItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let context = expect_context::<RadioGroupContextValue>();
    let is_disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));

    let item_value = StoredValue::new(value.clone());
    let checked = Signal::derive(move || {
        context
            .value
            .get()
            .as_ref()
            .is_some_and(|v| *v == item_value.get_value())
    });

    let item_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, item_ref]);

    let is_form_control = Signal::derive(move || {
        item_ref
            .get()
            .and_then(|button| {
                let el: &web_sys::Element = button.unchecked_ref();
                el.closest("form").ok()
            })
            .flatten()
            .is_some()
    });

    // Track arrow key presses on document for auto-check-on-focus behavior.
    let is_arrow_key_pressed = RwSignal::new(false);

    type HandleKeyDown = dyn Fn(web_sys::KeyboardEvent);
    let handle_keydown: Arc<SendWrapper<Closure<HandleKeyDown>>> = Arc::new(SendWrapper::new(
        Closure::new(move |event: web_sys::KeyboardEvent| {
            if ARROW_KEYS.contains(&event.key().as_str()) {
                is_arrow_key_pressed.set(true);
            }
        }),
    ));

    type HandleKeyUp = dyn Fn();
    let handle_keyup: Arc<SendWrapper<Closure<HandleKeyUp>>> =
        Arc::new(SendWrapper::new(Closure::new(move || {
            is_arrow_key_pressed.set(false);
        })));

    Effect::new({
        let handle_keydown = handle_keydown.clone();
        let handle_keyup = handle_keyup.clone();

        move |_| {
            let document = web_sys::window()
                .expect("Window should exist.")
                .document()
                .expect("Document should exist.");

            // Register in capture phase so the flag is set BEFORE element-level keydown
            // handlers fire. RovingFocusGroupItem's keydown handler moves focus to the
            // next item, which triggers our on_focus handler — it needs is_arrow_key_pressed
            // to already be true at that point.
            document
                .add_event_listener_with_callback_and_bool(
                    "keydown",
                    (*handle_keydown).as_ref().unchecked_ref(),
                    true,
                )
                .expect("Keydown event listener should be added.");

            document
                .add_event_listener_with_callback_and_bool(
                    "keyup",
                    (*handle_keyup).as_ref().unchecked_ref(),
                    true,
                )
                .expect("Keyup event listener should be added.");
        }
    });

    on_cleanup(move || {
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            let _ = document.remove_event_listener_with_callback_and_bool(
                "keydown",
                (*handle_keydown).as_ref().unchecked_ref(),
                true,
            );
            let _ = document.remove_event_listener_with_callback_and_bool(
                "keyup",
                (*handle_keyup).as_ref().unchecked_ref(),
                true,
            );
        }
    });

    let on_value_change = context.on_value_change;
    let item_value_for_check = item_value;
    let value_signal = Signal::derive(move || item_value.get_value());

    // Track whether the consumer's onClick handler called stopPropagation.
    // If so, the BubbleInput's change event should not bubble either.
    let has_consumer_stopped_propagation = RwSignal::new(false);

    let radio_context = RadioContextValue {
        checked,
        disabled: is_disabled,
    };

    view! {
        // Provider must wrap the children so each RadioGroupItem's RadioIndicator
        // finds its own RadioContextValue (not a sibling's). Using bare provide_context
        // at the component level doesn't scope correctly — all indicators end up reading
        // from the last item's context.
        <Provider value=radio_context>
            <RovingFocusGroupItem
                as_child=true
                focusable=Signal::derive(move || !is_disabled.get())
                active=checked
                on_key_down=Callback::new(move |event: ev::KeyboardEvent| {
                    // According to WAI ARIA, radio groups don't activate items on enter keypress.
                    if event.key() == "Enter" {
                        event.prevent_default();
                    }
                })
                on_focus=Callback::new(move |_: ev::FocusEvent| {
                    // Our `RovingFocusGroup` will focus the radio when navigating with arrow
                    // keys and we need to "check" it in that case. We click it to "check" it
                    // (instead of updating `context.value`) so that the radio change event fires.
                    if is_arrow_key_pressed.get()
                        && let Some(node) = item_ref.get()
                    {
                        let element: &web_sys::HtmlElement = node.unchecked_ref();
                        element.click();
                    }
                })
            >
                <RadioButton
                    checked=checked
                    disabled=is_disabled
                    is_form_control=is_form_control
                    on_click=on_click
                    has_consumer_stopped_propagation=has_consumer_stopped_propagation
                    on_check=Callback::new(move |_: ()| {
                        on_value_change.run(item_value_for_check.get_value());
                    })
                    as_child=as_child
                    node_ref=composed_refs
                >
                    {children.with_value(|children| children())}
                </RadioButton>
            </RovingFocusGroupItem>
        </Provider>
        <Show when=move || is_form_control.get()>
            <RadioBubbleInput
                control_ref=item_ref
                checked=checked
                bubbles=Signal::derive(move || !has_consumer_stopped_propagation.get())
                required=context.required
                disabled=is_disabled
                value=value_signal
                name=Signal::derive(move || context.name.get())
            />
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * RadioButton
 * -----------------------------------------------------------------------------------------------*/

/// Renders the radio button directly following the flat ToggleGroupItemImpl pattern.
///
/// Uses `AttributeInterceptor` → `Primitive(button)` → `{..attrs}` to ensure attributes
/// from `RovingFocusGroupItem` (tabindex, on:keydown, on:focus, on:mousedown) are properly
/// forwarded to the button element.
#[component]
fn RadioButton(
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] is_form_control: Signal<bool>,
    on_click: Option<Callback<ev::MouseEvent>>,
    has_consumer_stopped_propagation: RwSignal<bool>,
    #[prop(into, optional)] on_check: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:role="radio"
                attr:aria-checked=move || checked.get().to_string()
                attr:data-state=move || get_state(checked.get())
                attr:data-disabled=move || disabled.get().then_some("")
                attr:disabled=move || disabled.get().then_some("")
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |event: ev::MouseEvent| {
                        // Radios cannot be unchecked so we only communicate a checked state.
                        if !checked.get()
                            && let Some(on_check) = on_check
                        {
                            on_check.run(());
                        }
                        if is_form_control.get() {
                            // Check if the consumer's onClick handler called stopPropagation.
                            // cancelBubble returns true after stopPropagation() was called.
                            let stopped = event.cancel_bubble();
                            has_consumer_stopped_propagation.set(stopped);
                            // If the consumer didn't stop propagation, we stop it ourselves
                            // so that only the BubbleInput's change event propagates to the form.
                            if !stopped {
                                event.stop_propagation();
                            }
                        }
                    })),
                    None,
                )
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * RadioGroupIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn RadioGroupIndicator(
    /// Used to force mounting when more control is needed. Useful when
    /// controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <RadioIndicator
            force_mount=force_mount
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </RadioIndicator>
    }
}
