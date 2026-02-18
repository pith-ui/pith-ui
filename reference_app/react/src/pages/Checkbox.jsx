import {useState} from 'react';
import * as Checkbox from '@radix-ui/react-checkbox';
import '../../../shared/checkbox.css';

export default function CheckboxPage() {
    const [disabled, setDisabled] = useState(false);
    const [indeterminate, setIndeterminate] = useState(false);
    const [checked, setChecked] = useState(false);

    const checkedValue = indeterminate ? 'indeterminate' : checked;

    return (
        <>
            <label className="checkbox-label">
                <Checkbox.Root
                    className="checkbox-root"
                    disabled={disabled}
                    checked={checkedValue}
                    onCheckedChange={(value) => {
                        if (value === 'indeterminate') {
                            setIndeterminate(true);
                            setChecked(false);
                        } else {
                            setIndeterminate(false);
                            setChecked(value);
                        }
                    }}
                >
                    <Checkbox.Indicator className="checkbox-indicator">
                        {checkedValue === 'indeterminate' ? '−' : '✓'}
                    </Checkbox.Indicator>
                </Checkbox.Root>
                accept terms
            </label>

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
                    checked={indeterminate}
                    onChange={(event) => {
                        setIndeterminate(event.target.checked);
                        if (event.target.checked) {
                            setChecked(false);
                        }
                    }}
                />{' '}
                indeterminate
            </label>
        </>
    );
}
