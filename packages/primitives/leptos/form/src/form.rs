use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_id::use_id;
use radix_leptos_primitive::{Primitive, VoidPrimitive};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/* -------------------------------------------------------------------------------------------------
 * Validity types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Validity {
    pub bad_input: bool,
    pub custom_error: bool,
    pub pattern_mismatch: bool,
    pub range_overflow: bool,
    pub range_underflow: bool,
    pub step_mismatch: bool,
    pub too_long: bool,
    pub too_short: bool,
    pub type_mismatch: bool,
    pub valid: bool,
    pub value_missing: bool,
}

impl From<web_sys::ValidityState> for Validity {
    fn from(v: web_sys::ValidityState) -> Self {
        Self {
            bad_input: v.bad_input(),
            custom_error: v.custom_error(),
            pattern_mismatch: v.pattern_mismatch(),
            range_overflow: v.range_overflow(),
            range_underflow: v.range_underflow(),
            step_mismatch: v.step_mismatch(),
            too_long: v.too_long(),
            too_short: v.too_short(),
            type_mismatch: v.type_mismatch(),
            valid: v.valid(),
            value_missing: v.value_missing(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidityMatcher {
    BadInput,
    PatternMismatch,
    RangeOverflow,
    RangeUnderflow,
    StepMismatch,
    TooLong,
    TooShort,
    TypeMismatch,
    Valid,
    ValueMissing,
}

impl ValidityMatcher {
    fn matches(&self, validity: &Validity) -> bool {
        match self {
            Self::BadInput => validity.bad_input,
            Self::PatternMismatch => validity.pattern_mismatch,
            Self::RangeOverflow => validity.range_overflow,
            Self::RangeUnderflow => validity.range_underflow,
            Self::StepMismatch => validity.step_mismatch,
            Self::TooLong => validity.too_long,
            Self::TooShort => validity.too_short,
            Self::TypeMismatch => validity.type_mismatch,
            Self::Valid => validity.valid,
            Self::ValueMissing => validity.value_missing,
        }
    }

    fn default_message(&self) -> &'static str {
        match self {
            Self::BadInput => DEFAULT_INVALID_MESSAGE,
            Self::PatternMismatch => "This value does not match the required pattern",
            Self::RangeOverflow => "This value is too large",
            Self::RangeUnderflow => "This value is too small",
            Self::StepMismatch => "This value does not match the required step",
            Self::TooLong => "This value is too long",
            Self::TooShort => "This value is too short",
            Self::TypeMismatch => "This value does not match the required type",
            Self::Valid => DEFAULT_INVALID_MESSAGE,
            Self::ValueMissing => "This value is missing",
        }
    }
}

pub type SyncCustomMatcherFn = Rc<dyn Fn(String, web_sys::FormData) -> bool>;
pub type AsyncCustomMatcherFn =
    Rc<dyn Fn(String, web_sys::FormData) -> Pin<Box<dyn Future<Output = bool>>>>;

#[derive(Clone)]
pub enum CustomMatcher {
    Sync(SyncCustomMatcherFn),
    Async(AsyncCustomMatcherFn),
}

pub enum Match {
    BuiltIn(ValidityMatcher),
    Custom(SyncCustomMatcherFn),
    CustomAsync(AsyncCustomMatcherFn),
}

impl From<ValidityMatcher> for Match {
    fn from(matcher: ValidityMatcher) -> Self {
        Self::BuiltIn(matcher)
    }
}

#[derive(Clone)]
struct CustomMatcherEntry {
    id: String,
    matcher: CustomMatcher,
}

const DEFAULT_INVALID_MESSAGE: &str = "This value is not valid";

/* -------------------------------------------------------------------------------------------------
 * Context types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct ValidationContextValue {
    validity_map: RwSignal<HashMap<String, Validity>>,
    custom_matcher_entries_map: StoredValue<HashMap<String, Vec<SendWrapper<CustomMatcherEntry>>>>,
    custom_errors_map: RwSignal<HashMap<String, HashMap<String, bool>>>,
}

impl ValidationContextValue {
    fn get_field_validity(&self, field_name: &str) -> Option<Validity> {
        self.validity_map.get().get(field_name).cloned()
    }

    fn set_field_validity(&self, field_name: &str, validity: Validity) {
        self.validity_map.update(|map| {
            map.insert(field_name.to_string(), validity);
        });
    }

    fn get_field_custom_matcher_entries(
        &self,
        field_name: &str,
    ) -> Vec<SendWrapper<CustomMatcherEntry>> {
        self.custom_matcher_entries_map
            .with_value(|map| map.get(field_name).cloned().unwrap_or_default())
    }

    fn add_field_custom_matcher_entry(
        &self,
        field_name: &str,
        entry: SendWrapper<CustomMatcherEntry>,
    ) {
        self.custom_matcher_entries_map.update_value(|map| {
            map.entry(field_name.to_string()).or_default().push(entry);
        });
    }

    fn remove_field_custom_matcher_entry(&self, field_name: &str, entry_id: &str) {
        self.custom_matcher_entries_map.update_value(|map| {
            if let Some(entries) = map.get_mut(field_name) {
                entries.retain(|e| e.id != entry_id);
            }
        });
    }

    fn get_field_custom_errors(&self, field_name: &str) -> HashMap<String, bool> {
        self.custom_errors_map
            .get()
            .get(field_name)
            .cloned()
            .unwrap_or_default()
    }

    fn set_field_custom_errors(&self, field_name: &str, errors: HashMap<String, bool>) {
        self.custom_errors_map.update(|map| {
            let entry = map.entry(field_name.to_string()).or_default();
            entry.extend(errors);
        });
    }

    fn clear_field_validation(&self, field_name: &str) {
        self.validity_map.update(|map| {
            map.remove(field_name);
        });
        self.custom_errors_map.update(|map| {
            map.insert(field_name.to_string(), HashMap::new());
        });
    }
}

#[derive(Clone)]
struct AriaDescriptionContextValue {
    message_ids_map: RwSignal<HashMap<String, HashSet<String>>>,
}

impl AriaDescriptionContextValue {
    fn add_field_message_id(&self, field_name: &str, id: &str) {
        self.message_ids_map.update(|map| {
            map.entry(field_name.to_string())
                .or_default()
                .insert(id.to_string());
        });
    }

    fn remove_field_message_id(&self, field_name: &str, id: &str) {
        self.message_ids_map.update(|map| {
            if let Some(ids) = map.get_mut(field_name) {
                ids.remove(id);
            }
        });
    }

    fn get_field_description(&self, field_name: &str) -> Option<String> {
        let map = self.message_ids_map.get();
        map.get(field_name).and_then(|ids| {
            if ids.is_empty() {
                None
            } else {
                Some(ids.iter().cloned().collect::<Vec<_>>().join(" "))
            }
        })
    }
}

#[derive(Clone)]
struct FormFieldContextValue {
    id: String,
    name: String,
    server_invalid: Signal<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * Helper functions
 * -----------------------------------------------------------------------------------------------*/

