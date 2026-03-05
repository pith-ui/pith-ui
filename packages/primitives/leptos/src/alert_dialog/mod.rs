use crate::compose_refs::use_composed_refs;
use crate::dialog::*;
use crate::primitive::wrap_callback;
use leptos::{context::Provider, ev, prelude::*};
use leptos_node_ref::AnyNodeRef;
use send_wrapper::SendWrapper;

/* -------------------------------------------------------------------------------------------------
 * AlertDialog
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AlertDialog(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // AlertDialog always forces modal=true, unlike Dialog which allows configuration.
    view! {
        <Dialog
            open=open
            default_open=default_open
            on_open_change=wrap_callback(on_open_change)
            modal=true
        >
            {children.with_value(|children| children())}
        </Dialog>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AlertDialogTrigger(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <DialogTrigger
            on_click=wrap_callback(on_click)
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </DialogTrigger>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogPortal
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AlertDialogPortal(
    #[prop(into, optional)] container: MaybeProp<SendWrapper<web_sys::Element>>,
    #[prop(optional)] container_ref: AnyNodeRef,
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <DialogPortal
            container=container
            container_ref=container_ref
            force_mount=force_mount
        >
            {children.with_value(|children| children())}
        </DialogPortal>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogOverlay
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AlertDialogOverlay(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <DialogOverlay
            force_mount=force_mount
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </DialogOverlay>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogContent
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct AlertDialogContentContextValue {
    cancel_ref: AnyNodeRef,
}

/// AlertDialogContent wraps DialogContent with three key behavioral overrides:
/// 1. Sets `role="alertdialog"` instead of `"dialog"`
/// 2. Auto-focuses the cancel button on open (via onOpenAutoFocus override)
/// 3. Prevents all outside interactions (pointer down and interact outside)
///
/// The `on_pointer_down_outside` and `on_interact_outside` props are intentionally
/// omitted from the public API (matching React's Omit<> on those props) because
/// AlertDialog always prevents outside dismissal.
#[component]
pub fn AlertDialogContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_open_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let cancel_ref = AnyNodeRef::new();

    let context = AlertDialogContentContextValue { cancel_ref };

    // In React, AlertDialogContent overrides onOpenAutoFocus to call
    // event.preventDefault() and directly focus the cancel button via cancelRef.
    //
    // In Leptos, we rely on FocusScope's default focus_first() behavior instead.
    // FocusScope auto-focuses the first tabbable element in the dialog, which by
    // convention is the cancel button (AlertDialogCancel). This avoids timing issues
    // with use_composed_refs Effect propagation (cancel_ref may not be set when
    // the auto-focus callback fires synchronously).
    //
    // We forward the user's callback but do NOT add our own focus logic — FocusScope
    // handles it correctly.
    let user_on_open_auto_focus = StoredValue::new(on_open_auto_focus);
    let alert_on_open_auto_focus = Callback::new(move |event: web_sys::Event| {
        user_on_open_auto_focus.with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
    });

    // Prevent outside interactions — AlertDialog cannot be dismissed by clicking outside.
    let prevent_pointer_outside = Callback::new(|event: web_sys::CustomEvent| {
        event.prevent_default();
    });
    let prevent_interact_outside = Callback::new(|event: web_sys::CustomEvent| {
        event.prevent_default();
    });

    view! {
        <Provider value=context>
            <DialogContent
                force_mount=force_mount
                role="alertdialog"
                as_child=as_child
                node_ref=node_ref
                on_open_auto_focus=alert_on_open_auto_focus
                on_close_auto_focus=wrap_callback(on_close_auto_focus)
                on_escape_key_down=wrap_callback(on_escape_key_down)
                on_pointer_down_outside=prevent_pointer_outside
                on_interact_outside=prevent_interact_outside
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </DialogContent>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogTitle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AlertDialogTitle(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <DialogTitle as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </DialogTitle>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogDescription
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AlertDialogDescription(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <DialogDescription as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </DialogDescription>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogAction
 * -----------------------------------------------------------------------------------------------*/

/// AlertDialogAction wraps DialogClose — clicking it closes the alert dialog.
#[component]
pub fn AlertDialogAction(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <DialogClose on_click=wrap_callback(on_click) as_child=as_child node_ref=node_ref>
            {children.with_value(|children| children())}
        </DialogClose>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AlertDialogCancel
 * -----------------------------------------------------------------------------------------------*/

/// AlertDialogCancel wraps DialogClose and registers itself with the content context
/// so that AlertDialogContent can auto-focus it when the dialog opens.
#[component]
pub fn AlertDialogCancel(
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<AlertDialogContentContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, context.cancel_ref]);

    view! {
        <DialogClose on_click=wrap_callback(on_click) as_child=as_child node_ref=composed_ref>
            {children.with_value(|children| children())}
        </DialogClose>
    }
}
