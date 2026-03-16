use leptos::prelude::*;
use radix_leptos_primitives::accordion::*;

#[component]
fn AccordionItems() -> impl IntoView {
    view! {
        <AccordionItem
            value="item-1".to_string()
            attr:class="accordion-item"
            attr:data-testid="item-1"
            attr:data-custom="accordion-item-custom"
        >
            <AccordionHeader attr:class="accordion-header" attr:data-custom="accordion-header-custom">
                <AccordionTrigger attr:class="accordion-trigger" attr:data-custom="accordion-trigger-custom">"Item 1"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent attr:class="accordion-content" attr:data-custom="accordion-content-custom">"Content 1"</AccordionContent>
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
        <AccordionItem
            value="item-styled".to_string()
            attr:class="accordion-item"
            attr:data-testid="item-styled"
        >
            <AccordionHeader attr:class="accordion-header">
                <AccordionTrigger attr:class="accordion-trigger">"Styled Item"</AccordionTrigger>
            </AccordionHeader>
            <AccordionContent
                attr:class="accordion-content"
                attr:data-testid="styled-content"
                style:background="tomato"
                style:--radix-accordion-content-height="999px"
            >
                "Styled content"
            </AccordionContent>
        </AccordionItem>
    }
}

#[component]
pub fn AccordionPage() -> impl IntoView {
    let (is_multiple, set_is_multiple) = signal(false);
    let (collapsible, set_collapsible) = signal(false);

    view! {
        {move || {
            if is_multiple.get() {
                view! {
                    <Accordion
                        r#type=AccordionType::Multiple
                        attr:class="accordion-root"
                        attr:data-testid="accordion-root"
                    >
                        <AccordionItems />
                    </Accordion>
                }
            } else {
                view! {
                    <Accordion
                        r#type=AccordionType::Single
                        collapsible=collapsible
                        attr:class="accordion-root"
                        attr:data-testid="accordion-root"
                    >
                        <AccordionItems />
                    </Accordion>
                }
            }
        }}

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

        <br />
        <br />

        <ControlledAccordion />
    }
}

#[component]
fn ControlledAccordion() -> impl IntoView {
    let (controlled_value, set_controlled_value) = signal(String::new());

    view! {
        <h2 data-testid="controlled-heading">"Controlled Accordion"</h2>

        <div data-testid="controlled-value">{move || controlled_value.get()}</div>

        <button
            data-testid="controlled-open-item-1"
            on:click=move |_| set_controlled_value.set("ctrl-item-1".to_string())
        >
            "Open Item 1"
        </button>
        <button
            data-testid="controlled-open-item-2"
            on:click=move |_| set_controlled_value.set("ctrl-item-2".to_string())
        >
            "Open Item 2"
        </button>
        <button
            data-testid="controlled-close-all"
            on:click=move |_| set_controlled_value.set(String::new())
        >
            "Close All"
        </button>

        <Accordion
            r#type=AccordionType::Single
            collapsible=true
            value=controlled_value
            on_value_change=Callback::new(move |val: String| {
                set_controlled_value.set(val);
            })
            attr:class="accordion-root"
            attr:data-testid="controlled-accordion-root"
        >
            <AccordionItem
                value="ctrl-item-1".to_string()
                attr:class="accordion-item"
                attr:data-testid="ctrl-item-1"
            >
                <AccordionHeader attr:class="accordion-header">
                    <AccordionTrigger attr:class="accordion-trigger">"Ctrl Item 1"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class="accordion-content">"Controlled Content 1"</AccordionContent>
            </AccordionItem>
            <AccordionItem
                value="ctrl-item-2".to_string()
                attr:class="accordion-item"
                attr:data-testid="ctrl-item-2"
            >
                <AccordionHeader attr:class="accordion-header">
                    <AccordionTrigger attr:class="accordion-trigger">"Ctrl Item 2"</AccordionTrigger>
                </AccordionHeader>
                <AccordionContent attr:class="accordion-content">"Controlled Content 2"</AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
