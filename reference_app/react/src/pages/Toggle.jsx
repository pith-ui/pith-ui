import {useState} from 'react';
import * as Toggle from '@radix-ui/react-toggle';
import '../../../shared/toggle.css';

export default function TogglePage() {
    const [disabled, setDisabled] = useState(false);
    const [pressed, setPressed] = useState(false);

    return (
        <>
            <Toggle.Root
                className="toggle-root"
                disabled={disabled}
                pressed={pressed}
                onPressedChange={setPressed}
            >
                toggle
            </Toggle.Root>

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
                    checked={pressed}
                    onChange={(event) => setPressed(event.target.checked)}
                />{' '}
                pressed
            </label>
        </>
    );
}
