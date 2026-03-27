// ── Experiment: Style Override Order with {..} Spread ────────────────────────
//
// Hypothesis: When a component sets `style:` directives on a Primitive and the
// user spreads attrs that include a `style` attribute via `{..forwarded.spread()}`,
// the spread's `attr:style` should override individual `style:` properties because
// `attr:style` sets the entire inline style string while `style:` uses setProperty.
//
// If spread attrs override internal styles, we can safely set internal CSS vars
// via `style:` directives and let user spreads win when they conflict.
// If they don't override, the internal styles would always win.

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;
use leptos::{html, tachys::html::element::ElementType};

// ── Fixture 1: style: directive vs attr:style spread ────────────────────────
//
// Internal component sets style:--my-var="internal-value".
// User passes attr:style="--my-var: user-value" which goes through the spread.

#[component]
fn InternalStyleComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="style-directive-target"
                style:--my-var="internal-value"
                {..forwarded.spread()}
            >
                "style: directive first, then spread"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn SpreadAfterStyleDirective() -> impl IntoView {
    view! {
        <div data-testid="fixture-1">
            <InternalStyleComponent attr:style="--my-var: user-value" />
        </div>
    }
}

// ── Fixture 2: spread first, then style: directive ──────────────────────────

#[component]
fn SpreadFirstComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="spread-first-target"
                {..forwarded.spread()}
                style:--my-var="internal-value"
            >
                "spread first, then style: directive"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn SpreadFirstFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-2">
            <SpreadFirstComponent attr:style="--my-var: user-value" />
        </div>
    }
}

// ── Fixture 3: Multiple style: directives + spread with conflicting prop ────

#[component]
fn MultiStyleComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="multi-style-target"
                style:--internal-a="value-a"
                style:--internal-b="value-b"
                {..forwarded.spread()}
            >
                "Two internal vars, user overrides one"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn MultiStyleFixture() -> impl IntoView {
    // User only overrides --internal-a, leaves --internal-b untouched
    view! {
        <div data-testid="fixture-3">
            <MultiStyleComponent attr:style="--internal-a: user-override" />
        </div>
    }
}

// ── Fixture 4: No user style conflict (spread has no style attr) ────────────

#[component]
fn NoConflictComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="no-conflict-target"
                style:--internal-var="internal-only"
                {..forwarded.spread()}
            >
                "Internal var, user has no style attr"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn NoConflictFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-4">
            <NoConflictComponent attr:data-custom="hello" />
        </div>
    }
}

// ── Fixture 5: Accordion-like pattern ───────────────────────────────────────
//
// Simulates the accordion case: internal style: sets a CSS var alias,
// user can override it.

#[component]
fn AccordionLikeComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="accordion-like-target"
                style:--accordion-content-height="var(--collapsible-content-height)"
                style:--collapsible-content-height="42px"
                {..forwarded.spread()}
            >
                "Accordion-like CSS var alias pattern"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn AccordionLikeFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-5a">
            <h3>"No user override"</h3>
            <AccordionLikeComponent />
        </div>
        <div data-testid="fixture-5b">
            <h3>"User overrides height"</h3>
            <AccordionLikeComponent attr:style="--accordion-content-height: 999px" />
        </div>
    }
}

// ── Fixture 6: User style: directive (not attr:style) via ForwardedAttrs ─────
//
// Fixture 3 used attr:style="..." which clobbers. What if the user passes
// style:--internal-a="user-override" instead? ForwardedAttrs type-erases
// to AnyAttribute, so it may still end up as attr:style.

#[component]
fn StyleDirectiveOverrideComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="style-directive-override-target"
                style:--internal-a="value-a"
                style:--internal-b="value-b"
                {..forwarded.spread()}
            >
                "Internal style: directives + user style: directive via ForwardedAttrs"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn StyleDirectiveOverrideFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-6">
            <StyleDirectiveOverrideComponent style:--internal-a="user-override" />
        </div>
    }
}

// ── Fixture 7: Vanilla AttributeInterceptor + direct {..attrs} spread ───────
//
// No ForwardedAttrs — just the raw AttributeInterceptor let:attrs with
// {..attrs} spread. Tests whether type erasure from into_any_attr()
// causes the same clobbering.

#[component]
fn VanillaInterceptorComponent() -> impl IntoView {
    view! {
        <AttributeInterceptor let:attrs>
            <div
                data-testid="vanilla-interceptor-target"
                style:--internal-a="value-a"
                style:--internal-b="value-b"
                {..attrs}
            >
                "Vanilla AttributeInterceptor + direct spread"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn VanillaInterceptorFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-7">
            <VanillaInterceptorComponent style:--internal-a="user-override" />
        </div>
    }
}

// ── Fixture 8: No interceptor — let Leptos spread naturally ─────────────────
//
// No AttributeInterceptor at all. Leptos applies user attrs to the
// top-level element via its native add_any_attr pipeline. Tests whether
// the native pipeline preserves style: directives alongside user style:.

#[component]
fn NativeSpreadComponent() -> impl IntoView {
    view! {
        <div
            data-testid="native-spread-target"
            style:--internal-a="value-a"
            style:--internal-b="value-b"
        >
            "No interceptor — native Leptos attr spreading"
        </div>
    }
}

#[component]
fn NativeSpreadFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-8">
            <NativeSpreadComponent style:--internal-a="user-override" />
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn StyleOverridePage() -> impl IntoView {
    view! {
        <h1>"Experiment: Style Override Order"</h1>

        <h2>"1. style: directive, then spread (user overrides)"</h2>
        <SpreadAfterStyleDirective />

        <h2>"2. spread first, then style: directive"</h2>
        <SpreadFirstFixture />

        <h2>"3. Multiple internal vars, user overrides one (attr:style clobbers)"</h2>
        <MultiStyleFixture />

        <h2>"4. No conflict (user has no style attr)"</h2>
        <NoConflictFixture />

        <h2>"5. Accordion-like CSS var alias pattern"</h2>
        <AccordionLikeFixture />

        <h2>"6. User style: directive (not attr:style) via ForwardedAttrs"</h2>
        <StyleDirectiveOverrideFixture />

        <h2>"7. Vanilla AttributeInterceptor + direct spread"</h2>
        <VanillaInterceptorFixture />

        <h2>"8. No interceptor — native Leptos spreading"</h2>
        <NativeSpreadFixture />
    }
}
