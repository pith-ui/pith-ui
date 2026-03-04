import * as Popper from '@radix-ui/react-popper';
import '../../../shared/popper.css';

export default function PopperPage() {
    return (
        <div style={{padding: 100}}>
            <h2>Logical "start" alignment (LTR)</h2>
            <div
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 150,
                    border: '1px solid black',
                    padding: 20,
                }}
            >
                <Popper.Root>
                    <Popper.Anchor className="popper-anchor" data-testid="ltr-anchor">
                        LTR
                    </Popper.Anchor>
                    <Popper.Content
                        className="popper-content"
                        align="start"
                        sideOffset={5}
                        data-testid="ltr-content"
                    >
                        LTR content
                    </Popper.Content>
                </Popper.Root>
            </div>

            <h2>Logical "start" alignment (RTL)</h2>
            <div
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: 150,
                    border: '1px solid black',
                    padding: 20,
                }}
            >
                <Popper.Root>
                    <Popper.Anchor className="popper-anchor" data-testid="rtl-anchor">
                        RTL
                    </Popper.Anchor>
                    <Popper.Content
                        className="popper-content"
                        align="start"
                        sideOffset={5}
                        dir="rtl"
                        data-testid="rtl-content"
                    >
                        RTL content
                    </Popper.Content>
                </Popper.Root>
            </div>
        </div>
    );
}
