// ── Experiment: Attribute Spreading Under Presence/Show ──────────────────────
//
// Problem: When a component uses AttributeInterceptor + Show (or Presence),
// attributes spread via `{..attrs}` only apply on the initial render. When
// Show toggles false→true again, the children closure is re-called but the
// attrs from AttributeInterceptor are consumed (FnOnce-like) on first build,
// so subsequent renders get empty/stale attributes.
//
// This experiment demonstrates:
// 1. The broken baseline: attrs spread directly inside Show — lost on re-mount
// 2. The StoredValue+Effect workaround (used by MenuContent)
// 3. Alternative: StoredValue + create_effect on node_ref with cleanup

use leptos::attr::Attribute as _;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use web_sys::wasm_bindgen::JsCast;

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Extract (name, value) pairs from AnyAttribute by building on a temp element.
/// Same approach as `extract_attrs` in the main codebase (presence/mod.rs).
fn extract_attrs(attrs: leptos::attr::any_attribute::AnyAttribute) -> Vec<(String, String)> {
    let tmp = document()
        .create_element("div")
        .expect("Element should be created.");
    let _state = attrs.build(&tmp);
    let named = tmp.attributes();
    let mut pairs = vec![];
    for i in 0..named.length() {
        if let Some(attr) = named.item(i) {
            pairs.push((attr.name(), attr.value()));
        }
    }
    pairs
}

// ── Fixture 1: Broken Baseline ──────────────────────────────────────────────
//
// Attrs are spread directly onto the element inside Show. When the Show
// condition cycles false→true, the attrs may be lost because the
// AnyAttribute was consumed on first build.

#[component]
fn BrokenBaseline() -> impl IntoView {
    view! {
        <AttributeInterceptor let:attrs>
            <div data-testid="broken-target" {..attrs}>
                "I am the target"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn BrokenBaselineFixture() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div data-testid="broken-fixture">
            <Show when=move || visible.get()>
                <BrokenBaseline
                    attr:class="test-class"
                    attr:data-custom="test-value"
                    attr:aria-label="test-label"
                />
            </Show>
            <button data-testid="broken-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                {move || if visible.get() { "Hide" } else { "Show" }}
            </button>
        </div>
    }
}

// ── Fixture 2: StoredValue + Effect Workaround ──────────────────────────────
//
// The approach used by MenuContent: capture attrs into a StoredValue via
// extract_attrs, then apply them to the node via an Effect keyed on node_ref.

#[component]
fn EffectWorkaround() -> impl IntoView {
    let node_ref = AnyNodeRef::new();
    let forwarded_attrs: StoredValue<Vec<(String, String)>> = StoredValue::new(vec![]);

    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el: web_sys::Element = el.unchecked_into();
            forwarded_attrs.with_value(|attrs| {
                for (name, value) in attrs {
                    el.set_attribute(name, value).ok();
                }
            });
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded_attrs.set_value(extract_attrs(attrs))}
            <div data-testid="effect-target" node_ref=node_ref>
                "I am the target"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn EffectWorkaroundFixture() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <div data-testid="effect-fixture">
            <Show when=move || visible.get()>
                <EffectWorkaround
                    attr:class="test-class"
                    attr:data-custom="test-value"
                    attr:aria-label="test-label"
                />
            </Show>
            <button data-testid="effect-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                {move || if visible.get() { "Hide" } else { "Show" }}
            </button>
        </div>
    }
}

// ── Fixture 3: Internal Show — COMPILE ERROR ────────────────────────────────
//
// FINDING: You CANNOT spread `{..attrs}` (or `{..attrs.clone()}`) on an element
// inside a `Show` closure. This fails at compile time because:
//
//   - `Show` requires children to implement `ChildrenFn` (callable multiple times)
//   - Spreading `{..attrs}` produces a view type that involves `AnyAttribute`,
//     which makes the resulting `HtmlElement` type incompatible with `ToChildren`
//   - The compiler error: "required for `TypedChildrenFn<HtmlElement<Div, ...>>`
//     to implement `ToChildren<...>`"
//
// This is the CORE PROBLEM that forces the StoredValue+Effect workaround.
// The naive approach (spread attrs inside Show) is not possible in Leptos's
// type system.
//
// Commented-out code preserved for documentation:
//
// ```rust
// #[component]
// fn InternalShowBroken() -> impl IntoView {
//     let (visible, set_visible) = signal(true);
//     view! {
//         <AttributeInterceptor let:attrs>
//             <div>
//                 <Show when=move || visible.get()>
//                     // ERROR: this does not compile
//                     <div data-testid="target" {..attrs.clone()}>
//                         "I am the target"
//                     </div>
//                 </Show>
//             </div>
//         </AttributeInterceptor>
//     }
// }
// ```
//
// Instead, fixture 3a demonstrates what happens when attrs land on the WRAPPER
// (outside Show) instead of the TARGET (inside Show) — they survive cycling
// but are on the wrong element.

