use leptos::prelude::*;
use radix_leptos_primitives::direction::Direction;
use radix_leptos_primitives::roving_focus::Orientation;
use radix_leptos_primitives::toggle_group::ToggleGroupType;
use radix_leptos_primitives::toolbar::*;

#[component]
pub fn ToolbarPage() -> impl IntoView {
    let (action_output, set_action_output) = signal(String::new());

    view! {
        <Toolbar attr:class="toolbar-root" attr:aria-label="Formatting tools" attr:data-testid="horizontal-toolbar" attr:data-custom="toolbar-root-custom">
            <ToolbarButton
                attr:class="toolbar-button"
                attr:data-custom="toolbar-button-custom"
                on:click=move |_| set_action_output.set("Bold clicked".to_string())
            >
                "Bold"
            </ToolbarButton>
            <ToolbarButton
                attr:class="toolbar-button"
                on:click=move |_| set_action_output.set("Italic clicked".to_string())
            >
                "Italic"
            </ToolbarButton>

            <ToolbarSeparator attr:class="toolbar-separator" />

            <ToolbarLink attr:class="toolbar-link" attr:href="#">
                "Learn More"
            </ToolbarLink>

            <ToolbarSeparator attr:class="toolbar-separator" />

            <ToolbarToggleGroup r#type=ToggleGroupType::Single attr:class="toolbar-toggle-group">
                <ToolbarToggleItem value="left" attr:class="toolbar-toggle-item">
                    "Left"
                </ToolbarToggleItem>
                <ToolbarToggleItem value="center" attr:class="toolbar-toggle-item">
                    "Center"
                </ToolbarToggleItem>
                <ToolbarToggleItem value="right" attr:class="toolbar-toggle-item">
                    "Right"
                </ToolbarToggleItem>
            </ToolbarToggleGroup>

            <ToolbarButton
                attr:class="toolbar-button"
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
            attr:class="toolbar-root"
            attr:aria-label="Vertical tools"
            attr:data-testid="vertical-toolbar"
        >
            <ToolbarButton attr:class="toolbar-button">
                "VBold"
            </ToolbarButton>
            <ToolbarButton attr:class="toolbar-button">
                "VItalic"
            </ToolbarButton>
            <ToolbarButton attr:class="toolbar-button">
                "VUnderline"
            </ToolbarButton>
        </Toolbar>

        <br /><br />

        <div dir="rtl">
            <Toolbar
                dir=Direction::Rtl
                attr:class="toolbar-root"
                attr:aria-label="RTL tools"
                attr:data-testid="rtl-toolbar"
            >
                <ToolbarButton attr:class="toolbar-button">
                    "First"
                </ToolbarButton>
                <ToolbarButton attr:class="toolbar-button">
                    "Second"
                </ToolbarButton>
                <ToolbarButton attr:class="toolbar-button">
                    "Third"
                </ToolbarButton>
            </Toolbar>
        </div>
    }
}
