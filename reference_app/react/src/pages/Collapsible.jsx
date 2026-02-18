import {useState} from 'react';
import * as Collapsible from '@radix-ui/react-collapsible';
import '../../../shared/collapsible.css';

export default function CollapsiblePage() {
    const [disabled, setDisabled] = useState(false);
    const [open, setOpen] = useState(false);

    return (
        <>
            <Collapsible.Root
                className="collapsible-root"
                data-testid="collapsible-root"
                disabled={disabled}
                open={open}
                onOpenChange={setOpen}
            >
                <Collapsible.Trigger className="collapsible-trigger">toggle</Collapsible.Trigger>
                <Collapsible.Content className="collapsible-content">
                    <p>Collapsible content.</p>
                </Collapsible.Content>
            </Collapsible.Root>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    checked={disabled}
                    onChange={(event) => setDisabled(event.target.checked)}
                />{' '}
                disabled
            </label>

            <br />

            <label>
                <input
                    type="checkbox"
                    checked={open}
                    onChange={(event) => setOpen(event.target.checked)}
                />{' '}
                open
            </label>
        </>
    );
}
