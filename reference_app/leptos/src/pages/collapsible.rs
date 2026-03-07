use leptos::prelude::*;
use radix_leptos_primitives::collapsible::*;

#[component]
pub fn CollapsiblePage() -> impl IntoView {
    let (disabled, set_disabled) = signal(false);
    let (open, set_open) = signal(false);

    view! {
        <Collapsible
            attr:class="collapsible-root"
            attr:data-testid="collapsible-root"
            disabled=disabled
            open=open
            on_open_change=Callback::new(move |value: bool| set_open.set(value))
        >
            <CollapsibleTrigger attr:class="collapsible-trigger">
                "toggle"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class="collapsible-content">
                <p>"Collapsible content."</p>
            </CollapsibleContent>
        </Collapsible>

        <br />

        // Always-open collapsible for internal styles testing
        <Collapsible open=true>
            <CollapsibleContent
                attr:data-testid="styled-collapsible-content"
                attr:style="background: tomato"
            >
                <p>"Styled collapsible content."</p>
            </CollapsibleContent>
        </Collapsible>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || disabled.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_disabled.set(target.checked());
                }
            />
            " disabled"
        </label>

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || open.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_open.set(target.checked());
                }
            />
            " open"
        </label>
    }
}
