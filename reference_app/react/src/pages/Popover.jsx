import {useState} from 'react';
import * as Popover from '@radix-ui/react-popover';
import '../../../shared/popover.css';

export default function PopoverPage() {
    const [modal, setModal] = useState(false);
    const [count, setCount] = useState(0);
    const [controlledOpen, setControlledOpen] = useState(false);

    return (
        <>
            <Popover.Root modal={modal}>
                <Popover.Trigger className="popover-trigger" data-custom="popover-trigger-custom">open</Popover.Trigger>
                <Popover.Portal>
                    <Popover.Content className="popover-content" sideOffset={5} data-custom="popover-content-custom" aria-label="Popover" style={{color: 'rgb(255, 0, 0)'}}>
                        <p>Popover content</p>
                        <Popover.Close className="popover-close" data-custom="popover-close-custom">close</Popover.Close>
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

            <br />
            <br />

            <h3>Controlled Popover</h3>

            <Popover.Root open={controlledOpen} onOpenChange={setControlledOpen}>
                <Popover.Trigger className="popover-trigger" data-testid="controlled-trigger">
                    controlled open
                </Popover.Trigger>
                <Popover.Portal>
                    <Popover.Content className="popover-content" sideOffset={5} data-testid="controlled-content">
                        <p>Controlled popover content</p>
                        <Popover.Close className="popover-close">close</Popover.Close>
                        <Popover.Arrow className="popover-arrow" width={20} height={10} />
                    </Popover.Content>
                </Popover.Portal>
            </Popover.Root>

            <br />

            <label>
                <input
                    type="checkbox"
                    data-testid="controlled-checkbox"
                    checked={controlledOpen}
                    onChange={(e) => setControlledOpen(e.target.checked)}
                />{' '}
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

            <hr />

            <h3>Anchor</h3>
            <Popover.Root>
                <div style={{display: 'flex', gap: '200px', alignItems: 'flex-start'}}>
                    <Popover.Trigger className="popover-trigger" data-testid="anchor-trigger">
                        anchor open
                    </Popover.Trigger>
                    <Popover.Anchor data-testid="popover-anchor" asChild>
                        <div
                            style={{
                                width: '100px',
                                height: '30px',
                                background: '#ddd',
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                            }}
                        >
                            anchor
                        </div>
                    </Popover.Anchor>
                </div>
                <Popover.Portal>
                    <Popover.Content
                        className="popover-content"
                        sideOffset={5}
                        data-testid="anchor-content"
                    >
                        <p>Anchored popover content</p>
                        <Popover.Close className="popover-close" data-testid="anchor-close">
                            close
                        </Popover.Close>
                    </Popover.Content>
                </Popover.Portal>
            </Popover.Root>
        </>
    );
}
