use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::pages;

#[component]
fn Index() -> impl IntoView {
    view! {
        <h1>"Radix Experiments (Leptos)"</h1>
        <p>"Isolated experiments for validating Leptos framework assumptions."</p>
        <ul>
            <li><a href="/attribute-clone">"Attribute Clone"</a></li>
        </ul>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("/") view=Index />
                    <Route path=path!("/attribute-clone") view=pages::attribute_clone::AttributeClonePage />
                </Routes>
            </main>
        </Router>
    }
}
