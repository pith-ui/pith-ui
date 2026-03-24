use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

use crate::pages::{badges, buttons, checkboxes, home, switches, toggles};

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
                    <NavLink href="/button" label="Button" />
                    <NavLink href="/badge" label="Badge" />
                    <NavLink href="/checkbox" label="Checkbox" />
                    <NavLink href="/switch" label="Switch" />
                    <NavLink href="/toggle" label="Toggle" />
                </nav>

                // Content
                <main class="flex-1 p-8 overflow-y-auto">
                    <Routes fallback=|| "Not found">
                        <Route path=path!("/") view=home::HomePage />
                        <Route path=path!("/button") view=buttons::ButtonsPage />
                        <Route path=path!("/badge") view=badges::BadgesPage />
                        <Route path=path!("/checkbox") view=checkboxes::CheckboxesPage />
                        <Route path=path!("/switch") view=switches::SwitchesPage />
                        <Route path=path!("/toggle") view=toggles::TogglesPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
