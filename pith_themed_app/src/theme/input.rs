use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york input
// ---------------------------------------------------------------------------

const INPUT_CLASS: &str = "h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none placeholder:text-muted-foreground disabled:pointer-events-none disabled:disabled-cursor md:text-sm dark:bg-input/30 focus-visible:focus-ring";

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedInput(
    #[prop(into, optional)] r#type: Option<String>,
    #[prop(into, optional)] placeholder: Option<String>,
    #[prop(into, optional)] value: Option<String>,
    #[prop(into, optional)] disabled: Option<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] class: Option<String>,
) -> impl IntoView {
    let combined_class = match class {
        Some(extra) => format!("{INPUT_CLASS} {extra}"),
        None => INPUT_CLASS.to_string(),
    };

    view! {
        <input
            class=combined_class
            r#type=r#type.unwrap_or_else(|| "text".to_string())
            placeholder=placeholder
            value=value
            disabled=disabled.unwrap_or(false)
            name=name
        />
    }
}
