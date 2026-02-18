import * as HoverCard from '@radix-ui/react-hover-card';
import '../../../shared/hover-card.css';

export default function HoverCardPage() {
    return (
        <>
            <HoverCard.Root openDelay={0} closeDelay={0}>
                <HoverCard.Trigger className="hover-card-trigger" href="#" data-testid="hover-card-trigger">
                    trigger
                </HoverCard.Trigger>
                <HoverCard.Portal>
                    <HoverCard.Content className="hover-card-content" sideOffset={5} data-testid="hover-card-content">
                        <p>Hover card content</p>
                        <p>Supplementary information</p>
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
