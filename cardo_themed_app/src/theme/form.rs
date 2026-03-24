use cardo_ui::form::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york form
// ---------------------------------------------------------------------------

const FIELD_CLASS: &str = "grid gap-2";

const LABEL_CLASS: &str = "text-sm font-medium data-[invalid]:text-destructive";

const MESSAGE_CLASS: &str = "text-sm text-destructive";

// ---------------------------------------------------------------------------
// Re-exports for page convenience
// ---------------------------------------------------------------------------

pub use cardo_ui::form::ValidityMatcher;

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedForm(children: ChildrenFn) -> impl IntoView {
    view! {
        <Form>
            {children()}
        </Form>
    }
}

#[component]
pub fn ThemedFormField(
    #[prop(into)] name: String,
    #[prop(into, optional)] server_invalid: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(FIELD_CLASS);

    view! {
        <FormField attr:class=class.get_value() name=name server_invalid=server_invalid>
            {children()}
        </FormField>
    }
}

#[component]
pub fn ThemedFormLabel(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LABEL_CLASS);

    view! {
        <FormLabel attr:class=class.get_value()>
            {children()}
        </FormLabel>
    }
}

/// Themed pass-through for FormControl.
#[component]
pub fn ThemedFormControl(
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <FormControl>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </FormControl>
    }
}

/// Themed form message for a specific ValidityMatcher.
#[component]
pub fn ThemedFormMessage(
    r#match: ValidityMatcher,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let class = StoredValue::new(MESSAGE_CLASS);
    let children = StoredValue::new(children);
    let match_prop: Match = r#match.into();

    view! {
        <FormMessage attr:class=class.get_value() r#match=match_prop>
            {children.with_value(|c| c.as_ref().map(|c| c()))}
        </FormMessage>
    }
}

#[component]
pub fn ThemedFormSubmit(children: ChildrenFn) -> impl IntoView {
    view! {
        <FormSubmit>
            {children()}
        </FormSubmit>
    }
}
