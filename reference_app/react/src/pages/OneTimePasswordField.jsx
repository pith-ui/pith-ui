import {useState} from 'react';
import * as OTP from '@radix-ui/react-one-time-password-field';
import '../../../shared/one-time-password-field.css';

export default function OneTimePasswordFieldPage() {
    const [value, setValue] = useState('');
    const [disabled, setDisabled] = useState(false);
    const [readOnly, setReadOnly] = useState(false);
    const [submitted, setSubmitted] = useState('');

    const onSubmit = (event) => {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);
        const code = formData.get('code') || '';
        setSubmitted(`Submitted: ${code}`);
    };

    return (
        <>
            <form onSubmit={onSubmit} onReset={() => { setValue(''); setSubmitted(''); }}>
                <OTP.Root
                    className="otp-root"
                    value={value}
                    onValueChange={setValue}
                    disabled={disabled}
                    readOnly={readOnly}
                    name="code"
                >
                    <OTP.Input />
                    <OTP.Input />
                    <OTP.Input />
                    <OTP.Input />
                    <OTP.Input />
                    <OTP.Input />
                    <OTP.HiddenInput />
                </OTP.Root>

                <div className="controls">
                    <button type="submit">submit</button>
                    <button type="reset">reset</button>
                </div>
            </form>

            <output data-testid="otp-value">{value}</output>
            <pre data-testid="form-result">{submitted}</pre>

            <div className="controls">
                <label>
                    <input
                        type="checkbox"
                        checked={disabled}
                        onChange={(e) => setDisabled(e.target.checked)}
                    />
                    disabled
                </label>
                <label>
                    <input
                        type="checkbox"
                        checked={readOnly}
                        onChange={(e) => setReadOnly(e.target.checked)}
                    />
                    read-only
                </label>
            </div>

            <button data-testid="outside">outside</button>
        </>
    );
}
