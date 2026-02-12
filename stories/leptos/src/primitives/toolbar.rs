use leptos::prelude::*;
use radix_leptos_direction::DirectionProvider;
use radix_leptos_toggle::Toggle;
use radix_leptos_toolbar::*;

stylance::import_crate_style!(classes, "src/primitives/toolbar.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <ToolbarExample title="Horizontal" />
        <ToolbarExample title="Vertical" orientation=radix_leptos_roving_focus::Orientation::Vertical />
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <div style:padding="50px">
            <h1>"Example"</h1>
            <ToolbarExample title="" />
            <ToolbarExample title="" orientation=radix_leptos_roving_focus::Orientation::Vertical />

            <h1>"Direction"</h1>
            <h2>"Prop"</h2>
            <ToolbarExample title="" dir=radix_leptos_direction::Direction::Rtl />

            <h2>"Inherited"</h2>
            <DirectionProvider direction=Signal::derive(|| radix_leptos_direction::Direction::Rtl)>
                <ToolbarExample title="" />
            </DirectionProvider>
        </div>
    }
}

#[component]
fn ToolbarExample(
    #[prop(into)] title: String,
    #[prop(optional)] orientation: Option<radix_leptos_roving_focus::Orientation>,
    #[prop(into, optional)] dir: MaybeProp<radix_leptos_direction::Direction>,
) -> impl IntoView {
    let orientation = orientation.unwrap_or(radix_leptos_roving_focus::Orientation::Horizontal);

    let title = StoredValue::new(title);
    let aria_label = StoredValue::new(format!("{} toolbar", title.get_value()));
    let toggle_item_class = StoredValue::new(format!(
        "{} {}",
        classes::toolbarItem,
        classes::toolbarToggleItem
    ));
    let link_class = StoredValue::new(format!("{} {}", classes::toolbarItem, classes::toolbarLink));
    let toggle_button_class = StoredValue::new(format!(
        "{} {}",
        classes::toolbarItem,
        classes::toolbarToggleButton
    ));

    view! {
        <div style:padding="1px" style:margin="-1px">
            <h1>{title.get_value()}</h1>
            <Toolbar
                orientation=orientation
                r#loop=true
                dir=dir
                attr:class=classes::toolbar
                attr:aria-label=aria_label.get_value()
            >
                <ToolbarButton attr:class=classes::toolbarItem>"Button"</ToolbarButton>
                <ToolbarButton attr:class=classes::toolbarItem disabled=true>
                    "Button (disabled)"
                </ToolbarButton>
                <ToolbarSeparator attr:class=classes::toolbarSeparator />
                <ToolbarLink
                    attr:class=link_class.get_value()
                    attr:href="https://www.w3.org/TR/2019/WD-wai-aria-practices-1.2-20191218/examples/toolbar/toolbar.html"
                    attr:target="_blank"
                >
                    "Link"
                </ToolbarLink>
                <ToolbarSeparator attr:class=classes::toolbarSeparator />
                <ToolbarButton
                    as_child=true
                    attr:class=toggle_button_class.get_value()
                >
                    <Toggle>"Toggle"</Toggle>
                </ToolbarButton>
                <ToolbarSeparator attr:class=classes::toolbarSeparator />
                <ToolbarToggleGroup
                    r#type=radix_leptos_toggle_group::ToggleGroupType::Single
                    attr:class=classes::toolbarToggleGroup
                >
                    <ToolbarToggleItem value="left" attr:class=toggle_item_class.get_value()>
                        "Left"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem value="center" attr:class=toggle_item_class.get_value()>
                        "Center"
                    </ToolbarToggleItem>
                    <ToolbarToggleItem value="right" attr:class=toggle_item_class.get_value()>
                        "Right"
                    </ToolbarToggleItem>
                </ToolbarToggleGroup>
                // DropdownMenu integration omitted — not yet ported to Leptos
            </Toolbar>
        </div>
    }
}
