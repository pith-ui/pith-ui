import {useState} from 'react';
import * as Toolbar from '@radix-ui/react-toolbar';
import '../../../shared/toolbar.css';

export default function ToolbarPage() {
    const [actionOutput, setActionOutput] = useState('');

    return (
        <>
            <Toolbar.Root className="toolbar-root" aria-label="Formatting tools">
                <Toolbar.Button className="toolbar-button" onClick={() => setActionOutput('Bold clicked')}>
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
            </Toolbar.Root>

            <br />
            <br />

            <div data-testid="action-output">{actionOutput}</div>
        </>
    );
}
