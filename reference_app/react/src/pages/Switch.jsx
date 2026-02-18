import {useState} from 'react';
import * as Switch from '@radix-ui/react-switch';
import '../../../shared/switch.css';

export default function SwitchPage() {
    const [disabled, setDisabled] = useState(false);
    const [checked, setChecked] = useState(false);

    return (
        <>
            <label>
                <Switch.Root
                    className="switch-root"
                    disabled={disabled}
                    checked={checked}
                    onCheckedChange={setChecked}
                >
                    <Switch.Thumb className="switch-thumb" />
                </Switch.Root>{' '}
                airplane mode
            </label>

            <br />
            <br />

            <label>
                <input type="checkbox" checked={disabled} onChange={(e) => setDisabled(e.target.checked)} />{' '}
                disabled
            </label>

            <br />

            <label>
                <input type="checkbox" checked={checked} onChange={(e) => setChecked(e.target.checked)} />{' '}
                checked
            </label>
        </>
    );
}
