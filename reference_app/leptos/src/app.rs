use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages;

#[component]
fn Index() -> impl IntoView {
    view! {
        <h1>"Radix Reference App (Leptos)"</h1>
        <p>"Add component pages as needed. Each route maps to a Radix primitive test fixture."</p>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("/") view=Index />
                    <Route path=path!("/accordion") view=pages::accordion::AccordionPage />
                    <Route path=path!("/alert-dialog") view=pages::alert_dialog::AlertDialogPage />
                    <Route path=path!("/aspect-ratio") view=pages::aspect_ratio::AspectRatioPage />
                    <Route path=path!("/avatar") view=pages::avatar::AvatarPage />
                    <Route path=path!("/checkbox") view=pages::checkbox::CheckboxPage />
                    <Route path=path!("/collapsible") view=pages::collapsible::CollapsiblePage />
                    <Route path=path!("/context-menu") view=pages::context_menu::ContextMenuPage />
                    <Route path=path!("/dialog") view=pages::dialog::DialogPage />
                    <Route path=path!("/dropdown-menu") view=pages::dropdown_menu::DropdownMenuPage />
                    <Route path=path!("/dropdown-menu/with-tooltip") view=pages::dropdown_menu_with_tooltip::DropdownMenuWithTooltipPage />
                    <Route path=path!("/form") view=pages::form::FormPage />
                    <Route path=path!("/menu") view=pages::menu::MenuPage />
                    <Route path=path!("/menubar") view=pages::menubar::MenubarPage />
                    <Route path=path!("/hover-card") view=pages::hover_card::HoverCardPage />
                    <Route path=path!("/navigation-menu") view=pages::navigation_menu::NavigationMenuPage />
                    <Route path=path!("/one-time-password-field") view=pages::one_time_password_field::OneTimePasswordFieldPage />
                    <Route path=path!("/password-toggle-field") view=pages::password_toggle_field::PasswordToggleFieldPage />
                    <Route path=path!("/popper") view=pages::popper::PopperPage />
                    <Route path=path!("/popover") view=pages::popover::PopoverPage />
                    <Route path=path!("/progress") view=pages::progress::ProgressPage />
                    <Route path=path!("/radio-group") view=pages::radio_group::RadioGroupPage />
                    <Route path=path!("/scroll-area") view=pages::scroll_area::ScrollAreaPage />
                    <Route path=path!("/select") view=pages::select::SelectPage />
                    <Route path=path!("/select/forced-open") view=pages::select::SelectForcedOpenPage />
                    <Route path=path!("/separator") view=pages::separator::SeparatorPage />
                    <Route path=path!("/slider") view=pages::slider::SliderPage />
                    <Route path=path!("/switch") view=pages::switch::SwitchPage />
                    <Route path=path!("/tabs") view=pages::tabs::TabsPage />
                    <Route path=path!("/toast") view=pages::toast::ToastPage />
                    <Route path=path!("/toggle") view=pages::toggle::TogglePage />
                    <Route path=path!("/toggle-group") view=pages::toggle_group::ToggleGroupPage />
                    <Route path=path!("/toolbar") view=pages::toolbar::ToolbarPage />
                    <Route path=path!("/tooltip") view=pages::tooltip::TooltipPage />
                </Routes>
            </main>
        </Router>
    }
}
