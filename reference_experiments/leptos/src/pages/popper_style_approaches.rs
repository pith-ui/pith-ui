// ── Experiment: PopperContent-like Style Approaches ──────────────────────────
//
// Tests different approaches for getting component-specific CSS var aliases
// onto the inner element of a two-element (wrapper + inner) component,
// without using Effects to transfer styles post-mount.
//
// The goal: wrapper components (like DropdownMenuContent) need to set CSS var
// aliases on the inner Primitive element inside PopperContent. These must be
// non-overridable by user attrs and SSR-compatible.
//
// Approach A: Props-based — pass style pairs as a prop, apply as style: directives
// Approach B: Context-based — provide styles via context, read in inner component

use leptos::context::Provider;
use leptos::prelude::*;

// ── Approach A: Props-based forced styles ───────────────────────────────────
//
// The wrapper component passes forced_styles as a prop. The inner component
// iterates them and applies via an Effect (since we can't dynamically create
// style: directives from a Vec). BUT: we can apply them as a single attr:style
// string on the inner element if we're careful about ordering.
//
// Actually, the cleanest approach: the inner component renders style: directives
// for well-known props, and the wrapper passes them as named props.

/// Simulates PopperContent with a `content_style` prop that gets applied
/// directly to the inner element as `attr:style`.
#[component]
fn PopperLikeA(
    /// CSS var aliases to apply on the inner element (non-overridable).
    #[prop(into, optional)]
    inner_style: Option<String>,
    #[prop(into, optional)] node_ref: leptos_node_ref::AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    // Wrapper div: positioning only. User attrs land here via add_any_attr.
    // Inner div: content + inner_style. User attrs DON'T reach here.
    let inner_style = inner_style.unwrap_or_default();

    view! {
        <div style="position: relative" data-wrapper="true">
            <div
                data-testid="approach-a-inner"
                node_ref=node_ref
                style=inner_style
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
fn ApproachAConsumer() -> impl IntoView {
    let inner_styles = "\
        --radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin); \
        --radix-dropdown-menu-content-available-width: var(--radix-popper-available-width);";

    view! {
        <PopperLikeA
            inner_style=inner_styles
            attr:style="--radix-dropdown-menu-content-transform-origin: user-override"
            attr:data-custom="test"
        >
            "Approach A: inner_style prop"
        </PopperLikeA>
    }
}

// ── Approach B: Context-based forced styles ─────────────────────────────────
//
// The wrapper component provides styles via context. The inner element
// reads them and applies. This decouples the wrapper from the inner element.

#[derive(Clone, Default)]
struct InnerStyleContext {
    styles: Vec<(String, String)>,
}

#[component]
fn PopperLikeB(
    #[prop(into, optional)] node_ref: leptos_node_ref::AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    // Read context set by consumer wrapper
    let ctx = use_context::<InnerStyleContext>().unwrap_or_default();

    // Build style string from context
    let style_str = ctx.styles.iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("; ");

    view! {
        <div style="position: relative" data-wrapper="true">
            <div
                data-testid="approach-b-inner"
                node_ref=node_ref
                style=style_str
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
fn ApproachBConsumer() -> impl IntoView {
    let ctx = InnerStyleContext {
        styles: vec![
            ("--radix-dropdown-menu-content-transform-origin".into(), "var(--radix-popper-transform-origin)".into()),
            ("--radix-dropdown-menu-content-available-width".into(), "var(--radix-popper-available-width)".into()),
        ],
    };

    view! {
        <Provider value=ctx>
            <PopperLikeB
                attr:style="--radix-dropdown-menu-content-transform-origin: user-override"
                attr:data-custom="test"
            >
                "Approach B: context-based styles"
            </PopperLikeB>
        </Provider>
    }
}

// ── Approach C: attr:style on inner + user attrs on wrapper (current) ───────
//
// Control: this is what we do today with content_style. The attr:style goes
// on the outer component and lands on the wrapper via add_any_attr. Then the
// transfer Effect moves it. Here we simulate WITHOUT the transfer.

#[component]
fn PopperLikeC(
    #[prop(into, optional)] node_ref: leptos_node_ref::AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    // No transfer Effect — user attrs stay on wrapper, nothing reaches inner
    view! {
        <div style="position: relative" data-wrapper="true">
            <div
                data-testid="approach-c-inner"
                node_ref=node_ref
            >
                {children()}
            </div>
        </div>
    }
}

#[component]
fn ApproachCConsumer() -> impl IntoView {
    view! {
        <PopperLikeC
            attr:style="--radix-dropdown-menu-content-transform-origin: var(--radix-popper-transform-origin)"
            attr:data-custom="test"
        >
            "Approach C: attr:style on wrapper (no transfer)"
        </PopperLikeC>
    }
}

// ── Page ────────────────────────────────────────────────────────────────────

#[component]
pub fn PopperStyleApproachesPage() -> impl IntoView {
    view! {
        <h1>"Experiment: PopperContent Style Approaches"</h1>

        <h2>"A: inner_style prop (directly on inner element)"</h2>
        <p>"CSS vars set on inner via prop. User attr:style lands on wrapper. Non-overridable."</p>
        <ApproachAConsumer />

        <h2>"B: Context-based styles (read in inner component)"</h2>
        <p>"CSS vars provided via context, rendered on inner. User attr:style on wrapper."</p>
        <ApproachBConsumer />

        <h2>"C: Control — attr:style on wrapper (no transfer)"</h2>
        <p>"Current approach without transfer Effect. Styles stay on wrapper."</p>
        <ApproachCConsumer />
    }
}
