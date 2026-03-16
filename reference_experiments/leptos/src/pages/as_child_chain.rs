// ── Experiment: asChild Chain — Multi-Level AttributeInterceptor Forwarding ──
//
// Hypothesis: In the real Radix component chain, attrs pass through multiple
// "transparent" components (FocusScope → DismissableLayer → RovingFocusGroup →
// PopperContent), each with as_child=true. Each transparent component has its own
// AttributeInterceptor and adds its own attrs before forwarding to children.
//
// This experiment simulates that chain to understand:
// 1. Do attrs from the outermost parent reach the innermost element?
// 2. Does each layer's AttributeInterceptor capture BOTH parent attrs AND its
//    own attrs, or only its own?
// 3. What happens when intermediate layers add attrs (data-state, style:, etc.)?
// 4. Can we use ForwardedAttrs.spread() at each level to forward attrs down?
//
// The real chain in our codebase:
//   MenuContent → DismissableLayer (as_child) → FocusScope (as_child) →
//   RovingFocusGroup (as_child) → PopperContent → [wrapper div + inner Primitive]

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 1: Single transparent layer ──────────────────────────────────────
// ══════════════════════════════════════════════════════════════════════════════
//
// One component with AttributeInterceptor that captures parent attrs and
// spreads them onto its inner element, adding its own attrs too.

#[component]
fn SingleLayer() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="single-inner"
                data-layer="single"
                {..forwarded.spread()}
            >
                "Single layer inner"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture1() -> impl IntoView {
    view! {
        <div data-testid="fixture-1">
            <SingleLayer
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
            />
        </div>
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 2: Two transparent layers ────────────────────────────────────────
// ══════════════════════════════════════════════════════════════════════════════
//
// Outer → Inner, each with its own AttributeInterceptor.
// Outer captures parent attrs, adds data-layer="outer", forwards to Inner.
// Inner captures (outer's attrs + parent's attrs?), adds data-layer="inner",
// spreads onto the DOM element.

#[component]
fn TwoLayerInner() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="two-layer-inner"
                data-layer="inner"
                {..forwarded.spread()}
            >
                "Two-layer inner element"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn TwoLayerOuter() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <TwoLayerInner
                attr:data-layer-outer="was-here"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture2() -> impl IntoView {
    view! {
        <div data-testid="fixture-2">
            <TwoLayerOuter
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
            />
        </div>
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 3: Three transparent layers (simulates real chain) ───────────────
// ══════════════════════════════════════════════════════════════════════════════
//
// Simulates: FocusScope → DismissableLayer → RovingFocusGroup
// Each layer captures attrs, adds its own data attr, forwards to next.
// The innermost element should have ALL attrs from all layers + parent.

#[component]
fn ThreeLayerInnermost() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="three-layer-innermost"
                data-layer="innermost"
                {..forwarded.spread()}
            >
                "Three-layer innermost element"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn ThreeLayerMiddle() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <ThreeLayerInnermost
                attr:data-middle="was-here"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn ThreeLayerOuter() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <ThreeLayerMiddle
                attr:data-outer="was-here"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture3() -> impl IntoView {
    view! {
        <div data-testid="fixture-3">
            <ThreeLayerOuter
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
            />
        </div>
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 4: Chain with style: directives at each level ────────────────────
// ══════════════════════════════════════════════════════════════════════════════
//
// Each layer adds its own style: directive. Tests whether style: directives
// from multiple layers coexist on the final element.

#[component]
fn StyledInnermost() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="styled-innermost"
                style:--innermost-var="innermost-value"
                {..forwarded.spread()}
            >
                "Styled innermost"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn StyledMiddle() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <StyledInnermost
                style:--middle-var="middle-value"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn StyledOuter() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <StyledMiddle
                style:--outer-var="outer-value"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture4() -> impl IntoView {
    view! {
        <div data-testid="fixture-4">
            <StyledOuter
                style:--parent-var="parent-value"
            />
        </div>
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 5: Chain ending with wrapper + inner (PopperContent pattern) ─────
// ══════════════════════════════════════════════════════════════════════════════
//
// The real PopperContent has TWO elements: a wrapper div (positioning) and an
// inner Primitive (user content). This simulates the full chain ending at a
// two-element terminal component.
//
// Parent attrs should reach the inner Primitive, NOT the wrapper.
// Wrapper should have only positioning styles.

#[component]
fn PopperLikeTerminal() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="popper-wrapper"
                data-role="wrapper"
                style="position: fixed; top: 0; left: 0;"
            >
                <div
                    data-testid="popper-inner"
                    data-role="inner"
                    style:--popper-transform-origin="center"
                    {..forwarded.spread()}
                >
                    "PopperContent inner"
                </div>
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn DismissableLike() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <PopperLikeTerminal
                attr:data-dismissable="true"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn FocusScopeLike() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <DismissableLike
                attr:data-focus-scope="true"
                {..forwarded.spread()}
            />
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture5() -> impl IntoView {
    view! {
        <div data-testid="fixture-5">
            <FocusScopeLike
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
                style:--user-var="user-value"
            />
        </div>
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 6: Chain without ForwardedAttrs — using ChildrenFn passthrough ───
// ══════════════════════════════════════════════════════════════════════════════
//
// Alternative pattern: transparent layers use children instead of interceptor.
// Attrs applied to the outermost component flow via add_any_attr to the first
// DOM element Leptos encounters. Tests what happens with this approach.

#[component]
fn ChildrenPassthrough(children: ChildrenFn) -> impl IntoView {
    // Transparent — just renders children. Attrs on this component go to...
    // whatever Leptos decides is the "root element".
    view! {
        {children()}
    }
}

#[component]
fn Fixture6() -> impl IntoView {
    view! {
        <div data-testid="fixture-6">
            <ChildrenPassthrough
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
            >
                <div data-testid="children-inner" data-role="inner">
                    "Inner element via children passthrough"
                </div>
            </ChildrenPassthrough>
        </div>
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// ── Fixture 7: Mixed — some layers use interceptor, some use children ────────
// ══════════════════════════════════════════════════════════════════════════════
//
// Outer uses interceptor+forward, middle uses children passthrough,
// inner uses interceptor+forward.

#[component]
fn MixedInner() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="mixed-inner"
                data-layer="inner"
                {..forwarded.spread()}
            >
                "Mixed chain inner"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn MixedMiddlePassthrough(children: ChildrenFn) -> impl IntoView {
    view! {
        {children()}
    }
}

#[component]
fn MixedOuter() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <MixedMiddlePassthrough
                {..forwarded.spread()}
            >
                <MixedInner />
            </MixedMiddlePassthrough>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture7() -> impl IntoView {
    view! {
        <div data-testid="fixture-7">
            <MixedOuter
                attr:data-from-parent="parent-value"
                attr:class="parent-class"
            />
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn AsChildChainPage() -> impl IntoView {
    view! {
        <h1>"Experiment: asChild Chain — Multi-Level Attribute Forwarding"</h1>

        <h2>"1. Single transparent layer"</h2>
        <Fixture1 />

        <h2>"2. Two transparent layers"</h2>
        <Fixture2 />

        <h2>"3. Three transparent layers (simulates real chain)"</h2>
        <Fixture3 />

        <h2>"4. Chain with style: directives at each level"</h2>
        <Fixture4 />

        <h2>"5. Chain ending with wrapper + inner (PopperContent pattern)"</h2>
        <Fixture5 />

        <h2>"6. Children passthrough (no interceptor)"</h2>
        <Fixture6 />

        <h2>"7. Mixed — interceptor + children passthrough"</h2>
        <Fixture7 />
    }
}
