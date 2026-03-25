use leptos::prelude::*;
use pith_ui::tabs::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    let (orientation, set_orientation) = signal(Orientation::Horizontal);
    let (activation_mode, set_activation_mode) = signal(ActivationMode::Automatic);

    // Controlled tabs state
    let (controlled_value, set_controlled_value) = signal("ctab1".to_string());

    view! {
        <div data-testid="uncontrolled-tabs-section">
            <Tabs
                default_value="tab1".to_string()
                orientation=orientation
                activation_mode=activation_mode
                class:tabs-root=true
                attr:data-custom="tabs-root-custom"
            >
                <TabsList attr:aria-label="tabs example" class:tabs-list=true attr:data-custom="tabs-list-custom">
                    <TabsTrigger value="tab1".to_string() class:tabs-trigger=true attr:data-custom="tabs-trigger-custom">
                        "Tab 1"
                    </TabsTrigger>
                    <TabsTrigger value="tab2".to_string() disabled=true class:tabs-trigger=true>
                        "Tab 2"
                    </TabsTrigger>
                    <TabsTrigger value="tab3".to_string() class:tabs-trigger=true>
                        "Tab 3"
                    </TabsTrigger>
                </TabsList>
                <TabsContent value="tab1".to_string() class:tabs-content=true attr:data-custom="tabs-content-custom">
                    "Content 1"
                </TabsContent>
                <TabsContent value="tab2".to_string() class:tabs-content=true>
                    "Content 2"
                </TabsContent>
                <TabsContent value="tab3".to_string() class:tabs-content=true>
                    "Content 3"
                </TabsContent>
            </Tabs>
        </div>

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

        // Force-mounted tabs for hidden attribute test
        <div data-testid="force-mount-tabs-section" aria-hidden="true">
            <Tabs default_value="fm1".to_string() class:tabs-root=true>
                <TabsList attr:aria-label="force mount tabs" class:tabs-list=true>
                    <TabsTrigger value="fm1".to_string() class:tabs-trigger=true attr:data-testid="fm-trigger-1">
                        "FM 1"
                    </TabsTrigger>
                    <TabsTrigger value="fm2".to_string() class:tabs-trigger=true attr:data-testid="fm-trigger-2">
                        "FM 2"
                    </TabsTrigger>
                </TabsList>
                <TabsContent value="fm1".to_string() class:tabs-content=true attr:data-testid="fm-content-1" force_mount=true>
                    "FM Content 1"
                </TabsContent>
                <TabsContent value="fm2".to_string() class:tabs-content=true attr:data-testid="fm-content-2" force_mount=true>
                    "FM Content 2"
                </TabsContent>
            </Tabs>
        </div>

        <hr />

        // ── Controlled Tabs ──
        <div data-testid="controlled-tabs-section" aria-hidden="true">
            <h3>"Controlled Tabs"</h3>

            <fieldset>
                <legend>"External Tab Control"</legend>
                <button
                    data-testid="controlled-select-tab1"
                    on:click=move |_| set_controlled_value.set("ctab1".to_string())
                >
                    "Select Tab 1"
                </button>
                <button
                    data-testid="controlled-select-tab2"
                    on:click=move |_| set_controlled_value.set("ctab2".to_string())
                >
                    "Select Tab 2"
                </button>
                <button
                    data-testid="controlled-select-tab3"
                    on:click=move |_| set_controlled_value.set("ctab3".to_string())
                >
                    "Select Tab 3"
                </button>
            </fieldset>

            <span data-testid="controlled-value-display">{move || controlled_value.get()}</span>

            <Tabs
                value=Signal::derive(move || Some(controlled_value.get()))
                on_value_change=Callback::new(move |v: String| set_controlled_value.set(v))
                class:tabs-root=true
                attr:data-testid="controlled-tabs"
            >
                <TabsList attr:aria-label="controlled tabs example" class:tabs-list=true>
                    <TabsTrigger value="ctab1".to_string() class:tabs-trigger=true attr:data-testid="controlled-tab-trigger-1">
                        "CTab 1"
                    </TabsTrigger>
                    <TabsTrigger value="ctab2".to_string() class:tabs-trigger=true attr:data-testid="controlled-tab-trigger-2">
                        "CTab 2"
                    </TabsTrigger>
                    <TabsTrigger value="ctab3".to_string() class:tabs-trigger=true attr:data-testid="controlled-tab-trigger-3">
                        "CTab 3"
                    </TabsTrigger>
                </TabsList>
                <TabsContent value="ctab1".to_string() class:tabs-content=true attr:data-testid="controlled-tab-content-1">
                    "Controlled Content 1"
                </TabsContent>
                <TabsContent value="ctab2".to_string() class:tabs-content=true attr:data-testid="controlled-tab-content-2">
                    "Controlled Content 2"
                </TabsContent>
                <TabsContent value="ctab3".to_string() class:tabs-content=true attr:data-testid="controlled-tab-content-3">
                    "Controlled Content 3"
                </TabsContent>
            </Tabs>
        </div>
    }
}
