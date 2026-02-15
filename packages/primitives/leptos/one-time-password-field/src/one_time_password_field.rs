use std::marker::PhantomData;
use std::ops::Deref;

use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_collection::{
    CollectionItemSlot, CollectionItemValue, CollectionProvider, CollectionSlot, use_collection,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_primitive::{Primitive, VoidPrimitive, compose_callbacks};
use radix_leptos_roving_focus::{
    Orientation, RovingFocusGroup, RovingFocusGroupContext, RovingFocusGroupItem,
    RovingFocusGroupItemContext,
};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InputValidationType {
    Alpha,
    #[default]
    Numeric,
    Alphanumeric,
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum InputType {
    Password,
    #[default]
    Text,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Password => "password",
            InputType::Text => "text",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum AutoComplete {
    Off,
    #[default]
    OneTimeCode,
}

impl AutoComplete {
    fn as_str(&self) -> &'static str {
        match self {
            AutoComplete::Off => "off",
            AutoComplete::OneTimeCode => "one-time-code",
        }
    }
}

#[derive(Clone)]
struct ValidationInfo {
    regexp: &'static str,
    pattern: &'static str,
    input_mode: &'static str,
}

fn get_validation(validation_type: InputValidationType) -> Option<ValidationInfo> {
    match validation_type {
        InputValidationType::Numeric => Some(ValidationInfo {
            regexp: r"[^\d]",
            pattern: r"\d{1}",
            input_mode: "numeric",
        }),
        InputValidationType::Alpha => Some(ValidationInfo {
            regexp: r"[^a-zA-Z]",
            pattern: "[a-zA-Z]{1}",
            input_mode: "text",
        }),
        InputValidationType::Alphanumeric => Some(ValidationInfo {
            regexp: r"[^a-zA-Z0-9]",
            pattern: "[a-zA-Z0-9]{1}",
            input_mode: "text",
        }),
        InputValidationType::None => None,
    }
}

/* -------------------------------------------------------------------------------------------------
 * Actions
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
enum UpdateAction {
    SetChar {
        index: usize,
        char: String,
    },
    ClearChar {
        index: usize,
        reason: ClearCharReason,
    },
    Clear {
        reason: ClearReason,
    },
    Paste {
        value: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ClearCharReason {
    Backspace,
    Delete,
    Cut,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
enum ClearReason {
    Reset,
    Backspace,
    Delete,
}

#[derive(Clone, Debug)]
enum KeyboardActionDetails {
    Keydown {
        key: KeydownKey,
        meta_key: bool,
        ctrl_key: bool,
    },
    Cut,
    AutocompletePaste,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum KeydownKey {
    Backspace,
    Delete,
    Clear,
    Char,
}

/* -------------------------------------------------------------------------------------------------
 * Collection item data
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
struct ItemData;

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

/* -------------------------------------------------------------------------------------------------
 * Collection helpers
 * -----------------------------------------------------------------------------------------------*/

fn collection_at(
    items: &[CollectionItemValue<ItemData>],
    index: isize,
) -> Option<&CollectionItemValue<ItemData>> {
    if index < 0 {
        let actual = items.len() as isize + index;
        if actual >= 0 {
            items.get(actual as usize)
        } else {
            None
        }
    } else {
        items.get(index as usize)
    }
}

fn collection_index_of(
    items: &[CollectionItemValue<ItemData>],
    element: &web_sys::HtmlElement,
) -> Option<usize> {
    items.iter().position(|item| {
        item.r#ref
            .get()
            .is_some_and(|el| *el.deref().unchecked_ref::<web_sys::HtmlElement>() == *element)
    })
}

fn collection_from<'a>(
    items: &'a [CollectionItemValue<ItemData>],
    element: &web_sys::HtmlElement,
    direction: isize,
) -> Option<&'a CollectionItemValue<ItemData>> {
    let index = collection_index_of(items, element)?;
    let new_index = index as isize + direction;
    if new_index >= 0 && (new_index as usize) < items.len() {
        Some(&items[new_index as usize])
    } else {
        None
    }
}

