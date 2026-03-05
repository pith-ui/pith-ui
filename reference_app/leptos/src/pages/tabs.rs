use leptos::prelude::*;
use radix_leptos_primitives::tabs::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    let (orientation, set_orientation) = signal(Orientation::Horizontal);
    let (activation_mode, set_activation_mode) = signal(ActivationMode::Automatic);

    view! {
        <Tabs
            default_value="tab1".to_string()
            orientation=orientation
            activation_mode=activation_mode
            attr:class="tabs-root"
        >
            <TabsList attr:aria-label="tabs example" attr:class="tabs-list">
                <TabsTrigger value="tab1".to_string() attr:class="tabs-trigger">
                    "Tab 1"
                </TabsTrigger>
                <TabsTrigger value="tab2".to_string() disabled=true attr:class="tabs-trigger">
                    "Tab 2"
                </TabsTrigger>
                <TabsTrigger value="tab3".to_string() attr:class="tabs-trigger">
                    "Tab 3"
                </TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string() attr:class="tabs-content">
                "Content 1"
            </TabsContent>
            <TabsContent value="tab2".to_string() attr:class="tabs-content">
                "Content 2"
            </TabsContent>
            <TabsContent value="tab3".to_string() attr:class="tabs-content">
                "Content 3"
            </TabsContent>
        </Tabs>

        <br />
        <br />

        <fieldset>
            <legend>"Orientation"</legend>
            <label>
                <input
                    type="radio"
                    name="orientation"
                    value="horizontal"
                    prop:checked=move || orientation.get() == Orientation::Horizontal
                    on:change=move |_| set_orientation.set(Orientation::Horizontal)
                />
                " horizontal"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="orientation"
                    value="vertical"
                    prop:checked=move || orientation.get() == Orientation::Vertical
                    on:change=move |_| set_orientation.set(Orientation::Vertical)
                />
                " vertical"
            </label>
        </fieldset>

        <fieldset>
            <legend>"Activation Mode"</legend>
            <label>
                <input
                    type="radio"
                    name="activation"
                    value="automatic"
                    prop:checked=move || activation_mode.get() == ActivationMode::Automatic
                    on:change=move |_| set_activation_mode.set(ActivationMode::Automatic)
                />
                " automatic"
            </label>
            <br />
            <label>
                <input
                    type="radio"
                    name="activation"
                    value="manual"
                    prop:checked=move || activation_mode.get() == ActivationMode::Manual
                    on:change=move |_| set_activation_mode.set(ActivationMode::Manual)
                />
                " manual"
            </label>
        </fieldset>
    }
}
