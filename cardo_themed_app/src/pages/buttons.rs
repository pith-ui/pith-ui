use leptos::prelude::*;

use crate::theme::button::*;

/// Helper to render a labeled group of buttons.
#[component]
fn ButtonRow(label: &'static str, variant: ButtonVariant) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <h3 class="text-sm font-medium text-neutral-11">{label}</h3>
            <div class="flex items-center gap-3 flex-wrap">
                <Button variant=variant size=ButtonSize::Sm>"Small"</Button>
                <Button variant=variant size=ButtonSize::Md>"Medium"</Button>
                <Button variant=variant size=ButtonSize::Lg>"Large"</Button>
                <Button variant=variant disabled=true>"Disabled"</Button>
            </div>
        </div>
    }
}

#[component]
pub fn ButtonsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-neutral-12 mb-1">"Button"</h1>
                <p class="text-neutral-11 mb-6">
                    "A styled button with variant and size props. All colors reference "
                    <code class="text-accent-11 bg-accent-3 px-1 rounded-1 text-sm">"accent-*"</code>
                    " tokens."
                </p>
            </div>

            <ButtonRow label="Solid (default)" variant=ButtonVariant::Solid />
            <ButtonRow label="Soft" variant=ButtonVariant::Soft />
            <ButtonRow label="Outline" variant=ButtonVariant::Outline />
            <ButtonRow label="Ghost" variant=ButtonVariant::Ghost />

            // Code example
            <section class="space-y-2">
                <h2 class="text-lg font-semibold text-neutral-12">"Usage"</h2>
                <pre class="bg-neutral-2 border border-neutral-6 rounded-3 p-4 text-sm text-neutral-12 overflow-x-auto">
                    <code>{r#"<Button variant=ButtonVariant::Soft size=ButtonSize::Lg>
    "Click me"
</Button>"#}</code>
                </pre>
            </section>

            // TwClass definition
            <section class="space-y-2">
                <h2 class="text-lg font-semibold text-neutral-12">"Style Definition"</h2>
                <p class="text-sm text-neutral-11">
                    "Every utility class below references a token. No hard-coded colors."
                </p>
                <pre class="bg-neutral-2 border border-neutral-6 rounded-3 p-4 text-sm text-neutral-12 overflow-x-auto">
                    <code>{r##"#[derive(TwClass)]
#[tw(class = "inline-flex items-center justify-center font-medium
              transition-colors duration-normal ...
              focus-visible:ring-2 focus-visible:ring-focus-ring
              disabled:opacity-50")]
pub struct ButtonClass {
    pub variant: ButtonVariant, // Solid | Soft | Outline | Ghost
    pub size: ButtonSize,       // Sm | Md | Lg
}"##}</code>
                </pre>
            </section>
        </div>
    }
}
