use leptos::prelude::*;
use radix_leptos_direction::DirectionProvider;
use radix_leptos_toggle_group::*;

stylance::import_crate_style!(classes, "src/primitives/toggle-group.stories.module.css");

#[component]
pub fn Single() -> impl IntoView {
    let (value, set_value) = signal::<Vec<String>>(vec![]);
    let on_change = Callback::new(move |v| set_value.set(v));

    view! {
        <h1>"Uncontrolled"</h1>
        <ToggleGroup
            r#type=ToggleGroupType::Single
            default_value=vec!["1".to_string()]
            attr:class=classes::root
            attr:aria-label="Options"
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h1>"Controlled"</h1>
        <ToggleGroup
            r#type=ToggleGroupType::Single
            value=value
            on_value_change=on_change
            attr:class=classes::root
            attr:aria-label="Options"
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}

#[component]
pub fn Vertical() -> impl IntoView {
    view! {
        <ToggleGroup
            r#type=ToggleGroupType::Single
            orientation=radix_leptos_roving_focus::Orientation::Vertical
            default_value=vec!["1".to_string()]
            attr:class=classes::root
            attr:aria-label="Options"
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}

#[component]
pub fn Multiple() -> impl IntoView {
    let (value, set_value) = signal::<Vec<String>>(vec![]);
    let on_change = Callback::new(move |v| set_value.set(v));

    view! {
        <h1>"Uncontrolled"</h1>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            default_value=vec!["1".to_string()]
            attr:class=classes::root
            attr:aria-label="Options"
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h1>"Controlled"</h1>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            value=value
            on_value_change=on_change
            attr:class=classes::root
            attr:aria-label="Options"
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Single"</h1>
        <h2>"Off"</h2>
        <ToggleGroup r#type=ToggleGroupType::Single attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"On"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Single
            default_value=vec!["1".to_string()]
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"Disabled"</h2>
        <ToggleGroup r#type=ToggleGroupType::Single disabled=true attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h1>"Multiple"</h1>
        <h2>"Off"</h2>
        <ToggleGroup r#type=ToggleGroupType::Multiple attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"One on"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            default_value=vec!["1".to_string()]
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"One and two on"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            default_value=vec!["1".to_string(), "2".to_string()]
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"Disabled"</h2>
        <ToggleGroup r#type=ToggleGroupType::Multiple disabled=true attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h1>"Direction"</h1>
        <h2>"Prop"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Single
            default_value=vec!["1".to_string()]
            dir=radix_leptos_direction::Direction::Rtl
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"Inherited"</h2>
        <DirectionProvider direction=Signal::derive(|| radix_leptos_direction::Direction::Rtl)>
            <ToggleGroup
                r#type=ToggleGroupType::Single
                default_value=vec!["1".to_string()]
                attr:class=classes::root
            >
                <ToggleGroupItem value="1" attr:class=classes::item>
                    "Option 1"
                </ToggleGroupItem>
                <ToggleGroupItem value="2" attr:class=classes::item>
                    "Option 2"
                </ToggleGroupItem>
                <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                    "Option 3"
                </ToggleGroupItem>
            </ToggleGroup>
        </DirectionProvider>

        <h1>"State attributes"</h1>
        <h2>"Group disabled"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            default_value=vec!["1".to_string(), "2".to_string()]
            disabled=true
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::itemAttr>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::itemAttr>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::itemAttr>
                "Option 3"
            </ToggleGroupItem>
            <ToggleGroupItem value="4" attr:class=classes::itemAttr>
                "Option 4"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"Group enabled with button override"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            default_value=vec!["1".to_string(), "2".to_string()]
            disabled=false
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::itemAttr>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" disabled=true attr:class=classes::itemAttr>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::itemAttr>
                "Option 3"
            </ToggleGroupItem>
            <ToggleGroupItem value="4" disabled=true attr:class=classes::itemAttr>
                "Option 4"
            </ToggleGroupItem>
        </ToggleGroup>

        <h2>"Group disabled with button override"</h2>
        <ToggleGroup
            r#type=ToggleGroupType::Multiple
            default_value=vec!["1".to_string(), "2".to_string()]
            disabled=true
            attr:class=classes::root
        >
            <ToggleGroupItem value="1" attr:class=classes::itemAttr>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" disabled=false attr:class=classes::itemAttr>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::itemAttr>
                "Option 3"
            </ToggleGroupItem>
            <ToggleGroupItem value="4" disabled=false attr:class=classes::itemAttr>
                "Option 4"
            </ToggleGroupItem>
        </ToggleGroup>
    }
}
