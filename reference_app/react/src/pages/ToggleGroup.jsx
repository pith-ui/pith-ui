import {useState} from 'react';
import * as ToggleGroup from '@radix-ui/react-toggle-group';
import '../../../shared/toggle-group.css';

export default function ToggleGroupPage() {
    const [type, setType] = useState('single');
    const [orientation, setOrientation] = useState('horizontal');
    const [disabled, setDisabled] = useState(false);

    return (
        <>
            <ToggleGroup.Root
                className="toggle-group-root"
                type={type}
                orientation={orientation}
                disabled={disabled}
                aria-label="Options"
            >
                <ToggleGroup.Item className="toggle-group-item" value="1">
                    Item 1
                </ToggleGroup.Item>
                <ToggleGroup.Item className="toggle-group-item" value="2" disabled>
                    Item 2
                </ToggleGroup.Item>
                <ToggleGroup.Item className="toggle-group-item" value="3">
                    Item 3
                </ToggleGroup.Item>
            </ToggleGroup.Root>

            <br />
            <br />

            <fieldset>
                <legend>type</legend>
                <label>
                    <input
                        type="radio"
                        name="type"
                        value="single"
                        checked={type === 'single'}
                        onChange={() => setType('single')}
                    />{' '}
                    single
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="type"
                        value="multiple"
                        checked={type === 'multiple'}
                        onChange={() => setType('multiple')}
                    />{' '}
                    multiple
                </label>
            </fieldset>

            <fieldset>
                <legend>orientation</legend>
                <label>
                    <input
                        type="radio"
                        name="orientation"
                        value="horizontal"
                        checked={orientation === 'horizontal'}
                        onChange={() => setOrientation('horizontal')}
                    />{' '}
                    horizontal
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="orientation"
                        value="vertical"
                        checked={orientation === 'vertical'}
                        onChange={() => setOrientation('vertical')}
                    />{' '}
                    vertical
                </label>
            </fieldset>

            <label>
                <input type="checkbox" checked={disabled} onChange={(e) => setDisabled(e.target.checked)} /> disabled
            </label>
        </>
    );
}