#[component]
fn AttrsOnWrapperNotTarget() -> impl IntoView {
    let (visible, set_visible) = signal(true);

    view! {
        <AttributeInterceptor let:attrs>
            // Attrs land on this wrapper div (outside Show) because we can't
            // spread them inside Show. The target inside Show gets no attrs.
            <div data-testid="wrapper-with-attrs" {..attrs}>
                <Show when=move || visible.get()>
                    <div data-testid="target-without-attrs">
                        "I am the target (no attrs)"
                    </div>
                </Show>
                <button data-testid="wrapper-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn AttrsOnWrapperFixture() -> impl IntoView {
    view! {
        <div data-testid="wrapper-fixture">
            <AttrsOnWrapperNotTarget
                attr:class="test-class"
                attr:data-custom="test-value"
                attr:aria-label="test-label"
            />
        </div>
    }
}

// ── Fixture 4: Internal Show + Effect Workaround ────────────────────────────
//
// Same as fixture 3 but with the StoredValue+Effect pattern applied.

#[component]
fn InternalShowEffect() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let node_ref = AnyNodeRef::new();
    let forwarded_attrs: StoredValue<Vec<(String, String)>> = StoredValue::new(vec![]);

    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el: web_sys::Element = el.unchecked_into();
            forwarded_attrs.with_value(|attrs| {
                for (name, value) in attrs {
                    el.set_attribute(name, value).ok();
                }
            });
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded_attrs.set_value(extract_attrs(attrs))}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="internal-effect-target" node_ref=node_ref>
                        "I am the target"
                    </div>
                </Show>
                <button data-testid="internal-effect-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn InternalShowEffectFixture() -> impl IntoView {
    view! {
        <div data-testid="internal-effect-fixture">
            <InternalShowEffect
                attr:class="test-class"
                attr:data-custom="test-value"
                attr:aria-label="test-label"
            />
        </div>
    }
}

// ── Fixture 5: Reactive attrs + Show cycling ────────────────────────────────
//
// Tests whether reactive (signal-driven) attributes survive Show cycling
// with the Effect workaround.

#[component]
fn ReactiveShowEffect() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let node_ref = AnyNodeRef::new();
    let forwarded_attrs: StoredValue<Vec<(String, String)>> = StoredValue::new(vec![]);

    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el: web_sys::Element = el.unchecked_into();
            forwarded_attrs.with_value(|attrs| {
                for (name, value) in attrs {
                    el.set_attribute(name, value).ok();
                }
            });
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded_attrs.set_value(extract_attrs(attrs))}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="reactive-show-target" node_ref=node_ref>
                        "I am the target"
                    </div>
                </Show>
                <button data-testid="reactive-show-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn ReactiveShowEffectFixture() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div data-testid="reactive-show-fixture">
            <ReactiveShowEffect attr:data-count=move || count.get().to_string() />
            <button data-testid="reactive-show-increment" on:click=move |_| set_count.update(|c| *c += 1)>
                "Increment"
            </button>
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn PresenceAttrsPage() -> impl IntoView {
    view! {
        <h1>"Experiment: Presence/Show Attribute Spreading"</h1>

        <h2>"1. Broken Baseline (external Show)"</h2>
        <p>"Attrs spread directly on element. Parent Show toggles the component."</p>
        <BrokenBaselineFixture />

        <h2>"2. StoredValue + Effect Workaround (external Show)"</h2>
        <p>"Attrs extracted into StoredValue, applied via Effect on node_ref."</p>
        <EffectWorkaroundFixture />

        <h2>"3. Wrong-Element Problem (attrs on wrapper, not target)"</h2>
        <p>"Cannot spread inside Show (compile error). Attrs land on wrapper outside Show instead."</p>
        <AttrsOnWrapperFixture />

        <h2>"4. Internal Show + Effect Workaround"</h2>
        <p>"Component owns the Show. Attrs extracted + applied via Effect."</p>
        <InternalShowEffectFixture />

        <h2>"5. Reactive Attrs + Show Cycling (Effect workaround)"</h2>
        <p>"Signal-driven attrs with the Effect workaround. Tests if reactive updates survive cycling."</p>
        <ReactiveShowEffectFixture />
    }
}
