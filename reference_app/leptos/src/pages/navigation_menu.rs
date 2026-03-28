use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavigationMenuPage() -> impl IntoView {
    let (controlled_value, set_controlled_value) = signal(String::new());

    view! {
        <NavigationMenu class:nav-root=true attr:data-testid="nav-root" attr:aria-label="Main" delay_duration=0.0 skip_delay_duration=0.0>
            <NavigationMenuList class:nav-list=true>
                <NavigationMenuItem class:nav-item=true value="products".to_string()>
                    <NavigationMenuTrigger class:nav-trigger=true>
                        "Products"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent class:nav-content=true class:nav-content-products=true attr:data-testid="products-content" style:grid-template-columns="1fr 1fr">
                        <div class="nav-content-group" data-testid="products-featured">
                            <h3 class="nav-group-heading">"Featured"</h3>
                            <ul class="nav-content-list">
                                <li>
                                    <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                        "Product A"
                                    </NavigationMenuLink>
                                </li>
                                <li>
                                    <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                        "Product B"
                                    </NavigationMenuLink>
                                </li>
                            </ul>
                        </div>
                        <div class="nav-content-group" data-testid="products-all">
                            <h3 class="nav-group-heading">"All Products"</h3>
                            <ul class="nav-content-list">
                                <li>
                                    <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                        "Product C"
                                    </NavigationMenuLink>
                                </li>
                            </ul>
                        </div>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem class:nav-item=true value="resources".to_string()>
                    <NavigationMenuTrigger class:nav-trigger=true>
                        "Resources"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent class:nav-content=true class:nav-content-resources=true attr:data-testid="resources-content">
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                    "Blog"
                                </NavigationMenuLink>
                            </li>
                            <li>
                                <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                    "Docs"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem class:nav-item=true>
                    <NavigationMenuLink class:nav-link-direct=true attr:href="#" active=true>
                        "About"
                    </NavigationMenuLink>
                </NavigationMenuItem>

                <NavigationMenuIndicator class:nav-indicator=true attr:data-testid="nav-indicator" />
            </NavigationMenuList>

            <NavigationMenuViewport class:nav-viewport=true attr:data-testid="nav-viewport" />
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
            class:nav-root=true
            attr:data-testid="controlled-nav-root"
            attr:aria-label="Controlled"
            value=Signal::derive(move || controlled_value.get())
            on_value_change=Callback::new(move |v: String| set_controlled_value.set(v))
            delay_duration=0.0
            skip_delay_duration=0.0
        >
            <NavigationMenuList class:nav-list=true>
                <NavigationMenuItem class:nav-item=true value="c-products".to_string()>
                    <NavigationMenuTrigger
                        class:nav-trigger=true
                        attr:data-testid="controlled-products-trigger"
                    >
                        "CProducts"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent
                        class:nav-content=true
                        attr:data-testid="controlled-products-content"
                    >
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                    "CProduct A"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem class:nav-item=true value="c-resources".to_string()>
                    <NavigationMenuTrigger
                        class:nav-trigger=true
                        attr:data-testid="controlled-resources-trigger"
                    >
                        "CResources"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent
                        class:nav-content=true
                        attr:data-testid="controlled-resources-content"
                    >
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink class:nav-content-link=true attr:href="#">
                                    "CBlog"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>
            </NavigationMenuList>

            <NavigationMenuViewport class:nav-viewport=true attr:data-testid="controlled-nav-viewport" />
        </NavigationMenu>
    }
}
