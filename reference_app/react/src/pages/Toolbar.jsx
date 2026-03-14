import {useState} from 'react';
import * as Toolbar from '@radix-ui/react-toolbar';
import '../../../shared/toolbar.css';

export default function ToolbarPage() {
    const [actionOutput, setActionOutput] = useState('');

    return (
        <>
            <Toolbar.Root className="toolbar-root" aria-label="Formatting tools" data-testid="horizontal-toolbar" data-custom="toolbar-root-custom">
                <Toolbar.Button className="toolbar-button" data-custom="toolbar-button-custom" onClick={() => setActionOutput('Bold clicked')}>
                    Bold
                </Toolbar.Button>
                <Toolbar.Button className="toolbar-button" onClick={() => setActionOutput('Italic clicked')}>
                    Italic
                </Toolbar.Button>

                <Toolbar.Separator className="toolbar-separator" />

                <Toolbar.Link className="toolbar-link" href="#">
                    Learn More
                </Toolbar.Link>

                <Toolbar.Separator className="toolbar-separator" />

                <Toolbar.ToggleGroup type="single" className="toolbar-toggle-group">
                    <Toolbar.ToggleItem className="toolbar-toggle-item" value="left">
                        Left
                    </Toolbar.ToggleItem>
                    <Toolbar.ToggleItem className="toolbar-toggle-item" value="center">
                        Center
                    </Toolbar.ToggleItem>
                    <Toolbar.ToggleItem className="toolbar-toggle-item" value="right">
                        Right
                    </Toolbar.ToggleItem>
                </Toolbar.ToggleGroup>

                <Toolbar.Button className="toolbar-button" data-testid="disabled-button" disabled>
                    Disabled
                </Toolbar.Button>
            </Toolbar.Root>

            <br />
            <br />

            <div data-testid="action-output">{actionOutput}</div>

            <br />
            <br />

            <Toolbar.Root
                orientation="vertical"
                className="toolbar-root"
                aria-label="Vertical tools"
                data-testid="vertical-toolbar"
            >
                <Toolbar.Button className="toolbar-button">VBold</Toolbar.Button>
                <Toolbar.Button className="toolbar-button">VItalic</Toolbar.Button>
                <Toolbar.Button className="toolbar-button">VUnderline</Toolbar.Button>
            </Toolbar.Root>

            <br />
            <br />

            <div dir="rtl">
                <Toolbar.Root
                    dir="rtl"
                    className="toolbar-root"
                    aria-label="RTL tools"
                    data-testid="rtl-toolbar"
                >
                    <Toolbar.Button className="toolbar-button">First</Toolbar.Button>
                    <Toolbar.Button className="toolbar-button">Second</Toolbar.Button>
                    <Toolbar.Button className="toolbar-button">Third</Toolbar.Button>
                </Toolbar.Root>
            </div>
        </>
    );
}
