use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;

use crate::support::compose_refs::use_composed_refs;
use crate::support::id::use_id;
use crate::support::primitive::{Primitive, VoidPrimitive, prop_or_default};
use leptos::{context::Provider, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

mod form;
mod form_field;
mod form_message;

pub use form::*;
pub use form_field::*;
pub use form_message::*;

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
    pub(super) fn matches(&self, validity: &Validity) -> bool {
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

    pub(super) fn default_message(&self) -> &'static str {
        match self {
            Self::BadInput => DEFAULT_INVALID_MESSAGE,
            Self::PatternMismatch => "This value does not match the required pattern",
            Self::RangeOverflow => "This value is too large",
            Self::RangeUnderflow => "This value is too small",
            Self::StepMismatch => "This value does not match the required step",
            Self::TooLong => "This value is too long",
            Self::TooShort => "This value is too short",
            Self::TypeMismatch => "This value does not match the required type",
            Self::Valid => "",
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
pub(super) struct CustomMatcherEntry {
    pub(super) id: String,
    pub(super) matcher: CustomMatcher,
}

pub(super) const DEFAULT_INVALID_MESSAGE: &str = "This value is not valid";

/* -------------------------------------------------------------------------------------------------
 * Context types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
pub(super) struct ValidationContextValue {
    pub(super) validity_map: RwSignal<HashMap<String, Validity>>,
    pub(super) custom_matcher_entries_map:
        StoredValue<HashMap<String, Vec<SendWrapper<CustomMatcherEntry>>>>,
    pub(super) custom_errors_map: RwSignal<HashMap<String, HashMap<String, bool>>>,
}

impl ValidationContextValue {
    pub(super) fn get_field_validity(&self, field_name: &str) -> Option<Validity> {
        self.validity_map.get().get(field_name).cloned()
    }

    pub(super) fn set_field_validity(&self, field_name: &str, validity: Validity) {
        self.validity_map.update(|map| {
            map.insert(field_name.to_string(), validity);
        });
    }

    pub(super) fn get_field_custom_matcher_entries(
        &self,
        field_name: &str,
    ) -> Vec<SendWrapper<CustomMatcherEntry>> {
        self.custom_matcher_entries_map
            .with_value(|map| map.get(field_name).cloned().unwrap_or_default())
    }

    pub(super) fn add_field_custom_matcher_entry(
        &self,
        field_name: &str,
        entry: SendWrapper<CustomMatcherEntry>,
    ) {
        self.custom_matcher_entries_map.update_value(|map| {
            map.entry(field_name.to_string()).or_default().push(entry);
        });
    }

    pub(super) fn remove_field_custom_matcher_entry(&self, field_name: &str, entry_id: &str) {
        self.custom_matcher_entries_map.update_value(|map| {
            if let Some(entries) = map.get_mut(field_name) {
                entries.retain(|e| e.id != entry_id);
            }
        });
    }

    pub(super) fn get_field_custom_errors(&self, field_name: &str) -> HashMap<String, bool> {
        self.custom_errors_map
            .get()
            .get(field_name)
            .cloned()
            .unwrap_or_default()
    }

    pub(super) fn set_field_custom_errors(&self, field_name: &str, errors: HashMap<String, bool>) {
        self.custom_errors_map.update(|map| {
            let entry = map.entry(field_name.to_string()).or_default();
            entry.extend(errors);
        });
    }

    pub(super) fn clear_field_validation(&self, field_name: &str) {
        self.validity_map.update(|map| {
            map.remove(field_name);
        });
        self.custom_errors_map.update(|map| {
            map.insert(field_name.to_string(), HashMap::new());
        });
    }
}

#[derive(Clone)]
pub(super) struct AriaDescriptionContextValue {
    pub(super) message_ids_map: RwSignal<HashMap<String, HashSet<String>>>,
}

impl AriaDescriptionContextValue {
    pub(super) fn add_field_message_id(&self, field_name: &str, id: &str) {
        self.message_ids_map.update(|map| {
            map.entry(field_name.to_string())
                .or_default()
                .insert(id.to_string());
        });
    }

    pub(super) fn remove_field_message_id(&self, field_name: &str, id: &str) {
        self.message_ids_map.update(|map| {
            if let Some(ids) = map.get_mut(field_name) {
                ids.remove(id);
            }
        });
    }

    pub(super) fn get_field_description(&self, field_name: &str) -> Option<String> {
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
pub(super) struct FormFieldContextValue {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) server_invalid: Signal<bool>,
}

/* -------------------------------------------------------------------------------------------------
 * Helper functions
 * -----------------------------------------------------------------------------------------------*/

pub(super) fn has_built_in_error(validity: &Validity) -> bool {
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

pub(super) fn get_valid_attribute(
    validity: &Option<Validity>,
    server_invalid: bool,
) -> Option<&'static str> {
    if let Some(v) = validity
        && v.valid
        && !server_invalid
    {
        return Some("true");
    }
    None
}

pub(super) fn get_invalid_attribute(
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

pub(super) fn get_first_invalid_control(
    form: &web_sys::HtmlFormElement,
) -> Option<web_sys::HtmlElement> {
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

pub(super) fn update_control_validity(
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
 * Re-exports
 * -----------------------------------------------------------------------------------------------*/

pub use Form as Root;
pub use FormControl as Control;
pub use FormField as Field;
pub use FormLabel as Label;
pub use FormMessage as Message;
pub use FormSubmit as Submit;
pub use FormValidityState as ValidityState;

#[cfg(test)]
mod tests {
    use super::*;

    fn validity_all_false() -> Validity {
        Validity::default()
    }

    fn validity_with(f: impl FnOnce(&mut Validity)) -> Validity {
        let mut v = Validity::default();
        f(&mut v);
        v
    }

    // ── ValidityMatcher::matches ────────────────────────────

    #[test]
    fn matcher_bad_input() {
        let v = validity_with(|v| v.bad_input = true);
        assert!(ValidityMatcher::BadInput.matches(&v));
        assert!(!ValidityMatcher::BadInput.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_pattern_mismatch() {
        let v = validity_with(|v| v.pattern_mismatch = true);
        assert!(ValidityMatcher::PatternMismatch.matches(&v));
        assert!(!ValidityMatcher::PatternMismatch.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_range_overflow() {
        let v = validity_with(|v| v.range_overflow = true);
        assert!(ValidityMatcher::RangeOverflow.matches(&v));
        assert!(!ValidityMatcher::RangeOverflow.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_range_underflow() {
        let v = validity_with(|v| v.range_underflow = true);
        assert!(ValidityMatcher::RangeUnderflow.matches(&v));
        assert!(!ValidityMatcher::RangeUnderflow.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_step_mismatch() {
        let v = validity_with(|v| v.step_mismatch = true);
        assert!(ValidityMatcher::StepMismatch.matches(&v));
        assert!(!ValidityMatcher::StepMismatch.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_too_long() {
        let v = validity_with(|v| v.too_long = true);
        assert!(ValidityMatcher::TooLong.matches(&v));
        assert!(!ValidityMatcher::TooLong.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_too_short() {
        let v = validity_with(|v| v.too_short = true);
        assert!(ValidityMatcher::TooShort.matches(&v));
        assert!(!ValidityMatcher::TooShort.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_type_mismatch() {
        let v = validity_with(|v| v.type_mismatch = true);
        assert!(ValidityMatcher::TypeMismatch.matches(&v));
        assert!(!ValidityMatcher::TypeMismatch.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_valid() {
        let v = validity_with(|v| v.valid = true);
        assert!(ValidityMatcher::Valid.matches(&v));
        assert!(!ValidityMatcher::Valid.matches(&validity_all_false()));
    }

    #[test]
    fn matcher_value_missing() {
        let v = validity_with(|v| v.value_missing = true);
        assert!(ValidityMatcher::ValueMissing.matches(&v));
        assert!(!ValidityMatcher::ValueMissing.matches(&validity_all_false()));
    }

    // ── ValidityMatcher::default_message ────────────────────

    #[test]
    fn default_messages() {
        assert_eq!(
            ValidityMatcher::BadInput.default_message(),
            DEFAULT_INVALID_MESSAGE
        );
        assert_eq!(
            ValidityMatcher::PatternMismatch.default_message(),
            "This value does not match the required pattern"
        );
        assert_eq!(
            ValidityMatcher::RangeOverflow.default_message(),
            "This value is too large"
        );
        assert_eq!(
            ValidityMatcher::RangeUnderflow.default_message(),
            "This value is too small"
        );
        assert_eq!(
            ValidityMatcher::StepMismatch.default_message(),
            "This value does not match the required step"
        );
        assert_eq!(
            ValidityMatcher::TooLong.default_message(),
            "This value is too long"
        );
        assert_eq!(
            ValidityMatcher::TooShort.default_message(),
            "This value is too short"
        );
        assert_eq!(
            ValidityMatcher::TypeMismatch.default_message(),
            "This value does not match the required type"
        );
        assert_eq!(ValidityMatcher::Valid.default_message(), "");
        assert_eq!(
            ValidityMatcher::ValueMissing.default_message(),
            "This value is missing"
        );
    }

    // ── has_built_in_error ──────────────────────────────────

    #[test]
    fn no_errors_returns_false() {
        assert!(!has_built_in_error(&validity_all_false()));
    }

    #[test]
    fn valid_field_alone_is_not_an_error() {
        // valid=true but no actual error flags — should return false
        let v = validity_with(|v| v.valid = true);
        assert!(!has_built_in_error(&v));
    }

    #[test]
    fn each_error_field_triggers() {
        let fields: &[fn(&mut Validity)] = &[
            |v| v.bad_input = true,
            |v| v.pattern_mismatch = true,
            |v| v.range_overflow = true,
            |v| v.range_underflow = true,
            |v| v.step_mismatch = true,
            |v| v.too_long = true,
            |v| v.too_short = true,
            |v| v.type_mismatch = true,
            |v| v.value_missing = true,
        ];
        for f in fields {
            assert!(has_built_in_error(&validity_with(f)));
        }
    }

    // ── get_valid_attribute / get_invalid_attribute ──────────

    #[test]
    fn valid_attr_none_validity() {
        assert_eq!(get_valid_attribute(&None, false), None);
        assert_eq!(get_valid_attribute(&None, true), None);
    }

    #[test]
    fn valid_attr_valid_and_not_server_invalid() {
        let v = validity_with(|v| v.valid = true);
        assert_eq!(get_valid_attribute(&Some(v), false), Some("true"));
    }

    #[test]
    fn valid_attr_valid_but_server_invalid() {
        let v = validity_with(|v| v.valid = true);
        assert_eq!(get_valid_attribute(&Some(v), true), None);
    }

    #[test]
    fn valid_attr_not_valid() {
        assert_eq!(
            get_valid_attribute(&Some(validity_all_false()), false),
            None
        );
    }

    #[test]
    fn invalid_attr_none_validity_not_server_invalid() {
        assert_eq!(get_invalid_attribute(&None, false), None);
    }

    #[test]
    fn invalid_attr_none_validity_server_invalid() {
        assert_eq!(get_invalid_attribute(&None, true), Some("true"));
    }

    #[test]
    fn invalid_attr_not_valid() {
        // valid=false (default) → invalid
        assert_eq!(
            get_invalid_attribute(&Some(validity_all_false()), false),
            Some("true")
        );
    }

    #[test]
    fn invalid_attr_valid_not_server_invalid() {
        let v = validity_with(|v| v.valid = true);
        assert_eq!(get_invalid_attribute(&Some(v), false), None);
    }

    #[test]
    fn invalid_attr_valid_but_server_invalid() {
        let v = validity_with(|v| v.valid = true);
        assert_eq!(get_invalid_attribute(&Some(v), true), Some("true"));
    }
}
