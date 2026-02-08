use leptos::prelude::*;
use radix_leptos_collapsible::*;
use tailwind_fuse::*;

#[component]
pub fn Styled() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let trigger_class = Memo::new(move |_| TriggerClass::default().to_class());
    let content_class = Memo::new(move |_| ContentClass::default().to_class());

    view! {
        <Collapsible attr:class=root_class>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let trigger_class = Memo::new(move |_| TriggerClass::default().to_class());
    let content_class = Memo::new(move |_| ContentClass::default().to_class());

    let (open, set_open) = signal(false);

    view! {
        <Collapsible
            open=open
            on_open_change=Callback::new(move |value| set_open.set(value))
            attr:class=root_class
        >
            <CollapsibleTrigger attr:class=trigger_class>
                {move || if open.get() { "close" } else { "open" }}
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let trigger_class = Memo::new(move |_| TriggerClass::default().to_class());
    let animated_content_class = Memo::new(move |_| AnimatedContentClass::default().to_class());

    view! {
        <h1>"Closed by default"</h1>
        <Collapsible attr:class=root_class>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=animated_content_class>
                <div style="padding: 10px;">"Content 1"</div>
            </CollapsibleContent>
        </Collapsible>

        <h1>"Open by default"</h1>
        <Collapsible default_open=true attr:class=root_class>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=animated_content_class>
                <div style="padding: 10px;">"Content 1"</div>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn AnimatedHorizontal() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let trigger_class = Memo::new(move |_| TriggerClass::default().to_class());
    let animated_width_content_class =
        Memo::new(move |_| AnimatedWidthContentClass::default().to_class());

    view! {
        <Collapsible attr:class=root_class>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=animated_width_content_class>
                <div style="padding: 10px;">"Content"</div>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let root_class = Memo::new(move |_| RootClass::default().to_class());
    let trigger_class = Memo::new(move |_| TriggerClass::default().to_class());
    let content_class = Memo::new(move |_| ContentClass::default().to_class());
    let root_attr_class = Memo::new(move |_| RootAttrClass::default().to_class());
    let trigger_attr_class = Memo::new(move |_| TriggerAttrClass::default().to_class());
    let content_attr_class = Memo::new(move |_| ContentAttrClass::default().to_class());

    view! {
        <h1>"Uncontrolled"</h1>
        <h2>"Closed"</h2>
        <Collapsible attr:class=root_class>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Open"</h2>
        <Collapsible attr:class=root_class default_open=true>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h1>"Controlled"</h1>
        <h2>"Closed"</h2>
        <Collapsible attr:class=root_class open=false>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Open"</h2>
        <Collapsible attr:class=root_class open=true>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h1>"Disabled"</h1>
        <Collapsible attr:class=root_class disabled=true>
            <CollapsibleTrigger attr:class=trigger_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h1>"State attributes"</h1>
        <h2>"Closed"</h2>
        <Collapsible attr:class=root_attr_class>
            <CollapsibleTrigger attr:class=trigger_attr_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_attr_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Open"</h2>
        <Collapsible attr:class=root_attr_class default_open=true>
            <CollapsibleTrigger attr:class=trigger_attr_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_attr_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>

        <h2>"Disabled"</h2>
        <Collapsible attr:class=root_attr_class default_open=true disabled=true>
            <CollapsibleTrigger attr:class=trigger_attr_class>
                "Trigger"
            </CollapsibleTrigger>
            <CollapsibleContent attr:class=content_attr_class>
                "Content 1"
            </CollapsibleContent>
        </Collapsible>
    }
}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "max-w-[20em] font-sans")]
struct RootClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "w-full text-left appearance-none border-none p-[10px] bg-[var(--color-black)] text-white font-[inherit] text-[1.2em] [--shadow-color:crimson] focus:outline-none focus:[box-shadow:inset_0_-5px_0_0_var(--shadow-color)] focus:text-[var(--color-red)] data-[disabled]:text-[var(--color-gray300)] data-[state=open]:bg-[var(--color-red)] data-[state=open]:text-[var(--color-white)] data-[state=open]:focus:[--shadow-color:#111] data-[state=open]:focus:text-[var(--color-black)]"
)]
struct TriggerClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(class = "p-[10px] leading-[1.5]")]
struct ContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "overflow-hidden data-[state=open]:animate-[collapsible-slideDown_300ms_ease-out] data-[state=closed]:animate-[collapsible-slideUp_300ms_ease-in]"
)]
struct AnimatedContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "overflow-hidden data-[state=open]:animate-[collapsible-openRight_300ms_ease-out] data-[state=closed]:animate-[collapsible-closeRight_300ms_ease-in]"
)]
struct AnimatedWidthContentClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "block bg-[rgb(0_0_255/0.3)] border-2 border-solid border-blue-500 p-[10px] data-[state=closed]:border-red-500 data-[state=open]:border-green-500 data-[disabled]:border-dashed disabled:opacity-50"
)]
struct RootAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "block bg-[rgb(0_0_255/0.3)] border-2 border-solid border-blue-500 p-[10px] data-[state=closed]:border-red-500 data-[state=open]:border-green-500 data-[disabled]:border-dashed disabled:opacity-50"
)]
struct TriggerAttrClass {}

#[derive(TwClass, Default, Clone, Copy)]
#[tw(
    class = "block bg-[rgb(0_0_255/0.3)] border-2 border-solid border-blue-500 p-[10px] data-[state=closed]:border-red-500 data-[state=open]:border-green-500 data-[disabled]:border-dashed disabled:opacity-50"
)]
struct ContentAttrClass {}
