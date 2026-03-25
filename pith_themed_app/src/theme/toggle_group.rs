use pith_ui::toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupType};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york toggle-group
// ---------------------------------------------------------------------------

const GROUP_CLASS: &str = "group/toggle-group flex w-fit items-center rounded-md";

const ITEM_CLASS: &str = "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium whitespace-nowrap transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:focus-ring disabled:disabled-base data-[state=on]:bg-accent data-[state=on]:text-accent-foreground bg-transparent h-9 min-w-9 px-2 focus:z-10 focus-visible:z-10";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedToggleGroup(
    r#type: ToggleGroupType,
    #[prop(into, optional)] value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<String>>>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(GROUP_CLASS);

    let forward_cb = Callback::new(move |val: Vec<String>| {
        if let Some(cb) = on_value_change {
            cb.run(val);
        }
    });

    view! {
        <ToggleGroup
            attr:class=class.get_value()
            r#type=r#type
            value=value
            default_value=default_value
            on_value_change=forward_cb
            disabled=disabled
        >
            {children()}
        </ToggleGroup>
    }
}

#[component]
pub fn ThemedToggleGroupItem(
    #[prop(into)] value: Signal<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);

    view! {
        <ToggleGroupItem attr:class=class.get_value() value=value disabled=disabled>
            {children()}
        </ToggleGroupItem>
    }
}
