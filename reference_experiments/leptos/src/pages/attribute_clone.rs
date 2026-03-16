// ── Experiment: AttributeInterceptor + AnyAttribute Clone ────────────────────
//
// Hypothesis: AnyAttribute implements Clone, so inside an AttributeInterceptor
// closure we can call `attrs.clone()` and spread the same attributes onto
// multiple elements via `{..attrs.clone()}` and `{..attrs}`.
//
// What we're testing:
// 1. Both elements receive the attributes from the parent (class, data-*, aria-*)
// 2. The attributes are independent — modifying one element's DOM doesn't affect the other
// 3. Reactive attributes (signal-driven) update on both elements

use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

/// A component that clones intercepted attrs onto two inner elements.
#[component]
fn DualTarget() -> impl IntoView {
    view! {
        <AttributeInterceptor let:attrs>
            <div>
                <div data-testid="target-a" {..attrs.clone()}>
                    "Target A"
                </div>
                <div data-testid="target-b" {..attrs}>
                    "Target B"
                </div>
            </div>
        </AttributeInterceptor>
    }
}

/// Wrapper that passes static attributes to DualTarget.
#[component]
fn StaticAttrsFixture() -> impl IntoView {
    view! {
        <div data-testid="static-fixture">
            <DualTarget
                attr:class="shared-class"
                attr:data-custom="hello"
                attr:aria-label="shared label"
            />
        </div>
    }
}

/// Wrapper that passes a reactive (signal-driven) attribute to DualTarget.
#[component]
fn ReactiveAttrsFixture() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div data-testid="reactive-fixture">
            <DualTarget attr:data-count=move || count.get().to_string() />
            <button
                data-testid="increment"
                on:click=move |_| set_count.update(|c| *c += 1)
            >
                "Increment"
            </button>
        </div>
    }
}

/// Wrapper that passes multiple heterogeneous attributes to DualTarget.
#[component]
fn MultipleAttrsFixture() -> impl IntoView {
    view! {
        <div data-testid="multi-fixture">
            <DualTarget
                attr:class="class-one"
                attr:data-foo="foo-val"
                attr:data-bar="bar-val"
                attr:aria-describedby="desc"
                attr:title="tooltip text"
            />
        </div>
    }
}

#[component]
pub fn AttributeClonePage() -> impl IntoView {
    view! {
        <h1>"Experiment: Attribute Clone"</h1>

        <h2>"Static Attributes"</h2>
        <StaticAttrsFixture />

        <h2>"Reactive Attributes"</h2>
        <ReactiveAttrsFixture />

        <h2>"Multiple Attributes"</h2>
        <MultipleAttrsFixture />
    }
}
