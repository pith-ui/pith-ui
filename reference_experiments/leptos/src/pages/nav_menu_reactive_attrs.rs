// ── Experiment: NavigationMenu Reactive Attr Forwarding ──────────────────────
//
// Hypothesis: Reactive attrs (signal-driven) on NavigationMenuContent are
// frozen by extract_attrs when rendered through a Viewport. The inline path
// (no viewport) spreads {..attrs} directly and should preserve reactivity.
//
// This experiment tests both paths:
// 1. With viewport (standard usage) — attrs forwarded via ContentData.extra_attrs
// 2. Without viewport (if possible) — attrs spread directly via {..attrs}

use leptos::prelude::*;
use pith_ui::navigation_menu::*;

#[component]
pub fn NavMenuReactiveAttrsPage() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <h1>"Experiment: NavigationMenu Reactive Attrs"</h1>

        <h2>"With Viewport (standard)"</h2>
        <p>"Reactive data-count attr on NavigationMenuContent, rendered through Viewport."</p>

        <button
            data-testid="increment"
            on:click=move |_| set_count.update(|c| *c += 1)
        >
            "Increment: " {move || count.get().to_string()}
        </button>

        <NavigationMenu
            attr:class="nav-root"
            attr:data-testid="test-nav"
            delay_duration=0.0
            skip_delay_duration=0.0
        >
            <NavigationMenuList attr:class="nav-list">
                <NavigationMenuItem attr:class="nav-item" value="item-a".to_string()>
                    <NavigationMenuTrigger
                        attr:class="nav-trigger"
                        attr:data-testid="trigger-a"
                    >
                        "Item A"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent
                        attr:class="nav-content"
                        attr:data-testid="content-a"
                        attr:data-count=move || count.get().to_string()
                    >
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Link A"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class="nav-item" value="item-b".to_string()>
                    <NavigationMenuTrigger
                        attr:class="nav-trigger"
                        attr:data-testid="trigger-b"
                    >
                        "Item B"
                    </NavigationMenuTrigger>
                    <NavigationMenuContent
                        attr:class="nav-content"
                        attr:data-testid="content-b"
                    >
                        <ul class="nav-content-list">
                            <li>
                                <NavigationMenuLink attr:class="nav-content-link" attr:href="#">
                                    "Link B"
                                </NavigationMenuLink>
                            </li>
                        </ul>
                    </NavigationMenuContent>
                </NavigationMenuItem>
            </NavigationMenuList>

            <NavigationMenuViewport
                attr:class="nav-viewport"
                attr:data-testid="test-viewport"
            />
        </NavigationMenu>

        <button data-testid="outside">"outside"</button>
    }
}
