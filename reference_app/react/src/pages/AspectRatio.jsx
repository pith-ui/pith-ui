import * as AspectRatio from '@radix-ui/react-aspect-ratio';

export default function AspectRatioPage() {
    return (
        <div style={{maxWidth: '400px', margin: '20px'}}>
            <h2>Default (1:1)</h2>
            <AspectRatio.Root data-testid="default-ratio">
                <span>1:1 content</span>
            </AspectRatio.Root>

            <h2>Custom Ratio (16:9)</h2>
            <AspectRatio.Root ratio={16 / 9} data-testid="custom-ratio">
                <span>16:9 content</span>
            </AspectRatio.Root>

            <h2>With Custom Style (background)</h2>
            <AspectRatio.Root
                ratio={16 / 9}
                data-testid="with-custom-style"
                style={{background: 'tomato'}}
            >
                <span>Custom background</span>
            </AspectRatio.Root>

            <h2>With Conflicting Style (position + top)</h2>
            <AspectRatio.Root
                ratio={16 / 9}
                data-testid="with-conflicting-style"
                style={{position: 'relative', top: '10px', background: 'cornflowerblue'}}
            >
                <span>Conflicting styles</span>
            </AspectRatio.Root>
        </div>
    );
}