fn collection_element(item: &CollectionItemValue<ItemData>) -> Option<web_sys::HtmlInputElement> {
    item.r#ref.get().map(|el| {
        el.deref()
            .unchecked_ref::<web_sys::HtmlInputElement>()
            .clone()
    })
}

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct OneTimePasswordFieldContextValue {
    value: Signal<Vec<String>>,
    dispatch: StoredValue<SendWrapper<Box<dyn Fn(UpdateAction)>>>,
    attempt_submit: StoredValue<SendWrapper<Box<dyn Fn()>>>,
    auto_complete: Signal<AutoComplete>,
    auto_focus: Signal<bool>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    form: Signal<Option<String>>,
    name: Signal<Option<String>>,
    placeholder: Signal<Option<String>>,
    r#type: Signal<InputType>,
    orientation: Signal<Orientation>,
    validation_type: Signal<InputValidationType>,
    user_action: RwSignal<Option<KeyboardActionDetails>>,
    sanitize_value: StoredValue<SendWrapper<Box<dyn Fn(String) -> Vec<String>>>>,
    hidden_input_ref: AnyNodeRef,
}

/* -------------------------------------------------------------------------------------------------
 * OneTimePasswordField
 * -----------------------------------------------------------------------------------------------*/

/// Internal context to pass callback props from outer to inner component.
/// This avoids Leptos prop-forwarding issues with `Option<Callback<...>>`.
#[derive(Clone, Copy)]
struct OneTimePasswordFieldCallbacks {
    on_value_change: StoredValue<Option<Callback<String>>>,
    on_auto_submit: StoredValue<Option<Callback<String>>>,
    sanitize_value: StoredValue<Option<Callback<String, String>>>,
    on_paste: StoredValue<Option<Callback<ev::ClipboardEvent>>>,
}

#[component]
pub fn OneTimePasswordField(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] auto_submit: MaybeProp<bool>,
    #[prop(into, optional)] on_auto_submit: Option<Callback<String>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] auto_focus: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<InputType>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] validation_type: MaybeProp<InputValidationType>,
    #[prop(into, optional)] sanitize_value: Option<Callback<String, String>>,
    #[prop(into, optional)] on_paste: Option<Callback<ev::ClipboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let callbacks = OneTimePasswordFieldCallbacks {
        on_value_change: StoredValue::new(on_value_change),
        on_auto_submit: StoredValue::new(on_auto_submit),
        sanitize_value: StoredValue::new(sanitize_value),
        on_paste: StoredValue::new(on_paste),
    };
    provide_context(callbacks);

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
            <OneTimePasswordFieldImpl
                value=value
                default_value=default_value
                auto_submit=auto_submit
                disabled=disabled
                read_only=read_only
                auto_complete=auto_complete
                auto_focus=auto_focus
                form=form
                name=name
                placeholder=placeholder
                r#type=r#type
                orientation=orientation
                dir=dir
                validation_type=validation_type
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children())}
            </OneTimePasswordFieldImpl>
        </CollectionProvider>
    }
}

