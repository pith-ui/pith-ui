use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york card
// ---------------------------------------------------------------------------

const CARD_CLASS: &str =
    "flex flex-col gap-6 rounded-xl border bg-card py-6 text-card-foreground shadow-sm";

const CARD_HEADER_CLASS: &str =
    "grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-6";

const CARD_TITLE_CLASS: &str = "text-lg leading-none font-semibold";

const CARD_DESCRIPTION_CLASS: &str = "text-sm text-muted-foreground";

const CARD_CONTENT_CLASS: &str = "px-6";

const CARD_FOOTER_CLASS: &str = "flex items-center px-6";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class=CARD_CLASS>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(children: Children) -> impl IntoView {
    view! {
        <div class=CARD_HEADER_CLASS>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(children: Children) -> impl IntoView {
    view! {
        <h3 class=CARD_TITLE_CLASS>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(children: Children) -> impl IntoView {
    view! {
        <p class=CARD_DESCRIPTION_CLASS>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(children: Children) -> impl IntoView {
    view! {
        <div class=CARD_CONTENT_CLASS>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
    view! {
        <div class=CARD_FOOTER_CLASS>
            {children()}
        </div>
    }
}
