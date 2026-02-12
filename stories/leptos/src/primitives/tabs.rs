use leptos::prelude::*;
use radix_leptos_direction::DirectionProvider;
use radix_leptos_tabs::*;

stylance::import_crate_style!(classes, "src/primitives/tabs.stories.module.css");

fn animated_content_class() -> String {
    format!("{} {}", classes::content, classes::animatedContent)
}

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <h1>"Horizontal (automatic activation)"</h1>
        <Tabs default_value="tab1".to_string() attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h1>"Vertical (manual activation)"</h1>
        <Tabs
            default_value="tab1".to_string()
            attr:class=classes::root
            orientation=Orientation::Vertical
            activation_mode=ActivationMode::Manual
        >
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    let animated_class = StoredValue::new(animated_content_class());

    view! {
        <h1>"Horizontal (automatic activation)"</h1>
        <Tabs default_value="tab1".to_string() attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=animated_class.get_value()>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=animated_class.get_value()>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=animated_class.get_value()>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h1>"Vertical (manual activation)"</h1>
        <Tabs
            default_value="tab1".to_string()
            attr:class=classes::root
            orientation=Orientation::Vertical
            activation_mode=ActivationMode::Manual
        >
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=animated_class.get_value()>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=animated_class.get_value()>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=animated_class.get_value()>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    let animated_class = StoredValue::new(animated_content_class());

    view! {
        <h1>"Uncontrolled"</h1>
        <Tabs default_value="tab3".to_string() attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h1>"Controlled"</h1>
        <Tabs value="tab3".to_string() attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h1>"Vertical"</h1>
        <Tabs
            default_value="tab3".to_string()
            attr:class=classes::root
            orientation=Orientation::Vertical
            activation_mode=ActivationMode::Manual
        >
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h1>"Direction"</h1>
        <h2>"Prop"</h2>
        <Tabs default_value="tab3".to_string() dir=radix_leptos_direction::Direction::Rtl attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h2>"Inherited"</h2>
        <DirectionProvider direction=Signal::derive(|| radix_leptos_direction::Direction::Rtl)>
            <Tabs default_value="tab3".to_string() attr:class=classes::root>
                <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                    <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                        "Tab 1"
                    </TabsTrigger>
                    <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                        "Tab 2"
                    </TabsTrigger>
                    <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                        "Tab 3"
                    </TabsTrigger>
                </TabsList>
                <TabsContent value="tab1".to_string() attr:class=classes::content>
                    "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
                </TabsContent>
                <TabsContent value="tab2".to_string() attr:class=classes::content>
                    "You'll never find me!"
                </TabsContent>
                <TabsContent value="tab3".to_string() attr:class=classes::content>
                    "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
                </TabsContent>
            </Tabs>
        </DirectionProvider>

        <h1>"Animated"</h1>
        <p>"Should not animate on initial mount"</p>
        <Tabs value="tab1".to_string() attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent
                value="tab1".to_string()
                attr:class=animated_class.get_value()
                attr:style="animation-duration: 3000ms;"
            >
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=animated_class.get_value()>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=animated_class.get_value()>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>

        <h1>"Force mounted contents"</h1>
        <Tabs attr:class=classes::root>
            <TabsList attr:aria-label="tabs example" attr:class=classes::list>
                <TabsTrigger value="tab1".to_string() attr:class=classes::trigger>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() attr:class=classes::trigger>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::trigger>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::content force_mount=true>
                "Tab 1 content"
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::content force_mount=true>
                "Tab 2 content"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::content force_mount=true>
                "Tab 3 content"
            </TabsContent>
        </Tabs>

        <h1>"State attributes"</h1>
        <Tabs default_value="tab3".to_string() attr:class=classes::rootAttr>
            <TabsList attr:aria-label="tabs example" attr:class=classes::listAttr>
                <TabsTrigger value="tab1".to_string() attr:class=classes::triggerAttr>
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class=classes::triggerAttr>
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class=classes::triggerAttr>
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class=classes::contentAttr>
                "Dis metus rhoncus sit convallis sollicitudin vel cum, hac purus tincidunt eros sem himenaeos integer, faucibus varius nullam nostra bibendum consectetur mollis, gravida elementum pellentesque volutpat dictum ipsum."
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class=classes::contentAttr>
                "You'll never find me!"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class=classes::contentAttr>
                "Ut nisi elementum metus semper mauris dui fames accumsan aenean, maecenas ac sociis dolor quam tempus pretium."
            </TabsContent>
        </Tabs>
    }
}