fn has_built_in_error(validity: &Validity) -> bool {
    validity.bad_input
        || validity.pattern_mismatch
        || validity.range_overflow
        || validity.range_underflow
        || validity.step_mismatch
        || validity.too_long
        || validity.too_short
        || validity.type_mismatch
        || validity.value_missing
}

fn get_valid_attribute(validity: &Option<Validity>, server_invalid: bool) -> Option<&'static str> {
    if let Some(v) = validity
        && v.valid
        && !server_invalid
    {
        return Some("true");
    }
    None
}

fn get_invalid_attribute(
    validity: &Option<Validity>,
    server_invalid: bool,
) -> Option<&'static str> {
    if let Some(v) = validity
        && !v.valid
    {
        return Some("true");
    }
    if server_invalid {
        return Some("true");
    }
    None
}

fn get_first_invalid_control(form: &web_sys::HtmlFormElement) -> Option<web_sys::HtmlElement> {
    let elements = form.elements();
    let len = elements.length();
    for i in 0..len {
        if let Some(element) = elements.item(i)
            && let Ok(html_element) = element.dyn_into::<web_sys::HtmlElement>()
            && let Some(input) = html_element.dyn_ref::<web_sys::HtmlInputElement>()
            && (!input.validity().valid()
                || html_element.get_attribute("aria-invalid").as_deref() == Some("true"))
        {
            return Some(html_element);
        }
    }
    None
}

