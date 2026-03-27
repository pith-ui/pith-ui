use pith_ui::dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogPortal, DialogTitle,
    DialogTrigger,
};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york dialog
// ---------------------------------------------------------------------------

const OVERLAY_CLASS: &str = "fixed inset-0 z-50 bg-black/50";

const CONTENT_CLASS: &str = "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg";

const TITLE_CLASS: &str = "text-lg leading-none font-semibold";

const DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";

const CLOSE_X_CLASS: &str = "absolute top-4 right-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:ring-2 focus:ring-ring focus:ring-offset-2 focus:outline-hidden disabled:pointer-events-none";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedDialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let forward_cb = Callback::new(move |val: bool| {
        if let Some(cb) = on_open_change {
            cb.run(val);
        }
    });

    view! {
        <Dialog
            open=open
            default_open=default_open
            on_open_change=forward_cb
        >
            {children()}
        </Dialog>
    }
}

#[component]
pub fn ThemedDialogTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <DialogTrigger as_child=true>
            {children()}
        </DialogTrigger>
    }
}

/// Themed dialog content that composes DialogPortal > DialogOverlay + DialogContent.
/// Includes an X close button in the top-right corner.
#[component]
pub fn ThemedDialogContent(children: ChildrenFn) -> impl IntoView {
    let overlay_class = StoredValue::new(OVERLAY_CLASS);
    let content_class = StoredValue::new(CONTENT_CLASS);
    let close_class = StoredValue::new(CLOSE_X_CLASS);
    let children = StoredValue::new(children);

    view! {
        <DialogPortal>
            <DialogOverlay attr:class=overlay_class.get_value() />
            <DialogContent attr:class=content_class.get_value()>
                {children.with_value(|children| children())}
                <DialogClose attr:class=close_class.get_value()>
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
                    <span class="sr-only">"Close"</span>
                </DialogClose>
            </DialogContent>
        </DialogPortal>
    }
}

#[component]
pub fn ThemedDialogTitle(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(TITLE_CLASS);

    view! {
        <DialogTitle attr:class=class.get_value()>
            {children()}
        </DialogTitle>
    }
}

#[component]
pub fn ThemedDialogDescription(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(DESCRIPTION_CLASS);

    view! {
        <DialogDescription attr:class=class.get_value()>
            {children()}
        </DialogDescription>
    }
}

#[component]
pub fn ThemedDialogClose(children: ChildrenFn) -> impl IntoView {
    view! {
        <DialogClose as_child=true>
            {children()}
        </DialogClose>
    }
}
