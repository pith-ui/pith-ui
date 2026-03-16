// ── Experiment: ForwardedAttrs — Reactive Attribute Forwarding ───────────────
//
// Tests ForwardedAttrs as a replacement for the extract_attrs + StoredValue +
// Effect workaround used in MenuContent. ForwardedAttrs should:
//
// 1. Apply static attrs to elements inside Show (basic survival)
// 2. Preserve reactivity for signal-driven attrs (the key improvement)
// 3. Support multiple targets with independent reactive subscriptions
// 4. Survive multiple Show cycles without leaking subscriptions

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

// ── Fixture 1: Static Attrs + Internal Show ─────────────────────────────────

#[component]
fn StaticInternalShow() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let forwarded = ForwardedAttrs::new();
    let target_ref = forwarded.target();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="static-target" node_ref=target_ref>
                        "I am the target"
                    </div>
                </Show>
                <button data-testid="static-toggle" on:click=move |_| set_visible.update(|v| *v = !*v)>
                    {move || if visible.get() { "Hide" } else { "Show" }}
                </button>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn StaticInternalShowFixture() -> impl IntoView {
    view! {
        <div data-testid="static-fixture">
            <StaticInternalShow
                attr:class="test-class"
                attr:data-custom="test-value"
                attr:aria-label="test-label"
            />
        </div>
    }
}

// ── Fixture 2: Reactive Attrs + Internal Show ───────────────────────────────

#[component]
fn ReactiveInternalShow() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let forwarded = ForwardedAttrs::new();
    let target_ref = forwarded.target();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="reactive-target" node_ref=target_ref>
                        "I am the target"
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
fn ReactiveInternalShowFixture() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div data-testid="reactive-fixture">
            <ReactiveInternalShow attr:data-count=move || count.get().to_string() />
            <button data-testid="reactive-increment" on:click=move |_| set_count.update(|c| *c += 1)>
                "Increment"
            </button>
        </div>
    }
}

// ── Fixture 3: Multi-Target ─────────────────────────────────────────────────

#[component]
fn MultiTarget() -> impl IntoView {
    let (visible, set_visible) = signal(true);
    let forwarded = ForwardedAttrs::new();
    let ref_a = forwarded.target();
    let ref_b = forwarded.target();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                <Show when=move || visible.get()>
                    <div data-testid="multi-target-a" node_ref=ref_a>
                        "Target A"
                    </div>
                    <div data-testid="multi-target-b" node_ref=ref_b>
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
fn MultiTargetFixture() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div data-testid="multi-fixture">
            <MultiTarget
                attr:class="shared-class"
                attr:data-count=move || count.get().to_string()
            />
            <button data-testid="multi-increment" on:click=move |_| set_count.update(|c| *c += 1)>
                "Increment"
            </button>
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn ForwardedAttrsPage() -> impl IntoView {
    view! {
        <h1>"Experiment: ForwardedAttrs"</h1>

        <h2>"1. Static Attrs + Internal Show"</h2>
        <p>"Static attrs applied via ForwardedAttrs to element inside Show."</p>
        <StaticInternalShowFixture />

        <h2>"2. Reactive Attrs + Internal Show"</h2>
        <p>"Signal-driven attrs should maintain reactivity through Show cycling."</p>
        <ReactiveInternalShowFixture />

        <h2>"3. Multi-Target + Reactive Attrs"</h2>
        <p>"Same attrs applied to two elements, both reactive, both inside Show."</p>
        <MultiTargetFixture />
    }
}
