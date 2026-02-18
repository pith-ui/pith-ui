import {useState} from 'react';
import * as Popover from '@radix-ui/react-popover';
import '../../../shared/popover.css';

export default function PopoverPage() {
    const [modal, setModal] = useState(false);
    const [count, setCount] = useState(0);

    return (
        <>
            <Popover.Root modal={modal}>
                <Popover.Trigger className="popover-trigger">open</Popover.Trigger>
                <Popover.Portal>
                    <Popover.Content className="popover-content" sideOffset={5}>
                        <p>Popover content</p>
                        <Popover.Close className="popover-close">close</Popover.Close>
                        <Popover.Arrow className="popover-arrow" width={20} height={10} />
                    </Popover.Content>
                </Popover.Portal>
            </Popover.Root>

            <br />
            <br />

            <label>
                <input type="checkbox" checked={modal} onChange={(e) => setModal(e.target.checked)} /> modal
            </label>

            <br />
            <br />

            <button data-testid="count-button" onClick={() => setCount((c) => c + 1)}>
                count up
            </button>
            <span data-testid="count-value">{count}</span>

            <br />
            <br />

            <input data-testid="outside-input" placeholder="name" />
        </>
    );
}
