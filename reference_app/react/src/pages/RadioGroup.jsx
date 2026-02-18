import {useState} from 'react';
import * as RadioGroup from '@radix-ui/react-radio-group';
import '../../../shared/radio-group.css';

export default function RadioGroupPage() {
    const [disabled, setDisabled] = useState(false);

    return (
        <>
            <RadioGroup.Root className="radio-group-root" disabled={disabled} aria-label="Favourite pet">
                <label className="radio-group-label">
                    <RadioGroup.Item className="radio-group-item" value="cat">
                        <RadioGroup.Indicator className="radio-group-indicator" />
                    </RadioGroup.Item>
                    Cat
                </label>
                <label className="radio-group-label">
                    <RadioGroup.Item className="radio-group-item" value="dog" disabled>
                        <RadioGroup.Indicator className="radio-group-indicator" />
                    </RadioGroup.Item>
                    Dog
                </label>
                <label className="radio-group-label">
                    <RadioGroup.Item className="radio-group-item" value="rabbit">
                        <RadioGroup.Indicator className="radio-group-indicator" />
                    </RadioGroup.Item>
                    Rabbit
                </label>
            </RadioGroup.Root>

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
        </>
    );
}