#[component]
fn OneTimePasswordFieldImpl(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] auto_submit: MaybeProp<bool>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] read_only: MaybeProp<bool>,
    #[prop(into, optional)] auto_complete: MaybeProp<AutoComplete>,
    #[prop(into, optional)] auto_focus: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] placeholder: MaybeProp<String>,
    #[prop(into, optional)] r#type: MaybeProp<InputType>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] validation_type: MaybeProp<InputValidationType>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let callbacks = expect_context::<OneTimePasswordFieldCallbacks>();
    let on_value_change = callbacks.on_value_change.get_value();
    let on_auto_submit = callbacks.on_auto_submit.get_value();
    let sanitize_value = callbacks.sanitize_value.get_value();
    let on_paste = callbacks.on_paste.get_value();
    let children = StoredValue::new(children);

    let direction = use_direction(dir);
    let disabled_sig = Signal::derive(move || disabled.get().unwrap_or(false));
    let read_only_sig = Signal::derive(move || read_only.get().unwrap_or(false));
    let auto_complete_sig = Signal::derive(move || auto_complete.get().unwrap_or_default());
    let auto_focus_sig = Signal::derive(move || auto_focus.get().unwrap_or(false));
    let form_sig = Signal::derive(move || form.get());
    let name_sig = Signal::derive(move || name.get());
    let placeholder_sig = Signal::derive(move || placeholder.get());
    let type_sig = Signal::derive(move || r#type.get().unwrap_or_default());
    let orientation_sig =
        Signal::derive(move || orientation.get().unwrap_or(Orientation::Horizontal));
    let validation_type_sig = Signal::derive(move || validation_type.get().unwrap_or_default());
    let auto_submit_sig = Signal::derive(move || auto_submit.get().unwrap_or(false));

    let sanitize_value_fn: StoredValue<SendWrapper<Box<dyn Fn(String) -> Vec<String>>>> =
        StoredValue::new(SendWrapper::new(Box::new(move |input: String| {
            let input = remove_whitespace(&input);
            let validation = get_validation(validation_type_sig.get_untracked());
            let cleaned = if let Some(v) = validation {
                let re = js_sys::RegExp::new(v.regexp, "g");
                js_sys::JsString::from(input.as_str())
                    .replace_by_pattern(&re, "")
                    .as_string()
                    .unwrap_or_default()
            } else if let Some(sanitize) = sanitize_value {
                sanitize.run(input)
            } else {
                input
            };
            cleaned.chars().map(|c| c.to_string()).collect()
        })));

    let controlled_value: MaybeProp<Vec<String>> =
        MaybeProp::derive(move || value.get().map(|v| sanitize_value_fn.with_value(|f| f(v))));

    let default_value_vec: MaybeProp<Vec<String>> = MaybeProp::derive(move || {
        default_value
            .get()
            .map(|v| sanitize_value_fn.with_value(|f| f(v)))
    });

    let on_vec_change: Option<Callback<Option<Vec<String>>>> = on_value_change.map(|cb| {
        Callback::new(move |v: Option<Vec<String>>| {
            cb.run(v.map(|v| v.join("")).unwrap_or_default());
        })
    });

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: controlled_value,
        default_prop: default_value_vec,
        on_change: on_vec_change,
    });

    let value_vec = Signal::derive(move || value_signal.get().unwrap_or_default());

    let get_items: StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>>> =
        StoredValue::new(use_collection::<ItemData>());

    let hidden_input_ref = AnyNodeRef::new();
    let root_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, root_ref]);

    let user_action: RwSignal<Option<KeyboardActionDetails>> = RwSignal::new(None);

    let locate_form = move || -> Option<web_sys::HtmlFormElement> {
        if let Some(form_id) = form_sig.get() {
            let doc = document();
            let el = doc.get_element_by_id(&form_id)?;
            el.dyn_into::<web_sys::HtmlFormElement>().ok()
        } else if let Some(node) = hidden_input_ref.get() {
            let input: &web_sys::HtmlInputElement = node.deref().unchecked_ref();
            input.form()
        } else {
            let items = get_items.with_value(|f| f());
            let first = items.first()?;
            let el = collection_element(first)?;
            el.form()
        }
    };

    let attempt_submit: StoredValue<SendWrapper<Box<dyn Fn()>>> =
        StoredValue::new(SendWrapper::new(Box::new(move || {
            if let Some(form) = locate_form() {
                let _ = form.request_submit();
            }
        })));

    // dispatch function handles all value update actions
    let dispatch: StoredValue<SendWrapper<Box<dyn Fn(UpdateAction)>>> =
        StoredValue::new(SendWrapper::new(Box::new(move |action: UpdateAction| {
            let items = get_items.with_value(|f| f());
            let current_value = value_vec.get_untracked();

            match action {
                UpdateAction::SetChar { index, char } => {
                    let current_target =
                        collection_at(&items, index as isize).and_then(collection_element);

                    if current_value.get(index).is_some_and(|v| *v == char) {
                        // Same value — just move to next
                        if let Some(ct) = &current_target {
                            let next = collection_from(&items, ct, 1).and_then(collection_element);
                            focus_input(next.as_ref());
                        }
                        return;
                    }

                    if char.is_empty() {
                        return;
                    }

                    // Validate
                    let validation = get_validation(validation_type_sig.get_untracked());
                    if let Some(v) = validation {
                        let re = js_sys::RegExp::new(v.regexp, "g");
                        let clean = js_sys::JsString::from(char.as_str())
                            .replace_by_pattern(&re, "")
                            .as_string()
                            .unwrap_or_default();
                        if clean != char {
                            return;
                        }
                    }

                    let size = items.len();

                    if current_value.len() >= size {
                        // Replace current value; move to next input
                        let mut new_value = current_value.clone();
                        if index < new_value.len() {
                            new_value[index] = char;
                        }
                        set_value.run(Some(new_value));
                        if let Some(ct) = &current_target {
                            let next = collection_from(&items, ct, 1).and_then(collection_element);
                            focus_input(next.as_ref());
                        }
                        return;
                    }

                    let mut new_value = current_value.clone();
                    // Ensure the vec is large enough
                    while new_value.len() <= index {
                        new_value.push(String::new());
                    }
                    new_value[index] = char;

                    let last_element = collection_at(&items, -1).and_then(collection_element);

                    set_value.run(Some(new_value));

                    if current_target.as_ref() != last_element.as_ref() {
                        if let Some(ct) = &current_target {
                            let next = collection_from(&items, ct, 1).and_then(collection_element);
                            focus_input(next.as_ref());
                        }
                    } else if let Some(ct) = &current_target {
                        ct.select();
                    }
                }

                UpdateAction::ClearChar { index, reason } => {
                    if current_value.get(index).is_none()
                        || current_value.get(index).is_some_and(|v| v.is_empty())
                    {
                        return;
                    }

                    let new_value: Vec<String> = current_value
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != index)
                        .map(|(_, v)| v.clone())
                        .collect();

                    let current_target =
                        collection_at(&items, index as isize).and_then(collection_element);
                    let previous = current_target.as_ref().and_then(|ct| {
                        collection_from(&items, ct, -1).and_then(collection_element)
                    });

                    set_value.run(Some(new_value));

                    match reason {
                        ClearCharReason::Backspace => {
                            focus_input(previous.as_ref());
                        }
                        ClearCharReason::Delete | ClearCharReason::Cut => {
                            focus_input(current_target.as_ref());
                        }
                    }
                }

                UpdateAction::Clear { reason } => {
                    if current_value.is_empty() {
                        return;
                    }

                    match reason {
                        ClearReason::Backspace | ClearReason::Delete => {
                            set_value.run(Some(vec![]));
                            let first = collection_at(&items, 0).and_then(collection_element);
                            focus_input(first.as_ref());
                        }
                        ClearReason::Reset => {
                            set_value.run(Some(vec![]));
                        }
                    }
                }

                UpdateAction::Paste {
                    value: pasted_value,
                } => {
                    let new_value = sanitize_value_fn.with_value(|f| f(pasted_value));
                    if new_value.is_empty() {
                        return;
                    }

                    let focus_index = new_value.len() as isize - 1;
                    set_value.run(Some(new_value));
                    let target = collection_at(&items, focus_index).and_then(collection_element);
                    focus_input(target.as_ref());
                }
            }
        })));

    // Re-validate when the validation type changes
    let prev_validation_type: RwSignal<Option<InputValidationType>> = RwSignal::new(None);
    Effect::new(move |_| {
        let current_type = validation_type_sig.get();
        let prev = prev_validation_type.get_untracked();
        if prev.is_some() && prev != Some(current_type) && get_validation(current_type).is_some() {
            let current = value_vec.get_untracked().join("");
            let new_value = sanitize_value_fn.with_value(|f| f(current));
            set_value.run(Some(new_value));
        }
        prev_validation_type.set(Some(current_type));
    });

    // Form reset listener
    Effect::new(move |_| {
        if let Some(form) = locate_form() {
            let dispatch = dispatch;
            let closure = SendWrapper::new(
                web_sys::wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
                    dispatch.with_value(|d| {
                        d(UpdateAction::Clear {
                            reason: ClearReason::Reset,
                        })
                    });
                }),
            );
            let form = SendWrapper::new(form);
            form.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                .expect("Reset event listener should be added.");

            Owner::on_cleanup(move || {
                form.remove_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                    .expect("Reset event listener should be removed.");
            });
        }
    });

    // Auto-submit effect
    let prev_joined: RwSignal<String> = RwSignal::new(String::new());
    Effect::new(move |_| {
        let current = value_vec.get();
        let joined = current.join("");
        let prev = prev_joined.get_untracked();
        prev_joined.set(joined.clone());
        if prev == joined {
            return;
        }

        let items = get_items.with_value(|f| f());
        let size = items.len();

        if auto_submit_sig.get() && current.iter().all(|c| !c.is_empty()) && current.len() == size {
            if let Some(on_auto_submit) = on_auto_submit {
                on_auto_submit.run(joined);
            }
            attempt_submit.with_value(|f| f());
        }
    });

    let context_value = OneTimePasswordFieldContextValue {
        value: value_vec,
        dispatch,
        attempt_submit,
        auto_complete: auto_complete_sig,
        auto_focus: auto_focus_sig,
        disabled: disabled_sig,
        read_only: read_only_sig,
        form: form_sig,
        name: name_sig,
        placeholder: placeholder_sig,
        r#type: type_sig,
        orientation: orientation_sig,
        validation_type: validation_type_sig,
        user_action,
        sanitize_value: sanitize_value_fn,
        hidden_input_ref,
    };

    view! {
        <Provider value=context_value>
            <CollectionSlot item_data_type=ITEM_DATA_PHANTOM>
                <RovingFocusGroup
                    as_child=true
                    orientation=orientation_sig
                    dir=direction
                >
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_refs
                        attr:role="group"
                        on:paste=compose_callbacks(on_paste, Some(Callback::new(move |event: ev::ClipboardEvent| {
                            event.prevent_default();
                            let clipboard_event: &web_sys::ClipboardEvent = event.unchecked_ref();
                            if let Some(data) = clipboard_event.clipboard_data()
                                && let Ok(pasted) = data.get_data("Text") {
                                    dispatch.with_value(|d| d(UpdateAction::Paste { value: pasted }));
                                }
                        })), None)
                    >
                        {children.with_value(|children| children())}
                    </Primitive>
                </RovingFocusGroup>
            </CollectionSlot>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * OneTimePasswordFieldInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn OneTimePasswordFieldInput(
    #[prop(into, optional)] on_invalid_change: Option<Callback<String>>,
    #[prop(into, optional)] index: MaybeProp<usize>,
    #[prop(into, optional)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(into, optional)] on_cut: Option<Callback<ev::ClipboardEvent>>,
    #[prop(into, optional)] on_input: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_change: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let _children = children;

    let context = expect_context::<OneTimePasswordFieldContextValue>();
    let get_items: StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>>> =
        StoredValue::new(use_collection::<ItemData>());

    let input_ref = AnyNodeRef::new();
    let (element, set_element) = signal::<Option<SendWrapper<web_sys::HtmlInputElement>>>(None);
    let composed_input_ref = use_composed_refs(vec![node_ref, input_ref]);

    // Track element from ref
    Effect::new(move |_| {
        if let Some(node) = input_ref.get() {
            let el: web_sys::HtmlInputElement = node.deref().clone().unchecked_into();
            set_element.set(Some(SendWrapper::new(el)));
        }
    });

    let resolved_index = Signal::derive(move || {
        if let Some(idx) = index.get() {
            idx
        } else if let Some(el) = element.get() {
            let items = get_items.with_value(|f| f());
            let html_el: &web_sys::HtmlElement = el.unchecked_ref();
            collection_index_of(&items, html_el).unwrap_or(0)
        } else {
            0
        }
    });

    let char_value = Signal::derive(move || {
        let idx = resolved_index.get();
        context.value.get().get(idx).cloned().unwrap_or_default()
    });

    let total_value = Signal::derive(move || context.value.get().join("").trim().to_string());

    let last_selectable_index = Signal::derive(move || {
        let items = get_items.with_value(|f| f());
        let size = items.len();
        let len = total_value.get().len();
        len.min(size.saturating_sub(1))
    });

    let is_focusable = Signal::derive(move || resolved_index.get() <= last_selectable_index.get());

    let validation = Signal::derive(move || get_validation(context.validation_type.get()));

    let placeholder_value = Signal::derive(move || {
        let idx = resolved_index.get();
        if let Some(ph) = context.placeholder.get() {
            if context.value.get().is_empty() {
                ph.chars().nth(idx).map(|c| c.to_string())
            } else {
                None
            }
        } else {
            None
        }
    });

    let collection_size = Signal::derive(move || get_items.with_value(|f| f()).len());

    let aria_label = Signal::derive(move || {
        format!(
            "Character {} of {}",
            resolved_index.get() + 1,
            collection_size.get()
        )
    });

    let item_data = Signal::derive(move || ItemData);

    // Read roving focus contexts to determine supports_auto_complete
    let has_tab_stop = Signal::derive(move || {
        use_context::<RovingFocusGroupContext>()
            .map(|ctx| ctx.has_tab_stop.get())
            .unwrap_or(false)
    });
    let is_current_tab_stop_ctx = Signal::derive(move || {
        use_context::<RovingFocusGroupItemContext>()
            .map(|ctx| ctx.is_current_tab_stop.get())
            .unwrap_or(false)
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=item_data>
            <RovingFocusGroupItem
                as_child=true
                focusable=Signal::derive(move || !context.disabled.get() && is_focusable.get())
                active=Signal::derive(move || resolved_index.get() == last_selectable_index.get())
            >
                <OneTimePasswordFieldInputInner
                    context=context
                    get_items=get_items
                    composed_input_ref=composed_input_ref
                    resolved_index=resolved_index
                    char_value=char_value
                    last_selectable_index=last_selectable_index
                    validation=validation
                    placeholder_value=placeholder_value
                    collection_size=collection_size
                    aria_label=aria_label
                    has_tab_stop=has_tab_stop
                    is_current_tab_stop_ctx=is_current_tab_stop_ctx
                    on_invalid_change=on_invalid_change
                    on_focus=on_focus
                    on_cut=on_cut
                    on_input=on_input
                    on_change=on_change
                    on_key_down=on_key_down
                    on_pointer_down=on_pointer_down
                    as_child=as_child
                />
            </RovingFocusGroupItem>
        </CollectionItemSlot>
    }
}

/// Inner component that reads `RovingFocusGroupItemContext` after
/// `RovingFocusGroupItem` provides it.
#[component]
fn OneTimePasswordFieldInputInner(
    context: OneTimePasswordFieldContextValue,
    get_items: StoredValue<SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>>>,
    composed_input_ref: AnyNodeRef,
    resolved_index: Signal<usize>,
    char_value: Signal<String>,
    last_selectable_index: Signal<usize>,
    validation: Signal<Option<ValidationInfo>>,
    placeholder_value: Signal<Option<String>>,
    collection_size: Signal<usize>,
    aria_label: Signal<String>,
    has_tab_stop: Signal<bool>,
    is_current_tab_stop_ctx: Signal<bool>,
    on_invalid_change: Option<Callback<String>>,
    on_focus: Option<Callback<ev::FocusEvent>>,
    on_cut: Option<Callback<ev::ClipboardEvent>>,
    on_input: Option<Callback<ev::Event>>,
    on_change: Option<Callback<ev::Event>>,
    on_key_down: Option<Callback<ev::KeyboardEvent>>,
    on_pointer_down: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
) -> impl IntoView {
    let supports_auto_complete = Signal::derive(move || {
        if has_tab_stop.get() {
            is_current_tab_stop_ctx.get()
        } else {
            resolved_index.get() == 0
        }
    });

    let auto_complete_attr = Signal::derive(move || {
        if supports_auto_complete.get() {
            context.auto_complete.get().as_str().to_string()
        } else {
            "off".to_string()
        }
    });

    let max_length_attr = Signal::derive(move || {
        if supports_auto_complete.get() {
            collection_size.get()
        } else {
            1
        }
    });

    let pw_manager_ignore = Signal::derive(move || {
        if supports_auto_complete.get() {
            None
        } else {
            Some("true".to_string())
        }
    });

    let input_mode_attr =
        Signal::derive(move || validation.get().map(|v| v.input_mode.to_string()));

    let pattern_attr = Signal::derive(move || validation.get().map(|v| v.pattern.to_string()));

    let dispatch = context.dispatch;
    let user_action = context.user_action;

    view! {
        <VoidPrimitive
            element=html::input
            as_child=as_child
            node_ref=composed_input_ref
            attr:r#type=move || context.r#type.get().as_str()
            attr:disabled=move || context.disabled.get().then_some("")
            attr:aria-label=move || aria_label.get()
            attr:autocomplete=move || auto_complete_attr.get()
            attr:data-1p-ignore=move || pw_manager_ignore.get()
            attr:data-lpignore=move || pw_manager_ignore.get()
            attr:data-protonpass-ignore=move || pw_manager_ignore.get()
            attr:data-bwignore=move || pw_manager_ignore.get()
            attr:inputmode=move || input_mode_attr.get()
            attr:maxlength=move || max_length_attr.get().to_string()
            attr:pattern=move || pattern_attr.get()
            attr:readonly=move || context.read_only.get().then_some("")
            attr:value=move || char_value.get()
            attr:placeholder=move || placeholder_value.get()
            attr:data-radix-otp-input=""
            attr:data-radix-index=move || resolved_index.get().to_string()
            on:focus=compose_callbacks(on_focus, Some(Callback::new(move |event: ev::FocusEvent| {
                let target: web_sys::HtmlInputElement = event.current_target()
                    .expect("Event should have current target")
                    .unchecked_into();
                target.select();
            })), None)
            on:cut=compose_callbacks(on_cut, Some(Callback::new(move |event: ev::ClipboardEvent| {
                let target: web_sys::HtmlInputElement = event.current_target()
                    .expect("Event should have current target")
                    .unchecked_into();
                if !target.value().is_empty() {
                    user_action.set(Some(KeyboardActionDetails::Cut));
                    // Short timeout to clear after change handler completes
                    let user_action = user_action;
                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                        user_action.set(None);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist")
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            10,
                        );
                }
            })), None)
            on:input=compose_callbacks(on_input, Some(Callback::new(move |event: ev::Event| {
                let target: web_sys::HtmlInputElement = event.current_target()
                    .expect("Event should have current target")
                    .unchecked_into();
                let value = target.value();
                if value.len() > 1 {
                    // Password manager auto-fill
                    event.prevent_default();
                    user_action.set(Some(KeyboardActionDetails::AutocompletePaste));
                    dispatch.with_value(|d| d(UpdateAction::Paste { value }));
                    let user_action = user_action;
                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                        user_action.set(None);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist")
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            cb.unchecked_ref(),
                            10,
                        );
                }
            })), None)
            on:change=compose_callbacks(on_change, Some(Callback::new(move |event: ev::Event| {
                let target: web_sys::HtmlInputElement = event.target()
                    .expect("Event should have target")
                    .unchecked_into();
                let value = target.value();
                event.prevent_default();
                let action = user_action.get_untracked();
                user_action.set(None);

                let index = resolved_index.get_untracked();

                if let Some(action) = action {
                    match action {
                        KeyboardActionDetails::Cut => {
                            dispatch.with_value(|d| d(UpdateAction::ClearChar {
                                index,
                                reason: ClearCharReason::Cut,
                            }));
                            return;
                        }
                        KeyboardActionDetails::AutocompletePaste => {
                            // Already handled in input handler
                            return;
                        }
                        KeyboardActionDetails::Keydown { key, meta_key, ctrl_key } => {
                            if key == KeydownKey::Char {
                                return;
                            }
                            let is_clearing = key == KeydownKey::Backspace && (meta_key || ctrl_key);
                            if key == KeydownKey::Clear || is_clearing {
                                dispatch.with_value(|d| d(UpdateAction::Clear {
                                    reason: ClearReason::Backspace,
                                }));
                            } else {
                                let reason = match key {
                                    KeydownKey::Delete => ClearCharReason::Delete,
                                    _ => ClearCharReason::Backspace,
                                };
                                dispatch.with_value(|d| d(UpdateAction::ClearChar {
                                    index,
                                    reason,
                                }));
                            }
                            return;
                        }
                    }
                }

                // Only update if valid
                if target.validity().valid() {
                    if value.is_empty() {
                        let mut reason = ClearCharReason::Backspace;
                        // Check native InputEvent inputType
                        if let Ok(input_event) = event.clone().dyn_into::<web_sys::InputEvent>()
                            && let Some(input_type) = input_event.input_type().into() {
                                match input_type.as_str() {
                                    "deleteContentBackward" => reason = ClearCharReason::Backspace,
                                    "deleteByCut" => reason = ClearCharReason::Cut,
                                    _ => {}
                                }
                            }
                        dispatch.with_value(|d| d(UpdateAction::ClearChar {
                            index,
                            reason,
                        }));
                    } else {
                        dispatch.with_value(|d| d(UpdateAction::SetChar {
                            char: value,
                            index,
                        }));
                    }
                } else {
                    if let Some(on_invalid_change) = on_invalid_change {
                        on_invalid_change.run(target.value());
                    }
                    let target_clone = target.clone();
                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                        if let Some(active) = target_clone.owner_document()
                            .and_then(|doc| doc.active_element())
                            && active == *target_clone.unchecked_ref::<web_sys::Element>() {
                                target_clone.select();
                            }
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist")
                        .request_animation_frame(cb.unchecked_ref());
                }
            })), None)
            on:keydown=compose_callbacks(on_key_down, Some(Callback::new(move |event: ev::KeyboardEvent| {
                let index = resolved_index.get_untracked();
                let items = get_items.with_value(|f| f());
                let key = event.key();

                match key.as_str() {
                    "Clear" | "Delete" | "Backspace" => {
                        let current_value = event.current_target()
                            .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>().value())
                            .unwrap_or_default();

                        if current_value.is_empty() {
                            if key == "Delete" {
                                return;
                            }
                            let is_clearing = key == "Clear" || event.meta_key() || event.ctrl_key();
                            if is_clearing {
                                dispatch.with_value(|d| d(UpdateAction::Clear {
                                    reason: ClearReason::Backspace,
                                }));
                            } else {
                                let element = event.current_target()
                                    .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                                if let Some(el) = element {
                                    let prev = collection_from(&items, &el, -1)
                                        .and_then(collection_element);
                                    let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                        focus_input(prev.as_ref());
                                    });
                                    let _ = web_sys::window()
                                        .expect("Window should exist")
                                        .request_animation_frame(cb.unchecked_ref());
                                }
                            }
                        } else {
                            let keydown_key = match key.as_str() {
                                "Clear" => KeydownKey::Clear,
                                "Delete" => KeydownKey::Delete,
                                _ => KeydownKey::Backspace,
                            };
                            user_action.set(Some(KeyboardActionDetails::Keydown {
                                key: keydown_key,
                                meta_key: event.meta_key(),
                                ctrl_key: event.ctrl_key(),
                            }));
                            let user_action = user_action;
                            let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                user_action.set(None);
                            });
                            let _ = web_sys::window()
                                .expect("Window should exist")
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    cb.unchecked_ref(),
                                    10,
                                );
                        }
                    }
                    "Enter" => {
                        event.prevent_default();
                        context.attempt_submit.with_value(|f| f());
                    }
                    "ArrowDown" | "ArrowUp" => {
                        if context.orientation.get() == Orientation::Horizontal {
                            event.prevent_default();
                        }
                    }
                    _ => {
                        let current_value = event.current_target()
                            .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>().value())
                            .unwrap_or_default();

                        if current_value == key {
                            // Same value as key press — no change event; focus next
                            event.prevent_default();
                            let element = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlElement>());
                            if let Some(el) = element {
                                let next = collection_from(&items, &el, 1)
                                    .and_then(collection_element);
                                focus_input(next.as_ref().map(|e| {
                                    e.unchecked_ref::<web_sys::HtmlInputElement>()
                                }));
                            }
                        } else if !current_value.is_empty() {
                            // Check if selection is present
                            let ct = event.current_target()
                                .map(|ct| ct.unchecked_into::<web_sys::HtmlInputElement>());
                            if let Some(input) = ct {
                                let sel_start = input.selection_start().ok().flatten().unwrap_or(0);
                                let sel_end = input.selection_end().ok().flatten().unwrap_or(0);
                                let is_selected = sel_start == 0 && sel_end > 0;

                                if !is_selected {
                                    let attempted_value = key.clone();
                                    if key.len() > 1 || key == " " {
                                        return;
                                    }

                                    let html_el: web_sys::HtmlElement = input.clone().unchecked_into();
                                    let next_input = collection_from(&items, &html_el, 1)
                                        .and_then(collection_element);
                                    let last_input = collection_at(&items, -1)
                                        .and_then(collection_element);

                                    if next_input.as_ref() != last_input.as_ref()
                                        && last_input.as_ref().map(|e| e.unchecked_ref::<web_sys::HtmlElement>() != &html_el).unwrap_or(true)
                                    {
                                        if sel_start == 0 {
                                            dispatch.with_value(|d| d(UpdateAction::SetChar {
                                                char: attempted_value,
                                                index,
                                            }));
                                        } else {
                                            dispatch.with_value(|d| d(UpdateAction::SetChar {
                                                char: attempted_value,
                                                index: index + 1,
                                            }));
                                        }

                                        user_action.set(Some(KeyboardActionDetails::Keydown {
                                            key: KeydownKey::Char,
                                            meta_key: event.meta_key(),
                                            ctrl_key: event.ctrl_key(),
                                        }));
                                        let user_action = user_action;
                                        let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
                                            user_action.set(None);
                                        });
                                        let _ = web_sys::window()
                                            .expect("Window should exist")
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                cb.unchecked_ref(),
                                                10,
                                            );
                                    }
                                }
                            }
                        }
                    }
                }
            })), None)
            on:pointerdown=compose_callbacks(on_pointer_down, Some(Callback::new(move |event: ev::PointerEvent| {
                event.prevent_default();
                let index = resolved_index.get_untracked();
                let last = last_selectable_index.get_untracked();
                let index_to_focus = index.min(last);
                let items = get_items.with_value(|f| f());
                let element = collection_at(&items, index_to_focus as isize)
                    .and_then(collection_element);
                focus_input(element.as_ref());
            })), None)
        >
            {()}
        </VoidPrimitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * OneTimePasswordFieldHiddenInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn OneTimePasswordFieldHiddenInput(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<OneTimePasswordFieldContextValue>();
    let composed_ref = use_composed_refs(vec![context.hidden_input_ref, node_ref]);

    let value = Signal::derive(move || context.value.get().join("").trim().to_string());
    let name_sig = Signal::derive(move || name.get().or_else(|| context.name.get()));

    view! {
        <input
            node_ref=composed_ref
            name=move || name_sig.get().unwrap_or_default()
            value=move || value.get()
            autocomplete="off"
            autocapitalize="off"
            spellcheck="false"
            type="hidden"
            readonly
        />
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utility functions
 * -----------------------------------------------------------------------------------------------*/

fn remove_whitespace(value: &str) -> String {
    value.chars().filter(|c| !c.is_whitespace()).collect()
}

fn focus_input(element: Option<&web_sys::HtmlInputElement>) {
    let Some(element) = element else {
        return;
    };

    if let Some(active) = element
        .owner_document()
        .and_then(|doc| doc.active_element())
        && active == *element.unchecked_ref::<web_sys::Element>()
    {
        // Already focused — select in next frame
        let el = element.clone();
        let cb = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
            el.select();
        });
        let _ = web_sys::window()
            .expect("Window should exist")
            .request_animation_frame(cb.unchecked_ref());
        return;
    }

    let _ = element.focus();
}
