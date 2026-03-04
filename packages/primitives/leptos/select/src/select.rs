use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot,
    provide_collection_scope, use_collection, use_collection_scope,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_focus_guards::use_focus_guards;
use radix_leptos_focus_scope::FocusScope;
use radix_leptos_id::use_id;
use radix_leptos_popper::{
    Align, Padding, Popper, PopperAnchor, PopperArrow, PopperContent, Side, Sticky,
    UpdatePositionStrategy, provide_popper_scope, use_popper_scope,
};
use radix_leptos_portal::ScopedPortal;
use radix_leptos_primitive::{Primitive, compose_callbacks, data_attr, prop_or_default};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/* -------------------------------------------------------------------------------------------------
 * Constants
 * -----------------------------------------------------------------------------------------------*/

const OPEN_KEYS: &[&str] = &[" ", "Enter", "ArrowUp", "ArrowDown"];
const SELECTION_KEYS: &[&str] = &[" ", "Enter"];

/* -------------------------------------------------------------------------------------------------
 * Collection ItemData
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
pub struct SelectItemData {
    pub value: String,
    pub disabled: bool,
    pub text_value: String,
}

const ITEM_DATA_PHANTOM: PhantomData<SelectItemData> = PhantomData;

/* -------------------------------------------------------------------------------------------------
 * Contexts
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
struct SelectContextValue {
    trigger_ref: AnyNodeRef,
    value_node_ref: AnyNodeRef,
    value_node_has_children: ReadSignal<bool>,
    content_id: ReadSignal<String>,
    value: Signal<Option<String>>,
    on_value_change: Callback<String>,
    open: Signal<bool>,
    required: Signal<bool>,
    on_open_change: Callback<bool>,
    dir: Signal<Direction>,
    trigger_pointer_down_pos_ref: StoredValue<Option<(f64, f64)>>,
    disabled: Signal<bool>,
}

#[derive(Clone, Copy)]
struct SelectContentContextValue {
    #[allow(dead_code)]
    content_ref: AnyNodeRef,
    viewport_ref: AnyNodeRef,
    on_item_leave: Callback<()>,
    position: StoredValue<String>,
    is_positioned: ReadSignal<bool>,
    search_ref: StoredValue<String>,
}

#[derive(Clone, Copy)]
struct SelectViewportContextValue {
    #[allow(dead_code)]
    content_wrapper_ref: AnyNodeRef,
}

#[derive(Clone)]
struct SelectItemContextValue {
    #[allow(dead_code)]
    value: String,
    #[allow(dead_code)]
    disabled: bool,
    text_id: ReadSignal<String>,
    is_selected: Signal<bool>,
    on_item_text_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
}

#[derive(Clone, Copy)]
struct SelectGroupContextValue {
    id: ReadSignal<String>,
}

/* -------------------------------------------------------------------------------------------------
 * Select
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Select(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] auto_complete: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] required: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let trigger_ref = AnyNodeRef::new();
    let value_node_ref = AnyNodeRef::new();
    let (value_node_has_children, set_value_node_has_children) = signal(false);

    let direction = use_direction(dir);

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
    let open_state = Signal::derive(move || open_signal.get().unwrap_or(false));

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: MaybeProp::derive(move || value.get()),
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });
    let value_state = Signal::derive(move || value_signal.get());

    let trigger_pointer_down_pos_ref: StoredValue<Option<(f64, f64)>> = StoredValue::new(None);

    let content_id = use_id(None);
    let disabled_state = prop_or_default(disabled);
    let required_state = prop_or_default(required);

    let context = SelectContextValue {
        trigger_ref,
        value_node_ref,
        value_node_has_children,
        content_id,
        value: value_state,
        on_value_change: Callback::new(move |val: String| {
            set_value.run(Some(val));
        }),
        open: open_state,
        required: required_state,
        on_open_change: Callback::new(move |val: bool| {
            set_open.run(Some(val));
        }),
        dir: direction,
        trigger_pointer_down_pos_ref,
        disabled: disabled_state,
    };

    // Native select for form integration
    let name = StoredValue::new(name);
    let auto_complete = StoredValue::new(auto_complete);
    let form = StoredValue::new(form);

    view! {
        <Provider value=context>
            <Provider value=(set_value_node_has_children,)>
                <Popper>
                    <CollectionProvider<SelectItemData> item_data_type=ITEM_DATA_PHANTOM>
                        {children.try_with_value(|children| children())}
                    </CollectionProvider<SelectItemData>>

                    <SelectBubbleInput
                        value=value_state
                        name=Signal::derive(move || name.try_with_value(|n| n.get()).flatten())
                        auto_complete=Signal::derive(move || auto_complete.try_with_value(|a| a.get()).flatten())
                        form=Signal::derive(move || form.try_with_value(|f| f.get()).flatten())
                        disabled=disabled_state
                        required=required_state
                    />
                </Popper>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let is_disabled =
        Signal::derive(move || context.disabled.get() || disabled.get().unwrap_or(false));

    let composed_trigger_ref = use_composed_refs(vec![node_ref, context.trigger_ref]);

    let get_items = StoredValue::new(use_collection::<SelectItemData>());
    let pointer_type_ref: StoredValue<String> = StoredValue::new("touch".to_string());

    // Typeahead search on the trigger (changes selected value immediately)
    let (search_ref, handle_typeahead_search, reset_typeahead) =
        use_typeahead_search(Callback::new(move |search: String| {
            let _ = get_items.try_with_value(|get_items| {
                let items = get_items();
                let enabled_items: Vec<_> =
                    items.iter().filter(|item| !item.data.disabled).collect();
                let current_value = context.value.get_untracked();
                let current_item = enabled_items
                    .iter()
                    .find(|item| Some(&item.data.value) == current_value.as_ref());
                if let Some(next_item) =
                    find_next_item(&enabled_items, &search, current_item.copied())
                {
                    context.on_value_change.run(next_item.data.value.clone());
                }
            });
        }));

    let on_click_stored = StoredValue::new(on_click);
    let on_pointer_down_stored = StoredValue::new(on_pointer_down);
    let on_key_down_stored = StoredValue::new(on_key_down);

    let handle_open = move |pointer_event: Option<(f64, f64)>| {
        if !is_disabled.get_untracked() {
            context.on_open_change.run(true);
            reset_typeahead.run(());
        }
        if let Some(pos) = pointer_event {
            context.trigger_pointer_down_pos_ref.set_value(Some(pos));
        }
    };

    view! {
        <PopperAnchor as_child=true>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_trigger_ref
                    attr:r#type="button"
                    attr:role="combobox"
                    attr:aria-controls=move || context.content_id.get()
                    attr:aria-expanded=move || context.open.get().to_string()
                    attr:aria-required=move || if context.required.get() { Some("true".to_string()) } else { None }
                    attr:aria-autocomplete="none"
                    attr:dir=move || context.dir.get().to_string()
                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                    attr:disabled=data_attr(is_disabled)
                    attr:data-disabled=data_attr(is_disabled)
                    attr:data-placeholder=move || should_show_placeholder(&context.value.get()).then_some("")
                    on:click=compose_callbacks(
                        on_click_stored.get_value(),
                        Some(Callback::new(move |event: ev::MouseEvent| {
                            // Focus for Safari label compatibility
                            if let Some(target) = event.current_target() {
                                let el: web_sys::HtmlElement = target.unchecked_into();
                                let _ = el.focus();
                            }
                            // Open on click for touch/pen devices
                            if pointer_type_ref.try_get_value().is_some_and(|v| v != "mouse") {
                                handle_open(Some((event.page_x() as f64, event.page_y() as f64)));
                            }
                        })),
                        None,
                    )
                    on:pointerdown=compose_callbacks(
                        on_pointer_down_stored.get_value(),
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            let _ = pointer_type_ref.try_set_value(event.pointer_type());

                            // Release implicit pointer capture
                            if let Some(target) = event.target() {
                                let el: web_sys::HtmlElement = target.unchecked_into();
                                if el.has_pointer_capture(event.pointer_id()) {
                                    let _ = el.release_pointer_capture(event.pointer_id());
                                }
                            }

                            // Only open on left mouse button click (not touch/pen)
                            if event.button() == 0 && !event.ctrl_key() && event.pointer_type() == "mouse" {
                                handle_open(Some((event.page_x() as f64, event.page_y() as f64)));
                                event.prevent_default();
                            }
                        })),
                        None,
                    )
                    on:keydown=compose_callbacks(
                        on_key_down_stored.get_value(),
                        Some(Callback::new(move |event: ev::KeyboardEvent| {
                            let is_typing_ahead = search_ref.try_get_value().is_some_and(|s| !s.is_empty());
                            let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();
                            if !is_modifier_key && event.key().len() == 1 {
                                handle_typeahead_search.run(event.key());
                            }
                            if is_typing_ahead && event.key() == " " {
                                return;
                            }
                            if OPEN_KEYS.contains(&event.key().as_str()) {
                                handle_open(None);
                                event.prevent_default();
                            }
                        })),
                        None,
                    )
                    {..attrs}
                >
                    {children.try_with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </AttributeInterceptor>
        </PopperAnchor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectValue
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectValue(
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let (set_value_node_has_children,) = expect_context::<(WriteSignal<bool>,)>();

    let has_children = children.try_with_value(|c| c.is_some()).unwrap_or(false);
    set_value_node_has_children.set(has_children);

    let composed_ref = use_composed_refs(vec![node_ref, context.value_node_ref]);

    // The selected item's text will render its content via a portal into this span
    // when it is selected AND this component has no static children.
    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=composed_ref
                attr:style="pointer-events: none;"
                {..attrs}
            >
                {move || {
                    if should_show_placeholder(&context.value.get()) {
                        let ph = placeholder.get().unwrap_or_default();
                        Some(ph.into_any())
                    } else {
                        children.try_with_value(|c| c.as_ref().map(|c| c().into_any())).flatten()
                    }
                }}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectIcon
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectIcon(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=node_ref
                attr:aria-hidden="true"
                {..attrs}
            >
                {children.try_with_value(|children| {
                    children.as_ref().map(|children| children()).unwrap_or_else(|| "▼".into_any())
                })}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // Capture contexts for re-provision inside the portal boundary
    let select_context = expect_context::<SelectContextValue>();
    let popper_scope = use_popper_scope();
    let collection_scope = use_collection_scope::<SelectItemData>();

    view! {
        <ScopedPortal container=container container_ref=container_ref force_mount=force_mount>
            <Provider value=select_context>
                {
                    if let Some(scope) = popper_scope {
                        provide_popper_scope(scope);
                    }
                    if let Some(scope) = collection_scope {
                        provide_collection_scope(scope);
                    }
                    children.try_with_value(|children| children())
                }
            </Provider>
        </ScopedPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectContent
 * -----------------------------------------------------------------------------------------------*/

