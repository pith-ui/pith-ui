pub use cardo_ui::toggle_group::ToggleGroupType;
use cardo_ui::toolbar::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york toolbar
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str =
    "inline-flex items-center gap-1 rounded-md border bg-background p-1 shadow-xs";

const SEPARATOR_CLASS: &str = "w-px mx-1 h-6 bg-border";

const LINK_CLASS: &str = "inline-flex items-center justify-center rounded-md px-2 py-1 text-sm font-medium hover:bg-accent hover:text-accent-foreground";

const TOGGLE_GROUP_CLASS: &str = "flex items-center gap-1";

const TOGGLE_ITEM_CLASS: &str = "inline-flex items-center justify-center rounded-md text-sm font-medium h-8 min-w-8 px-2 hover:bg-muted hover:text-muted-foreground data-[state=on]:bg-accent data-[state=on]:text-accent-foreground focus-visible:focus-ring disabled:disabled-base";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedToolbar(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(ROOT_CLASS);

    view! {
        <Toolbar attr:class=class.get_value()>
            {children()}
        </Toolbar>
    }
}

#[component]
pub fn ThemedToolbarSeparator() -> impl IntoView {
    let class = StoredValue::new(SEPARATOR_CLASS);

    view! {
        <ToolbarSeparator attr:class=class.get_value() />
    }
}

#[component]
pub fn ThemedToolbarButton(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <ToolbarButton as_child=true disabled=disabled>
            {children()}
        </ToolbarButton>
    }
}

#[component]
pub fn ThemedToolbarLink(
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(LINK_CLASS);

    view! {
        <ToolbarLink attr:class=class.get_value()>
            {children()}
        </ToolbarLink>
    }
}

#[component]
pub fn ThemedToolbarToggleGroup(
    r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(TOGGLE_GROUP_CLASS);

    let forward_cb = Callback::new(move |val: Vec<String>| {
        if let Some(cb) = on_value_change {
            cb.run(val);
        }
    });

    view! {
        <ToolbarToggleGroup
            attr:class=class.get_value()
            r#type=r#type
            value=value
            default_value=default_value
            on_value_change=forward_cb
            disabled=disabled
        >
            {children()}
        </ToolbarToggleGroup>
    }
}

#[component]
pub fn ThemedToolbarToggleItem(
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(TOGGLE_ITEM_CLASS);

    view! {
        <ToolbarToggleItem attr:class=class.get_value() value=value disabled=disabled>
            {children()}
        </ToolbarToggleItem>
    }
}
