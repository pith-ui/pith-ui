// Experiment: Generic Toggle Group
//
// Hypothesis: We can achieve type-safe single/multiple mode discrimination
// on a toggle group component using Rust generics and trait dispatch, while
// maintaining ergonomic JSX-like usage in Leptos view! macros.
//
// Approaches tested:
// 1. Trait-based mode with generic function — called via type alias wrappers
// 2. Split components with shared internals (current Accordion pattern)
// 3. Enum-dispatched value with From impls for ergonomics
//
// Goal: The consumer should NOT be able to pass Vec<String> callbacks when in
// single mode, or String callbacks when in multiple mode.

use leptos::{context::Provider, prelude::*};

// ─── Shared infrastructure ───────────────────────────────────────────────────

#[derive(Clone)]
struct ToggleGroupValueContext {
    value: Signal<Vec<String>>,
    on_item_activate: Callback<String>,
    on_item_deactivate: Callback<String>,
}

fn toggle_item(
    ctx: ToggleGroupValueContext,
    value: String,
    testid_prefix: &'static str,
    children: Children,
) -> impl IntoView {
    let item_value = StoredValue::new(value);
    let pressed = Signal::derive(move || {
        ctx.value.get().contains(&item_value.get_value())
    });

    view! {
        <button
            data-testid=format!("{}-item-{}", testid_prefix, item_value.get_value())
            data-state=move || if pressed.get() { "on" } else { "off" }
            aria-pressed=move || pressed.get().to_string()
            on:click=move |_| {
                if pressed.get_untracked() {
                    ctx.on_item_deactivate.run(item_value.get_value());
                } else {
                    ctx.on_item_activate.run(item_value.get_value());
                }
            }
        >
            {children()}
        </button>
    }
}

// ─── Approach 1: Trait + generic function + type alias wrappers ──────────────
//
// A generic function provides the core logic. Since Leptos RSX doesn't
// support turbofish, we provide thin #[component] wrappers that fix the
// type parameter. This gives us:
// - Type safety: String vs Vec<String> enforced at compile time
// - Ergonomics: <ToggleGroupSingle .../> and <ToggleGroupMultiple .../> in RSX
// - Code reuse: the generic function contains all the logic once

mod approach1 {
    use super::*;

    pub trait ToggleGroupMode: Send + 'static {
        type Value: Clone + Send + Sync + PartialEq + 'static;

        fn default_value() -> Self::Value;
        fn to_vec(value: &Self::Value) -> Vec<String>;
        fn on_activate(current: &Self::Value, item: &str) -> Self::Value;
        fn on_deactivate(current: &Self::Value, item: &str) -> Self::Value;
    }

    pub struct Single;
    pub struct Multiple;

    impl ToggleGroupMode for Single {
        type Value = String;

        fn default_value() -> String {
            String::new()
        }

        fn to_vec(value: &String) -> Vec<String> {
            if value.is_empty() { vec![] } else { vec![value.clone()] }
        }

        fn on_activate(_current: &String, item: &str) -> String {
            item.to_string()
        }

        fn on_deactivate(_current: &String, _item: &str) -> String {
            String::new()
        }
    }

    impl ToggleGroupMode for Multiple {
        type Value = Vec<String>;

        fn default_value() -> Vec<String> {
            vec![]
        }

        fn to_vec(value: &Vec<String>) -> Vec<String> {
            value.clone()
        }

        fn on_activate(current: &Vec<String>, item: &str) -> Vec<String> {
            let mut v = current.clone();
            v.push(item.to_string());
            v
        }

        fn on_deactivate(current: &Vec<String>, item: &str) -> Vec<String> {
            current.iter().filter(|v| v.as_str() != item).cloned().collect()
        }
    }

    /// Generic core — all logic lives here, parameterized by mode.
    fn toggle_group_core<M: ToggleGroupMode>(
        default_value: Option<M::Value>,
        on_value_change: Option<Callback<M::Value>>,
        children: ChildrenFn,
    ) -> impl IntoView {
        let default = default_value.unwrap_or_else(M::default_value);
        let (internal_value, set_internal_value) = signal(default);
        let current_value: Signal<M::Value> = internal_value.into();

        let value_as_vec = Signal::derive(move || M::to_vec(&current_value.get()));

        let on_activate = Callback::new(move |item: String| {
            let new_val = M::on_activate(&current_value.get_untracked(), &item);
            set_internal_value.set(new_val.clone());
            if let Some(cb) = on_value_change {
                cb.run(new_val);
            }
        });

        let on_deactivate = Callback::new(move |item: String| {
            let new_val = M::on_deactivate(&current_value.get_untracked(), &item);
            set_internal_value.set(new_val.clone());
            if let Some(cb) = on_value_change {
                cb.run(new_val);
            }
        });

        let ctx = ToggleGroupValueContext {
            value: value_as_vec,
            on_item_activate: on_activate,
            on_item_deactivate: on_deactivate,
        };

        view! {
            <Provider value=ctx>
                <div role="group" data-testid="approach1-group">
                    {children()}
                </div>
            </Provider>
        }
    }

    /// Single-mode wrapper — fixes M=Single, usable in RSX.
    #[component]
    pub fn ToggleGroupSingle(
        #[prop(into, optional)] default_value: Option<String>,
        #[prop(into, optional)] on_value_change: Option<Callback<String>>,
        children: ChildrenFn,
    ) -> impl IntoView {
        toggle_group_core::<Single>(default_value, on_value_change, children)
    }

    /// Multiple-mode wrapper — fixes M=Multiple, usable in RSX.
    #[component]
    pub fn ToggleGroupMultiple(
        #[prop(into, optional)] default_values: Option<Vec<String>>,
        #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
        children: ChildrenFn,
    ) -> impl IntoView {
        toggle_group_core::<Multiple>(default_values, on_values_change, children)
    }

    #[component]
    pub fn ToggleGroupItem(
        #[prop(into)] value: String,
        children: Children,
    ) -> impl IntoView {
        let ctx = expect_context::<ToggleGroupValueContext>();
        toggle_item(ctx, value, "approach1", children)
    }
}

