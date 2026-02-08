use leptos::{attribute_interceptor::AttributeInterceptor, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_id::use_id;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{compose_callbacks, Primitive};
use radix_leptos_use_controllable_state::{use_controllable_state, UseControllableStateParams};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{closure::Closure, JsCast};

/* -------------------------------------------------------------------------------------------------
 * Collapsible
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct CollapsibleContextValue {
    content_id: ReadSignal<String>,
    disabled: Signal<bool>,
    open: Signal<bool>,
    on_open_toggle: Callback<()>,
}

#[component]
pub fn Collapsible(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));

    let (open_signal, set_open) = use_controllable_state(UseControllableStateParams {
        prop: open,
        default_prop: default_open,
        on_change: on_open_change.map(|on_open_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_open_change.run(value);
                }
            })
        }),
    });
    let open = Signal::derive(move || open_signal.get().unwrap_or(false));

    let content_id = use_id(None);

    let context = CollapsibleContextValue {
        content_id,
        disabled,
        open,
        on_open_toggle: Callback::new(move |_| {
            set_open.run(Some(!open.get()));
        }),
    };
    provide_context(context);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-state=move || get_state(open.get())
                attr:data-disabled=move || disabled.get().then_some("")
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * CollapsibleTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn CollapsibleTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let context = expect_context::<CollapsibleContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=node_ref
                attr:r#type="button"
                attr:aria-controls=move || context.content_id.get()
                attr:aria-expanded=move || context.open.get().to_string()
                attr:data-state=move || get_state(context.open.get())
                attr:data-disabled=move || context.disabled.get().then_some("")
                attr:disabled=move || context.disabled.get().then_some("")
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |_: ev::MouseEvent| {
                        context.on_open_toggle.run(());
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
 * CollapsibleContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn CollapsibleContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<TypedChildrenFn<impl IntoView + 'static>>,
) -> impl IntoView {
    let children = StoredValue::new(children.map(|c| c.into_inner()));

    let context = expect_context::<CollapsibleContextValue>();

    let present =
        Signal::derive(move || force_mount.get().unwrap_or(false) || context.open.get());

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=present node_ref=presence_ref>
                    <CollapsibleContentImpl
                        as_child=as_child
                        node_ref=node_ref
                        presence_ref=presence_ref
                        present=present
                    >
                        {children.with_value(|children| children.as_ref().map(|children| children()))}
                    </CollapsibleContentImpl>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * CollapsibleContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn CollapsibleContentImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(into)] present: Signal<bool>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<CollapsibleContextValue>();

    let (is_present, set_is_present) = signal(present.get_untracked());
    let height: RwSignal<Option<f64>> = RwSignal::new(Some(0.0));
    let width: RwSignal<Option<f64>> = RwSignal::new(Some(0.0));

    // when opening we want it to immediately open to retrieve dimensions
    // when closing we delay `present` to retrieve dimensions before closing
    let is_open = Signal::derive(move || context.open.get() || is_present.get());

    let is_mount_animation_prevented = RwSignal::new(is_open.get_untracked());
    let original_transition_duration: RwSignal<Option<String>> = RwSignal::new(None);
    let original_animation_name: RwSignal<Option<String>> = RwSignal::new(None);

    let composed_ref = use_composed_refs(vec![node_ref, presence_ref]);

    // Prevent mount animation via requestAnimationFrame
    let raf_handle: RwSignal<Option<i32>> = RwSignal::new(None);
    let raf_closure: SendWrapper<Closure<dyn Fn()>> =
        SendWrapper::new(Closure::new(move || {
            is_mount_animation_prevented.set(false);
        }));
    let raf_closure = StoredValue::new(raf_closure);

    // Schedule rAF on mount
    raf_closure.with_value(|closure| {
        let window = web_sys::window().expect("Window should exist.");
        let handle = window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("requestAnimationFrame should succeed.");
        raf_handle.set(Some(handle));
    });

    Owner::on_cleanup(move || {
        if let Some(handle) = raf_handle.get() {
            web_sys::window()
                .expect("Window should exist.")
                .cancel_animation_frame(handle)
                .expect("cancelAnimationFrame should succeed.");
        }
    });

    // Measure dimensions and control animation blocking
    Effect::new(move |_| {
        // Track these signals to re-run when open or present changes
        let _open = context.open.get();
        let _present = present.get();

        if let Some(node) = composed_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();

            // Save original styles (only first time)
            if original_transition_duration.get_untracked().is_none() {
                original_transition_duration.set(Some(
                    node.style()
                        .get_property_value("transition-duration")
                        .unwrap_or_default(),
                ));
                original_animation_name.set(Some(
                    node.style()
                        .get_property_value("animation-name")
                        .unwrap_or_default(),
                ));
            }

            // Block any animations/transitions so the element renders at its full dimensions
            node.style()
                .set_property("transition-duration", "0s")
                .expect("Style should be set.");
            node.style()
                .set_property("animation-name", "none")
                .expect("Style should be set.");

            // Get width and height from full dimensions
            let rect = node.get_bounding_client_rect();
            height.set(Some(rect.height()));
            width.set(Some(rect.width()));

            // Kick off any animations/transitions that were originally set up
            // if it isn't the initial mount
            if !is_mount_animation_prevented.get_untracked() {
                node.style()
                    .set_property(
                        "transition-duration",
                        &original_transition_duration
                            .get_untracked()
                            .unwrap_or_default(),
                    )
                    .expect("Style should be set.");
                node.style()
                    .set_property(
                        "animation-name",
                        &original_animation_name.get_untracked().unwrap_or_default(),
                    )
                    .expect("Style should be set.");
            }

            set_is_present.set(present.get());
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:data-state=move || get_state(context.open.get())
                attr:data-disabled=move || context.disabled.get().then_some("")
                attr:id=move || context.content_id.get()
                attr:hidden=move || !is_open.get()
                attr:style=move || {
                    let mut style = String::new();
                    if let Some(h) = height.get()
                        && h > 0.0
                    {
                        style.push_str(&format!(
                            "--radix-collapsible-content-height: {}px; ",
                            h
                        ));
                    }
                    if let Some(w) = width.get()
                        && w > 0.0
                    {
                        style.push_str(&format!(
                            "--radix-collapsible-content-width: {}px;",
                            w
                        ));
                    }
                    (!style.is_empty()).then_some(style)
                }
                {..attrs}
            >
                <Show when=move || is_open.get()>
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Show>
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

fn get_state(open: bool) -> &'static str {
    match open {
        true => "open",
        false => "closed",
    }
}
