use leptos::prelude::*;
use radix_leptos_direction::{Direction, DirectionProvider};
use radix_leptos_navigation_menu::*;

stylance::import_crate_style!(classes, "src/primitives/navigation_menu.stories.module.css");

#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <StoryFrame>
            <NavigationMenu>
                <NavigationMenuList attr:class=classes::mainList>
                    <NavigationMenuItem attr:class=classes::expandableItem>
                        <TriggerWithIndicator>"Products"</TriggerWithIndicator>
                        <NavigationMenuContent attr:class=classes::basicContent>
                            <LinkGroup
                                bordered=false
                                items=vec![
                                    "Fusce pellentesque",
                                    "Aliquam porttitor",
                                    "Pellentesque",
                                    "Fusce pellentesque",
                                    "Aliquam porttitor",
                                    "Pellentesque",
                                ]
                            />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem attr:class=classes::expandableItem>
                        <TriggerWithIndicator>"Company"</TriggerWithIndicator>
                        <NavigationMenuContent attr:class=classes::basicContent>
                            <LinkGroup
                                bordered=false
                                items=vec![
                                    "Fusce pellentesque",
                                    "Aliquam porttitor",
                                    "Pellentesque",
                                ]
                            />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem attr:class=classes::expandableItem>
                        <TriggerWithIndicator disabled=true>"Developers"</TriggerWithIndicator>
                        <NavigationMenuContent attr:class=classes::basicContent>
                            <LinkGroup
                                bordered=false
                                items=vec!["Aliquam porttitor", "Pellentesque"]
                            />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink attr:href="#example" attr:class=classes::link>
                            "Link"
                        </NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </StoryFrame>
    }
}

#[component]
pub fn CustomDurations() -> impl IntoView {
    view! {
        <div style="min-height: 100vh; background-color: #e5e8eb; padding-bottom: 150px; display: flex; flex-direction: column; align-items: center;">
            <h1>"Delay duration"</h1>
            <h2>"Default (200ms)"</h2>
            <DurationNavigation />

            <h2>"Custom (0ms = instant open)"</h2>
            <DurationNavigation delay_duration=0.0 />

            <h2>"Custom (700ms)"</h2>
            <DurationNavigation delay_duration=700.0 />

            <h1 style="margin-top: 50px;">"Skip delay duration"</h1>
            <h2>"Default (300ms to move from one trigger to another)"</h2>
            <DurationNavigation />

            <h2>"Custom (0ms to move from one trigger to another = never skip)"</h2>
            <DurationNavigation skip_delay_duration=0.0 />

            <h2>"Custom (2000ms to move from one trigger to another)"</h2>
            <DurationNavigation delay_duration=500.0 skip_delay_duration=2000.0 />
        </div>
    }
}

#[component]
pub fn Viewport() -> impl IntoView {
    view! {
        <StoryFrame>
            <NavigationMenu>
                <NavigationMenuList attr:class=classes::mainList>
                    <NavigationMenuItem>
                        <TriggerWithIndicator>"Products"</TriggerWithIndicator>
                        <NavigationMenuContent
                            attr:class=classes::viewportContent
                            attr:style="grid-template-columns: 1fr 2fr; width: 600px;"
                        >
                            <LinkGroup items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ] />
                            <LinkGroup items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ] />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <TriggerWithIndicator>"Company"</TriggerWithIndicator>
                        <NavigationMenuContent
                            attr:class=classes::viewportContent
                            attr:style="grid-template-columns: 1fr 1fr; width: 450px;"
                        >
                            <LinkGroup items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                                "Aliquam porttitor",
                            ] />
                            <LinkGroup items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ] />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <TriggerWithIndicator disabled=true>"Developers"</TriggerWithIndicator>
                        <NavigationMenuContent
                            attr:class=classes::viewportContent
                            attr:style="grid-template-columns: 1.6fr 1fr; width: 650px;"
                        >
                            <LinkGroup items=vec!["Donec quis dui", "Vestibulum"] />
                            <LinkGroup items=vec!["Fusce pellentesque", "Aliquam porttitor"] />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink attr:href="#example" attr:class=classes::link>
                            "Link"
                        </NavigationMenuLink>
                    </NavigationMenuItem>

                    <NavigationMenuIndicator attr:class=classes::viewportIndicator>
                        <div class=classes::viewportInnerIndicator />
                    </NavigationMenuIndicator>
                </NavigationMenuList>

                <div style="position: absolute; display: flex; justify-content: center; width: 100%; top: 100%; left: 0;">
                    <NavigationMenuViewport attr:class=classes::viewportViewport />
                </div>
            </NavigationMenu>
        </StoryFrame>
    }
}

