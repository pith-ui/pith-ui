use leptos::prelude::*;
use pith_ui::direction::Direction;
use pith_ui::roving_focus::Orientation;
use pith_ui::toggle_group::ToggleGroupType;
use pith_ui::toolbar::*;

#[component]
pub fn ToolbarPage() -> impl IntoView {
    let (action_output, set_action_output) = signal(String::new());

    view! {
        <Toolbar class:toolbar-root=true attr:aria-label="Formatting tools" attr:data-testid="horizontal-toolbar" attr:data-custom="toolbar-root-custom">
            <ToolbarButton
                class:toolbar-button=true
                attr:data-custom="toolbar-button-custom"
                on:click=move |_| set_action_output.set("Bold clicked".to_string())
            >
                "Bold"
            </ToolbarButton>
            <ToolbarButton
                class:toolbar-button=true
                on:click=move |_| set_action_output.set("Italic clicked".to_string())
            >
                "Italic"
            </ToolbarButton>

            <ToolbarSeparator class:toolbar-separator=true />

            <ToolbarLink class:toolbar-link=true attr:href="#">
                "Learn More"
            </ToolbarLink>

            <ToolbarSeparator class:toolbar-separator=true />

            <ToolbarToggleGroup r#type=ToggleGroupType::Single class:toolbar-toggle-group=true>
                <ToolbarToggleItem value="left" class:toolbar-toggle-item=true>
                    "Left"
                </ToolbarToggleItem>
                <ToolbarToggleItem value="center" class:toolbar-toggle-item=true>
                    "Center"
                </ToolbarToggleItem>
                <ToolbarToggleItem value="right" class:toolbar-toggle-item=true>
                    "Right"
                </ToolbarToggleItem>
            </ToolbarToggleGroup>

            <ToolbarButton
                class:toolbar-button=true
                attr:data-testid="disabled-button"
                disabled=true
            >
                "Disabled"
            </ToolbarButton>
        </Toolbar>

        <br /><br />

        <div data-testid="action-output">{move || action_output.get()}</div>

        <br /><br />

        <Toolbar
            orientation=Orientation::Vertical
            class:toolbar-root=true
            attr:aria-label="Vertical tools"
            attr:data-testid="vertical-toolbar"
        >
            <ToolbarButton class:toolbar-button=true>
                "VBold"
            </ToolbarButton>
            <ToolbarButton class:toolbar-button=true>
                "VItalic"
            </ToolbarButton>
            <ToolbarButton class:toolbar-button=true>
                "VUnderline"
            </ToolbarButton>
        </Toolbar>

        <br /><br />

        <div dir="rtl">
            <Toolbar
                dir=Direction::Rtl
                class:toolbar-root=true
                attr:aria-label="RTL tools"
                attr:data-testid="rtl-toolbar"
            >
                <ToolbarButton class:toolbar-button=true>
                    "First"
                </ToolbarButton>
                <ToolbarButton class:toolbar-button=true>
                    "Second"
                </ToolbarButton>
                <ToolbarButton class:toolbar-button=true>
                    "Third"
                </ToolbarButton>
            </Toolbar>
        </div>
    }
}
