import * as Tooltip from '@radix-ui/react-tooltip';
import '../../../shared/tooltip.css';

export default function TooltipPage() {
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

            <button data-testid="outside-button">outside</button>
        </Tooltip.Provider>
    );
}
