import {useState} from 'react';
import * as Progress from '@radix-ui/react-progress';
import '../../../shared/progress.css';

export default function ProgressPage() {
    const [value, setValue] = useState(30);

    return (
        <>
            <Progress.Root className="progress-root" value={value} max={100}>
                <Progress.Indicator
                    className="progress-indicator"
                    data-testid="progress-indicator"
                    style={{width: value != null ? `${value}%` : undefined}}
                />
            </Progress.Root>

            <br />
            <br />

            <span data-testid="progress-value">{value != null ? value : 'indeterminate'}</span>

            <br />
            <br />

            <button onClick={() => setValue((v) => (v != null ? Math.min(v + 10, 100) : 10))}>increment</button>{' '}
            <button onClick={() => setValue(100)}>set complete</button>{' '}
            <button onClick={() => setValue(null)}>set indeterminate</button>{' '}
            <button data-testid="set-negative" onClick={() => setValue(-10)}>set negative</button>{' '}
            <button data-testid="set-over-max" onClick={() => setValue(200)}>set over max</button>
        </>
    );
}