#[component]
pub fn Submenus() -> impl IntoView {
    view! {
        <StoryFrame>
            <NavigationMenu>
                <NavigationMenuList attr:class=classes::mainList>
                    <NavigationMenuItem>
                        <TriggerWithIndicator>"Products"</TriggerWithIndicator>
                        <NavigationMenuContent attr:class=classes::submenusContent>
                            <NavigationMenuSub
                                attr:class=classes::submenusRoot
                                default_value="extensibility"
                            >
                                <NavigationMenuList attr:class=classes::mainList>
                                    <NavigationMenuItem value="extensibility">
                                        <NavigationMenuTrigger attr:class=classes::submenusSubTrigger>
                                            "Extensibility"
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent
                                            attr:class=classes::submenusSubContent
                                            attr:style="grid-template-columns: 1.5fr 1fr 1fr;"
                                        >
                                            <LinkGroup items=vec![
                                                "Donec quis dui",
                                                "Vestibulum",
                                                "Nunc dignissim",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>

                                    <NavigationMenuItem value="security">
                                        <NavigationMenuTrigger attr:class=classes::submenusSubTrigger>
                                            "Security"
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent
                                            attr:class=classes::submenusSubContent
                                            attr:style="grid-template-columns: 1fr 1fr 1fr;"
                                        >
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                                "Vestibulum",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                            ] />
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>

                                    <NavigationMenuItem value="authentication">
                                        <NavigationMenuTrigger attr:class=classes::submenusSubTrigger>
                                            "Authentication"
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent
                                            attr:class=classes::submenusSubContent
                                            attr:style="grid-template-columns: 1.5fr 1fr 1fr;"
                                        >
                                            <LinkGroup items=vec![
                                                "Donec quis dui",
                                                "Vestibulum",
                                                "Nunc dignissim",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>

                                    <NavigationMenuIndicator attr:class=classes::submenusSubIndicator />
                                </NavigationMenuList>

                                <NavigationMenuViewport attr:class=classes::submenusSubViewport />
                            </NavigationMenuSub>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <TriggerWithIndicator>"Company"</TriggerWithIndicator>
                        <NavigationMenuContent attr:class=classes::submenusContent>
                            <NavigationMenuSub
                                attr:class=classes::submenusRoot
                                orientation=Orientation::Vertical
                                default_value="customers"
                            >
                                <NavigationMenuList attr:class=classes::mainList>
                                    <NavigationMenuItem value="customers">
                                        <NavigationMenuTrigger attr:class=classes::submenusSubTrigger>
                                            "Customers"
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent
                                            attr:class=classes::submenusSubContent
                                            attr:style="grid-template-columns: 1.5fr 1fr;"
                                        >
                                            <LinkGroup items=vec![
                                                "Donec quis dui",
                                                "Vestibulum",
                                                "Nunc dignissim",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>

                                    <NavigationMenuItem value="partners">
                                        <NavigationMenuTrigger attr:class=classes::submenusSubTrigger>
                                            "Partners"
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent
                                            attr:class=classes::submenusSubContent
                                            attr:style="grid-template-columns: 1fr 1fr;"
                                        >
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                                "Vestibulum",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>

                                    <NavigationMenuItem value="enterprise">
                                        <NavigationMenuTrigger attr:class=classes::submenusSubTrigger>
                                            "Enterprise"
                                        </NavigationMenuTrigger>
                                        <NavigationMenuContent
                                            attr:class=classes::submenusSubContent
                                            attr:style="grid-template-columns: 1.5fr 1fr;"
                                        >
                                            <LinkGroup items=vec![
                                                "Donec quis dui",
                                                "Vestibulum",
                                                "Nunc dignissim",
                                            ] />
                                            <LinkGroup items=vec![
                                                "Fusce pellentesque",
                                                "Aliquam porttitor",
                                                "Pellentesque",
                                            ] />
                                        </NavigationMenuContent>
                                    </NavigationMenuItem>

                                    <NavigationMenuIndicator attr:class=classes::submenusSubIndicator />
                                </NavigationMenuList>

                                <NavigationMenuViewport attr:class=classes::submenusSubViewport />
                            </NavigationMenuSub>
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <TriggerWithIndicator disabled=true>"Developers"</TriggerWithIndicator>
                        <NavigationMenuContent
                            attr:class=classes::submenusSubContent
                            attr:style="grid-template-columns: 1fr 1fr;"
                        >
                            <LinkGroup items=vec!["Donec quis dui", "Vestibulum"] />
                            <LinkGroup items=vec!["Fusce pellentesque", "Aliquam porttitor"] />
                        </NavigationMenuContent>
                    </NavigationMenuItem>

                    <NavigationMenuItem>
                        <NavigationMenuLink attr:href="#example" attr:class=classes::link>
                            "Link"
                        </NavigationMenuLink>
                    </NavigationMenuItem>
                </NavigationMenuList>

                <NavigationMenuViewport attr:class=classes::submenusViewport />
            </NavigationMenu>
        </StoryFrame>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Helper components
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn StoryFrame(children: Children) -> impl IntoView {
    let (rtl, set_rtl) = signal(false);

    view! {
        <div style="height: 100vh; background-color: #e5e8eb;">
            <div style="display: flex; justify-content: center; padding-top: 20px; padding-bottom: 30px;">
                <label>
                    <input
                        type="checkbox"
                        prop:checked=move || rtl.get()
                        on:change=move |ev| {
                            set_rtl.set(event_target_checked(&ev));
                        }
                    />
                    " Right-to-left"
                </label>
            </div>
            <DirectionProvider direction=Signal::derive(move || if rtl.get() { Direction::Rtl } else { Direction::Ltr })>
                <div dir=move || if rtl.get() { "rtl" } else { "ltr" }>
                    <div style="position: relative; display: flex; box-sizing: border-box; align-items: center; padding: 15px 20px; justify-content: space-between; width: 100%; background-color: white; box-shadow: 0 50px 100px -20px rgba(50,50,93,0.1), 0 30px 60px -30px rgba(0,0,0,0.2);">
                        <button>"Logo"</button>
                        {children()}
                        <button>"Login"</button>
                    </div>
                    <div style="max-width: 800px; margin: auto; line-height: 1.5; padding-top: 25px;">
                        <h2>"Test page content"</h2>
                        <p>
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam purus odio, vestibulum in dictum et, "
                            <a href="#example">"sagittis vel nibh"</a>
                            ". Fusce placerat arcu lorem, a scelerisque odio fringilla sit amet. Suspendisse volutpat sed diam ut cursus. Nulla facilisi. Ut at volutpat nibh. Nullam justo mi, elementum vitae ex eu, "
                            <a href="#example">"gravida dictum metus"</a>
                            ". Morbi vulputate consectetur cursus. Fusce vitae nisi nunc. Suspendisse pellentesque aliquet tincidunt. Aenean molestie pulvinar ipsum."
                        </p>
                        <button>"Button"</button>
                    </div>
                </div>
            </DirectionProvider>
        </div>
    }
}

#[component]
fn DurationNavigation(
    #[prop(into, optional)] delay_duration: Option<f64>,
    #[prop(into, optional)] skip_delay_duration: Option<f64>,
) -> impl IntoView {
    view! {
        <NavigationMenu
            delay_duration=delay_duration.unwrap_or(200.0)
            skip_delay_duration=skip_delay_duration.unwrap_or(300.0)
            attr:style="background-color: white; border-radius: 500px; padding: 2px 12px;"
        >
            <NavigationMenuList attr:class=classes::mainList>
                <NavigationMenuItem attr:class=classes::expandableItem>
                    <TriggerWithIndicator>"Products"</TriggerWithIndicator>
                    <NavigationMenuContent attr:class=classes::basicContent>
                        <LinkGroup
                            bordered=false
                            items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ]
                        />
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class=classes::expandableItem>
                    <TriggerWithIndicator>"Company"</TriggerWithIndicator>
                    <NavigationMenuContent attr:class=classes::basicContent>
                        <LinkGroup
                            bordered=false
                            items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ]
                        />
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class=classes::expandableItem>
                    <TriggerWithIndicator>"Developers"</TriggerWithIndicator>
                    <NavigationMenuContent attr:class=classes::basicContent>
                        <LinkGroup
                            bordered=false
                            items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ]
                        />
                    </NavigationMenuContent>
                </NavigationMenuItem>

                <NavigationMenuItem attr:class=classes::expandableItem>
                    <TriggerWithIndicator>"About"</TriggerWithIndicator>
                    <NavigationMenuContent attr:class=classes::basicContent>
                        <LinkGroup
                            bordered=false
                            items=vec![
                                "Fusce pellentesque",
                                "Aliquam porttitor",
                                "Pellentesque",
                            ]
                        />
                    </NavigationMenuContent>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}

#[component]
fn TriggerWithIndicator(
    children: ChildrenFn,
    #[prop(optional, default = false)] disabled: bool,
) -> impl IntoView {
    let children = StoredValue::new(children);
    view! {
        <NavigationMenuTrigger attr:class=classes::trigger disabled=disabled>
            {children.with_value(|c| c())}
            <CaretDownIcon />
        </NavigationMenuTrigger>
    }
}

#[allow(non_snake_case)]
fn CaretDownIcon() -> impl IntoView {
    view! {
        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M4.18179 6.18181C4.35753 6.00608 4.64245 6.00608 4.81819 6.18181L7.49999 8.86362L10.1818 6.18181C10.3575 6.00608 10.6424 6.00608 10.8182 6.18181C10.9939 6.35755 10.9939 6.64247 10.8182 6.81821L7.81819 9.81821C7.73379 9.9026 7.61934 9.95001 7.49999 9.95001C7.38064 9.95001 7.26618 9.9026 7.18179 9.81821L4.18179 6.81821C4.00605 6.64247 4.00605 6.35755 4.18179 6.18181Z"
                fill="currentColor"
                fill-rule="evenodd"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
fn LinkGroup(
    items: Vec<&'static str>,
    #[prop(optional, default = true)] bordered: bool,
) -> impl IntoView {
    let class = if bordered {
        format!("{} {}", classes::borderdList, classes::list)
    } else {
        classes::list.to_string()
    };

    view! {
        <ul class=class>
            {items
                .into_iter()
                .map(|item| {
                    view! {
                        <li>
                            <NavigationMenuLink
                                attr:href="#example"
                                attr:style="display: flex; align-items: center; color: black;"
                            >
                                {item}
                            </NavigationMenuLink>
                        </li>
                    }
                })
                .collect_view()}
        </ul>
    }
}
