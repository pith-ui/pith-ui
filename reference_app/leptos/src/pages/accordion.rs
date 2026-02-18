use leptos::prelude::*;
use radix_leptos_accordion::*;

#[component]
fn AccordionItems() -> impl IntoView {
    view! {
        <AccordionItem
            value="item-1".to_string()
            attr:class="accordion-item"
            attr:data-testid="item-1"
        >
            <AccordionHeader attr:class="accordion-header">
                <AccordionTrigger attr:class="accordion-trigger">"Item 1"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class="accordion-content">"Content 1"</AccordionContent>
        </AccordionItem>
        <AccordionItem
            value="item-2".to_string()
            disabled=true
            attr:class="accordion-item"
            attr:data-testid="item-2"
        >
            <AccordionHeader attr:class="accordion-header">
                <AccordionTrigger attr:class="accordion-trigger">"Item 2"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class="accordion-content">"Content 2"</AccordionContent>
        </AccordionItem>
        <AccordionItem
            value="item-3".to_string()
            attr:class="accordion-item"
            attr:data-testid="item-3"
        >
            <AccordionHeader attr:class="accordion-header">
                <AccordionTrigger attr:class="accordion-trigger">"Item 3"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class="accordion-content">"Content 3"</AccordionContent>
        </AccordionItem>
    }
}

#[component]
pub fn AccordionPage() -> impl IntoView {
    let (is_multiple, set_is_multiple) = signal(false);
    let (collapsible, set_collapsible) = signal(false);

    view! {
        <Show
            when=move || !is_multiple.get()
            fallback=move || {
                view! {
                    <Accordion
                        r#type=AccordionType::Multiple
                        attr:class="accordion-root"
                        attr:data-testid="accordion-root"
                    >
                        <AccordionItems />
                    </Accordion>
                }
            }
        >
            <Accordion
                r#type=AccordionType::Single
                collapsible=collapsible
                attr:class="accordion-root"
                attr:data-testid="accordion-root"
            >
                <AccordionItems />
            </Accordion>
        </Show>

        <br />
        <br />

        <fieldset>
            <legend>"type"</legend>
            <label>
                <input
                    type="radio"
                    name="type"
                    value="single"
                    prop:checked=move || !is_multiple.get()
                    on:change=move |_| set_is_multiple.set(false)
                />
                " single"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="type"
                    value="multiple"
                    prop:checked=move || is_multiple.get()
                    on:change=move |_| set_is_multiple.set(true)
                />
                " multiple"
            </label>
        </fieldset>

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || collapsible.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_collapsible.set(target.checked());
                }
            />
            " collapsible"
        </label>
    }
}
