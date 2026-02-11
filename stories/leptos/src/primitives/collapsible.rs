use leptos::prelude::*;
use radix_leptos_collapsible::*;

stylance::import_crate_style!(classes, "src/primitives/collapsible.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <Collapsible attr:class=classes::root>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::content>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Collapsible
            open=open
            on_open_change=Callback::new(move |value| set_open.set(value))
            attr:class=classes::root
        >
            <CollapsibleTrigger attr:class=classes::trigger>
                {move || if open.get() { "close" } else { "open" }}
            </CollapsibleTrigger>
            <CollapsibleContent as_child=true attr:class=classes::content>
                <article>"Content 1"</article>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {
        <h1>"Closed by default"</h1>
        <Collapsible attr:class=classes::root>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::animatedContent>
                <div style="padding: 10px;">"Content 1"</div>
            </CollapsibleContent>
        </Collapsible>

        <h1>"Open by default"</h1>
        <Collapsible default_open=true attr:class=classes::root>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::animatedContent>
                <div style="padding: 10px;">"Content 1"</div>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn AnimatedHorizontal() -> impl IntoView {
    view! {
        <Collapsible attr:class=classes::root>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::animatedWidthContent>
                <div style="padding: 10px;">"Content"</div>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Closed"</h2>
        <Collapsible attr:class=classes::root>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::content>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Open"</h2>
        <Collapsible attr:class=classes::root default_open=true>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::content>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h1>"Controlled"</h1>
        <h2>"Closed"</h2>
        <Collapsible attr:class=classes::root open=false>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::content>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Open"</h2>
        <Collapsible attr:class=classes::root open=true>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::content>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h1>"Disabled"</h1>
        <Collapsible attr:class=classes::root disabled=true>
            <CollapsibleTrigger attr:class=classes::trigger>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::content>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h1>"State attributes"</h1>
        <h2>"Closed"</h2>
        <Collapsible attr:class=classes::rootAttr>
            <CollapsibleTrigger attr:class=classes::triggerAttr>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::contentAttr>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Open"</h2>
        <Collapsible attr:class=classes::rootAttr default_open=true>
            <CollapsibleTrigger attr:class=classes::triggerAttr>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::contentAttr>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Disabled"</h2>
        <Collapsible attr:class=classes::rootAttr default_open=true disabled=true>
            <CollapsibleTrigger attr:class=classes::triggerAttr>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=classes::contentAttr>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>
    }
}
