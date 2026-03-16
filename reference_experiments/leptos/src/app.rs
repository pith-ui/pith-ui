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
            <li><a href="/presence-attrs">"Presence/Show Attribute Spreading"</a></li>
            <li><a href="/forwarded-attrs">"ForwardedAttrs (reactive solution)"</a></li>
            <li><a href="/spreadable-attrs">"Spreadable ForwardedAttrs"</a></li>
            <li><a href="/nav-menu-reactive-attrs">"NavigationMenu Reactive Attrs"</a></li>
            <li><a href="/style-override">"Style Override Order"</a></li>
            <li><a href="/interceptor-capture">"Interceptor Capture Scope"</a></li>
            <li><a href="/non-overridable-styles">"Non-Overridable Styles"</a></li>
            <li><a href="/popper-style-approaches">"Popper Style Approaches"</a></li>
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
                    <Route path=path!("/forwarded-attrs") view=pages::forwarded_attrs::ForwardedAttrsPage />
                    <Route path=path!("/presence-attrs") view=pages::presence_attrs::PresenceAttrsPage />
                    <Route path=path!("/spreadable-attrs") view=pages::spreadable_attrs::SpreadableAttrsPage />
                    <Route path=path!("/nav-menu-reactive-attrs") view=pages::nav_menu_reactive_attrs::NavMenuReactiveAttrsPage />
                    <Route path=path!("/style-override") view=pages::style_override::StyleOverridePage />
                    <Route path=path!("/interceptor-capture") view=pages::interceptor_capture::InterceptorCapturePage />
                    <Route path=path!("/non-overridable-styles") view=pages::non_overridable_styles::NonOverridableStylesPage />
                    <Route path=path!("/popper-style-approaches") view=pages::popper_style_approaches::PopperStyleApproachesPage />
                </Routes>
            </main>
        </Router>
    }
}
