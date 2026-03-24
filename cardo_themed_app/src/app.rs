use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

use crate::pages::{
    accordion, alert_dialog, aspect_ratio, avatar, badges, buttons, card, checkboxes, collapsible,
    context_menu, dialog, dropdown_menu, home, hover_card, input, label, menubar, navigation_menu,
    one_time_password_field, password_toggle_field, popover, progress, radio_group, scroll_area,
    select, separator, slider, switches, tabs, textarea, toast, toggle_group, toggles, tooltip,
};

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="block px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors no-underline"
        >
            {label}
        </A>
    }
}

#[component]
fn DarkModeToggle() -> impl IntoView {
    let (dark, set_dark) = signal(false);

    let toggle = move |_| {
        let next = !dark.get();
        set_dark.set(next);

        if let Some(el) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let _ = if next {
                el.class_list().add_1("dark")
            } else {
                el.class_list().remove_1("dark")
            };
        }
    };

    view! {
        <button
            class="inline-flex items-center justify-center size-8 rounded-md text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
            on:click=toggle
            title="Toggle dark mode"
        >
            {move || if dark.get() { "\u{2600}\u{fe0f}" } else { "\u{1f319}" }}
        </button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="flex min-h-screen">
                // Sidebar
                <nav class="w-56 shrink-0 border-r border-border bg-card p-4 flex flex-col gap-1">
                    <div class="flex items-center justify-between mb-4">
                        <span class="text-sm font-semibold text-foreground">"Cardo Themed"</span>
                        <DarkModeToggle />
                    </div>

                    <NavLink href="/" label="Design System" />

                    <div class="mt-4 mb-1 px-3 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
                        "Components"
                    </div>
                    <NavLink href="/accordion" label="Accordion" />
                    <NavLink href="/alert-dialog" label="Alert Dialog" />
                    <NavLink href="/aspect-ratio" label="Aspect Ratio" />
                    <NavLink href="/avatar" label="Avatar" />
                    <NavLink href="/badge" label="Badge" />
                    <NavLink href="/button" label="Button" />
                    <NavLink href="/card" label="Card" />
                    <NavLink href="/checkbox" label="Checkbox" />
                    <NavLink href="/collapsible" label="Collapsible" />
                    <NavLink href="/context-menu" label="Context Menu" />
                    <NavLink href="/dialog" label="Dialog" />
                    <NavLink href="/dropdown-menu" label="Dropdown Menu" />
                    <NavLink href="/hover-card" label="Hover Card" />
                    <NavLink href="/input" label="Input" />
                    <NavLink href="/label" label="Label" />
                    <NavLink href="/menubar" label="Menubar" />
                    <NavLink href="/navigation-menu" label="Navigation Menu" />
                    <NavLink href="/one-time-password-field" label="OTP Field" />
                    <NavLink href="/password-toggle-field" label="Password Toggle" />
                    <NavLink href="/popover" label="Popover" />
                    <NavLink href="/progress" label="Progress" />
                    <NavLink href="/radio-group" label="Radio Group" />
                    <NavLink href="/scroll-area" label="Scroll Area" />
                    <NavLink href="/select" label="Select" />
                    <NavLink href="/separator" label="Separator" />
                    <NavLink href="/slider" label="Slider" />
                    <NavLink href="/switch" label="Switch" />
                    <NavLink href="/tabs" label="Tabs" />
                    <NavLink href="/textarea" label="Textarea" />
                    <NavLink href="/toast" label="Toast" />
                    <NavLink href="/toggle" label="Toggle" />
                    <NavLink href="/toggle-group" label="Toggle Group" />
                    <NavLink href="/tooltip" label="Tooltip" />
                </nav>

                // Content
                <main class="flex-1 p-8 overflow-y-auto">
                    <Routes fallback=|| "Not found">
                        <Route path=path!("/") view=home::HomePage />
                        <Route path=path!("/accordion") view=accordion::AccordionPage />
                        <Route path=path!("/alert-dialog") view=alert_dialog::AlertDialogPage />
                        <Route path=path!("/aspect-ratio") view=aspect_ratio::AspectRatioPage />
                        <Route path=path!("/avatar") view=avatar::AvatarPage />
                        <Route path=path!("/badge") view=badges::BadgesPage />
                        <Route path=path!("/button") view=buttons::ButtonsPage />
                        <Route path=path!("/card") view=card::CardPage />
                        <Route path=path!("/checkbox") view=checkboxes::CheckboxesPage />
                        <Route path=path!("/collapsible") view=collapsible::CollapsiblePage />
                        <Route path=path!("/context-menu") view=context_menu::ContextMenuPage />
                        <Route path=path!("/dialog") view=dialog::DialogPage />
                        <Route path=path!("/dropdown-menu") view=dropdown_menu::DropdownMenuPage />
                        <Route path=path!("/hover-card") view=hover_card::HoverCardPage />
                        <Route path=path!("/input") view=input::InputPage />
                        <Route path=path!("/label") view=label::LabelPage />
                        <Route path=path!("/menubar") view=menubar::MenubarPage />
                        <Route path=path!("/navigation-menu") view=navigation_menu::NavigationMenuPage />
                        <Route path=path!("/one-time-password-field") view=one_time_password_field::OneTimePasswordFieldPage />
                        <Route path=path!("/password-toggle-field") view=password_toggle_field::PasswordToggleFieldPage />
                        <Route path=path!("/popover") view=popover::PopoverPage />
                        <Route path=path!("/progress") view=progress::ProgressPage />
                        <Route path=path!("/radio-group") view=radio_group::RadioGroupPage />
                        <Route path=path!("/scroll-area") view=scroll_area::ScrollAreaPage />
                        <Route path=path!("/select") view=select::SelectPage />
                        <Route path=path!("/separator") view=separator::SeparatorPage />
                        <Route path=path!("/slider") view=slider::SliderPage />
                        <Route path=path!("/switch") view=switches::SwitchesPage />
                        <Route path=path!("/tabs") view=tabs::TabsPage />
                        <Route path=path!("/textarea") view=textarea::TextareaPage />
                        <Route path=path!("/toast") view=toast::ToastPage />
                        <Route path=path!("/toggle") view=toggles::TogglesPage />
                        <Route path=path!("/toggle-group") view=toggle_group::ToggleGroupPage />
                        <Route path=path!("/tooltip") view=tooltip::TooltipPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
