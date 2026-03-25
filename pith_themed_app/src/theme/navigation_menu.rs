use pith_ui::navigation_menu::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york navigation-menu
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str = "relative flex max-w-max flex-1 items-center justify-center";

const LIST_CLASS: &str = "group flex flex-1 list-none items-center justify-center gap-1";

const TRIGGER_CLASS: &str = "group inline-flex h-9 w-max items-center justify-center rounded-md bg-background px-4 py-2 text-sm font-medium outline-none transition-colors hover:bg-accent hover:text-accent-foreground menu-item-focus data-[state=open]:bg-accent/50 disabled:disabled-base";

const CONTENT_CLASS: &str = "absolute top-0 left-0 w-full sm:w-auto";

const LINK_CLASS: &str = "block select-none rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground menu-item-focus";

const VIEWPORT_CLASS: &str = "relative mt-1.5 h-[var(--radix-navigation-menu-viewport-height)] w-full overflow-hidden rounded-md border bg-popover text-popover-foreground shadow-md md:w-[var(--radix-navigation-menu-viewport-width)]";

const INDICATOR_CLASS: &str = "top-full z-[1] flex h-1.5 items-end justify-center overflow-hidden";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedNavigationMenu(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(ROOT_CLASS);

    view! {
        <NavigationMenu attr:class=class.get_value()>
            {children()}
        </NavigationMenu>
    }
}

#[component]
pub fn ThemedNavigationMenuList(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(LIST_CLASS);

    view! {
        <NavigationMenuList attr:class=class.get_value()>
            {children()}
        </NavigationMenuList>
    }
}

#[component]
pub fn ThemedNavigationMenuItem(
    #[prop(into, optional)] value: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView {
    view! {
        <NavigationMenuItem value=value>
            {children()}
        </NavigationMenuItem>
    }
}

#[component]
pub fn ThemedNavigationMenuTrigger(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(TRIGGER_CLASS);
    let children = StoredValue::new(children);

    view! {
        <NavigationMenuTrigger attr:class=class.get_value()>
            {children.with_value(|children| children())}
            <ChevronDownIcon />
        </NavigationMenuTrigger>
    }
}

#[component]
pub fn ThemedNavigationMenuContent(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(CONTENT_CLASS);
    let children = StoredValue::new(children);

    view! {
        <NavigationMenuContent attr:class=class.get_value()>
            {children.with_value(|children| children())}
        </NavigationMenuContent>
    }
}

#[component]
pub fn ThemedNavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let class = StoredValue::new(LINK_CLASS);
    let children = StoredValue::new(children);

    view! {
        <NavigationMenuLink attr:class=class.get_value() active=active>
            {children.with_value(|children| children())}
        </NavigationMenuLink>
    }
}

#[component]
pub fn ThemedNavigationMenuViewport() -> impl IntoView {
    let class = StoredValue::new(VIEWPORT_CLASS);

    view! {
        <NavigationMenuViewport attr:class=class.get_value() />
    }
}

#[allow(dead_code)]
#[component]
pub fn ThemedNavigationMenuIndicator(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(INDICATOR_CLASS);

    view! {
        <NavigationMenuIndicator attr:class=class.get_value()>
            {children()}
        </NavigationMenuIndicator>
    }
}

// ---------------------------------------------------------------------------
// Shared icons
// ---------------------------------------------------------------------------

#[component]
fn ChevronDownIcon() -> impl IntoView {
    view! {
        <svg
            class="relative top-px ml-1 size-3 transition-transform duration-200 group-data-[state=open]:rotate-180"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m6 9 6 6 6-6" />
        </svg>
    }
}
