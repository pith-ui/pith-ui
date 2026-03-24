use cardo_ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york tabs
// ---------------------------------------------------------------------------

const TABS_LIST_CLASS: &str = "inline-flex h-9 w-fit items-center justify-center rounded-lg bg-muted p-[3px] text-muted-foreground";

const TABS_TRIGGER_CLASS: &str = "inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-2 py-1 text-sm font-medium whitespace-nowrap transition-all hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm";

const TABS_CONTENT_CLASS: &str = "flex-1 outline-none";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedTabs(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Tabs
            value=value
            default_value=default_value
            on_value_change=move |val: String| {
                if let Some(cb) = on_value_change {
                    cb.run(val);
                }
            }
        >
            {children()}
        </Tabs>
    }
}

#[component]
pub fn ThemedTabsList(children: ChildrenFn) -> impl IntoView {
    view! {
        <TabsList attr:class=TABS_LIST_CLASS>
            {children()}
        </TabsList>
    }
}

#[component]
pub fn ThemedTabsTrigger(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(TABS_TRIGGER_CLASS);

    view! {
        <TabsTrigger attr:class=class.get_value() value=value disabled=disabled>
            {children()}
        </TabsTrigger>
    }
}

#[component]
pub fn ThemedTabsContent(
    #[prop(into)] value: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(TABS_CONTENT_CLASS);

    view! {
        <TabsContent attr:class=class.get_value() value=value>
            {children()}
        </TabsContent>
    }
}
