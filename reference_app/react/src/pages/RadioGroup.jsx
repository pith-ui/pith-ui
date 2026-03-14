import {useState} from 'react';
import * as RadioGroup from '@radix-ui/react-radio-group';
import '../../../shared/radio-group.css';

export default function RadioGroupPage() {
    const [disabled, setDisabled] = useState(false);
    const [value, setValue] = useState('');

    return (
        <>
            <RadioGroup.Root
                className="radio-group-root"
                data-custom="radio-group-root-custom"
                disabled={disabled}
                aria-label="Favourite pet"
                value={value}
                onValueChange={setValue}
            >
                <label className="radio-group-label">
                    <RadioGroup.Item className="radio-group-item" data-custom="radio-group-item-custom" value="cat">
                        <RadioGroup.Indicator className="radio-group-indicator" data-custom="radio-group-indicator-custom" />
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

            <br />

            <span data-testid="radio-value">{value}</span>
            <button data-testid="set-rabbit" onClick={() => setValue('rabbit')}>
                set rabbit
            </button>
            <button data-testid="clear-value" onClick={() => setValue('')}>
                clear
            </button>
        </>
    );
}
