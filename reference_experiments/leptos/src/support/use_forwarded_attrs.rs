// ── ForwardedAttrs ───────────────────────────────────────────────────────────
//
// Stores `AnyAttribute` from an `AttributeInterceptor` closure and provides
// a `spread()` method that returns a clone suitable for `{..}` spreading.
//
// Because `AnyAttribute` is `Clone`, `spread()` can be called multiple times
// to apply the same attrs to multiple elements. Each spread gets its own
// independent reactive subscriptions (built internally by `RenderEffect`
// when the attribute contains signal-backed closures).

use leptos::attr::Attribute as _;
use leptos::attr::any_attribute::{AnyAttribute, IntoAnyAttribute};
use leptos::prelude::*;
use send_wrapper::SendWrapper;

/// Stores intercepted attributes and provides a spreadable clone.
///
/// # Usage
///
/// ```rust,ignore
/// let forwarded = ForwardedAttrs::new();
///
/// view! {
///     <AttributeInterceptor let:attrs>
///         {forwarded.set(attrs)}
///         <div>
///             <Show when=move || visible.get()>
///                 <div {..forwarded.spread()}>"Target A"</div>
///                 <div {..forwarded.spread()}>"Target B"</div>
///             </Show>
///         </div>
///     </AttributeInterceptor>
/// }
/// ```
#[derive(Clone, Copy)]
pub struct ForwardedAttrs {
    attrs: StoredValue<Option<SendWrapper<AnyAttribute>>>,
}

impl ForwardedAttrs {
    /// Create a new `ForwardedAttrs` instance.
    pub fn new() -> Self {
        Self {
            attrs: StoredValue::new(None),
        }
    }

    /// Capture attributes from an `AttributeInterceptor` closure.
    ///
    /// Returns `()` so it can be used as an expression in `view!`:
    ///
    /// ```rust,ignore
    /// <AttributeInterceptor let:attrs>
    ///     {forwarded.set(attrs)}
    ///     // ...
    /// </AttributeInterceptor>
    /// ```
    pub fn set(&self, attrs: AnyAttribute) {
        self.attrs.set_value(Some(SendWrapper::new(attrs)));
    }

    /// Return a clone of the stored attributes, suitable for `{..}` spreading.
    ///
    /// If no attrs have been captured yet, returns a no-op attribute.
    /// Can be called multiple times — each call produces an independent clone.
    pub fn spread(&self) -> AnyAttribute {
        self.attrs
            .with_value(|a| a.as_ref().map(|a| AnyAttribute::clone(a)))
            .unwrap_or_else(|| ().into_any_attr())
    }
}
