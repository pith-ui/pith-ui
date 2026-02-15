use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_id::use_id;
use radix_leptos_primitive::{Primitive, VoidPrimitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AutoComplete {
    #[default]
    CurrentPassword,
    NewPassword,
}

impl AutoComplete {
    fn as_str(&self) -> &'static str {
        match self {
            AutoComplete::CurrentPassword => "current-password",
            AutoComplete::NewPassword => "new-password",
        }
    }
}

#[derive(Clone)]
struct InternalFocusState {
    click_triggered: bool,
    selection_start: Option<u32>,
    selection_end: Option<u32>,
}

const INITIAL_FOCUS_STATE: InternalFocusState = InternalFocusState {
    click_triggered: false,
    selection_start: None,
    selection_end: None,
};

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct PasswordToggleFieldContextValue {
    input_id: Signal<String>,
    input_ref: AnyNodeRef,
    visible: Signal<bool>,
    set_visible: Callback<Option<bool>>,
    sync_input_id: Callback<Option<String>>,
    focus_state: StoredValue<InternalFocusState>,
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleField
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleField(
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] visible: MaybeProp<bool>,
    #[prop(into, optional)] default_visible: MaybeProp<bool>,
    #[prop(into, optional)] on_visibility_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let base_id = use_id(id.get());
    let default_input_id = Signal::derive(move || format!("{}-input", base_id.get()));

    let (input_id_state, set_input_id_state) = signal::<Option<String>>(None);
    let input_id = Signal::derive(move || {
        input_id_state
            .get()
            .unwrap_or_else(|| default_input_id.get())
    });

    let sync_input_id = Callback::new(move |provided_id: Option<String>| {
        set_input_id_state.set(provided_id);
    });

    let (visible_signal, set_visible) = use_controllable_state(UseControllableStateParams {
        prop: visible,
        default_prop: MaybeProp::derive(move || Some(default_visible.get().unwrap_or(false))),
        on_change: on_visibility_change.map(|on_visibility_change| {
            Callback::new(move |value: Option<bool>| {
                if let Some(value) = value {
                    on_visibility_change.run(value);
                }
            })
        }),
    });
    let visible = Signal::derive(move || visible_signal.get().unwrap_or(false));

    let input_ref = AnyNodeRef::new();
    let focus_state = StoredValue::new(INITIAL_FOCUS_STATE);

    let context = PasswordToggleFieldContextValue {
        input_id,
        input_ref,
        visible,
        set_visible: Callback::new(move |value: Option<bool>| {
            set_visible.run(value);
        }),
        sync_input_id,
        focus_state,
    };

    view! {
        <Provider value=context>
            {children.with_value(|children| children())}
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldInput
 * -----------------------------------------------------------------------------------------------*/

#[allow(clippy::unused_unit)]
#[component]
pub fn PasswordToggleFieldInput(
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let _children = children;
    let context = expect_context::<PasswordToggleFieldContextValue>();

    let auto_complete_value =
        Signal::derive(move || auto_complete.get().unwrap_or_default().as_str());

    // Sync user-provided id prop into context
    Effect::new(move |_| {
        context.sync_input_id.run(id.get());
    });

    // Form reset/submit listener — reset visibility to false
    let set_visible = context.set_visible;
    let input_ref = context.input_ref;
    Effect::new(move |_| {
        let Some(node) = input_ref.get() else {
            return;
        };
        let input: &web_sys::HtmlInputElement = node.unchecked_ref();
        let Some(form) = input.form() else {
            return;
        };

        let set_visible = set_visible;
        let reset_closure = SendWrapper::new(Closure::<dyn Fn(web_sys::Event)>::new(
            move |event: web_sys::Event| {
                if !event.default_prevented() {
                    set_visible.run(Some(false));
                }
            },
        ));
        let submit_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            set_visible.run(Some(false));
        }));

        let form = SendWrapper::new(form);
        form.add_event_listener_with_callback("reset", reset_closure.as_ref().unchecked_ref())
            .expect("Reset event listener should be added.");
        form.add_event_listener_with_callback("submit", submit_closure.as_ref().unchecked_ref())
            .expect("Submit event listener should be added.");

        Owner::on_cleanup(move || {
            form.remove_event_listener_with_callback(
                "reset",
                reset_closure.as_ref().unchecked_ref(),
            )
            .expect("Reset event listener should be removed.");
            form.remove_event_listener_with_callback(
                "submit",
                submit_closure.as_ref().unchecked_ref(),
            )
            .expect("Submit event listener should be removed.");
        });
    });

    let composed_ref = use_composed_refs(vec![node_ref, context.input_ref]);

    let focus_state = context.focus_state;

    let resolved_id = Signal::derive(move || id.get().unwrap_or_else(|| context.input_id.get()));

    view! {
        <AttributeInterceptor let:attrs>
            <VoidPrimitive
                element=html::input
                as_child=as_child
                node_ref=composed_ref
                attr:id=move || resolved_id.get()
                attr:autocapitalize="off"
                attr:autocomplete=move || auto_complete_value.get()
                attr:spellcheck="false"
                attr:r#type=move || if context.visible.get() { "text" } else { "password" }
                on:blur=compose_callbacks(
                    on_blur,
                    Some(Callback::new(move |event: ev::FocusEvent| {
                        let target: web_sys::HtmlInputElement = event
                            .current_target()
                            .expect("Event should have current target")
                            .unchecked_into();
                        focus_state.update_value(|state| {
                            state.selection_start = target.selection_start().ok().flatten();
                            state.selection_end = target.selection_end().ok().flatten();
                        });
                    })),
                    None,
                )
                {..attrs}
            >
                {()}
            </VoidPrimitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldToggle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleFieldToggle(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_cancel: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<PasswordToggleFieldContextValue>();
    let (internal_aria_label, set_internal_aria_label) = signal::<Option<String>>(None);
    let element_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, element_ref]);

    let focus_state = context.focus_state;

    // Auto aria-label via MutationObserver
    Effect::new(move |_| {
        let visible = context.visible.get();

        let Some(node) = element_ref.get() else {
            set_internal_aria_label.set(None);
            return;
        };

        if aria_label.get().is_some() {
            set_internal_aria_label.set(None);
            return;
        }

        let element: &web_sys::HtmlElement = node.unchecked_ref();
        let default_aria_label = if visible {
            "Hide password"
        } else {
            "Show password"
        };

        // Check current text content
        let text_content = element.text_content().unwrap_or_default();
        if !text_content.is_empty() {
            set_internal_aria_label.set(None);
        } else {
            set_internal_aria_label.set(Some(default_aria_label.to_string()));
        }

        // Set up MutationObserver for text changes
        let default_label = default_aria_label.to_string();
        let observer_callback = SendWrapper::new(Closure::<dyn Fn(web_sys::js_sys::Array)>::new(
            move |entries: web_sys::js_sys::Array| {
                let mut text_content: Option<String> = None;
                for i in 0..entries.length() {
                    let entry: web_sys::MutationRecord = entries.get(i).unchecked_into();
                    if entry.type_() == "characterData"
                        && let Some(target) = entry.target()
                    {
                        let el: &web_sys::Node = target.unchecked_ref();
                        if let Some(tc) = el.text_content()
                            && !tc.is_empty()
                        {
                            text_content = Some(tc);
                        }
                    }
                }
                if let Some(text) = text_content {
                    if !text.is_empty() {
                        set_internal_aria_label.set(None);
                    } else {
                        set_internal_aria_label.set(Some(default_label.clone()));
                    }
                }
            },
        ));

        let observer = SendWrapper::new(
            web_sys::MutationObserver::new(observer_callback.as_ref().unchecked_ref())
                .expect("MutationObserver should be created."),
        );

        let init = web_sys::MutationObserverInit::new();
        init.set_character_data(true);
        init.set_subtree(true);
        observer
            .observe_with_options(element, &init)
            .expect("MutationObserver should observe.");

        Owner::on_cleanup(move || {
            observer.disconnect();
            // prevent drop of the closure before the observer is disconnected
            drop(observer_callback);
        });
    });

    let resolved_aria_label =
        Signal::derive(move || aria_label.get().or_else(|| internal_aria_label.get()));

    // CSR-only: always provide aria-controls (no hydration gate needed)
    let aria_controls = Signal::derive(move || context.input_id.get());

    // Global pointerup listener for click_triggered reset
    Effect::new(move |_| {
        let window = web_sys::window().expect("Window should exist.");

        let focus_state = focus_state;
        let reset = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            focus_state.update_value(|state| state.click_triggered = false);
        }));

        let cleanup_handle: StoredValue<Option<i32>> = StoredValue::new(None);

        let reset_ref = SendWrapper::new(
            reset
                .as_ref()
                .unchecked_ref::<web_sys::js_sys::Function>()
                .clone(),
        );
        let handler = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            let window = web_sys::window().expect("Window should exist.");
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(&reset_ref, 1)
                .expect("setTimeout should succeed.");
            cleanup_handle.set_value(Some(handle));
        }));

        let window_sw = SendWrapper::new(window.clone());
        window_sw
            .add_event_listener_with_callback("pointerup", handler.as_ref().unchecked_ref())
            .expect("pointerup event listener should be added.");

        Owner::on_cleanup(move || {
            if let Some(handle) = cleanup_handle.get_value() {
                window_sw.clear_timeout_with_handle(handle);
            }
            window_sw
                .remove_event_listener_with_callback("pointerup", handler.as_ref().unchecked_ref())
                .expect("pointerup event listener should be removed.");
            // prevent drop of the reset closure before the handler is removed
            drop(reset);
        });
    });

    let set_visible = context.set_visible;
    let input_ref = context.input_ref;

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::button
                as_child=as_child
                node_ref=composed_ref
                attr:r#type="button"
                attr:aria-controls=move || aria_controls.get()
                attr:aria-label=move || resolved_aria_label.get()
                on:pointerdown=compose_callbacks(
                    on_pointer_down,
                    Some(Callback::new(move |_: ev::PointerEvent| {
                        focus_state.update_value(|state| state.click_triggered = true);
                    })),
                    None,
                )
                on:pointercancel=move |event: ev::PointerEvent| {
                    // Do not use compose_callbacks — always reset regardless of preventDefault
                    if let Some(on_pointer_cancel) = on_pointer_cancel {
                        on_pointer_cancel.run(event);
                    }
                    focus_state.set_value(INITIAL_FOCUS_STATE);
                }
                on:click=move |event: ev::MouseEvent| {
                    // Do not use compose_callbacks — always reset focus state
                    if let Some(on_click) = on_click {
                        on_click.run(event.clone());
                    }
                    if event.default_prevented() {
                        focus_state.set_value(INITIAL_FOCUS_STATE);
                        return;
                    }

                    // Toggle visibility (synchronous in Leptos CSR — no flushSync needed)
                    let current = context.visible.get_untracked();
                    set_visible.run(Some(!current));

                    if focus_state.with_value(|s| s.click_triggered)
                        && let Some(node) = input_ref.get()
                    {
                        let input: &web_sys::HtmlInputElement = node.unchecked_ref();
                        let selection_start = focus_state.with_value(|s| s.selection_start);
                        let selection_end = focus_state.with_value(|s| s.selection_end);
                        let _ = input.focus();

                        if selection_start.is_some() || selection_end.is_some() {
                            let input_clone = input.clone();
                            // Wait a tick so focus has settled, then restore selection
                            let cb = Closure::once_into_js(move || {
                                if let Some(doc) = input_clone.owner_document()
                                    && let Some(active) = doc.active_element()
                                    && active
                                        == *input_clone.unchecked_ref::<web_sys::Element>()
                                {
                                    let _ = input_clone.set_selection_start(selection_start);
                                    let _ = input_clone.set_selection_end(selection_end);
                                }
                            });
                            let _ = web_sys::window()
                                .expect("Window should exist.")
                                .request_animation_frame(cb.unchecked_ref());
                        }
                    }
                    focus_state.set_value(INITIAL_FOCUS_STATE);
                }
                on:pointerup=move |event: ev::PointerEvent| {
                    // Do not use compose_callbacks — always reset
                    if let Some(on_pointer_up) = on_pointer_up {
                        on_pointer_up.run(event);
                    }
                    // If click handler hasn't been called, reset after a short delay
                    let focus_state = focus_state;
                    let cb = Closure::once_into_js(move || {
                        focus_state.set_value(INITIAL_FOCUS_STATE);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist.")
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            50,
                        );
                }
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldSlot
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleFieldSlot(
    #[prop(into, optional)] render: Option<Callback<bool, AnyView>>,
    #[prop(into, optional)] visible_content: Option<ChildrenFn>,
    #[prop(into, optional)] hidden_content: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<PasswordToggleFieldContextValue>();

    move || {
        let visible = context.visible.get();
        if let Some(render) = render {
            render.run(visible)
        } else if visible {
            visible_content
                .as_ref()
                .map(|children| children().into_any())
                .unwrap_or_else(|| ().into_any())
        } else {
            hidden_content
                .as_ref()
                .map(|children| children().into_any())
                .unwrap_or_else(|| ().into_any())
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * PasswordToggleFieldIcon
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn PasswordToggleFieldIcon(
    #[prop(into)] visible_icon: ViewFn,
    #[prop(into)] hidden_icon: ViewFn,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<PasswordToggleFieldContextValue>();
    let visible_icon = StoredValue::new(visible_icon);
    let hidden_icon = StoredValue::new(hidden_icon);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=leptos::svg::svg
                as_child=true
                node_ref=node_ref
                attr:aria-hidden="true"
                {..attrs}
            >
                {move || {
                    if context.visible.get() {
                        visible_icon.with_value(|icon| icon.run())
                    } else {
                        hidden_icon.with_value(|icon| icon.run())
                    }
                }}
            </Primitive>
        </AttributeInterceptor>
    }
}
