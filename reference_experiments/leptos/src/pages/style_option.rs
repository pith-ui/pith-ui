// ── Experiment: style: Directives with Option Values ─────────────────────────
//
// Hypothesis: When a `style:` directive receives `None`, Leptos should either
// not set the property or call `removeProperty()` to clear it. When it receives
// `Some("value")`, it should call `setProperty()` with that value.
//
// This is critical for Category 3 conversions where CSS property NAMES are
// dynamic (e.g., `left` vs `right` based on direction). If `style:left=None`
// properly removes/omits the property, we can set BOTH directions and
// conditionally activate one:
//
//   style:left=move || (dir == Ltr).then_some("0")
//   style:right=move || (dir == Rtl).then_some("0")

use crate::support::use_forwarded_attrs::ForwardedAttrs;
use leptos::attribute_interceptor::AttributeInterceptor;
use leptos::prelude::*;

// ── Fixture 1: Static Some vs None ──────────────────────────────────────────
//
// One element with style:left=Some("10px"), another with style:left=None.

#[component]
fn Fixture1() -> impl IntoView {
    let no_left: Option<&str> = None;

    view! {
        <div data-testid="fixture-1">
            <div data-testid="with-some" style:left="10px" style:position="absolute">
                "style:left = Some(10px)"
            </div>
            <div data-testid="with-none" style:left=no_left style:position="absolute">
                "style:left = None"
            </div>
        </div>
    }
}

// ── Fixture 2: Reactive toggle between Some and None ─────────────────────────
//
// Toggle a signal that switches style:left between Some("20px") and None.
// When None, the property should be absent from the element's inline style.

#[component]
fn Fixture2() -> impl IntoView {
    let (active, set_active) = signal(true);

    let left_value = move || active.get().then_some("20px");

    view! {
        <div data-testid="fixture-2">
            <button on:click=move |_| set_active.update(|v| *v = !*v)>
                "toggle left"
            </button>
            <div
                data-testid="reactive-toggle"
                style:position="absolute"
                style:left=left_value
            >
                "Reactive style:left toggle"
            </div>
        </div>
    }
}

// ── Fixture 3: Bidirectional (LTR/RTL) pattern ──────────────────────────────
//
// Simulates scroll_area_corner: position absolute, bottom: 0, and EITHER
// right: 0 (LTR) or left: 0 (RTL). Both properties are set, but only one
// is active via Option.

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    Ltr,
    Rtl,
}

#[component]
fn Fixture3() -> impl IntoView {
    let (dir, set_dir) = signal(Dir::Ltr);

    view! {
        <div data-testid="fixture-3" style="position: relative; width: 200px; height: 100px; border: 1px solid black;">
            <button on:click=move |_| set_dir.update(|d| *d = if *d == Dir::Ltr { Dir::Rtl } else { Dir::Ltr })>
                "toggle dir"
            </button>
            <span data-testid="dir-value">{move || if dir.get() == Dir::Ltr { "ltr" } else { "rtl" }}</span>
            <div
                data-testid="corner"
                style:position="absolute"
                style:bottom="0"
                style:width="20px"
                style:height="20px"
                style:background="tomato"
                style:right=move || (dir.get() == Dir::Ltr).then_some("0")
                style:left=move || (dir.get() == Dir::Rtl).then_some("0")
            >
                "corner"
            </div>
        </div>
    }
}

// ── Fixture 4: Slider-like pattern (orientation-dependent edges) ─────────────
//
// Simulates slider range: horizontal uses left/right, vertical uses top/bottom.
// All four edges are set, but only two are active based on orientation.

#[derive(Clone, Copy, PartialEq)]
enum Orient {
    Horizontal,
    Vertical,
}

#[component]
fn Fixture4() -> impl IntoView {
    let (orient, set_orient) = signal(Orient::Horizontal);
    let start = 20.0_f64;
    let end = 30.0_f64;

    view! {
        <div data-testid="fixture-4" style="position: relative; width: 200px; height: 200px; border: 1px solid black;">
            <button on:click=move |_| set_orient.update(|o| *o = if *o == Orient::Horizontal { Orient::Vertical } else { Orient::Horizontal })>
                "toggle orientation"
            </button>
            <span data-testid="orient-value">{move || if orient.get() == Orient::Horizontal { "horizontal" } else { "vertical" }}</span>
            <div
                data-testid="range"
                style:position="absolute"
                style:background="cornflowerblue"
                style:left=move || (orient.get() == Orient::Horizontal).then(|| format!("{}%", start))
                style:right=move || (orient.get() == Orient::Horizontal).then(|| format!("{}%", end))
                style:top=move || (orient.get() == Orient::Vertical).then(|| format!("{}%", end))
                style:bottom=move || (orient.get() == Orient::Vertical).then(|| format!("{}%", start))
                style:width=move || (orient.get() == Orient::Vertical).then_some("100%")
                style:height=move || (orient.get() == Orient::Horizontal).then_some("100%")
            >
                "range"
            </div>
        </div>
    }
}

// ── Fixture 5: Option style through ForwardedAttrs ──────────────────────────
//
// Test that Option-based style: directives survive AttributeInterceptor +
// ForwardedAttrs.spread().

#[component]
fn InnerWithSpread() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="spread-target"
                {..forwarded.spread()}
            >
                "Inner with spread"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture5() -> impl IntoView {
    let (active, set_active) = signal(true);

    view! {
        <div data-testid="fixture-5">
            <button on:click=move |_| set_active.update(|v| *v = !*v)>
                "toggle"
            </button>
            <InnerWithSpread
                style:left=move || active.get().then_some("10px")
                style:right=move || (!active.get()).then_some("10px")
                style:position="absolute"
            />
        </div>
    }
}

// ── Fixture 6: Component receiving class: and style: directives ──────────────
//
// Tests the user's key insight: instead of a component accepting `class` and
// `content_style` as MaybeProp<String> props, callers should use class: and
// style: directives directly. These flow through AttributeInterceptor and
// land on the inner element alongside other attrs.

#[component]
fn MenuContentLike() -> impl IntoView {
    let forwarded = ForwardedAttrs::new();

    view! {
        <AttributeInterceptor let:attrs>
            {forwarded.set(attrs)}
            <div
                data-testid="menu-content-inner"
                style:outline="none"
                {..forwarded.spread()}
            >
                "Menu content (no class/style props)"
            </div>
        </AttributeInterceptor>
    }
}

#[component]
fn Fixture6() -> impl IntoView {
    let (animated, set_animated) = signal(false);

    view! {
        <div data-testid="fixture-6">
            <button on:click=move |_| set_animated.update(|v| *v = !*v)>
                "toggle animated"
            </button>
            <MenuContentLike
                class:menu-content=true
                class:menu-animated=animated
                style:--radix-menu-transform-origin="var(--radix-popper-transform-origin)"
                attr:data-state="open"
                attr:role="menu"
            />
        </div>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn StyleOptionPage() -> impl IntoView {
    view! {
        <h1>"Experiment: style: Directives with Option Values"</h1>

        <h2>"1. Static Some vs None"</h2>
        <Fixture1 />

        <h2>"2. Reactive toggle between Some and None"</h2>
        <Fixture2 />

        <h2>"3. Bidirectional (LTR/RTL) pattern"</h2>
        <Fixture3 />

        <h2>"4. Slider-like orientation-dependent edges"</h2>
        <Fixture4 />

        <h2>"5. Option style through ForwardedAttrs"</h2>
        <Fixture5 />

        <h2>"6. Component with class:/style: directives instead of string props"</h2>
        <Fixture6 />
    }
}
