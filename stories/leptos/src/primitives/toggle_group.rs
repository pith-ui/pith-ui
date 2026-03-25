use leptos::prelude::*;
use pith_ui::direction::DirectionProvider;
use pith_ui::toggle_group::*;

stylance::import_crate_style!(classes, "src/primitives/toggle-group.stories.module.css");

#[component]
pub fn Single() -> impl IntoView {
    let (value, set_value) = signal::<String>(String::new());
    let on_change = Callback::new(move |v: String| set_value.set(v));

    view! {
        <h1>"Uncontrolled"</h1>
        <ToggleGroupSingle
            default_value="1".to_string()
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
        </ToggleGroupSingle>

        <h1>"Controlled"</h1>
        <ToggleGroupSingle
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
        </ToggleGroupSingle>
    }
}

#[component]
pub fn Vertical() -> impl IntoView {
    view! {
        <ToggleGroupSingle
            orientation=pith_ui::roving_focus::Orientation::Vertical
            default_value="1".to_string()
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
        </ToggleGroupSingle>
    }
}

#[component]
pub fn Multiple() -> impl IntoView {
    let (value, set_value) = signal::<Vec<String>>(vec![]);
    let on_change = Callback::new(move |v: Vec<String>| set_value.set(v));

    view! {
        <h1>"Uncontrolled"</h1>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>

        <h1>"Controlled"</h1>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>"Single"</h1>
        <h2>"Off"</h2>
        <ToggleGroupSingle attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroupSingle>

        <h2>"On"</h2>
        <ToggleGroupSingle
            default_value="1".to_string()
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
        </ToggleGroupSingle>

        <h2>"Disabled"</h2>
        <ToggleGroupSingle disabled=true attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroupSingle>

        <h1>"Multiple"</h1>
        <h2>"Off"</h2>
        <ToggleGroupMultiple attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" disabled=true attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroupMultiple>

        <h2>"One on"</h2>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>

        <h2>"One and two on"</h2>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>

        <h2>"Disabled"</h2>
        <ToggleGroupMultiple disabled=true attr:class=classes::root>
            <ToggleGroupItem value="1" attr:class=classes::item>
                "Option 1"
            </ToggleGroupItem>
            <ToggleGroupItem value="2" attr:class=classes::item>
                "Option 2"
            </ToggleGroupItem>
            <ToggleGroupItem value="3" attr:class=classes::item>
                "Option 3"
            </ToggleGroupItem>
        </ToggleGroupMultiple>

        <h1>"Direction"</h1>
        <h2>"Prop"</h2>
        <ToggleGroupSingle
            default_value="1".to_string()
            dir=pith_ui::direction::Direction::Rtl
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
        </ToggleGroupSingle>

        <h2>"Inherited"</h2>
        <DirectionProvider direction=Signal::derive(|| pith_ui::direction::Direction::Rtl)>
            <ToggleGroupSingle
                default_value="1".to_string()
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
            </ToggleGroupSingle>
        </DirectionProvider>

        <h1>"State attributes"</h1>
        <h2>"Group disabled"</h2>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>

        <h2>"Group enabled with button override"</h2>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>

        <h2>"Group disabled with button override"</h2>
        <ToggleGroupMultiple
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
        </ToggleGroupMultiple>
    }
}
