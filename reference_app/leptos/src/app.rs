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
                    <Route path=path!("/dialog") view=pages::dialog::DialogPage />
                    // <Route path=path!("/form") view=pages::form::FormPage />
                </Routes>
            </main>
        </Router>
    }
}
