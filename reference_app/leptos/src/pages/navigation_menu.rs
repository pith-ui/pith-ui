use leptos::prelude::*;
use radix_leptos_primitives::navigation_menu::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    let (controlled_value, set_controlled_value) = signal(String::new());

    view! {
        <NavigationMenu attr:class="nav-root" attr:data-testid="nav-root" delay_duration=0.0 skip_delay_duration=0.0>
            <NavigationMenuList attr:class="nav-list">
                <NavigationMenuItem attr:class="nav-item" value="products".to_string()>
                    <NavigationMenuTrigger attr:class="nav-trigger">
                        "Products"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content nav-content-products" attr:data-testid="products-content" attr:style="grid-template-columns: 1fr 1fr;">
                        <div class="nav-content-group" data-testid="products-featured">
                            <h3 class="nav-group-heading">"Featured"</h3>
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
                            </ul>
                        </div>
                        <div class="nav-content-group" data-testid="products-all">
                            <h3 class="nav-group-heading">"All Products"</h3>
                            <ul class="nav-content-list">
                                <li>
                                    <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                        "Product C"
                                    </NavigationMenuLink>
                                </li>
                            </ul>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class="nav-item" value="resources".to_string()>
                    <NavigationMenuTrigger attr:class="nav-trigger">
                        "Resources"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent attr:class="nav-content nav-content-resources" attr:data-testid="resources-content">
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

        <hr />

        <h3>"Controlled"</h3>
        <button
            data-testid="set-products"
            on:click=move |_| set_controlled_value.set("c-products".to_string())
        >
            "open products"
        </button>
        <button
            data-testid="set-resources"
            on:click=move |_| set_controlled_value.set("c-resources".to_string())
        >
            "open resources"
        </button>
        <button
            data-testid="close-all"
            on:click=move |_| set_controlled_value.set(String::new())
        >
            "close all"
        </button>
        <span data-testid="controlled-nav-value">
            {move || {
                let v = controlled_value.get();
                if v.is_empty() { "(none)".to_string() } else { v }
            }}
        </span>

        <NavigationMenu
            attr:class="nav-root"
            attr:data-testid="controlled-nav-root"
            value=Signal::derive(move || controlled_value.get())
            on_value_change=Callback::new(move |v: String| set_controlled_value.set(v))
            delay_duration=0.0
            skip_delay_duration=0.0
        >
            <NavigationMenuList attr:class="nav-list">
                <NavigationMenuItem attr:class="nav-item" value="c-products".to_string()>
                    <NavigationMenuTrigger
                        attr:class="nav-trigger"
                        attr:data-testid="controlled-products-trigger"
                    >
                        "CProducts"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent
                        attr:class="nav-content"
                        attr:data-testid="controlled-products-content"
                    >
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "CProduct A"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class="nav-item" value="c-resources".to_string()>
                    <NavigationMenuTrigger
                        attr:class="nav-trigger"
                        attr:data-testid="controlled-resources-trigger"
                    >
                        "CResources"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent
                        attr:class="nav-content"
                        attr:data-testid="controlled-resources-content"
                    >
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "CBlog"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>
            </NavigationMenuList>

            <NavigationMenuViewport attr:class="nav-viewport" attr:data-testid="controlled-nav-viewport" />
        </NavigationMenu>
    }
}