/// When closed, renders children into a DocumentFragment to keep collection items registered.
/// When open, renders the full SelectContentImpl.
#[component]
pub fn SelectContent(
    #[prop(into, optional)] position: MaybeProp<String>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(10.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();

    let on_close_auto_focus = StoredValue::new(on_close_auto_focus);
    let on_escape_key_down = StoredValue::new(on_escape_key_down);
    let on_pointer_down_outside = StoredValue::new(on_pointer_down_outside);

    // Track whether the select has ever been opened. Before the first open, we render
    // children in a hidden container so that SelectItemText can detect the selected item
    // and copy its text into SelectValue. This mirrors React's DocumentFragment approach.
    let has_been_opened = Memo::new(move |prev: Option<&bool>| {
        if prev == Some(&true) {
            true
        } else {
            context.open.get()
        }
    });

    // Minimal context for the hidden pre-mount container. Children (SelectViewport,
    // SelectItem, etc.) call expect_context for these, so we must provide them.
    let hidden_content_context = SelectContentContextValue {
        content_ref: AnyNodeRef::new(),
        viewport_ref: AnyNodeRef::new(),
        on_item_leave: Callback::new(|_| {}),
        position: StoredValue::new("popper".to_string()),
        is_positioned: signal(false).0,
        search_ref: StoredValue::new(String::new()),
    };
    let hidden_item_ref_callback: Callback<(
        Option<SendWrapper<web_sys::HtmlElement>>,
        String,
        bool,
    )> = Callback::new(|_| {});

    // React's SelectContent renders SelectContentImpl directly when open (no Presence
    // wrapper). When closed, it renders children into a DocumentFragment to keep
    // collection items registered. We mirror this with a simple conditional.
    view! {
        {move || {
            if context.open.get() {
                // When open, render SelectContentImpl directly (matches React).
                Some(view! {
                    <SelectContentImpl
                        position=position
                        on_close_auto_focus=on_close_auto_focus
                        on_escape_key_down=on_escape_key_down
                        on_pointer_down_outside=on_pointer_down_outside
                        side=side
                        side_offset=side_offset
                        align=align
                        align_offset=align_offset
                        arrow_padding=arrow_padding
                        avoid_collisions=avoid_collisions
                        collision_boundary=collision_boundary
                        collision_padding=collision_padding
                        sticky=sticky
                        hide_when_detached=hide_when_detached
                        update_position_strategy=update_position_strategy
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                    </SelectContentImpl>
                }.into_any())
            } else if !has_been_opened.get() {
                // Before first open: render children in a hidden container so that
                // SelectItemText can portal the selected item's text into SelectValue.
                Some(view! {
                    <Provider value=hidden_content_context>
                        <Provider value=hidden_item_ref_callback>
                            <div style="display: none; position: absolute; overflow: hidden; pointer-events: none;">
                                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                            </div>
                        </Provider>
                    </Provider>
                }.into_any())
            } else {
                None
            }
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SelectContentImpl(
    #[prop(into, optional)] position: MaybeProp<String>,
    on_close_auto_focus: StoredValue<Option<Callback<web_sys::Event>>>,
    #[allow(unused_variables)] on_escape_key_down: StoredValue<
        Option<Callback<web_sys::KeyboardEvent>>,
    >,
    #[allow(unused_variables)] on_pointer_down_outside: StoredValue<
        Option<Callback<web_sys::CustomEvent>>,
    >,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(10.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let content_ref = AnyNodeRef::new();
    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    let _get_items = StoredValue::new(use_collection::<SelectItemData>());
    let (is_positioned, set_is_positioned) = signal(false);

    let position_mode = StoredValue::new(
        position
            .get_untracked()
            .unwrap_or_else(|| "item-aligned".to_string()),
    );

    let search_ref: StoredValue<String> = StoredValue::new(String::new());

    // Focus guards
    use_focus_guards();

    // selectedItem tracking (minimal)
    let selected_item_ref: StoredValue<Option<SendWrapper<web_sys::HtmlElement>>> =
        StoredValue::new(None);
    let first_valid_item_found_ref: StoredValue<bool> = StoredValue::new(false);

    let item_ref_callback = Callback::new(
        move |args: (Option<SendWrapper<web_sys::HtmlElement>>, String, bool)| {
            let (node, value, disabled) = args;
            let is_first_valid_item =
                !first_valid_item_found_ref.try_get_value().unwrap_or(true) && !disabled;
            let is_selected_item = context
                .value
                .get_untracked()
                .as_ref()
                .is_some_and(|v| v == &value);
            if is_selected_item || is_first_valid_item {
                let _ = selected_item_ref.try_set_value(node);
                if is_first_valid_item {
                    let _ = first_valid_item_found_ref.try_set_value(true);
                }
            }
        },
    );

    // Handle item leave: focus the content element
    let on_item_leave = Callback::new(move |_: ()| {
        if let Some(content_el) = content_ref.get_untracked() {
            let el: web_sys::HtmlElement = (*content_el).clone().unchecked_into();
            let _ = el.focus();
        }
    });

    let content_context = SelectContentContextValue {
        content_ref,
        viewport_ref,
        on_item_leave,
        position: position_mode,
        is_positioned,
        search_ref,
    };

    let content_wrapper_ref = AnyNodeRef::new();
    let viewport_context = SelectViewportContextValue {
        content_wrapper_ref,
    };

    let is_popper = position_mode.get_value() == "popper";

    // onCloseAutoFocus: restore focus to trigger
    let on_unmount_auto_focus = Callback::new(move |event: web_sys::Event| {
        let _ = on_close_auto_focus.try_with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        if let Some(trigger) = context.trigger_ref.get_untracked() {
            let el: &web_sys::HtmlElement = (*trigger).unchecked_ref();
            let opts = web_sys::FocusOptions::new();
            opts.set_prevent_scroll(true);
            let _ = el.focus_with_options(&opts);
        }
        event.prevent_default();
    });

    let on_dismiss = Callback::new(move |_: ()| {
        context.on_open_change.run(false);
    });

    // Reset first_valid_item tracking on each render
    let _ = first_valid_item_found_ref.try_set_value(false);

    // Typeahead search
    let get_items = use_collection::<SelectItemData>();
    let get_items = StoredValue::new(get_items);

    let (_typeahead_search_ref, handle_typeahead_search) =
        use_typeahead_search_no_reset(Callback::new(move |search: String| {
            let _ = content_context.search_ref.try_set_value(search.clone());

            // Find and focus the matching item (mirrors React behavior)
            let _ = get_items.try_with_value(|get_items| {
                let items = get_items();
                let enabled_items: Vec<_> =
                    items.iter().filter(|item| !item.data.disabled).collect();
                let current_item = enabled_items.iter().find(|item| {
                    item.r#ref.get_untracked().is_some_and(|el| {
                        let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.active_element())
                            .is_some_and(|active| {
                                let active_node: &web_sys::Node = active.unchecked_ref();
                                let el_node: &web_sys::Node = el.unchecked_ref();
                                active_node.is_same_node(Some(el_node))
                            })
                    })
                });
                if let Some(next_item) =
                    find_next_item(&enabled_items, &search, current_item.copied())
                    && let Some(el) = next_item.r#ref.get_untracked()
                {
                    let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
                    // Use setTimeout to avoid focus during keydown (matches React)
                    let closure = Closure::once_into_js(move || {
                        let opts = web_sys::FocusOptions::new();
                        opts.set_prevent_scroll(true);
                        let _ = el.focus_with_options(&opts);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist.")
                        .set_timeout_with_callback(closure.unchecked_ref());
                }
            });
        }));

    // Focus selected item after positioned
    Effect::new(move |_| {
        if is_positioned.get() {
            if let Some(selected_el) = selected_item_ref.try_get_value().flatten() {
                let opts = web_sys::FocusOptions::new();
                opts.set_prevent_scroll(true);
                let _ = selected_el.focus_with_options(&opts);
            } else if let Some(content_el) = content_ref.get() {
                let el: &web_sys::HtmlElement = (*content_el).unchecked_ref();
                let opts = web_sys::FocusOptions::new();
                opts.set_prevent_scroll(true);
                let _ = el.focus_with_options(&opts);
            }
        }
    });

    // Keyboard handler
    let on_key_down = move |event: web_sys::KeyboardEvent| {
        let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();

        // Prevent tab navigation
        if event.key() == "Tab" {
            event.prevent_default();
            return;
        }

        // Typeahead search for single printable characters
        if !is_modifier_key && event.key().len() == 1 {
            handle_typeahead_search.run(event.key());
        }

        let key = event.key();
        if ["ArrowUp", "ArrowDown", "Home", "End"].contains(&key.as_str()) {
            let _ = get_items.try_with_value(|get_items| {
                let items = get_items();
                let enabled_items: Vec<_> =
                    items.iter().filter(|item| !item.data.disabled).collect();
                let mut candidate_nodes: Vec<web_sys::HtmlElement> = enabled_items
                    .iter()
                    .filter_map(|item| {
                        item.r#ref
                            .get_untracked()
                            .map(|el| (*el).clone().unchecked_into::<web_sys::HtmlElement>())
                    })
                    .collect();

                if key == "ArrowUp" || key == "End" {
                    candidate_nodes.reverse();
                }
                if (key == "ArrowUp" || key == "ArrowDown")
                    && let Some(target) = event.target()
                {
                    let current_el: web_sys::HtmlElement = target.unchecked_into();
                    if let Some(current_index) =
                        candidate_nodes.iter().position(|n| *n == current_el)
                    {
                        candidate_nodes = candidate_nodes[current_index + 1..].to_vec();
                    }
                }

                // Focus first candidate
                if let Some(first) = candidate_nodes.first() {
                    let opts = web_sys::FocusOptions::new();
                    opts.set_prevent_scroll(true);
                    let _ = first.focus_with_options(&opts);
                }
            });
            event.prevent_default();
        }
    };

    // Item-aligned positioning Effect
    if !is_popper {
        // Track the pending rAF handle so we can cancel it if the content
        // unmounts before the callback fires (prevents WASM panic from
        // accessing disposed signals).
        let raf_id: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
        {
            let raf_id = raf_id.clone();
            on_cleanup(move || {
                let id = raf_id.load(Ordering::Relaxed);
                if id != 0 {
                    if let Some(win) = web_sys::window() {
                        let _ = win.cancel_animation_frame(id);
                    }
                }
            });
        }

        Effect::new(move |_| {
            // Track value changes so this Effect re-runs when the selection changes.
            // This ensures item-aligned positioning recalculates after a new item is
            // selected (e.g., in always-open Chromatic stories).
            let _value = context.value.get();

            // Cancel any pending rAF from a previous Effect run.
            let prev = raf_id.load(Ordering::Relaxed);
            if prev != 0 {
                if let Some(win) = web_sys::window() {
                    let _ = win.cancel_animation_frame(prev);
                }
                raf_id.store(0, Ordering::Relaxed);
            }

            // Reset so the focus Effect re-runs after repositioning completes.
            set_is_positioned.set(false);

            let raf_id_inner = raf_id.clone();
            let cb = Closure::once_into_js(move || {
                raf_id_inner.store(0, Ordering::Relaxed);

                let Some(wrapper_el) = content_wrapper_ref.get_untracked() else {
                    return;
                };
                let Some(content_el) = content_ref.get_untracked() else {
                    return;
                };
                let Some(trigger_el) = context.trigger_ref.get_untracked() else {
                    return;
                };
                let Some(value_node_el) = context.value_node_ref.get_untracked() else {
                    return;
                };
                let Some(viewport_el) = viewport_ref.get_untracked() else {
                    return;
                };

                let wrapper: &web_sys::HtmlElement = (*wrapper_el).unchecked_ref();
                let content: &web_sys::HtmlElement = (*content_el).unchecked_ref();
                let trigger: &web_sys::HtmlElement = (*trigger_el).unchecked_ref();
                let value_node: &web_sys::HtmlElement = (*value_node_el).unchecked_ref();
                let viewport: &web_sys::HtmlElement = (*viewport_el).unchecked_ref();

                // Clear old positioning properties before recalculating.
                // position_item_aligned conditionally sets top OR bottom, so stale
                // values from a previous layout must be removed.
                let ws = wrapper.style();
                let _ = ws.remove_property("top");
                let _ = ws.remove_property("bottom");
                let _ = ws.remove_property("left");
                let _ = ws.remove_property("right");
                let _ = ws.remove_property("height");
                let _ = ws.remove_property("min-width");
                let _ = ws.remove_property("min-height");
                let _ = ws.remove_property("max-height");
                let _ = ws.remove_property("margin");

                // Look up the selected item from the collection based on the
                // current value. This is fresher than selected_item_ref which is
                // only populated during the initial render pass.
                let selected_item = get_items
                    .try_with_value(|get_items| {
                        let items = get_items();
                        let current_value = context.value.get_untracked();
                        items.iter().find_map(|item| {
                            if current_value
                                .as_ref()
                                .is_some_and(|v| v == &item.data.value)
                            {
                                item.r#ref.get_untracked().map(|el| {
                                    SendWrapper::new(
                                        (*el).clone().unchecked_into::<web_sys::HtmlElement>(),
                                    )
                                })
                            } else {
                                None
                            }
                        })
                    })
                    .flatten()
                    // Fall back to selected_item_ref (first-valid-item from initial render)
                    .or_else(|| selected_item_ref.try_get_value().flatten());

                // Update selected_item_ref so the focus Effect uses the right element.
                let _ = selected_item_ref.try_set_value(selected_item.clone());

                // Determine is_first/is_last from collection items
                let (is_first, is_last) = get_items
                    .try_with_value(|get_items| {
                        let items = get_items();
                        let is_first = items.first().is_some_and(|first| {
                            first.r#ref.get_untracked().is_some_and(|el| {
                                selected_item.as_ref().is_some_and(|si| {
                                    let el: &web_sys::Node = (*el).unchecked_ref();
                                    let si: &web_sys::Node = (**si).unchecked_ref();
                                    el.is_same_node(Some(si))
                                })
                            })
                        });
                        let is_last = items.last().is_some_and(|last| {
                            last.r#ref.get_untracked().is_some_and(|el| {
                                selected_item.as_ref().is_some_and(|si| {
                                    let el: &web_sys::Node = (*el).unchecked_ref();
                                    let si: &web_sys::Node = (**si).unchecked_ref();
                                    el.is_same_node(Some(si))
                                })
                            })
                        });
                        (is_first, is_last)
                    })
                    .unwrap_or((false, false));

                if let Some(selected_item) = selected_item.as_deref() {
                    let dir = context.dir.get_untracked();
                    position_item_aligned(
                        wrapper,
                        content,
                        trigger,
                        value_node,
                        viewport,
                        selected_item,
                        dir,
                        is_first,
                        is_last,
                    );
                } else {
                    // No selected item — position near the trigger as a fallback
                    let trigger_rect = trigger.get_bounding_client_rect();
                    let _ = wrapper
                        .style()
                        .set_property("left", &format!("{}px", trigger_rect.left()));
                    let _ = wrapper
                        .style()
                        .set_property("top", &format!("{}px", trigger_rect.bottom()));
                    let _ = wrapper
                        .style()
                        .set_property("min-width", &format!("{}px", trigger_rect.width()));
                }

                // Copy z-index from content to wrapper
                if let Ok(Some(styles)) = web_sys::window()
                    .expect("Window should exist.")
                    .get_computed_style(content)
                {
                    let z_index = styles.get_property_value("z-index").unwrap_or_default();
                    if !z_index.is_empty() && z_index != "auto" {
                        let _ = wrapper.style().set_property("z-index", &z_index);
                    }
                }

                set_is_positioned.set(true);
            });
            if let Ok(id) = web_sys::window()
                .expect("Window should exist.")
                .request_animation_frame(cb.unchecked_ref())
            {
                raf_id.store(id, Ordering::Relaxed);
            }
        });
    }

    if is_popper {
        view! {
            <Provider value=content_context>
                <Provider value=viewport_context>
                    <Provider value=item_ref_callback>
                        <FocusScope
                            as_child=true
                            trapped=Signal::derive(move || context.open.get())
                            on_mount_auto_focus=Callback::new(move |event: web_sys::Event| {
                                event.prevent_default();
                            })
                            on_unmount_auto_focus=on_unmount_auto_focus
                        >
                            <DismissableLayer
                                as_child=true
                                disable_outside_pointer_events=true
                                on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                                    let _ = on_escape_key_down.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    let _ = on_pointer_down_outside.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    event.prevent_default();
                                })
                                on_dismiss=Callback::new(move |_: ()| {
                                    on_dismiss.run(());
                                })
                            >
                                <PopperContent
                                    side=side
                                    side_offset=side_offset
                                    align=align
                                    align_offset=align_offset
                                    arrow_padding=arrow_padding
                                    avoid_collisions=avoid_collisions
                                    collision_boundary=collision_boundary
                                    collision_padding=collision_padding
                                    sticky=sticky
                                    hide_when_detached=hide_when_detached
                                    update_position_strategy=update_position_strategy
                                    as_child=as_child
                                    node_ref=composed_refs
                                    on_placed=Some(Callback::new(move |_: ()| {
                                        set_is_positioned.set(true);
                                    }))
                                    attr:role="listbox"
                                    attr:id=move || context.content_id.get()
                                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                                    attr:dir=move || context.dir.get().to_string()
                                    attr:style="display: flex; flex-direction: column; outline: none; box-sizing: border-box; --radix-select-content-transform-origin: var(--radix-popper-transform-origin); --radix-select-content-available-width: var(--radix-popper-available-width); --radix-select-content-available-height: var(--radix-popper-available-height); --radix-select-trigger-width: var(--radix-popper-anchor-width); --radix-select-trigger-height: var(--radix-popper-anchor-height);"
                                    on:keydown=on_key_down
                                    on:contextmenu=move |event: ev::MouseEvent| {
                                        event.prevent_default();
                                    }
                                >
                                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                                </PopperContent>
                            </DismissableLayer>
                        </FocusScope>
                    </Provider>
                </Provider>
            </Provider>
        }
        .into_any()
    } else {
        view! {
            <Provider value=content_context>
                <Provider value=viewport_context>
                    <Provider value=item_ref_callback>
                        <FocusScope
                            as_child=true
                            trapped=Signal::derive(move || context.open.get())
                            on_mount_auto_focus=Callback::new(move |event: web_sys::Event| {
                                event.prevent_default();
                            })
                            on_unmount_auto_focus=on_unmount_auto_focus
                        >
                            <DismissableLayer
                                as_child=true
                                disable_outside_pointer_events=true
                                on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                                    let _ = on_escape_key_down.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    let _ = on_pointer_down_outside.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    event.prevent_default();
                                })
                                on_dismiss=Callback::new(move |_: ()| {
                                    on_dismiss.run(());
                                })
                            >
                                <SelectItemAlignedPosition
                                    content_wrapper_ref=content_wrapper_ref
                                    as_child=as_child
                                    node_ref=composed_refs
                                    on_key_down=Callback::new(on_key_down)
                                >
                                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                                </SelectItemAlignedPosition>
                            </DismissableLayer>
                        </FocusScope>
                    </Provider>
                </Provider>
            </Provider>
        }
        .into_any()
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItemAlignedPosition
 * Separate component so that user attrs (class, data-testid, etc.) forwarded via
 * FocusScope/DismissableLayer land on the inner content div, not the outer
 * positioning wrapper. This mirrors React's SelectItemAlignedPosition which
 * explicitly spreads contentProps on the inner Primitive.
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SelectItemAlignedPosition(
    content_wrapper_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into)] on_key_down: Callback<web_sys::KeyboardEvent>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SelectContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_content_ref = use_composed_refs(vec![node_ref, content_ref]);

    // Transfer caller-added attributes from the wrapper div to the inner content div.
    //
    // In React, SelectItemAlignedPosition explicitly spreads contentProps (className,
    // data-testid, etc.) onto the inner Primitive.div, while the wrapper div only has
    // hardcoded positioning styles. In Leptos, attrs from FocusScope/DismissableLayer's
    // as_child chain bypass the component boundary and land on the first DOM element
    // (the wrapper). This Effect moves class, non-positioning styles, and user attrs
    // from the wrapper to the inner content div after mount.
    Effect::new(move |_| {
        let (Some(wrapper), Some(inner)) = (content_wrapper_ref.get(), content_ref.get()) else {
            return;
        };
        let wrapper: web_sys::HtmlElement = (*wrapper).clone().unchecked_into();
        let inner: web_sys::HtmlElement = (*inner).clone().unchecked_into();

        // Transfer class
        let wrapper_class = wrapper.get_attribute("class").unwrap_or_default();
        if !wrapper_class.is_empty() {
            let inner_class = inner.get_attribute("class").unwrap_or_default();
            let combined = if inner_class.is_empty() {
                wrapper_class
            } else {
                format!("{inner_class} {wrapper_class}")
            };
            inner.set_attribute("class", &combined).ok();
            wrapper.remove_attribute("class").ok();
        }

        // Transfer non-positioning style properties
        let wrapper_style = wrapper.style();
        let inner_style = inner.style();
        let mut caller_props: Vec<(String, String)> = Vec::new();
        let len = wrapper_style.length();
        for i in 0..len {
            let prop = wrapper_style.item(i);
            if prop.is_empty() {
                continue;
            }
            // Skip properties managed by SelectItemAlignedPosition's positioning
            let is_positioning = matches!(
                prop.as_str(),
                "display"
                    | "flex-direction"
                    | "position"
                    | "top"
                    | "bottom"
                    | "left"
                    | "right"
                    | "min-width"
                    | "height"
                    | "min-height"
                    | "max-height"
                    | "margin"
                    | "z-index"
                    | "pointer-events"
            );
            if !is_positioning && let Ok(value) = wrapper_style.get_property_value(&prop) {
                caller_props.push((prop, value));
            }
        }
        for (prop, value) in &caller_props {
            let _ = inner_style.set_property(prop, value);
            let _ = wrapper_style.remove_property(prop);
        }

        // Restore critical wrapper styles that may have been overwritten by
        // attr:style forwarding through the as_child chain. When a caller sets
        // attr:style on SelectContent (e.g., `attr:style="opacity: 0.7;"`), Leptos
        // forwards it via FocusScope/DismissableLayer's as_child=true, which
        // replaces the wrapper's entire style attribute — destroying the hardcoded
        // `display: flex; flex-direction: column; position: fixed;`. We restore
        // these here after transferring caller styles to the inner div.
        let _ = wrapper_style.set_property("display", "flex");
        let _ = wrapper_style.set_property("flex-direction", "column");
        let _ = wrapper_style.set_property("position", "fixed");

        // Transfer non-internal attributes from wrapper to inner
        let attrs = wrapper.attributes();
        let mut attrs_to_transfer: Vec<(String, String)> = Vec::new();
        for i in 0..attrs.length() {
            if let Some(attr) = attrs.item(i) {
                let name = attr.name();
                // Skip wrapper's own attributes
                if matches!(name.as_str(), "style" | "class") {
                    continue;
                }
                attrs_to_transfer.push((name, attr.value()));
            }
        }
        for (name, value) in &attrs_to_transfer {
            inner.set_attribute(name, value).ok();
            wrapper.remove_attribute(name).ok();
        }
    });

    view! {
        <div
            node_ref=content_wrapper_ref
            style="display: flex; flex-direction: column; position: fixed;"
        >
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_content_ref
                attr:role="listbox"
                attr:id=move || context.content_id.get()
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:dir=move || context.dir.get().to_string()
                attr:style="box-sizing: border-box; max-height: 100%; outline: none;"
                on:keydown=move |event: web_sys::KeyboardEvent| {
                    on_key_down.run(event);
                }
                on:contextmenu=move |event: ev::MouseEvent| {
                    event.prevent_default();
                }
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </div>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectViewport(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let content_context = expect_context::<SelectContentContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, content_context.viewport_ref]);

    view! {
        <>
            // Hide scrollbars cross-browser
            <style>"[data-radix-select-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-select-viewport]::-webkit-scrollbar{display:none}"</style>
            <CollectionSlot<SelectItemData> item_data_type=ITEM_DATA_PHANTOM>
                <AttributeInterceptor let:attrs>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_ref
                        attr:data-radix-select-viewport=""
                        attr:role="presentation"
                        attr:style="position: relative; flex: 1; overflow: hidden auto;"
                        {..attrs}
                    >
                        {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                    </Primitive>
                </AttributeInterceptor>
            </CollectionSlot<SelectItemData>>
        </>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectGroup
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectGroup(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let group_id = use_id(None);

    let group_context = SelectGroupContextValue { id: group_id };

    view! {
        <Provider value=group_context>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    attr:role="group"
                    attr:aria-labelledby=move || group_id.get()
                    {..attrs}
                >
                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </AttributeInterceptor>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectLabel(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let group_context = expect_context::<SelectGroupContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:id=move || group_context.id.get()
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] text_value: MaybeProp<String>,
    #[prop(into, optional)] on_pointer_up: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let content_context = expect_context::<SelectContentContextValue>();
    let item_ref_callback =
        expect_context::<Callback<(Option<SendWrapper<web_sys::HtmlElement>>, String, bool)>>();

    let disabled = prop_or_default(disabled);
    let value = StoredValue::new(value);
    let is_selected = Signal::derive(move || {
        value
            .try_get_value()
            .is_some_and(|val| context.value.get().is_some_and(|v| v == val))
    });
    let (is_focused, set_is_focused) = signal(false);
    let (text_value_state, set_text_value) = signal(text_value.get_untracked().unwrap_or_default());
    let text_id = use_id(None);
    let pointer_type_ref: StoredValue<String> = StoredValue::new("touch".to_string());
    let item_node_ref = AnyNodeRef::new();

    // Register with item_ref_callback when mounted
    let composed_item_ref = use_composed_refs(vec![node_ref, item_node_ref]);
    Effect::new(move |_| {
        if let Some(val) = value.try_get_value() {
            let node = item_node_ref.get().map(|el| {
                let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
                SendWrapper::new(el)
            });
            item_ref_callback.run((node, val, disabled.get_untracked()));
        }
    });

    let handle_select = move || {
        if !disabled.get_untracked()
            && let Some(val) = value.try_get_value()
        {
            context.on_value_change.run(val);
            // Defer the close to the next task so that reactive effects triggered by the
            // value change (e.g. text copying in SelectItemText) can settle before the
            // content is unmounted. Synchronous close would dispose child scopes while
            // queued effects still reference their StoredValues, causing WASM panics.
            let cb = Closure::once_into_js(move || {
                context.on_open_change.run(false);
            });
            web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback(cb.unchecked_ref())
                .expect("setTimeout should succeed.");
        }
    };

    let on_item_text_change =
        Callback::new(move |node: Option<SendWrapper<web_sys::HtmlElement>>| {
            if let Some(node) = &node {
                let text = node.text_content().unwrap_or_default().trim().to_string();
                if !text.is_empty() {
                    set_text_value.set(text);
                }
            }
        });

    let item_context = SelectItemContextValue {
        value: value.get_value(),
        disabled: disabled.get_untracked(),
        text_id,
        is_selected,
        on_item_text_change,
    };

    let on_pointer_up_stored = StoredValue::new(on_pointer_up);
    let on_pointer_down_stored = StoredValue::new(on_pointer_down);
    let on_pointer_move_stored = StoredValue::new(on_pointer_move);
    let on_pointer_leave_stored = StoredValue::new(on_pointer_leave);
    let on_key_down_stored = StoredValue::new(on_key_down);
    let on_focus_stored = StoredValue::new(on_focus);
    let on_blur_stored = StoredValue::new(on_blur);
    let on_click_stored = StoredValue::new(on_click);

    view! {
        <Provider value=item_context>
            <CollectionItemSlot
                item_data_type=ITEM_DATA_PHANTOM
                item_data=MaybeProp::derive(move || {
                    value.try_get_value().map(|val| SelectItemData {
                        value: val,
                        disabled: disabled.get(),
                        text_value: text_value_state.get(),
                    })
                })
                node_ref=composed_item_ref
            >
                <AttributeInterceptor let:attrs>
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_item_ref
                        attr:role="option"
                        attr:aria-labelledby=move || text_id.get()
                        attr:data-highlighted=move || is_focused.get().then_some("")
                        attr:aria-selected=move || if is_selected.get() && is_focused.get() { Some("true".to_string()) } else { None }
                        attr:data-state=move || if is_selected.get() { "checked" } else { "unchecked" }
                        attr:aria-disabled=move || disabled.get().then_some("true".to_string())
                        attr:data-disabled=data_attr(disabled)
                        attr:tabindex=move || if disabled.get() { None } else { Some("-1".to_string()) }
                        // Event handlers are inlined rather than using compose_callbacks
                        // with Callback::new(...) because Callback::new creates a StoredValue
                        // in the reactive scope. When the scope is disposed during unmount,
                        // browser events (e.g. blur from focus restoration) can fire after
                        // disposal and try to invoke the disposed Callback, causing a WASM panic.
                        on:focus=move |event: ev::FocusEvent| {
                            if let Some(Some(cb)) = on_focus_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                set_is_focused.set(true);
                            }
                        }
                        on:blur=move |event: ev::FocusEvent| {
                            if let Some(Some(cb)) = on_blur_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                set_is_focused.set(false);
                            }
                        }
                        on:click=move |event: ev::MouseEvent| {
                            if let Some(Some(cb)) = on_click_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented()
                                && pointer_type_ref.try_get_value().is_some_and(|v| v != "mouse")
                            {
                                handle_select();
                            }
                        }
                        on:pointerup=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_up_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented()
                                && pointer_type_ref.try_get_value().is_some_and(|v| v == "mouse")
                            {
                                handle_select();
                            }
                        }
                        on:pointerdown=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_down_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                let _ = pointer_type_ref.try_set_value(event.pointer_type());
                            }
                        }
                        on:pointermove=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_move_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                let _ = pointer_type_ref.try_set_value(event.pointer_type());
                                if disabled.get_untracked() {
                                    content_context.on_item_leave.run(());
                                } else if event.pointer_type() == "mouse"
                                    && let Some(target) = event.current_target()
                                {
                                    let el: web_sys::HtmlElement = target.unchecked_into();
                                    let mut opts = web_sys::FocusOptions::new();
                                    opts.set_prevent_scroll(true);
                                    let _ = el.focus_with_options(&opts);
                                }
                            }
                        }
                        on:pointerleave=move |event: ev::PointerEvent| {
                            if let Some(Some(cb)) = on_pointer_leave_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented()
                                && let Some(target) = event.current_target()
                            {
                                let el: web_sys::Element = target.unchecked_into();
                                let active = web_sys::window()
                                    .and_then(|w| w.document())
                                    .and_then(|d| d.active_element());
                                if active.as_ref() == Some(&el) {
                                    content_context.on_item_leave.run(());
                                }
                            }
                        }
                        on:keydown=move |event: ev::KeyboardEvent| {
                            if let Some(Some(cb)) = on_key_down_stored.try_get_value() {
                                cb.run(event.clone());
                            }
                            if !event.default_prevented() {
                                let is_typing_ahead = content_context.search_ref.try_get_value().is_some_and(|s| !s.is_empty());
                                if is_typing_ahead && event.key() == " " {
                                    return;
                                }
                                if SELECTION_KEYS.contains(&event.key().as_str()) {
                                    handle_select();
                                }
                                if event.key() == " " {
                                    event.prevent_default();
                                }
                            }
                        }
                        {..attrs}
                    >
                        {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                    </Primitive>
                </AttributeInterceptor>
            </CollectionItemSlot>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItemText
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectItemText(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let item_context = expect_context::<SelectItemContextValue>();
    let item_text_ref = AnyNodeRef::new();

    let composed_ref = use_composed_refs(vec![node_ref, item_text_ref]);

    // Notify parent about text node for textValue extraction
    Effect::new(move |_| {
        if let Some(el) = item_text_ref.get() {
            let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
            item_context
                .on_item_text_change
                .run(Some(SendWrapper::new(el)));
        }
    });

    // When this item is selected AND the SelectValue has no static children,
    // portal the text content into SelectValue's span.
    // In Leptos, we can't use React portals, so we use an Effect to copy text content.
    let is_selected = item_context.is_selected;
    Effect::new(move |_| {
        if is_selected.get() && !context.value_node_has_children.get() {
            // Copy text content from this item text into the value node
            if let (Some(text_el), Some(value_el)) =
                (item_text_ref.get(), context.value_node_ref.get())
            {
                let text_el: &web_sys::HtmlElement = (*text_el).unchecked_ref();
                let value_el: &web_sys::HtmlElement = (*value_el).unchecked_ref();
                let text = text_el.text_content().unwrap_or_default();
                value_el.set_text_content(Some(&text));
            }
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=composed_ref
                attr:id=move || item_context.text_id.get()
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItemIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectItemIndicator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let item_context = expect_context::<SelectItemContextValue>();

    view! {
        <Show when=move || item_context.is_selected.get()>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::span
                    as_child=as_child
                    node_ref=node_ref
                    attr:aria-hidden="true"
                    {..attrs}
                >
                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                </Primitive>
            </AttributeInterceptor>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectScrollUpButton
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectScrollUpButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let content_context = expect_context::<SelectContentContextValue>();

    let (can_scroll_up, set_can_scroll_up) = signal(false);

    Effect::new(move |_| {
        if content_context.is_positioned.get()
            && let Some(viewport_el) = content_context.viewport_ref.get()
        {
            let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
            let viewport_clone = viewport.clone();

            let handle_scroll: Closure<dyn FnMut()> = Closure::new(move || {
                set_can_scroll_up.set(viewport_clone.scroll_top() > 0);
            });
            let scroll_fn: js_sys::Function = handle_scroll.into_js_value().unchecked_into();
            let scroll_fn_cleanup = SendWrapper::new(scroll_fn.clone());

            // Initial check
            set_can_scroll_up.set(viewport.scroll_top() > 0);

            let _ = viewport.add_event_listener_with_callback("scroll", &scroll_fn);

            let viewport_cleanup = SendWrapper::new(viewport.clone());
            on_cleanup(move || {
                let _ = viewport_cleanup
                    .remove_event_listener_with_callback("scroll", &scroll_fn_cleanup);
            });
        }
    });

    let viewport_ref = content_context.viewport_ref;

    view! {
        <Show when=move || can_scroll_up.get()>
            <SelectScrollButtonImpl
                as_child=as_child
                node_ref=node_ref
                on_auto_scroll=Callback::new(move |_: ()| {
                    if let Some(viewport_el) = viewport_ref.get_untracked() {
                        let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
                        // Scroll up by approximately one item height (32px default)
                        let item_height = 32;
                        viewport.set_scroll_top(viewport.scroll_top() - item_height);
                    }
                })
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </SelectScrollButtonImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectScrollDownButton
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectScrollDownButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let content_context = expect_context::<SelectContentContextValue>();

    let (can_scroll_down, set_can_scroll_down) = signal(false);

    Effect::new(move |_| {
        if content_context.is_positioned.get()
            && let Some(viewport_el) = content_context.viewport_ref.get()
        {
            let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
            let viewport_clone = viewport.clone();

            let handle_scroll: Closure<dyn FnMut()> = Closure::new(move || {
                let max_scroll = viewport_clone.scroll_height() - viewport_clone.client_height();
                let can_scroll = (viewport_clone.scroll_top() as f64).ceil() < max_scroll as f64;
                set_can_scroll_down.set(can_scroll);
            });
            let scroll_fn: js_sys::Function = handle_scroll.into_js_value().unchecked_into();
            let scroll_fn_cleanup = SendWrapper::new(scroll_fn.clone());

            // Initial check
            let max_scroll = viewport.scroll_height() - viewport.client_height();
            let can_scroll = (viewport.scroll_top() as f64).ceil() < max_scroll as f64;
            set_can_scroll_down.set(can_scroll);

            let _ = viewport.add_event_listener_with_callback("scroll", &scroll_fn);

            let viewport_cleanup = SendWrapper::new(viewport.clone());
            on_cleanup(move || {
                let _ = viewport_cleanup
                    .remove_event_listener_with_callback("scroll", &scroll_fn_cleanup);
            });
        }
    });

    let viewport_ref = content_context.viewport_ref;

    view! {
        <Show when=move || can_scroll_down.get()>
            <SelectScrollButtonImpl
                as_child=as_child
                node_ref=node_ref
                on_auto_scroll=Callback::new(move |_: ()| {
                    if let Some(viewport_el) = viewport_ref.get_untracked() {
                        let viewport: web_sys::HtmlElement = (*viewport_el).clone().unchecked_into();
                        let item_height = 32;
                        viewport.set_scroll_top(viewport.scroll_top() + item_height);
                    }
                })
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </SelectScrollButtonImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectScrollButtonImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SelectScrollButtonImpl(
    on_auto_scroll: Callback<()>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let content_context = expect_context::<SelectContentContextValue>();
    let auto_scroll_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let clear_auto_scroll_timer = move || {
        if let Some(timer_id) = auto_scroll_timer_ref.try_get_value().flatten() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_interval_with_handle(timer_id);
            let _ = auto_scroll_timer_ref.try_set_value(None);
        }
    };

    on_cleanup(clear_auto_scroll_timer);

    // When mounted, scroll active item into view
    let get_items = StoredValue::new(use_collection::<SelectItemData>());
    Effect::new(move |_| {
        let _ = get_items.try_with_value(|get_items| {
            let items = get_items();
            let active = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element());
            if let Some(active) = active {
                for item in &items {
                    if let Some(el) = item.r#ref.get() {
                        let el: &web_sys::Element = (*el).unchecked_ref();
                        if el == &active {
                            let el: &web_sys::HtmlElement = el.unchecked_ref();
                            let options = web_sys::ScrollIntoViewOptions::new();
                            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                            el.scroll_into_view_with_scroll_into_view_options(&options);
                            break;
                        }
                    }
                }
            }
        });
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:aria-hidden="true"
                attr:style="flex-shrink: 0;"
                on:pointerdown=move |_: ev::PointerEvent| {
                    if auto_scroll_timer_ref.try_get_value().flatten().is_none() {
                        let timer_id = web_sys::window()
                            .expect("Window should exist.")
                            .set_interval_with_callback_and_timeout_and_arguments_0(
                                Closure::wrap(Box::new(move || {
                                    on_auto_scroll.run(());
                                }) as Box<dyn FnMut()>)
                                    .into_js_value()
                                    .unchecked_ref(),
                                50,
                            )
                            .expect("setInterval should succeed.");
                        let _ = auto_scroll_timer_ref.try_set_value(Some(timer_id));
                    }
                }
                on:pointermove=move |_: ev::PointerEvent| {
                    content_context.on_item_leave.run(());
                    if auto_scroll_timer_ref.try_get_value().flatten().is_none() {
                        let timer_id = web_sys::window()
                            .expect("Window should exist.")
                            .set_interval_with_callback_and_timeout_and_arguments_0(
                                Closure::wrap(Box::new(move || {
                                    on_auto_scroll.run(());
                                }) as Box<dyn FnMut()>)
                                    .into_js_value()
                                    .unchecked_ref(),
                                50,
                            )
                            .expect("setInterval should succeed.");
                        let _ = auto_scroll_timer_ref.try_set_value(Some(timer_id));
                    }
                }
                on:pointerleave=move |_: ev::PointerEvent| {
                    clear_auto_scroll_timer();
                }
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectSeparator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectSeparator(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:aria-hidden="true"
                {..attrs}
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectArrow
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SelectArrow(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional, default = 10.0.into())] width: Signal<f64>,
    #[prop(into, optional, default = 5.0.into())] height: Signal<f64>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SelectContextValue>();
    let content_context = expect_context::<SelectContentContextValue>();

    let should_show = Signal::derive(move || {
        context.open.get()
            && content_context
                .position
                .try_get_value()
                .is_some_and(|p| p == "popper")
    });

    view! {
        <Show when=move || should_show.get()>
            <PopperArrow
                as_child=as_child
                node_ref=node_ref
                width=width
                height=height
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </PopperArrow>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectBubbleInput (internal)
 * -----------------------------------------------------------------------------------------------*/

/// Hidden native <select> element for form integration.
#[component]
fn SelectBubbleInput(
    value: Signal<Option<String>>,
    name: Signal<Option<String>>,
    auto_complete: Signal<Option<String>>,
    form: Signal<Option<String>>,
    disabled: Signal<bool>,
    required: Signal<bool>,
) -> impl IntoView {
    let select_ref = AnyNodeRef::new();
    let prev_value: StoredValue<Option<String>> = StoredValue::new(None);

    // Bubble value change to parent forms
    Effect::new(move |_| {
        let current_value = value.get();
        let previous = prev_value.try_get_value().flatten();
        let _ = prev_value.try_set_value(current_value.clone());

        if previous != current_value
            && let Some(select_el) = select_ref.get()
        {
            let select_el: web_sys::HtmlSelectElement = (*select_el).clone().unchecked_into();
            select_el.set_value(&current_value.clone().unwrap_or_default());
            let event_init = web_sys::EventInit::new();
            event_init.set_bubbles(true);
            let event = web_sys::Event::new_with_event_init_dict("change", &event_init)
                .expect("Event should be created.");
            let _ = select_el.dispatch_event(&event);
        }
    });

    view! {
        <select
            node_ref=select_ref
            aria-hidden="true"
            tabindex="-1"
            name=move || name.get()
            autocomplete=move || auto_complete.get()
            form=move || form.get()
            disabled=move || disabled.get()
            required=move || required.get()
            style=VISUALLY_HIDDEN_STYLES_STR
            prop:value=move || value.get().unwrap_or_default()
        >
            <option value="">"" </option>
            {move || {
                value.get().filter(|v| !v.is_empty()).map(|v| {
                    let v2 = v.clone();
                    view! { <option value=v>{v2}</option> }
                })
            }}
        </select>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utilities
 * -----------------------------------------------------------------------------------------------*/

fn should_show_placeholder(value: &Option<String>) -> bool {
    match value {
        None => true,
        Some(v) => v.is_empty(),
    }
}

/// Typeahead search hook that returns (search_ref, handle_typeahead_search, reset_typeahead)
fn use_typeahead_search(
    on_search_change: Callback<String>,
) -> (StoredValue<String>, Callback<String>, Callback<()>) {
    let search_ref: StoredValue<String> = StoredValue::new(String::new());
    let timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);

    let handle_typeahead_search = Callback::new(move |key: String| {
        let current = search_ref.try_get_value().unwrap_or_default();
        let search = format!("{}{}", current, key);
        on_search_change.run(search.clone());

        let _ = search_ref.try_set_value(search.clone());
        if let Some(timer) = timer_ref.try_get_value().flatten() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(timer);
        }
        if !search.is_empty() {
            let timer = web_sys::window()
                .expect("Window should exist.")
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    Closure::once_into_js(move || {
                        let _ = search_ref.try_set_value(String::new());
                    })
                    .unchecked_ref(),
                    1000,
                )
                .expect("setTimeout should succeed.");
            let _ = timer_ref.try_set_value(Some(timer));
        }
    });

    let reset_typeahead = Callback::new(move |_: ()| {
        let _ = search_ref.try_set_value(String::new());
        if let Some(timer) = timer_ref.try_get_value().flatten() {
            web_sys::window()
                .expect("Window should exist.")
                .clear_timeout_with_handle(timer);
        }
    });

    on_cleanup(move || {
        if let Some(timer) = timer_ref.try_get_value().flatten()
            && let Some(window) = web_sys::window()
        {
            window.clear_timeout_with_handle(timer);
        }
    });

    (search_ref, handle_typeahead_search, reset_typeahead)
}

/// Variant without reset (for content-level typeahead)
fn use_typeahead_search_no_reset(
    on_search_change: Callback<String>,
) -> (StoredValue<String>, Callback<String>) {
    let (search_ref, handle, _) = use_typeahead_search(on_search_change);
    (search_ref, handle)
}

/// Find the next item matching the typeahead search
fn find_next_item<'a>(
    items: &'a [&'a CollectionItemValue<SelectItemData>],
    search: &str,
    current_item: Option<&'a CollectionItemValue<SelectItemData>>,
) -> Option<&'a CollectionItemValue<SelectItemData>> {
    if search.is_empty() {
        return None;
    }

    // Normalize repeated characters (e.g., "aaa" → "a")
    let is_repeated =
        search.len() > 1 && search.chars().all(|c| c == search.chars().next().unwrap());
    let normalized_search = if is_repeated {
        search.chars().next().unwrap().to_string()
    } else {
        search.to_string()
    };

    let current_index = current_item
        .and_then(|current| items.iter().position(|item| std::ptr::eq(*item, current)))
        .unwrap_or(0);

    // Wrap array starting from current position
    let mut wrapped_items: Vec<&CollectionItemValue<SelectItemData>> = items.to_vec();
    let start = current_index.min(wrapped_items.len());
    let (a, b) = wrapped_items.split_at(start);
    wrapped_items = [b, a].concat();

    // For single character search, exclude current item to enable cycling
    let exclude_current = normalized_search.len() == 1;
    if exclude_current && let Some(current) = current_item {
        wrapped_items.retain(|item| !std::ptr::eq(*item, current));
    }

    let normalized_lower = normalized_search.to_lowercase();
    let next = wrapped_items.iter().find(|item| {
        item.data
            .text_value
            .to_lowercase()
            .starts_with(&normalized_lower)
    });

    // Don't return current item if it's the same
    next.and_then(|item| {
        if current_item.is_some_and(|current| std::ptr::eq(*item, current)) {
            None
        } else {
            Some(*item)
        }
    })
}

/// aria-hide everything except the given element.
/// Simplified version — sets aria-hidden on siblings.
#[allow(dead_code)]
fn hide_others(content: &web_sys::HtmlElement) {
    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };
    let body = match document.body() {
        Some(b) => b,
        None => return,
    };

    let content_node: &web_sys::Node = content.unchecked_ref();

    // Walk up from content to body, hiding siblings at each level
    let mut current: web_sys::Node = content_node.clone();
    loop {
        let parent = match current.parent_node() {
            Some(p) => p,
            None => break,
        };
        let parent_el: &web_sys::Element = match parent.dyn_ref() {
            Some(el) => el,
            None => break,
        };

        let children = parent_el.children();
        for i in 0..children.length() {
            if let Some(child) = children.item(i) {
                let child_node: &web_sys::Node = child.unchecked_ref();
                if !child_node.is_same_node(Some(&current)) {
                    let tag = child.tag_name().to_lowercase();
                    if tag != "script" && tag != "style" {
                        let _ = child.set_attribute("aria-hidden", "true");
                        let _ = child.set_attribute("data-radix-select-hide", "");
                    }
                }
            }
        }

        let body_node: &web_sys::Node = body.unchecked_ref();
        if parent.is_same_node(Some(body_node)) {
            break;
        }
        current = parent;
    }
}

/// Margin around the select content for item-aligned positioning.
const CONTENT_MARGIN: f64 = 10.0;

/// Position the select content so the selected item aligns with the trigger.
/// This is a port of React's `SelectItemAlignedPosition` positioning logic.
#[allow(clippy::too_many_arguments)]
fn position_item_aligned(
    wrapper: &web_sys::HtmlElement,
    content: &web_sys::HtmlElement,
    trigger: &web_sys::HtmlElement,
    value_node: &web_sys::HtmlElement,
    viewport: &web_sys::HtmlElement,
    selected_item: &web_sys::HtmlElement,
    dir: Direction,
    is_first_item: bool,
    is_last_item: bool,
) {
    let window = web_sys::window().expect("Window should exist.");
    let inner_width = window.inner_width().unwrap().as_f64().unwrap();
    let inner_height = window.inner_height().unwrap().as_f64().unwrap();

    let trigger_rect = trigger.get_bounding_client_rect();
    let content_rect = content.get_bounding_client_rect();
    let value_node_rect = value_node.get_bounding_client_rect();

    // Find the text span within the selected item for horizontal alignment
    let item_text_rect = selected_item
        .query_selector("span[id]")
        .ok()
        .flatten()
        .map(|el| el.get_bounding_client_rect())
        .unwrap_or_else(|| selected_item.get_bounding_client_rect());

    let wrapper_style = wrapper.style();

    // ── Horizontal positioning ──────────────────────────────
    if dir != Direction::Rtl {
        let item_text_offset = item_text_rect.left() - content_rect.left();
        let left = value_node_rect.left() - item_text_offset;
        let left_delta = trigger_rect.left() - left;
        let min_content_width = trigger_rect.width() + left_delta;
        let content_width = min_content_width.max(content_rect.width());
        let right_edge = inner_width - CONTENT_MARGIN;
        let clamped_left = left
            .max(CONTENT_MARGIN)
            .min((right_edge - content_width).max(CONTENT_MARGIN));

        let _ = wrapper_style.set_property("min-width", &format!("{}px", min_content_width));
        let _ = wrapper_style.set_property("left", &format!("{}px", clamped_left));
    } else {
        let item_text_offset = content_rect.right() - item_text_rect.right();
        let right = inner_width - value_node_rect.right() - item_text_offset;
        let right_delta = inner_width - trigger_rect.right() - right;
        let min_content_width = trigger_rect.width() + right_delta;
        let content_width = min_content_width.max(content_rect.width());
        let left_edge = inner_width - CONTENT_MARGIN;
        let clamped_right = right
            .max(CONTENT_MARGIN)
            .min((left_edge - content_width).max(CONTENT_MARGIN));

        let _ = wrapper_style.set_property("min-width", &format!("{}px", min_content_width));
        let _ = wrapper_style.set_property("right", &format!("{}px", clamped_right));
    }

    // ── Vertical positioning ────────────────────────────────
    let available_height = inner_height - CONTENT_MARGIN * 2.0;
    let items_height = viewport.scroll_height() as f64;

    let content_styles = window.get_computed_style(content).unwrap().unwrap();
    let parse_px = |prop: &str| -> f64 {
        content_styles
            .get_property_value(prop)
            .unwrap_or_default()
            .replace("px", "")
            .parse::<f64>()
            .unwrap_or(0.0)
    };
    let content_border_top = parse_px("border-top-width");
    let content_padding_top = parse_px("padding-top");
    let content_border_bottom = parse_px("border-bottom-width");
    let content_padding_bottom = parse_px("padding-bottom");

    let full_content_height = content_border_top
        + content_padding_top
        + items_height
        + content_padding_bottom
        + content_border_bottom;
    let min_content_height = (selected_item.offset_height() as f64 * 5.0).min(full_content_height);

    let viewport_styles = window.get_computed_style(viewport).unwrap().unwrap();
    let viewport_padding_top = viewport_styles
        .get_property_value("padding-top")
        .unwrap_or_default()
        .replace("px", "")
        .parse::<f64>()
        .unwrap_or(0.0);
    let viewport_padding_bottom = viewport_styles
        .get_property_value("padding-bottom")
        .unwrap_or_default()
        .replace("px", "")
        .parse::<f64>()
        .unwrap_or(0.0);

    let top_edge_to_trigger_middle =
        trigger_rect.top() + trigger_rect.height() / 2.0 - CONTENT_MARGIN;
    let trigger_middle_to_bottom_edge = available_height - top_edge_to_trigger_middle;

    let selected_item_half_height = selected_item.offset_height() as f64 / 2.0;
    let item_offset_middle = selected_item.offset_top() as f64 + selected_item_half_height;
    let content_top_to_item_middle = content_border_top + content_padding_top + item_offset_middle;
    let item_middle_to_content_bottom = full_content_height - content_top_to_item_middle;

    let will_align_without_top_overflow = content_top_to_item_middle <= top_edge_to_trigger_middle;

    if will_align_without_top_overflow {
        let _ = wrapper_style.set_property("bottom", "0px");
        let viewport_offset_bottom = content.client_height() as f64
            - viewport.offset_top() as f64
            - viewport.offset_height() as f64;
        let clamped_trigger_middle_to_bottom_edge = trigger_middle_to_bottom_edge.max(
            selected_item_half_height
                + if is_last_item {
                    viewport_padding_bottom
                } else {
                    0.0
                }
                + viewport_offset_bottom
                + content_border_bottom,
        );
        let height = content_top_to_item_middle + clamped_trigger_middle_to_bottom_edge;
        let _ = wrapper_style.set_property("height", &format!("{}px", height));
    } else {
        let _ = wrapper_style.set_property("top", "0px");
        let clamped_top_edge_to_trigger_middle = top_edge_to_trigger_middle.max(
            content_border_top
                + viewport.offset_top() as f64
                + if is_first_item {
                    viewport_padding_top
                } else {
                    0.0
                }
                + selected_item_half_height,
        );
        let height = clamped_top_edge_to_trigger_middle + item_middle_to_content_bottom;
        let _ = wrapper_style.set_property("height", &format!("{}px", height));
        viewport.set_scroll_top(
            (content_top_to_item_middle - top_edge_to_trigger_middle + viewport.offset_top() as f64)
                as i32,
        );
    }

    let _ = wrapper_style.set_property("margin", &format!("{}px 0", CONTENT_MARGIN));
    let _ = wrapper_style.set_property("min-height", &format!("{}px", min_content_height));
    let _ = wrapper_style.set_property("max-height", &format!("{}px", available_height));
}

/// Visually hidden styles for the bubble select element
const VISUALLY_HIDDEN_STYLES_STR: &str = "position: absolute; border: 0; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); white-space: nowrap; word-wrap: normal;";
