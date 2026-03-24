use cardo_ui::accordion::{
    Accordion, AccordionContent, AccordionHeader, AccordionItem, AccordionTrigger, AccordionType,
};
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york accordion
// ---------------------------------------------------------------------------

const HEADER_CLASS: &str = "flex";

const ITEM_CLASS: &str = "border-b last:border-b-0";

const TRIGGER_CLASS: &str = "flex flex-1 items-start justify-between gap-4 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:focus-ring disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180";

const CONTENT_CLASS: &str =
    "overflow-hidden text-sm data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down";
const CONTENT_INNER_CLASS: &str = "pt-0 pb-4";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedAccordion(
    r#type: AccordionType,
    #[prop(into, optional)] collapsible: MaybeProp<bool>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <Accordion
            r#type=r#type
            collapsible=collapsible
            default_value=default_value
            disabled=disabled
        >
            {children()}
        </Accordion>
    }
}

#[component]
pub fn ThemedAccordionItem(
    #[prop(into)] value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(ITEM_CLASS);

    view! {
        <AccordionItem attr:class=class.get_value() value=value disabled=disabled>
            {children()}
        </AccordionItem>
    }
}

#[component]
pub fn ThemedAccordionTrigger(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(TRIGGER_CLASS);
    let header_class = StoredValue::new(HEADER_CLASS);
    let children = StoredValue::new(children);

    view! {
        <AccordionHeader attr:class=header_class.get_value()>
            <AccordionTrigger attr:class=class.get_value()>
                {children.with_value(|children| children())}
                <svg
                    class="pointer-events-none size-4 shrink-0 translate-y-0.5 text-muted-foreground transition-transform duration-200"
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                >
                    <path d="m6 9 6 6 6-6" />
                </svg>
            </AccordionTrigger>
        </AccordionHeader>
    }
}

#[component]
pub fn ThemedAccordionContent(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let inner_class = StoredValue::new(CONTENT_INNER_CLASS);

    view! {
        <AccordionContent attr:class=class.get_value()>
            <div class=inner_class.get_value()>
                {children()}
            </div>
        </AccordionContent>
    }
}