// ─── Approach 2: Split components, no generic core ───────────────────────────
//
// The "just split them" approach. Each mode is its own component with
// no shared generic infrastructure. Simple, explicit, some duplication.

mod approach2 {
    use super::*;

    #[component]
    pub fn ToggleGroupSingle(
        #[prop(into, optional)] default_value: MaybeProp<String>,
        #[prop(into, optional)] on_value_change: Option<Callback<String>>,
        children: ChildrenFn,
    ) -> impl IntoView {
        let (internal, set_internal) = signal(
            default_value.get_untracked().unwrap_or_default(),
        );
        let current: Signal<String> = internal.into();

        let value_as_vec = Signal::derive(move || {
            let v = current.get();
            if v.is_empty() { vec![] } else { vec![v] }
        });

        let on_activate = Callback::new(move |item: String| {
            set_internal.set(item.clone());
            if let Some(cb) = on_value_change { cb.run(item); }
        });

        let on_deactivate = Callback::new(move |_item: String| {
            set_internal.set(String::new());
            if let Some(cb) = on_value_change { cb.run(String::new()); }
        });

        let ctx = ToggleGroupValueContext {
            value: value_as_vec,
            on_item_activate: on_activate,
            on_item_deactivate: on_deactivate,
        };

        view! {
            <Provider value=ctx>
                <div role="group" data-testid="approach2-group-single">
                    {children()}
                </div>
            </Provider>
        }
    }

    #[component]
    pub fn ToggleGroupMultiple(
        #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
        #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,
        children: ChildrenFn,
    ) -> impl IntoView {
        let (internal, set_internal) = signal(
            default_values.get_untracked().unwrap_or_default(),
        );
        let current: Signal<Vec<String>> = internal.into();

        let value_as_vec = Signal::derive(move || current.get());

        let on_activate = Callback::new(move |item: String| {
            let mut v = current.get_untracked();
            v.push(item);
            set_internal.set(v.clone());
            if let Some(cb) = on_values_change { cb.run(v); }
        });

        let on_deactivate = Callback::new(move |item: String| {
            let v: Vec<String> = current.get_untracked().into_iter().filter(|v| v != &item).collect();
            set_internal.set(v.clone());
            if let Some(cb) = on_values_change { cb.run(v); }
        });

        let ctx = ToggleGroupValueContext {
            value: value_as_vec,
            on_item_activate: on_activate,
            on_item_deactivate: on_deactivate,
        };

        view! {
            <Provider value=ctx>
                <div role="group" data-testid="approach2-group-multiple">
                    {children()}
                </div>
            </Provider>
        }
    }

