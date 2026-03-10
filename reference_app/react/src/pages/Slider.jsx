import {useState} from 'react';
import * as Slider from '@radix-ui/react-slider';
import '../../../shared/slider.css';

export default function SliderPage() {
    const [disabled, setDisabled] = useState(false);
    const [orientation, setOrientation] = useState('horizontal');
    const [value, setValue] = useState([50]);

    return (
        <>
            <Slider.Root
                className="slider-root"
                disabled={disabled}
                orientation={orientation}
                value={value}
                onValueChange={setValue}
                min={0}
                max={100}
                step={1}
            >
                <Slider.Track className="slider-track">
                    <Slider.Range className="slider-range" />
                </Slider.Track>
                <Slider.Thumb className="slider-thumb" aria-label="Volume" />
            </Slider.Root>

            <br />
            <br />

            <span data-testid="slider-value">{value[0]}</span>

            <br />
            <br />

            <hr />

            <h3>Uncontrolled</h3>
            <Slider.Root
                className="slider-root"
                defaultValue={[30]}
                min={0}
                max={100}
                step={1}
                data-testid="uncontrolled-slider"
            >
                <Slider.Track className="slider-track">
                    <Slider.Range className="slider-range" />
                </Slider.Track>
                <Slider.Thumb className="slider-thumb" aria-label="Uncontrolled volume" data-testid="uncontrolled-thumb" />
            </Slider.Root>

            <hr />

            <label>
                <input
                    type="checkbox"
                    checked={disabled}
                    onChange={(event) => setDisabled(event.target.checked)}
                />{' '}
                disabled
            </label>

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
        </>
    );
}
