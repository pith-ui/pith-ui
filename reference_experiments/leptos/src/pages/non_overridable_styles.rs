// ── Experiment: Non-Overridable Internal Styles ─────────────────────────────
//
// Goal: Find an approach where internal CSS var aliases cannot be overridden
// by user attrs, matching React's {…props.style, …internal} pattern.
//
// Approaches tested:
// 1. style: directives on the INNER element (Primitive-level), after spread
// 2. style: directives on a wrapper component that renders the DOM element
// 3. content_style string applied via attr:style alongside user attr:style
// 4. Nested component: outer captures user attrs, inner sets style: directives

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::html;
use leptos::prelude::*;

// ── Helper: A "Primitive-like" component that renders a div ─────────────────

#[component]
fn InnerDiv(
    #[prop(into, optional)] node_ref: leptos_node_ref::AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref>
            {children()}
        </div>
    }
}

// ── Fixture 1: style: directives on the innermost element ───────────────────
//
// The component has a wrapper that captures user attrs, and an inner
// element where style: directives are set AFTER the user spread.

#[component]
fn InnerStyleDirectives() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div>
                // Inner div: spread user attrs first, then set internal style: directives
                <div
                    data-testid="inner-style-target"
                    {..forwarded.spread()}
                    style:--internal-var="internal-value"
                    style:--internal-only="only-internal"
                >
                    "style: directives after spread on same element"
                </div>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn InnerStyleDirectivesFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-1">
            <InnerStyleDirectives
                attr:style="--internal-var: user-override"
                attr:data-custom="test"
            />
        </div>
    }
}

// ── Fixture 2: style: on inner component, user attrs on outer ───────────────
//
// Outer component captures user attrs via ForwardedAttrs and spreads onto
// an inner component. The inner component sets style: directives on its
// own rendered element.

#[component]
fn OuterWrapper() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <InnerWithStyles testid="inner-attr-style-target" {..forwarded.spread()} />
        </AttributeInterceptor>
    }
}

#[component]
fn OuterWrapperB() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <InnerWithStyles testid="inner-style-dir-target" {..forwarded.spread()} />
        </AttributeInterceptor>
    }
}

#[component]
fn InnerWithStyles(#[prop(into)] testid: String) -> impl IntoView {
    // This component sets style: directives on its own DOM element.
    // User attrs arrive via add_any_attr (from the {..spread} on <InnerWithStyles>).
    view! {
        <div
            data-testid=testid
            style:--internal-var="internal-value"
            style:--internal-only="only-internal"
        >
            "Inner component with style: directives"
        </div>
    }
}

#[component]
fn InnerComponentFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-2">
            <OuterWrapper
                attr:style="--internal-var: user-override"
                attr:data-custom="test"
            />
        </div>
    }
}

// ── Fixture 3: Same as 2 but user uses style: directive ─────────────────────

#[component]
fn InnerComponentStyleDirectiveFixture() -> impl IntoView {
    view! {
        <div data-testid="fixture-3">
            <OuterWrapperB
                style:--internal-var="user-override"
                attr:data-custom="test"
            />
        </div>
    }
}

// ── Fixture 4: style: on a deeper nested Primitive-like element ─────────────
//
// Simulates the real PopperContent pattern: outer component captures user
// attrs, inner component renders a wrapper div + inner Primitive. Style
// directives go on the Primitive.

#[component]
fn PopperLikeOuter(#[prop(into)] testid: String) -> impl IntoView {
    let forwarded = ForwardedAttrs::new();
    let testid = StoredValue::new(testid);

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <PopperLikeInner {..forwarded.spread()} testid=testid.get_value() />
        </AttributeInterceptor>
    }
}

#[component]
fn PopperLikeInner(#[prop(into)] testid: String) -> impl IntoView {
    // Simulates PopperContent: wrapper div for positioning, inner div for content.
    // User attrs land on wrapper via add_any_attr, internal styles on inner.
    view! {
        <div style="position: relative">
            <div
                data-testid=testid
                style:--internal-var="internal-value"
                style:--internal-only="only-internal"
            >
                "Popper-like inner element with style: directives"
            </div>
        </div>
    }
}

#[component]
fn PopperLikeFixtureAttrStyle() -> impl IntoView {
    view! {
        <div data-testid="fixture-4a">
            <h4>"User passes attr:style"</h4>
            <PopperLikeOuter
                testid="popper-attr-style-target"
                attr:style="--internal-var: user-override"
                attr:data-custom="test"
            />
        </div>
    }
}

#[component]
fn PopperLikeFixtureStyleDirective() -> impl IntoView {
    view! {
        <div data-testid="fixture-4b">
            <h4>"User passes style: directive"</h4>
            <PopperLikeOuter
                testid="popper-style-dir-target"
                style:--internal-var="user-override"
                attr:data-custom="test"
            />
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn NonOverridableStylesPage() -> impl IntoView {
    view! {
        <h1>"Experiment: Non-Overridable Internal Styles"</h1>

        <h2>"1. style: directives after spread on same element"</h2>
        <InnerStyleDirectivesFixture />

        <h2>"2. Inner component sets style: (user attrs via add_any_attr)"</h2>
        <InnerComponentFixture />

        <h2>"3. Same as 2 but user uses style: directive"</h2>
        <InnerComponentStyleDirectiveFixture />

        <h2>"4a. Popper-like nested (user attr:style)"</h2>
        <PopperLikeFixtureAttrStyle />

        <h2>"4b. Popper-like nested (user style: directive)"</h2>
        <PopperLikeFixtureStyleDirective />
    }
}
