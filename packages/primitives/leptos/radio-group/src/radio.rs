use leptos::{html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::Primitive;
use radix_leptos_use_previous::use_previous;
use radix_leptos_use_size::use_size;
use web_sys::wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * RadioContextValue
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug)]
pub(crate) struct RadioContextValue {
    pub checked: Signal<bool>,
    pub disabled: Signal<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * RadioIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub(crate) fn RadioIndicator(
    /// Used to force mounting when more control is needed. Useful when
    /// controlling animation with animation libraries.
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let force_mount = Signal::derive(move || force_mount.get().unwrap_or(false));
    let context = expect_context::<RadioContextValue>();

    let present = Signal::derive(move || force_mount.get() || context.checked.get());

    // Presence needs a ref to the actual DOM element to observe animation events.
    // We create a separate presence_ref and compose it with the forwarded node_ref.
    let presence_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, presence_ref]);

    view! {
        <Presence present=present node_ref=presence_ref>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=composed_ref
                attr:data-state=move || get_state(context.checked.get())
                attr:data-disabled=move || context.disabled.get().then_some("")
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * RadioBubbleInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub(crate) fn RadioBubbleInput(
    #[prop(into)] control_ref: AnyNodeRef,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] bubbles: Signal<bool>,
    #[prop(into)] required: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] name: Signal<Option<String>>,
) -> impl IntoView {
    let input_ref: AnyNodeRef = AnyNodeRef::new();
    let prev_checked = use_previous(Signal::derive(move || Some(checked.get())));
    let control_size = use_size(control_ref);

    // Bubble checked change to parents (e.g. form change event).
    // Only dispatch when becoming checked — radios cannot be unchecked by the user,
    // so only the newly checked radio should fire a change event. Without this guard,
    // both the newly checked and previously checked radios would fire, and the form
    // handler would see the wrong (stale) value from the unchecking radio last.
    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let input: &web_sys::HtmlInputElement = input.unchecked_ref();
            let prev = prev_checked.get();
            let current = Some(checked.get());

            if prev != current {
                input.set_checked(checked.get());

                if checked.get() {
                    let init = web_sys::EventInit::new();
                    init.set_bubbles(bubbles.get());

                    // React dispatches "click" because React's synthetic event system maps
                    // onChange to click for radio/checkbox inputs. In native DOM (Leptos),
                    // we dispatch "change" so that native form change handlers fire correctly.
                    let event = web_sys::Event::new_with_event_init_dict("change", &init)
                        .expect("Change event should be instantiated.");

                    input
                        .dispatch_event(&event)
                        .expect("Change event should be dispatched.");
                }
            }
        }
    });

    view! {
        <input
            node_ref=input_ref
            type="radio"
            aria-hidden="true"
            checked=move || checked.get().then_some("")
            required=move || required.get().then_some("")
            disabled=move || disabled.get().then_some("")
            name=move || name.get()
            value=move || value.get()
            tabindex="-1"
            // We transform because the input is absolutely positioned but we have
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

/* ---------------------------------------------------------------------------------------------- */

pub(crate) fn get_state(checked: bool) -> &'static str {
    if checked { "checked" } else { "unchecked" }
}
