// ── use_forwarded_attrs ──────────────────────────────────────────────────────
//
// A reactive attribute forwarding primitive that solves two problems with
// Leptos's `AttributeInterceptor`:
//
// 1. **Show/Presence survival**: Attrs can be applied to elements inside `Show`
//    or `Presence`, surviving mount/unmount cycles. (The naive `{..attrs}` spread
//    cannot be placed inside `Show` due to ChildrenFn type constraints.)
//
// 2. **Reactivity preservation**: Unlike `extract_attrs` (which flattens reactive
//    closures to static strings), this preserves the original `AnyAttribute` and
//    calls `build()` on the target element. The attribute's internal `RenderEffect`
//    maintains signal subscriptions independently.
//
// 3. **Multi-target support**: `target()` can be called multiple times to apply
//    the same attributes to multiple elements, each with independent reactive
//    subscriptions.
//
// ## How it works
//
// - `set()` stores the `AnyAttribute` (which wraps reactive closures) in a
//   `StoredValue`. This is called inside the `AttributeInterceptor` closure.
//
// - `target()` creates an `AnyNodeRef` + `Effect` pair. The Effect watches the
//   node_ref. When an element mounts, it clones the stored `AnyAttribute` and
//   calls `build()` on the element. The `build()` call creates internal
//   `RenderEffect`s for reactive attribute values, which independently track
//   signal changes and update the DOM.
//
// - The `AnyAttributeState` returned by `build()` is kept alive in a
//   `StoredValue<SendWrapper<_>>` for the duration of the mount. When the element
//   unmounts (Show hides), the state is dropped, cleaning up RenderEffects.
//   On remount, fresh `build()` creates new subscriptions.

use leptos::attr::Attribute as _;
use leptos::attr::any_attribute::{AnyAttribute, AnyAttributeState};
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

/// Reactive attribute forwarder for use with `AttributeInterceptor`.
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
#[derive(Clone, Copy)]
pub struct ForwardedAttrs {
    // Use SendWrapper because AnyAttribute may not implement Send+Sync
    // (its inner Erased is Send+Sync but the auto-trait derivation may not
    // propagate through the function pointer fields on all platforms).
    attrs: StoredValue<Option<SendWrapper<AnyAttribute>>>,
}

impl ForwardedAttrs {
    /// Create a new `ForwardedAttrs` instance. Call this at the top of your
    /// component function, before the `view!` macro.
    pub fn new() -> Self {
        Self {
            attrs: StoredValue::new(None),
        }
    }

    /// Capture attributes from an `AttributeInterceptor` closure.
    ///
    /// Call this inside the `AttributeInterceptor` closure body. Returns `()`
    /// so it can be used as an expression in `view!`:
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

    /// Create a reactive target for attribute application.
    ///
    /// Returns an `AnyNodeRef` that should be applied to the target element
    /// via `node_ref=`. When the element mounts, attributes are built on it
    /// with full reactivity preserved. When it unmounts, subscriptions are
    /// cleaned up. On remount, fresh subscriptions are created.
    ///
    /// Can be called multiple times to apply the same attributes to multiple
    /// elements. Each target gets independent reactive subscriptions.
    ///
    /// **Important**: Call this outside of `Show`/`Presence` so the Effect
    /// lives at the component scope, not inside the conditional.
    pub fn target(&self) -> AnyNodeRef {
        let node_ref = AnyNodeRef::new();
        let stored_attrs = self.attrs;

        // State storage for the built attribute. SendWrapper is needed because
        // AnyAttributeState contains ErasedLocal (not Send).
        let attr_state: StoredValue<Option<SendWrapper<AnyAttributeState>>> =
            StoredValue::new(None);

        Effect::new(move |_| {
            if let Some(el) = node_ref.get() {
                let el: web_sys::Element = el.unchecked_into();

                stored_attrs.with_value(|attrs: &Option<SendWrapper<AnyAttribute>>| {
                    if let Some(attrs) = attrs {
                        // Clone the AnyAttribute (cheap — clones the closure/signal
                        // references, not the values) and build it on this element.
                        //
                        // build() creates internal RenderEffects for reactive values.
                        // These RenderEffects independently track signal changes and
                        // update the DOM — they don't pollute this outer Effect.
                        let state = AnyAttribute::clone(attrs).build(&el);
                        attr_state.set_value(Some(SendWrapper::new(state)));
                    }
                });
            } else {
                // Element unmounted — drop the AnyAttributeState.
                // This cleans up the internal RenderEffects and their signal
                // subscriptions.
                attr_state.set_value(None);
            }
        });

        node_ref
    }
}