    #[component]
    pub fn ToggleGroupItem(
        #[prop(into)] value: String,
        children: Children,
    ) -> impl IntoView {
        let ctx = expect_context::<ToggleGroupValueContext>();
        toggle_item(ctx, value, "approach2", children)
    }
}

// ─── Approach 3: Enum value with From impls ──────────────────────────────────
//
// Single component, but the value type is an enum. The mode is inferred
// from the default_value variant. From impls provide ergonomics.
// Downside: callback receives the enum, consumer must destructure.

mod approach3 {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    pub enum ToggleValue {
        Single(String),
        Multiple(Vec<String>),
    }

    impl From<&str> for ToggleValue {
        fn from(s: &str) -> Self { ToggleValue::Single(s.to_string()) }
    }

    impl From<String> for ToggleValue {
        fn from(s: String) -> Self { ToggleValue::Single(s) }
    }

    impl From<Vec<String>> for ToggleValue {
        fn from(v: Vec<String>) -> Self { ToggleValue::Multiple(v) }
    }

    impl ToggleValue {
        fn to_vec(&self) -> Vec<String> {
            match self {
                ToggleValue::Single(s) if s.is_empty() => vec![],
                ToggleValue::Single(s) => vec![s.clone()],
                ToggleValue::Multiple(v) => v.clone(),
            }
        }

        fn is_multiple(&self) -> bool {
            matches!(self, ToggleValue::Multiple(_))
        }
    }

    #[component]
    pub fn ToggleGroup(
        #[prop(into, optional)] default_value: Option<ToggleValue>,
        #[prop(into, optional)] on_value_change: Option<Callback<ToggleValue>>,
        children: ChildrenFn,
    ) -> impl IntoView {
        let default = default_value.unwrap_or(ToggleValue::Single(String::new()));
        let is_multiple = default.is_multiple();
        let (internal, set_internal) = signal(default);

        let value_as_vec = Signal::derive(move || internal.get().to_vec());

        let on_activate = Callback::new(move |item: String| {
            let new_val = if is_multiple {
                let mut v = internal.get_untracked().to_vec();
                v.push(item);
                ToggleValue::Multiple(v)
            } else {
                ToggleValue::Single(item)
            };
            set_internal.set(new_val.clone());
            if let Some(cb) = on_value_change { cb.run(new_val); }
        });

        let on_deactivate = Callback::new(move |item: String| {
            let new_val = if is_multiple {
                let v: Vec<String> = internal.get_untracked().to_vec().into_iter().filter(|v| v != &item).collect();
                ToggleValue::Multiple(v)
            } else {
                ToggleValue::Single(String::new())
            };
            set_internal.set(new_val.clone());
            if let Some(cb) = on_value_change { cb.run(new_val); }
        });

        let ctx = ToggleGroupValueContext {
            value: value_as_vec,
            on_item_activate: on_activate,
            on_item_deactivate: on_deactivate,
        };

        view! {
            <Provider value=ctx>
                <div role="group" data-testid="approach3-group">
                    {children()}
                </div>
            </Provider>
        }
    }

    #[component]
    pub fn ToggleGroupItem(
        #[prop(into)] value: String,
        children: Children,
    ) -> impl IntoView {
        let ctx = expect_context::<ToggleGroupValueContext>();
        toggle_item(ctx, value, "approach3", children)
    }
}

// ─── Demo page ───────────────────────────────────────────────────────────────