/* -------------------------------------------------------------------------------------------------
 * Form
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Form(
    #[prop(into, optional)] on_clear_server_errors: Option<Callback<()>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let validation_context = ValidationContextValue {
        validity_map: RwSignal::new(HashMap::new()),
        custom_matcher_entries_map: StoredValue::new(HashMap::new()),
        custom_errors_map: RwSignal::new(HashMap::new()),
    };
    let aria_description_context = AriaDescriptionContextValue {
        message_ids_map: RwSignal::new(HashMap::new()),
    };

    let internal_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, internal_ref]);

    // The `invalid` event does NOT bubble, so `on:invalid` on a <form> element won't catch
    // invalid events from child inputs. React works around this via event delegation with capture.
    // We use a capture-phase event listener to intercept invalid events from all controls.
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(form_el) = node.dyn_ref::<web_sys::HtmlFormElement>()
        {
            let form_clone = form_el.clone();
            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                let target = event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());

                if let Some(target) = target
                    && let Some(first_invalid) = get_first_invalid_control(&form_clone)
                    && first_invalid == target
                {
                    first_invalid.focus().ok();
                }

                // Prevent default browser UI for form validation (built-in tooltips).
                event.prevent_default();
            });

            // Use capture phase (third arg = true) since `invalid` doesn't bubble.
            form_el
                .add_event_listener_with_callback_and_bool(
                    "invalid",
                    closure.as_ref().unchecked_ref(),
                    true,
                )
                .ok();

            let form_cleanup = SendWrapper::new(form_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                form_cleanup
                    .remove_event_listener_with_callback_and_bool(
                        "invalid",
                        closure_cleanup.as_ref().unchecked_ref(),
                        true,
                    )
                    .ok();
            });
        }
    });

    let on_clear = on_clear_server_errors;
    let children = StoredValue::new(children.into_inner());

    view! {
        <Provider value=validation_context>
            <Provider value=aria_description_context>
                <Primitive
                    element=html::form
                    as_child=as_child
                    node_ref=composed_ref
                    on:submit=move |_event: web_sys::SubmitEvent| {
                        if let Some(on_clear) = &on_clear {
                            on_clear.run(());
                        }
                    }
                    on:reset=move |_event: web_sys::Event| {
                        if let Some(on_clear) = &on_clear {
                            on_clear.run(());
                        }
                    }
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormField
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormField(
    #[prop(into)] name: String,
    #[prop(into, optional)] server_invalid: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let id = use_id(None);

    let field_name = name.clone();
    let server_invalid_signal = Signal::derive(move || server_invalid.get().unwrap_or(false));

    let field_context = FormFieldContextValue {
        id: id.get_untracked(),
        name: field_name.clone(),
        server_invalid: server_invalid_signal,
    };

    let validity_name = field_name.clone();
    let validity = Memo::new(move |_| validation_context.get_field_validity(&validity_name));
    let valid_attr =
        Memo::new(move |_| get_valid_attribute(&validity.get(), server_invalid_signal.get()));
    let invalid_attr =
        Memo::new(move |_| get_invalid_attribute(&validity.get(), server_invalid_signal.get()));

    let children = StoredValue::new(children.into_inner());

    view! {
        <Provider value=field_context>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-valid=move || valid_attr.get()
                attr:data-invalid=move || invalid_attr.get()
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormLabel
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormLabel(
    #[prop(into, optional)] html_for: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let field_context = expect_context::<FormFieldContextValue>();

    let html_for = html_for.unwrap_or_else(|| field_context.id.clone());
    let field_name = field_context.name.clone();
    let server_invalid = field_context.server_invalid;

    let validity = Memo::new(move |_| validation_context.get_field_validity(&field_name));
    let valid_attr = Memo::new(move |_| get_valid_attribute(&validity.get(), server_invalid.get()));
    let invalid_attr =
        Memo::new(move |_| get_invalid_attribute(&validity.get(), server_invalid.get()));

    let children = StoredValue::new(children.into_inner());

    view! {
        <radix_leptos_label::Label
            attr:r#for=html_for
            as_child=as_child
            node_ref=node_ref
            attr:data-valid=move || valid_attr.get()
            attr:data-invalid=move || invalid_attr.get()
        >
            {children.with_value(|children| children())}
        </radix_leptos_label::Label>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormControl
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormControl(
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] on_invalid: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_change: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let field_context = expect_context::<FormFieldContextValue>();
    let aria_description_context = expect_context::<AriaDescriptionContextValue>();

    let control_name = name.unwrap_or_else(|| field_context.name.clone());
    let control_id = id.unwrap_or_else(|| field_context.id.clone());
    let server_invalid = field_context.server_invalid;

    let internal_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, internal_ref]);

    // Derived validity for rendering
    let validity_name = control_name.clone();
    let validity_ctx = validation_context.clone();
    let validity = Memo::new(move |_| validity_ctx.get_field_validity(&validity_name));
    let valid_attr = Memo::new(move |_| get_valid_attribute(&validity.get(), server_invalid.get()));
    let invalid_attr =
        Memo::new(move |_| get_invalid_attribute(&validity.get(), server_invalid.get()));

    let desc_name = control_name.clone();
    let aria_describedby =
        Memo::new(move |_| aria_description_context.get_field_description(&desc_name));

    let aria_invalid_attr = Memo::new(move |_| {
        if server_invalid.get() {
            Some("true")
        } else {
            None
        }
    });

    // Set up native `change` event listener for validation.
    // We use the native `change` event (not Leptos `on:change` which fires on input)
    // to validate only when the user finishes changing the value, not on every keystroke.
    let change_name = control_name.clone();
    let change_validation_context = validation_context.clone();
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
        {
            let name = change_name.clone();
            let ctx = change_validation_context.clone();
            let control_clone = control_el.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                update_control_validity(&control_clone, &name, &ctx);
            });

            control_el
                .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .ok();

            let control_cleanup = SendWrapper::new(control_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                control_cleanup
                    .remove_event_listener_with_callback(
                        "change",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    // Set up form `reset` event listener to clear validation
    let reset_name = control_name.clone();
    let reset_validation_context = validation_context.clone();
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
            && let Some(form) = control_el.form()
        {
            let name = reset_name.clone();
            let ctx = reset_validation_context.clone();
            let control_clone = control_el.clone();

            let closure = Closure::<dyn Fn()>::new(move || {
                control_clone.set_custom_validity("");
                ctx.clear_field_validation(&name);
            });

            form.add_event_listener_with_callback("reset", closure.as_ref().unchecked_ref())
                .ok();

            let form_cleanup = SendWrapper::new(form);
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                form_cleanup
                    .remove_event_listener_with_callback(
                        "reset",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    // Focus first invalid control when fields are set as invalid by server
    Effect::new(move |_| {
        if server_invalid.get()
            && let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
            && let Some(form) = control_el.closest("form").ok().flatten()
        {
            let form: web_sys::HtmlFormElement = form.unchecked_into();
            if let Some(first_invalid) = get_first_invalid_control(&form) {
                let control_html: &web_sys::HtmlElement = control_el.as_ref();
                if first_invalid == *control_html {
                    first_invalid.focus().ok();
                }
            }
        }
    });

    // Set up native `invalid` event listener for validation.
    // We use `addEventListener` directly because `on:invalid` on a component (VoidPrimitive)
    // may not reliably forward to the underlying <input> element through component layers.
    let invalid_name = control_name.clone();
    let invalid_validation_context = validation_context.clone();
    let on_invalid_prop = on_invalid;
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
        {
            let name = invalid_name.clone();
            let ctx = invalid_validation_context.clone();
            let control_clone = control_el.clone();
            let on_invalid_prop = on_invalid_prop;

            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Some(cb) = &on_invalid_prop {
                    cb.run(event.clone());
                }

                // Always update validity — don't check defaultPrevented here because
                // the Form's capture-phase invalid listener already calls preventDefault
                // to suppress browser tooltips. React doesn't check defaultPrevented either.
                update_control_validity(&control_clone, &name, &ctx);
            });

            control_el
                .add_event_listener_with_callback("invalid", closure.as_ref().unchecked_ref())
                .ok();

            let control_cleanup = SendWrapper::new(control_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                control_cleanup
                    .remove_event_listener_with_callback(
                        "invalid",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    // Set up native `input` event listener to reset validity when user changes value.
    // React's `onChange` is actually the native `input` event (fires on every keystroke).
    let reset_validity_name = control_name.clone();
    let reset_validity_ctx = validation_context.clone();
    let on_change_prop = on_change;
    Effect::new(move |_| {
        if let Some(node) = internal_ref.get()
            && let Some(control_el) = node.dyn_ref::<web_sys::HtmlInputElement>()
        {
            let name = reset_validity_name.clone();
            let ctx = reset_validity_ctx.clone();
            let control_clone = control_el.clone();
            let on_change_prop = on_change_prop;

            let closure = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
                if let Some(cb) = &on_change_prop {
                    cb.run(event);
                }

                control_clone.set_custom_validity("");
                ctx.clear_field_validation(&name);
            });

            control_el
                .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
                .ok();

            let control_cleanup = SendWrapper::new(control_el.clone());
            let closure_cleanup = SendWrapper::new(closure);
            Owner::on_cleanup(move || {
                control_cleanup
                    .remove_event_listener_with_callback(
                        "input",
                        closure_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let children = StoredValue::new(children);

    view! {
        <VoidPrimitive
            element=html::input
            as_child=as_child
            node_ref=composed_ref
            attr:data-valid=move || valid_attr.get()
            attr:data-invalid=move || invalid_attr.get()
            attr:aria-invalid=move || aria_invalid_attr.get()
            attr:aria-describedby=move || aria_describedby.get()
            attr:title=""
            attr:id=control_id.clone()
            attr:name=control_name.clone()
        >
            {children.with_value(|children| children.as_ref().map(|c| c()))}
        </VoidPrimitive>
    }
}

fn update_control_validity(
    control: &web_sys::HtmlInputElement,
    name: &str,
    validation_context: &ValidationContextValue,
) {
    // 1. First, if we have built-in errors we stop here
    let validity: Validity = control.validity().into();
    if has_built_in_error(&validity) {
        validation_context.set_field_validity(name, validity);
        return;
    }

    // 2. Gather the form data to give to custom matchers for cross-comparisons
    let form_data = control
        .form()
        .and_then(|form| web_sys::FormData::new_with_form(&form).ok())
        .unwrap_or_else(|| web_sys::FormData::new().expect("FormData::new() should succeed"));
    let value = control.value();

    // 3. Split sync and async custom matcher entries
    let custom_matcher_entries = validation_context.get_field_custom_matcher_entries(name);
    let mut sync_entries = Vec::new();
    let mut async_entries = Vec::new();

    for entry in &custom_matcher_entries {
        match &entry.matcher {
            CustomMatcher::Sync(_) => sync_entries.push(entry),
            CustomMatcher::Async(_) => async_entries.push(entry),
        }
    }

    // 4. Run sync custom matchers and update control validity / internal validity + errors
    let mut sync_custom_errors = HashMap::new();
    for entry in &sync_entries {
        if let CustomMatcher::Sync(matcher) = &entry.matcher {
            let matches = matcher(value.clone(), form_data.clone());
            sync_custom_errors.insert(entry.id.clone(), matches);
        }
    }
    let has_sync_custom_errors = sync_custom_errors.values().any(|&v| v);
    let has_custom_error = has_sync_custom_errors;
    control.set_custom_validity(if has_custom_error {
        DEFAULT_INVALID_MESSAGE
    } else {
        ""
    });
    let control_validity: Validity = control.validity().into();
    validation_context.set_field_validity(name, control_validity);
    validation_context.set_field_custom_errors(name, sync_custom_errors);

    // 5. Run async custom matchers and update control validity / internal validity + errors
    if !has_sync_custom_errors && !async_entries.is_empty() {
        let name = name.to_string();
        let validation_context = validation_context.clone();
        let control = control.clone();

        type AsyncMatcherFutures = Vec<(String, Pin<Box<dyn Future<Output = bool>>>)>;
        let mut futures: AsyncMatcherFutures = Vec::new();
        for entry in &async_entries {
            if let CustomMatcher::Async(matcher) = &entry.matcher {
                futures.push((entry.id.clone(), matcher(value.clone(), form_data.clone())));
            }
        }

        leptos::task::spawn_local(async move {
            let mut async_custom_errors = HashMap::new();
            for (id, future) in futures {
                let matches = future.await;
                async_custom_errors.insert(id, matches);
            }
            let has_async_custom_errors = async_custom_errors.values().any(|&v| v);
            control.set_custom_validity(if has_async_custom_errors {
                DEFAULT_INVALID_MESSAGE
            } else {
                ""
            });
            let control_validity: Validity = control.validity().into();
            validation_context.set_field_validity(&name, control_validity);
            validation_context.set_field_custom_errors(&name, async_custom_errors);
        });
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormMessage
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormMessage(
    #[prop(into, optional)] r#match: Option<Match>,
    #[prop(into, optional)] force_match: Option<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let field_context = expect_context::<FormFieldContextValue>();
    let name = name.unwrap_or_else(|| field_context.name.clone());
    let generated_id = use_id(None);
    let id = id.unwrap_or_else(|| generated_id.get_untracked());
    let children = StoredValue::new(children);

    match r#match {
        None => view! {
            <FormMessageImpl name=name.clone() id=id.clone() as_child=as_child node_ref=node_ref>
                {children.with_value(|c| match c {
                    Some(c) => c().into_any(),
                    None => view! { {DEFAULT_INVALID_MESSAGE} }.into_any(),
                })}
            </FormMessageImpl>
        }
        .into_any(),
        Some(Match::BuiltIn(matcher)) => {
            let default_msg = matcher.default_message();
            view! {
                <FormBuiltInMessage
                    r#match=matcher
                    force_match=force_match.unwrap_or(false)
                    name=name.clone()
                    id=id.clone()
                    as_child=as_child
                    node_ref=node_ref
                >
                    {children.with_value(|c| match c {
                        Some(c) => c().into_any(),
                        None => view! { {default_msg} }.into_any(),
                    })}
                </FormBuiltInMessage>
            }
            .into_any()
        }
        Some(Match::Custom(matcher)) => view! {
            <FormCustomMessage
                matcher=CustomMatcher::Sync(matcher)
                force_match=force_match.unwrap_or(false)
                name=name.clone()
                id=id.clone()
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|c| match c {
                    Some(c) => c().into_any(),
                    None => view! { {DEFAULT_INVALID_MESSAGE} }.into_any(),
                })}
            </FormCustomMessage>
        }
        .into_any(),
        Some(Match::CustomAsync(matcher)) => view! {
            <FormCustomMessage
                matcher=CustomMatcher::Async(matcher)
                force_match=force_match.unwrap_or(false)
                name=name.clone()
                id=id.clone()
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|c| match c {
                    Some(c) => c().into_any(),
                    None => view! { {DEFAULT_INVALID_MESSAGE} }.into_any(),
                })}
            </FormCustomMessage>
        }
        .into_any(),
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormBuiltInMessage
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FormBuiltInMessage(
    r#match: ValidityMatcher,
    #[prop(into, optional)] force_match: bool,
    #[prop(into)] name: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let matcher = r#match;
    let field_name = name.clone();

    let validity = Memo::new(move |_| validation_context.get_field_validity(&field_name));

    let matches = Memo::new(move |_| {
        if force_match {
            return true;
        }
        if let Some(v) = validity.get() {
            matcher.matches(&v)
        } else {
            false
        }
    });

    let children = StoredValue::new(children);

    view! {
        <Show when=move || matches.get()>
            <FormMessageImpl name=name.clone() id=id.clone() as_child=as_child node_ref=node_ref>
                {children.with_value(|children| children())}
            </FormMessageImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormCustomMessage
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FormCustomMessage(
    matcher: CustomMatcher,
    #[prop(into, optional)] force_match: bool,
    #[prop(into)] name: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();

    let entry = SendWrapper::new(CustomMatcherEntry {
        id: id.clone(),
        matcher,
    });

    let entry_name = name.clone();
    validation_context.add_field_custom_matcher_entry(&entry_name, entry);

    let cleanup_name = name.clone();
    let cleanup_id = id.clone();
    let cleanup_ctx = validation_context.clone();
    Owner::on_cleanup(move || {
        cleanup_ctx.remove_field_custom_matcher_entry(&cleanup_name, &cleanup_id);
    });

    let matches_name = name.clone();
    let matches_id = id.clone();
    let matches = Memo::new(move |_| {
        if force_match {
            return true;
        }
        let validity = validation_context.get_field_validity(&matches_name);
        let custom_errors = validation_context.get_field_custom_errors(&matches_name);
        let has_matching_error = custom_errors.get(&matches_id).copied().unwrap_or(false);
        validity.is_some() && !has_built_in_error(validity.as_ref().unwrap()) && has_matching_error
    });

    let children = StoredValue::new(children);

    view! {
        <Show when=move || matches.get()>
            <FormMessageImpl name=name.clone() id=id.clone() as_child=as_child node_ref=node_ref>
                {children.with_value(|children| children())}
            </FormMessageImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormMessageImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FormMessageImpl(
    #[prop(into)] name: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let aria_description_context = expect_context::<AriaDescriptionContextValue>();

    aria_description_context.add_field_message_id(&name, &id);

    let cleanup_name = name;
    let cleanup_id = id.clone();
    Owner::on_cleanup(move || {
        aria_description_context.remove_field_message_id(&cleanup_name, &cleanup_id);
    });

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:id=id
        >
            {children()}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormValidityState
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormValidityState(
    #[prop(into, optional)] name: Option<String>,
    children: Callback<Option<Validity>, AnyView>,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let field_context = expect_context::<FormFieldContextValue>();
    let name = name.unwrap_or_else(|| field_context.name.clone());

    let validity = Memo::new(move |_| validation_context.get_field_validity(&name));

    move || children.run(validity.get())
}

/* -------------------------------------------------------------------------------------------------
 * FormSubmit
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormSubmit(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="submit"
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Re-exports
 * -----------------------------------------------------------------------------------------------*/

pub use Form as Root;
pub use FormControl as Control;
pub use FormField as Field;
pub use FormLabel as Label;
pub use FormMessage as Message;
pub use FormSubmit as Submit;
pub use FormValidityState as ValidityState;
