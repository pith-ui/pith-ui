import { useState } from 'react';
import * as Tooltip from '@radix-ui/react-tooltip';
import '../../../shared/tooltip.css';

export default function TooltipPage() {
    const [controlledOpen, setControlledOpen] = useState(false);

    return (
        <Tooltip.Provider delayDuration={0} skipDelayDuration={0}>
            <Tooltip.Root>
                <Tooltip.Trigger className="tooltip-trigger" data-testid="tooltip-trigger-1">
                    trigger 1
                </Tooltip.Trigger>
                <Tooltip.Portal>
                    <Tooltip.Content className="tooltip-content" sideOffset={5} data-testid="tooltip-content-1">
                        Tooltip 1
                        <Tooltip.Arrow className="tooltip-arrow" width={12} height={6} />
                    </Tooltip.Content>
                </Tooltip.Portal>
            </Tooltip.Root>

            <br />
            <br />

            <Tooltip.Root>
                <Tooltip.Trigger className="tooltip-trigger" data-testid="tooltip-trigger-2">
                    trigger 2
                </Tooltip.Trigger>
                <Tooltip.Portal>
                    <Tooltip.Content className="tooltip-content" sideOffset={5} data-testid="tooltip-content-2">
                        Tooltip 2
                        <Tooltip.Arrow className="tooltip-arrow" width={12} height={6} />
                    </Tooltip.Content>
                </Tooltip.Portal>
            </Tooltip.Root>

            <br />
            <br />

            <Tooltip.Root open={controlledOpen} onOpenChange={setControlledOpen}>
                <Tooltip.Trigger className="tooltip-trigger" data-testid="tooltip-trigger-controlled">
                    controlled trigger
                </Tooltip.Trigger>
                <Tooltip.Portal>
                    <Tooltip.Content className="tooltip-content" sideOffset={5} data-testid="tooltip-content-controlled">
                        Controlled Tooltip
                        <Tooltip.Arrow className="tooltip-arrow" width={12} height={6} />
                    </Tooltip.Content>
                </Tooltip.Portal>
            </Tooltip.Root>

            <br />

            <label>
                <input
                    type="checkbox"
                    checked={controlledOpen}
                    onChange={(e) => setControlledOpen(e.target.checked)}
                />
                open controlled
            </label>
            <button
                type="button"
                data-testid="controlled-external-close"
                onClick={() => setControlledOpen(false)}
            >
                external close
            </button>
            <span data-testid="controlled-open-state">{controlledOpen ? 'open' : 'closed'}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>
        </Tooltip.Provider>
    );
}