#[component]
pub fn GenericToggleGroupPage() -> impl IntoView {
    // Approach 1
    let (a1_single_value, set_a1_single) = signal(String::new());
    let (a1_multi_value, set_a1_multi) = signal(Vec::<String>::new());

    // Approach 2
    let (a2_single_value, set_a2_single) = signal(String::new());
    let (a2_multi_value, set_a2_multi) = signal(Vec::<String>::new());

    // Approach 3
    let (a3_single_value, set_a3_single) = signal(String::new());
    let (a3_multi_value, set_a3_multi) = signal(Vec::<String>::new());

    view! {
        <h1>"Generic Toggle Group Experiment"</h1>

        // ── Approach 1: Generic core + thin wrappers ──
        <h2>"Approach 1: Generic trait core with thin #[component] wrappers"</h2>
        <p>"Type safety via generics. Logic deduplicated. RSX-friendly wrappers."</p>

        <h3>"Single"</h3>
        <p data-testid="a1-single-output">"Value: " {move || a1_single_value.get()}</p>
        <approach1::ToggleGroupSingle on_value_change=Callback::new(move |v: String| set_a1_single.set(v))>
            <approach1::ToggleGroupItem value="a">"A"</approach1::ToggleGroupItem>
            <approach1::ToggleGroupItem value="b">"B"</approach1::ToggleGroupItem>
            <approach1::ToggleGroupItem value="c">"C"</approach1::ToggleGroupItem>
        </approach1::ToggleGroupSingle>

        <h3>"Multiple"</h3>
        <p data-testid="a1-multi-output">"Values: " {move || a1_multi_value.get().join(", ")}</p>
        <approach1::ToggleGroupMultiple on_values_change=Callback::new(move |v: Vec<String>| set_a1_multi.set(v))>
            <approach1::ToggleGroupItem value="x">"X"</approach1::ToggleGroupItem>
            <approach1::ToggleGroupItem value="y">"Y"</approach1::ToggleGroupItem>
            <approach1::ToggleGroupItem value="z">"Z"</approach1::ToggleGroupItem>
        </approach1::ToggleGroupMultiple>

        <hr />

        // ── Approach 2: Split components, no generics ──
        <h2>"Approach 2: Split components (Accordion pattern, no generics)"</h2>
        <p>"Same end result as approach 1 but without trait infrastructure. More duplication."</p>

        <h3>"Single"</h3>
        <p data-testid="a2-single-output">"Value: " {move || a2_single_value.get()}</p>
        <approach2::ToggleGroupSingle on_value_change=Callback::new(move |v: String| set_a2_single.set(v))>
            <approach2::ToggleGroupItem value="a">"A"</approach2::ToggleGroupItem>
            <approach2::ToggleGroupItem value="b">"B"</approach2::ToggleGroupItem>
            <approach2::ToggleGroupItem value="c">"C"</approach2::ToggleGroupItem>
        </approach2::ToggleGroupSingle>

        <h3>"Multiple"</h3>
        <p data-testid="a2-multi-output">"Values: " {move || a2_multi_value.get().join(", ")}</p>
        <approach2::ToggleGroupMultiple on_values_change=Callback::new(move |v: Vec<String>| set_a2_multi.set(v))>
            <approach2::ToggleGroupItem value="x">"X"</approach2::ToggleGroupItem>
            <approach2::ToggleGroupItem value="y">"Y"</approach2::ToggleGroupItem>
            <approach2::ToggleGroupItem value="z">"Z"</approach2::ToggleGroupItem>
        </approach2::ToggleGroupMultiple>

        <hr />

        // ── Approach 3: Enum value ──
        <h2>"Approach 3: Enum value (single component)"</h2>
        <p>"Single component, mode inferred from default_value. Consumer must destructure callback."</p>

        <h3>"Single"</h3>
        <p data-testid="a3-single-output">"Value: " {move || a3_single_value.get()}</p>
        <approach3::ToggleGroup
            default_value=approach3::ToggleValue::from("")
            on_value_change=Callback::new(move |v: approach3::ToggleValue| {
                if let approach3::ToggleValue::Single(s) = v { set_a3_single.set(s); }
            })
        >
            <approach3::ToggleGroupItem value="a">"A"</approach3::ToggleGroupItem>
            <approach3::ToggleGroupItem value="b">"B"</approach3::ToggleGroupItem>
            <approach3::ToggleGroupItem value="c">"C"</approach3::ToggleGroupItem>
        </approach3::ToggleGroup>

        <h3>"Multiple"</h3>
        <p data-testid="a3-multi-output">"Values: " {move || a3_multi_value.get().join(", ")}</p>
        <approach3::ToggleGroup
            default_value=approach3::ToggleValue::from(vec![])
            on_value_change=Callback::new(move |v: approach3::ToggleValue| {
                if let approach3::ToggleValue::Multiple(m) = v { set_a3_multi.set(m); }
            })
        >
            <approach3::ToggleGroupItem value="x">"X"</approach3::ToggleGroupItem>
            <approach3::ToggleGroupItem value="y">"Y"</approach3::ToggleGroupItem>
            <approach3::ToggleGroupItem value="z">"Z"</approach3::ToggleGroupItem>
        </approach3::ToggleGroup>
    }
}
