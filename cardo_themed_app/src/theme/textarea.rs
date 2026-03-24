use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york textarea
// ---------------------------------------------------------------------------

const TEXTAREA_CLASS: &str = "flex min-h-16 w-full rounded-md border border-input bg-transparent px-3 py-2 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30 focus-visible:focus-ring";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedTextarea(
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] disabled: Option<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] rows: Option<i32>,
) -> impl IntoView {
    view! {
        <textarea
            class=TEXTAREA_CLASS
            placeholder=placeholder
            disabled=disabled.unwrap_or(false)
            name=name
            rows=rows
        >
            {value}
        </textarea>
    }
}
