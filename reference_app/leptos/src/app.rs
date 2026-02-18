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
                    <Route path=path!("/collapsible") view=pages::collapsible::CollapsiblePage />
                    <Route path=path!("/dialog") view=pages::dialog::DialogPage />
                    <Route path=path!("/form") view=pages::form::FormPage />
                    <Route path=path!("/hover-card") view=pages::hover_card::HoverCardPage />
                    <Route path=path!("/navigation-menu") view=pages::navigation_menu::NavigationMenuPage />
                    <Route path=path!("/popover") view=pages::popover::PopoverPage />
                    <Route path=path!("/progress") view=pages::progress::ProgressPage />
                    <Route path=path!("/radio-group") view=pages::radio_group::RadioGroupPage />
                    <Route path=path!("/scroll-area") view=pages::scroll_area::ScrollAreaPage />
                    <Route path=path!("/separator") view=pages::separator::SeparatorPage />
                    <Route path=path!("/slider") view=pages::slider::SliderPage />
                    <Route path=path!("/tabs") view=pages::tabs::TabsPage />
                    <Route path=path!("/toggle") view=pages::toggle::TogglePage />
                    <Route path=path!("/toggle-group") view=pages::toggle_group::ToggleGroupPage />
                    <Route path=path!("/toolbar") view=pages::toolbar::ToolbarPage />
                </Routes>
            </main>
        </Router>
    }
}
