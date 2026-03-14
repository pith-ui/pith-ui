import {useState} from 'react';
import * as HoverCard from '@radix-ui/react-hover-card';
import '../../../shared/hover-card.css';

export default function HoverCardPage() {
    const [controlled, setControlled] = useState(false);
    const [controlledOpen, setControlledOpen] = useState(false);

    return (
        <>
            <HoverCard.Root openDelay={0} closeDelay={0}>
                <HoverCard.Trigger className="hover-card-trigger" href="#" data-testid="hover-card-trigger">
                    trigger
                </HoverCard.Trigger>
                <HoverCard.Portal>
                    <HoverCard.Content className="hover-card-content" sideOffset={5} data-testid="hover-card-content" style={{background: 'tomato'}}>
                        <p>Hover card content</p>
                        <p>Supplementary information</p>
                        <HoverCard.Arrow className="hover-card-arrow" width={20} height={10} />
                    </HoverCard.Content>
                </HoverCard.Portal>
            </HoverCard.Root>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    checked={controlled}
                    onChange={(e) => setControlled(e.target.checked)}
                />{' '}
                controlled
            </label>

            <br />
            <br />

            {controlled && (
                <>
                    <button data-testid="open-controlled" onClick={() => setControlledOpen(true)}>
                        open
                    </button>
                    <button data-testid="close-controlled" onClick={() => setControlledOpen(false)}>
                        close
                    </button>
                    <span data-testid="controlled-state">{controlledOpen ? 'open' : 'closed'}</span>

                    <br />
                    <br />

                    <HoverCard.Root open={controlledOpen} onOpenChange={setControlledOpen} openDelay={0} closeDelay={0}>
                        <HoverCard.Trigger
                            className="hover-card-trigger"
                            href="#"
                            data-testid="controlled-trigger"
                        >
                            controlled trigger
                        </HoverCard.Trigger>
                        <HoverCard.Portal>
                            <HoverCard.Content
                                className="hover-card-content"
                                sideOffset={5}
                                data-testid="controlled-content"
                            >
                                <p>Controlled hover card content</p>
                                <HoverCard.Arrow className="hover-card-arrow" width={20} height={10} />
                            </HoverCard.Content>
                        </HoverCard.Portal>
                    </HoverCard.Root>

                    <br />
                    <br />
                </>
            )}

            <hr />

            <h3>Delayed</h3>
            <HoverCard.Root openDelay={500} closeDelay={300}>
                <HoverCard.Trigger className="hover-card-trigger" href="#" data-testid="delayed-trigger">
                    delayed trigger
                </HoverCard.Trigger>
                <HoverCard.Portal>
                    <HoverCard.Content className="hover-card-content" sideOffset={5} data-testid="delayed-content">
                        <p>Delayed hover card content</p>
                        <HoverCard.Arrow className="hover-card-arrow" width={20} height={10} />
                    </HoverCard.Content>
                </HoverCard.Portal>
            </HoverCard.Root>

            <br />
            <br />

            <button data-testid="outside-element">outside</button>
        </>
    );
}
