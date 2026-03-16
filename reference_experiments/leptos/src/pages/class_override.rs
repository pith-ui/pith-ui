// ── Experiment: Class Override Order with {..} Spread ────────────────────────
//
// Hypothesis: `attr:class` uses `setAttribute("class", ...)` which replaces the
// entire class attribute, while `class:` directives use `classList.toggle()` which
// is additive. When both are on the same element, `attr:class` should clobber
// `class:` directives (same as `attr:style` clobbers `style:` directives).
//
// If `attr:class` does NOT clobber `class:`, then classes behave differently from
// styles and we can freely mix both approaches.

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

// ── Fixture 1: class: directive first, then spread with attr:class ───────────
//
// Internal component sets class:internal-class=true.
// User passes attr:class="user-class" which goes through the spread.
// Question: Does attr:class clobber class: directive?

#[component]
fn ClassDirectiveThenSpread() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="class-directive-then-spread"
                class:internal-class=true
                {..forwarded.spread()}
            >
                "class: directive first, then spread"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture1() -> impl IntoView {
    view! {
        <div data-testid="fixture-1">
            <ClassDirectiveThenSpread attr:class="user-class" />
        </div>
    }
}

// ── Fixture 2: spread first, then class: directive ───────────────────────────
//
// Spread is before class: directive. Does class: directive after spread survive
// attr:class clobber?

#[component]
fn SpreadThenClassDirective() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="spread-then-class-directive"
                {..forwarded.spread()}
                class:internal-class=true
            >
                "spread first, then class: directive"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture2() -> impl IntoView {
    view! {
        <div data-testid="fixture-2">
            <SpreadThenClassDirective attr:class="user-class" />
        </div>
    }
}

// ── Fixture 3: User passes class: directive (not attr:class) ─────────────────
//
// Internal sets class:internal-class=true.
// User passes class:user-class=true (a class: directive, not attr:class).
// Both should be additive since both use classList.toggle().

#[component]
fn ClassDirectiveBothSides() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="class-directive-both"
                class:internal-class=true
                {..forwarded.spread()}
            >
                "Both internal and user use class: directives"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture3() -> impl IntoView {
    view! {
        <div data-testid="fixture-3">
            <ClassDirectiveBothSides class:user-class=true />
        </div>
    }
}

// ── Fixture 4: Multiple class: directives + user attr:class ──────────────────
//
// Internal sets class:internal-a=true and class:internal-b=true.
// User passes attr:class="user-class".
// Question: Does attr:class clobber ALL class: directives?

#[component]
fn MultiClassDirectiveVsAttr() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="multi-class-vs-attr"
                class:internal-a=true
                class:internal-b=true
                {..forwarded.spread()}
            >
                "Multiple internal class: + user attr:class"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture4() -> impl IntoView {
    view! {
        <div data-testid="fixture-4">
            <MultiClassDirectiveVsAttr attr:class="user-class" />
        </div>
    }
}

// ── Fixture 5: Vanilla AttributeInterceptor (no ForwardedAttrs) ──────────────
//
// Direct {..attrs} spread + class: directives.

#[component]
fn VanillaInterceptorClass() -> impl IntoView {
    view! {
        <AttributeInterceptor let:attrs>
            <div
                data-testid="vanilla-interceptor-class"
                class:internal-class=true
                {..attrs}
            >
                "Vanilla interceptor + class: directive"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture5() -> impl IntoView {
    view! {
        <div data-testid="fixture-5">
            <VanillaInterceptorClass class:user-class=true />
        </div>
    }
}

// ── Fixture 6: Native Leptos spreading (no interceptor) ──────────────────────
//
// No AttributeInterceptor at all. Leptos applies user attrs to the
// top-level element via its native add_any_attr pipeline.

#[component]
fn NativeSpreadClass() -> impl IntoView {
    view! {
        <div
            data-testid="native-spread-class"
            class:internal-class=true
        >
            "No interceptor — native Leptos class spreading"
        </div>
    }
}

#[component]
fn Fixture6() -> impl IntoView {
    view! {
        <div data-testid="fixture-6">
            <NativeSpreadClass class:user-class=true />
        </div>
    }
}

// ── Fixture 7: No conflict (user passes non-class attr) ─────────────────────
//
// Internal sets class:internal-class=true.
// User passes only data-custom (no class attr at all).
// class: directive should be unaffected.

#[component]
fn NoClassConflict() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="no-class-conflict"
                class:internal-class=true
                {..forwarded.spread()}
            >
                "Internal class:, user passes non-class attr"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture7() -> impl IntoView {
    view! {
        <div data-testid="fixture-7">
            <NoClassConflict attr:data-custom="hello" />
        </div>
    }
}

// ── Fixture 8: class: directive after spread, user passes class: directive ───
//
// Internal sets class:internal-class=true AFTER spread.
// User passes class:user-class=true (additive).
// Both should coexist.

#[component]
fn ClassDirectiveAfterSpreadUserDirective() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="directive-after-spread-user-directive"
                {..forwarded.spread()}
                class:internal-class=true
            >
                "spread, then class: directive; user also uses class: directive"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture8() -> impl IntoView {
    view! {
        <div data-testid="fixture-8">
            <ClassDirectiveAfterSpreadUserDirective class:user-class=true />
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn ClassOverridePage() -> impl IntoView {
    view! {
        <h1>"Experiment: Class Override Order"</h1>

        <h2>"1. class: directive first, then spread (user attr:class)"</h2>
        <Fixture1 />

        <h2>"2. spread first, then class: directive (user attr:class)"</h2>
        <Fixture2 />

        <h2>"3. Both sides use class: directives (should be additive)"</h2>
        <Fixture3 />

        <h2>"4. Multiple internal class: + user attr:class (clobber all?)"</h2>
        <Fixture4 />

        <h2>"5. Vanilla interceptor + class: directives"</h2>
        <Fixture5 />

        <h2>"6. Native Leptos spreading (no interceptor)"</h2>
        <Fixture6 />

        <h2>"7. No conflict (user passes non-class attr)"</h2>
        <Fixture7 />

        <h2>"8. class: after spread, user also class: (both additive)"</h2>
        <Fixture8 />
    }
}
