import * as Separator from '@radix-ui/react-separator';
import '../../../shared/separator.css';

export default function SeparatorPage() {
    return (
        <>
            <p>Content above horizontal separator</p>

            <Separator.Root className="separator-root" data-testid="horizontal-separator" />

            <p>Content below horizontal separator</p>

            <div className="separator-vertical-container">
                <span>Left</span>
                <Separator.Root
                    className="separator-root"
                    orientation="vertical"
                    data-testid="vertical-separator"
                />
                <span>Right</span>
            </div>

            <br />

            <Separator.Root className="separator-root" decorative data-testid="decorative-separator" />

            <p>Content below decorative separator</p>
        </>
    );
}
