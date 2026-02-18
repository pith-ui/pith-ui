use leptos::prelude::*;
use radix_leptos_navigation_menu::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    view! {
        <NavigationMenu attr:class="nav-root" attr:data-testid="nav-root" delay_duration=0.0 skip_delay_duration=0.0>
            <NavigationMenuList attr:class="nav-list">
                <NavigationMenuItem attr:class="nav-item" value="products".to_string()>
                    <NavigationMenuTrigger attr:class="nav-trigger">
                        "Products"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content" attr:data-testid="products-content">
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Product A"
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Product B"
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Product C"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class="nav-item" value="resources".to_string()>
                    <NavigationMenuTrigger attr:class="nav-trigger">
                        "Resources"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content" attr:data-testid="resources-content">
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Blog"
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Docs"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class="nav-item">
                    <NavigationMenuLink attr:class="nav-link-direct" attr:href="#" active=true>
                        "About"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuIndicator attr:class="nav-indicator" attr:data-testid="nav-indicator" />
            </NavigationMenuList>

            <NavigationMenuViewport attr:class="nav-viewport" attr:data-testid="nav-viewport" />
        </NavigationMenu>

        <br />
        <br />

        <button data-testid="outside-element">"outside"</button>
    }
}
