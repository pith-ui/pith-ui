use leptos::prelude::*;

use crate::theme::button::*;

#[component]
fn ButtonRow(label: &'static str, variant: ButtonVariant) -> impl IntoView {
    view! {
        <div class="space-y-2">
            <h3 class="text-sm font-medium text-muted-foreground">{label}</h3>
            <div class="flex items-center gap-3 flex-wrap">
                <Button variant=variant size=ButtonSize::Sm>"Small"</Button>
                <Button variant=variant size=ButtonSize::Default>"Default"</Button>
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
                <h1 class="text-2xl font-bold text-foreground mb-1">"Button"</h1>
                <p class="text-muted-foreground mb-6">
                    "shadcn/ui new-york button with 6 variants and 4 sizes."
                </p>
            </div>

            <ButtonRow label="Default" variant=ButtonVariant::Default />
            <ButtonRow label="Secondary" variant=ButtonVariant::Secondary />
            <ButtonRow label="Outline" variant=ButtonVariant::Outline />
            <ButtonRow label="Ghost" variant=ButtonVariant::Ghost />
            <ButtonRow label="Destructive" variant=ButtonVariant::Destructive />
            <ButtonRow label="Link" variant=ButtonVariant::Link />

            <section class="space-y-2">
                <h2 class="text-lg font-semibold text-foreground">"Icon Size"</h2>
                <div class="flex items-center gap-3">
                    <Button size=ButtonSize::Icon>"\u{2606}"</Button>
                </div>
            </section>
        </div>
    }
}
