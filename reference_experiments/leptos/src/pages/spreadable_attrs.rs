// ── Experiment: Spreadable ForwardedAttrs ────────────────────────────────────
//
// Tests whether ForwardedAttrs.spread() can be used with {..} syntax to apply
// intercepted attrs to elements — including inside Show, multiple times, and
// with reactive values.

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

// ── Fixture 1: Basic spread outside Show ────────────────────────────────────

#[component]
fn BasicSpread() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div data-testid="basic-target" {..forwarded.spread()}>
                "Basic target"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn BasicSpreadFixture() -> impl IntoView {
    view! {
        <div data-testid="basic-fixture">
            <BasicSpread
                attr:class="test-class"
                attr:data-custom="test-value"
                attr:aria-label="test-label"
            />
        </div>
    }
}

// ── Fixture 2: Spread inside Show ───────────────────────────────────────────

#[component]
fn ShowSpread() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="show-target" {..forwarded.spread()}>
                        "Show target"
                    </div>
                </Show>
                <button data-testid="show-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn ShowSpreadFixture() -> impl IntoView {
    view! {
        <div data-testid="show-fixture">
            <ShowSpread
                attr:class="test-class"
                attr:data-custom="test-value"
                attr:aria-label="test-label"
            />
        </div>
    }
}

// ── Fixture 3: Multiple spreads ─────────────────────────────────────────────

#[component]
fn MultiSpread() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="multi-a" {..forwarded.spread()}>
                        "Target A"
                    </div>
                    <div data-testid="multi-b" {..forwarded.spread()}>
                        "Target B"
                    </div>
                </Show>
                <button data-testid="multi-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn MultiSpreadFixture() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div data-testid="multi-fixture">
            <MultiSpread
                attr:class="shared-class"
                attr:data-count=move || count.get().to_string()
            />
            <button data-testid="multi-increment" on:click=move |_| set_count.update(|c| *c += 1)>
                "Increment"
            </button>
        </div>
    }
}

// ── Fixture 4: Reactive attrs with Show cycling ─────────────────────────────

#[component]
fn ReactiveShowSpread() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="reactive-target" {..forwarded.spread()}>
                        "Reactive target"
                    </div>
                </Show>
                <button data-testid="reactive-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn ReactiveShowSpreadFixture() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div data-testid="reactive-fixture">
            <ReactiveShowSpread attr:data-count=move || count.get().to_string() />
            <button data-testid="reactive-increment" on:click=move |_| set_count.update(|c| *c += 1)>
                "Increment"
            </button>
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn SpreadableAttrsPage() -> impl IntoView {
    view! {
        <h1>"Experiment: Spreadable ForwardedAttrs"</h1>

        <h2>"1. Basic spread (outside Show)"</h2>
        <BasicSpreadFixture />

        <h2>"2. Spread inside Show"</h2>
        <ShowSpreadFixture />

        <h2>"3. Multiple spreads + reactive attrs"</h2>
        <MultiSpreadFixture />

        <h2>"4. Reactive attrs + Show cycling"</h2>
        <ReactiveShowSpreadFixture />
    }
}
