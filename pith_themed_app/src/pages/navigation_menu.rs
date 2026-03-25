use leptos::prelude::*;

use crate::theme::navigation_menu::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Navigation Menu"</h1>
                <p class="text-muted-foreground mb-6">
                    "A collection of links for navigating websites."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <ThemedNavigationMenu>
                    <ThemedNavigationMenuList>
                        <ThemedNavigationMenuItem>
                            <ThemedNavigationMenuTrigger>"Getting Started"</ThemedNavigationMenuTrigger>
                            <ThemedNavigationMenuContent>
                                <ul class="grid gap-3 p-4 md:w-[400px] lg:w-[500px] lg:grid-cols-[.75fr_1fr]">
                                    <li class="row-span-3">
                                        <ThemedNavigationMenuLink>
                                            <div class="flex h-full w-full select-none flex-col justify-end rounded-md bg-gradient-to-b from-muted/50 to-muted p-6 no-underline outline-none">
                                                <div class="mb-2 mt-4 text-lg font-medium">"Pith UI"</div>
                                                <p class="text-sm leading-tight text-muted-foreground">
                                                    "Beautifully designed components built with Radix UI and Tailwind CSS."
                                                </p>
                                            </div>
                                        </ThemedNavigationMenuLink>
                                    </li>
                                    <ListItem title="Introduction">"Re-usable components built using Radix UI and Tailwind CSS."</ListItem>
                                    <ListItem title="Installation">"How to install dependencies and structure your app."</ListItem>
                                    <ListItem title="Typography">"Styles for headings, paragraphs, lists, and more."</ListItem>
                                </ul>
                            </ThemedNavigationMenuContent>
                        </ThemedNavigationMenuItem>

                        <ThemedNavigationMenuItem>
                            <ThemedNavigationMenuTrigger>"Components"</ThemedNavigationMenuTrigger>
                            <ThemedNavigationMenuContent>
                                <ul class="grid w-[400px] gap-3 p-4 md:w-[500px] md:grid-cols-2 lg:w-[600px]">
                                    <ListItem title="Alert Dialog">"A modal dialog that interrupts the user."</ListItem>
                                    <ListItem title="Hover Card">"For sighted users to preview content."</ListItem>
                                    <ListItem title="Progress">"Displays an indicator of completion."</ListItem>
                                    <ListItem title="Scroll Area">"Augments native scroll functionality."</ListItem>
                                    <ListItem title="Tabs">"A set of layered sections of content."</ListItem>
                                    <ListItem title="Tooltip">"A popup that shows info on hover."</ListItem>
                                </ul>
                            </ThemedNavigationMenuContent>
                        </ThemedNavigationMenuItem>

                        <ThemedNavigationMenuItem>
                            <ThemedNavigationMenuLink>
                                <a class="block select-none rounded-md p-3 text-sm font-medium leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground" href="#">
                                    "Documentation"
                                </a>
                            </ThemedNavigationMenuLink>
                        </ThemedNavigationMenuItem>
                        <ThemedNavigationMenuIndicator>
                            <div class="relative top-[60%] size-2 rotate-45 rounded-tl-sm bg-border shadow-md" />
                        </ThemedNavigationMenuIndicator>
                    </ThemedNavigationMenuList>

                    <div class="absolute left-0 top-full flex justify-center">
                        <ThemedNavigationMenuViewport />
                    </div>
                </ThemedNavigationMenu>
            </section>
        </div>
    }
}

#[component]
fn ListItem(
    title: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <li>
            <ThemedNavigationMenuLink>
                <a class="block select-none space-y-1 rounded-md p-3 leading-none no-underline outline-none transition-colors hover:bg-accent hover:text-accent-foreground focus:bg-accent focus:text-accent-foreground" href="#">
                    <div class="text-sm font-medium leading-none">{title}</div>
                    <p class="line-clamp-2 text-sm leading-snug text-muted-foreground">
                        {children.with_value(|children| children())}
                    </p>
                </a>
            </ThemedNavigationMenuLink>
        </li>
    }
}
