import {useState} from 'react';
import * as OTP from '@radix-ui/react-one-time-password-field';
import '../../../shared/one-time-password-field.css';

export default function OneTimePasswordFieldPage() {
    const [value, setValue] = useState('');
    const [disabled, setDisabled] = useState(false);
    const [readOnly, setReadOnly] = useState(false);
    const [vertical, setVertical] = useState(false);
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
                    orientation={vertical ? 'vertical' : 'horizontal'}
                    name="code"
                    data-testid="main-otp-root"
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
                <label>
                    <input
                        type="checkbox"
                        checked={vertical}
                        onChange={(e) => setVertical(e.target.checked)}
                    />
                    vertical
                </label>
            </div>

            <button data-testid="outside">outside</button>

            <div aria-hidden="true">
                <hr />

                <h3>Uncontrolled</h3>
                <UncontrolledOtp />

                <hr />

                <h3>Password Type</h3>
                <PasswordOtp />

                <hr />

                <h3>Placeholder</h3>
                <PlaceholderOtp />

                <hr />

                <h3>AutoSubmit</h3>
                <AutoSubmitOtp />

                <hr />

                <h3>AutoComplete</h3>
                <AutoCompleteOtp />
            </div>
        </>
    );
}

function UncontrolledOtp() {
    const [submitted, setSubmitted] = useState('');
    const onSubmit = (event) => {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);
        setSubmitted(`Submitted: ${formData.get('uncontrolled-code') || ''}`);
    };
    return (
        <form onSubmit={onSubmit}>
            <OTP.Root
                className="otp-root"
                defaultValue="12"
                name="uncontrolled-code"
                data-testid="uncontrolled-root"
            >
                <OTP.Input />
                <OTP.Input />
                <OTP.Input />
                <OTP.Input />
                <OTP.HiddenInput />
            </OTP.Root>
            <button type="submit" data-testid="uncontrolled-submit">submit</button>
            <pre data-testid="uncontrolled-result">{submitted}</pre>
        </form>
    );
}

function PasswordOtp() {
    return (
        <OTP.Root className="otp-root" type="password" data-testid="password-root">
            <OTP.Input />
            <OTP.Input />
            <OTP.Input />
            <OTP.Input />
            <OTP.HiddenInput />
        </OTP.Root>
    );
}

function PlaceholderOtp() {
    return (
        <OTP.Root className="otp-root" placeholder="○○○○" data-testid="placeholder-root">
            <OTP.Input />
            <OTP.Input />
            <OTP.Input />
            <OTP.Input />
            <OTP.HiddenInput />
        </OTP.Root>
    );
}

function AutoSubmitOtp() {
    const [autoSubmitted, setAutoSubmitted] = useState('');
    return (
        <>
            <OTP.Root
                className="otp-root"
                autoSubmit
                onAutoSubmit={(v) => setAutoSubmitted(`AutoSubmitted: ${v}`)}
                data-testid="autosubmit-root"
            >
                <OTP.Input />
                <OTP.Input />
                <OTP.Input />
                <OTP.Input />
                <OTP.HiddenInput />
            </OTP.Root>
            <pre data-testid="autosubmit-result">{autoSubmitted}</pre>
        </>
    );
}

function AutoCompleteOtp() {
    return (
        <OTP.Root className="otp-root" autoComplete="one-time-code" data-testid="autocomplete-root">
            <OTP.Input />
            <OTP.Input />
            <OTP.Input />
            <OTP.Input />
            <OTP.HiddenInput />
        </OTP.Root>
    );
}
