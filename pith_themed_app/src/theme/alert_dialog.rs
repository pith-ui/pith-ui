use pith_ui::alert_dialog::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york alert-dialog
// ---------------------------------------------------------------------------

const OVERLAY_CLASS: &str = "fixed inset-0 z-50 bg-black/50";

const CONTENT_CLASS: &str = "fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-lg border bg-background p-6 shadow-lg sm:max-w-lg";

const TITLE_CLASS: &str = "text-lg font-semibold";

const DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedAlertDialog(
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
        <AlertDialog
            open=open
            default_open=default_open
            on_open_change=forward_cb
        >
            {children()}
        </AlertDialog>
    }
}

#[component]
pub fn ThemedAlertDialogTrigger(children: ChildrenFn) -> impl IntoView {
    view! {
        <AlertDialogTrigger as_child=true>
            {children()}
        </AlertDialogTrigger>
    }
}

/// Themed alert dialog content that composes AlertDialogPortal > AlertDialogOverlay + AlertDialogContent.
/// Unlike Dialog, alert dialogs do NOT include an X close button.
#[component]
pub fn ThemedAlertDialogContent(children: ChildrenFn) -> impl IntoView {
    let overlay_class = StoredValue::new(OVERLAY_CLASS);
    let content_class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <AlertDialogPortal>
            <AlertDialogOverlay attr:class=overlay_class.get_value() />
            <AlertDialogContent attr:class=content_class.get_value()>
                {children.with_value(|children| children())}
            </AlertDialogContent>
        </AlertDialogPortal>
    }
}

#[component]
pub fn ThemedAlertDialogTitle(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(TITLE_CLASS);

    view! {
        <AlertDialogTitle attr:class=class.get_value()>
            {children()}
        </AlertDialogTitle>
    }
}

#[component]
pub fn ThemedAlertDialogDescription(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(DESCRIPTION_CLASS);

    view! {
        <AlertDialogDescription attr:class=class.get_value()>
            {children()}
        </AlertDialogDescription>
    }
}

#[component]
pub fn ThemedAlertDialogAction(children: ChildrenFn) -> impl IntoView {
    view! {
        <AlertDialogAction as_child=true>
            {children()}
        </AlertDialogAction>
    }
}

#[component]
pub fn ThemedAlertDialogCancel(children: ChildrenFn) -> impl IntoView {
    view! {
        <AlertDialogCancel as_child=true>
            {children()}
        </AlertDialogCancel>
    }
}

#[component]
pub fn ThemedAlertDialogHeader(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 text-center sm:text-left">
            {children()}
        </div>
    }
}

#[component]
pub fn ThemedAlertDialogFooter(children: ChildrenFn) -> impl IntoView {
    view! {
        <div class="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
            {children()}
        </div>
    }
}
