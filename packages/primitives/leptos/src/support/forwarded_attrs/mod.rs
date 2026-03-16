// ── ForwardedAttrs ───────────────────────────────────────────────────────────
//
// A reactive attribute forwarding primitive for use with `AttributeInterceptor`.
//
// Solves two problems:
// 1. **Show/Presence survival**: Attrs can be applied to elements inside `Show`
//    or `Presence`, surviving mount/unmount cycles.
// 2. **Reactivity preservation**: Unlike `extract_attrs` (which flattens reactive
//    closures to static strings), this preserves the original `AnyAttribute` and
//    calls `build()` on the target element. The attribute's internal `RenderEffect`
//    maintains signal subscriptions independently.

use leptos::attr::Attribute as _;
use leptos::attr::any_attribute::{AnyAttribute, AnyAttributeState};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

/// Reactive attribute forwarder for use with `AttributeInterceptor`.
///
/// Captures `AnyAttribute` from an `AttributeInterceptor` closure and applies
/// them to one or more target elements via `node_ref`, preserving reactivity
/// and surviving `Show`/`Presence` mount cycles.
///
/// # Usage
///
/// ```rust,ignore
/// let forwarded = ForwardedAttrs::new();
/// let target_ref = forwarded.target();
///
/// view! {
///     <AttributeInterceptor let:attrs>
///         {forwarded.set(attrs)}
///         <div>
///             <Show when=move || visible.get()>
///                 <div node_ref=target_ref>"I receive the attrs"</div>
///             </Show>
///         </div>
///     </AttributeInterceptor>
/// }
/// ```
///
/// When the target element is managed by another component (e.g., the node_ref
/// is passed through props), use `apply_to()` instead of `target()`:
///
/// ```rust,ignore
/// let forwarded = ForwardedAttrs::new();
/// forwarded.apply_to(node_ref);
///
/// view! {
///     <AttributeInterceptor let:attrs>
///         {forwarded.set(attrs)}
///         <InnerComponent node_ref=node_ref />
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
    ///     // ... children ...
    /// </AttributeInterceptor>
    /// ```
    pub fn set(&self, attrs: AnyAttribute) {
        self.attrs.set_value(Some(SendWrapper::new(attrs)));
    }

    /// Create a new `AnyNodeRef` that will have attrs applied when mounted.
    ///
    /// Use when your component owns the element directly. The returned ref
    /// should be applied via `node_ref=`.
    ///
    /// Can be called multiple times for multi-target support.
    pub fn target(&self) -> AnyNodeRef {
        let node_ref = AnyNodeRef::new();
        self.apply_to(node_ref);
        node_ref
    }

    /// Apply attrs to an existing `AnyNodeRef`.
    ///
    /// Use when the target element's node_ref is managed externally (e.g.,
    /// passed as a prop through inner components). Creates an Effect that
    /// watches the ref and builds attrs when the element mounts.
    pub fn apply_to(&self, node_ref: AnyNodeRef) {
        let stored_attrs = self.attrs;

        let attr_state: StoredValue<Option<SendWrapper<AnyAttributeState>>> =
            StoredValue::new(None);

        Effect::new(move |_| {
            if let Some(el) = node_ref.get() {
                let el: web_sys::Element = el.unchecked_into();

                stored_attrs.with_value(|attrs: &Option<SendWrapper<AnyAttribute>>| {
                    if let Some(attrs) = attrs {
                        let state = AnyAttribute::clone(attrs).build(&el);
                        attr_state.set_value(Some(SendWrapper::new(state)));
                    }
                });
            } else {
                attr_state.set_value(None);
            }
        });
    }
}
