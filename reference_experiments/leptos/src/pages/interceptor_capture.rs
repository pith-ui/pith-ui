// ── Experiment: What Does AttributeInterceptor Actually Capture? ─────────────
//
// The claim is that AttributeInterceptor only captures attrs set explicitly
// in the view! macro, NOT attrs from parent add_any_attr propagation.
// Let's test this claim.
//
// If the claim is WRONG (interceptor captures everything), we can eliminate
// the transfer Effect entirely by redirecting all attrs to the inner element
// via ForwardedAttrs.spread().

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

// ── Fixture 1: Does AttributeInterceptor capture parent add_any_attr? ───────
//
// InnerComponent has an AttributeInterceptor at the top and spreads captured
// attrs onto the inner div. OuterComponent passes attrs to InnerComponent
// via the normal Leptos mechanism (attr: on the component tag).

#[component]
fn InnerComponent() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div data-testid="wrapper" data-wrapper="true">
                <div data-testid="inner" {..forwarded.spread()}>
                    "Inner element"
                </div>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture1() -> impl IntoView {
    view! {
        <div data-testid="fixture-1">
            <InnerComponent
                attr:class="from-parent"
                attr:data-custom="parent-value"
                attr:style="--my-var: parent-set"
            />
        </div>
    }
}

// ── Fixture 2: Same but with style: directive from parent ───────────────────

#[component]
fn Fixture2() -> impl IntoView {
    view! {
        <div data-testid="fixture-2">
            <InnerComponent
                style:--my-var="parent-set-via-directive"
                attr:data-custom="parent-value"
            />
        </div>
    }
}

// ── Fixture 3: Two-level nesting (simulates DropdownMenu → Menu → Popper) ──
//
// OuterWrapper passes attrs to MiddleWrapper, which passes them to
// InnerComponent. Tests whether attrs survive multiple component boundaries.

#[component]
fn MiddleWrapper(children: ChildrenFn) -> impl IntoView {
    // MiddleWrapper is transparent — just renders children.
    // Attrs passed to MiddleWrapper should flow through to its child.
    view! {
        {children()}
    }
}

#[component]
fn Fixture3() -> impl IntoView {
    view! {
        <div data-testid="fixture-3">
            <MiddleWrapper
                attr:class="from-outer"
                attr:data-custom="outer-value"
            >
                <InnerComponent />
            </MiddleWrapper>
        </div>
    }
}

// ── Fixture 4: InnerComponent explicitly sets attrs + parent also sets ───────
//
// Tests whether internal attrs (from view! macro) and parent attrs
// (from add_any_attr) are BOTH captured by the interceptor.

#[component]
fn InnerWithOwnAttrs() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div data-testid="wrapper-4" data-wrapper="true">
                <div
                    data-testid="inner-4"
                    data-internal="set-in-view-macro"
                    {..forwarded.spread()}
                >
                    "Inner with own attrs"
                </div>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture4() -> impl IntoView {
    view! {
        <div data-testid="fixture-4">
            <InnerWithOwnAttrs
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
            />
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn InterceptorCapturePage() -> impl IntoView {
    view! {
        <h1>"Experiment: AttributeInterceptor Capture Scope"</h1>

        <h2>"1. Does interceptor capture parent add_any_attr attrs?"</h2>
        <Fixture1 />

        <h2>"2. Does interceptor capture parent style: directives?"</h2>
        <Fixture2 />

        <h2>"3. Two-level nesting (outer → middle → inner)"</h2>
        <Fixture3 />

        <h2>"4. Both internal and parent attrs captured?"</h2>
        <Fixture4 />
    }
}
