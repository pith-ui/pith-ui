use pith_ui::toast::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york toast
// ---------------------------------------------------------------------------

const VIEWPORT_CLASS: &str = "fixed top-0 right-0 z-[100] flex max-h-screen w-full flex-col-reverse gap-2 p-4 sm:flex-col sm:max-w-[420px]";

const TOAST_CLASS: &str = "group pointer-events-auto relative flex w-full items-center justify-between gap-4 overflow-hidden rounded-md border bg-background p-4 shadow-lg transition-all";

const TITLE_CLASS: &str = "text-sm font-semibold";

const DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";

const CLOSE_CLASS: &str = "absolute top-2 right-2 rounded-md p-1 text-foreground/50 hover:text-foreground opacity-0 transition-opacity group-hover:opacity-100 focus-visible:focus-ring";

const ACTION_CLASS: &str = "inline-flex h-8 shrink-0 items-center justify-center rounded-md border border-border bg-transparent px-3 text-sm font-medium outline-none transition-colors hover:bg-secondary focus-visible:focus-ring disabled:disabled-base";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

/// Wraps `ToastProvider` and includes a styled `ToastViewport`.
/// Place this once near the root of your app.
#[component]
pub fn ThemedToastProvider(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(VIEWPORT_CLASS);

    view! {
        <ToastProvider>
            {children()}
            <ToastViewport attr:class=class.get_value() />
        </ToastProvider>
    }
}

#[component]
pub fn ThemedToast(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] duration: MaybeProp<i32>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(TOAST_CLASS);
    let children = StoredValue::new(children);

    view! {
        <Toast
            attr:class=class.get_value()
            open=open
            default_open=default_open
            on_open_change=move |val: bool| {
                if let Some(cb) = on_open_change {
                    cb.run(val);
                }
            }
            duration=duration
        >
            {children.with_value(|children| children())}
        </Toast>
    }
}

#[component]
pub fn ThemedToastTitle(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(TITLE_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ToastTitle attr:class=class.get_value()>
            {children.with_value(|children| children())}
        </ToastTitle>
    }
}

#[component]
pub fn ThemedToastDescription(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(DESCRIPTION_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ToastDescription attr:class=class.get_value()>
            {children.with_value(|children| children())}
        </ToastDescription>
    }
}

#[component]
pub fn ThemedToastClose(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(CLOSE_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ToastClose attr:class=class.get_value()>
            {children.with_value(|children| children())}
        </ToastClose>
    }
}

#[component]
pub fn ThemedToastAction(
    #[prop(into)] alt_text: String,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ACTION_CLASS);
    let children = StoredValue::new(children);

    view! {
        <ToastAction attr:class=class.get_value() alt_text=alt_text>
            {children.with_value(|children| children())}
        </ToastAction>
    }
}

// ---------------------------------------------------------------------------
// Shared icons
// ---------------------------------------------------------------------------

#[component]
pub fn XIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
        </svg>
    }
}
