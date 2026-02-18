import {useState} from 'react';
import * as ScrollArea from '@radix-ui/react-scroll-area';
import '../../../shared/scroll-area.css';

const ITEMS = Array.from({length: 50}, (_, i) => `Item ${i + 1}`);

export default function ScrollAreaPage() {
    const [showHorizontal, setShowHorizontal] = useState(false);
    const [scrollType, setScrollType] = useState('hover');

    return (
        <>
            <ScrollArea.Root className="scroll-area-root" data-testid="scroll-area-root" type={scrollType}>
                <ScrollArea.Viewport className="scroll-area-viewport" data-testid="scroll-area-viewport">
                    <div className={`scroll-area-content ${showHorizontal ? 'scroll-area-wide-content' : ''}`}>
                        {ITEMS.map((item) => (
                            <div key={item} className="scroll-area-item">
                                {item}
                            </div>
                        ))}
                    </div>
                </ScrollArea.Viewport>
                <ScrollArea.Scrollbar
                    className="scroll-area-scrollbar"
                    orientation="vertical"
                    data-testid="scrollbar-vertical"
                >
                    <ScrollArea.Thumb className="scroll-area-thumb" data-testid="thumb-vertical" />
                </ScrollArea.Scrollbar>
                {showHorizontal && (
                    <ScrollArea.Scrollbar
                        className="scroll-area-scrollbar"
                        orientation="horizontal"
                        data-testid="scrollbar-horizontal"
                    >
                        <ScrollArea.Thumb className="scroll-area-thumb" data-testid="thumb-horizontal" />
                    </ScrollArea.Scrollbar>
                )}
                <ScrollArea.Corner className="scroll-area-corner" />
            </ScrollArea.Root>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    checked={showHorizontal}
                    onChange={() => setShowHorizontal((prev) => !prev)}
                />{' '}
                show horizontal scrollbar
            </label>

            <br />
            <br />

            <fieldset>
                <legend>Scroll Type</legend>
                <label>
                    <input
                        type="radio"
                        name="scroll-type"
                        value="hover"
                        checked={scrollType === 'hover'}
                        onChange={() => setScrollType('hover')}
                    />{' '}
                    hover
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="scroll-type"
                        value="scroll"
                        checked={scrollType === 'scroll'}
                        onChange={() => setScrollType('scroll')}
                    />{' '}
                    scroll
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="scroll-type"
                        value="auto"
                        checked={scrollType === 'auto'}
                        onChange={() => setScrollType('auto')}
                    />{' '}
                    auto
                </label>
                <br />
                <label>
                    <input
                        type="radio"
                        name="scroll-type"
                        value="always"
                        checked={scrollType === 'always'}
                        onChange={() => setScrollType('always')}
                    />{' '}
                    always
                </label>
            </fieldset>

            <br />

            <button
                data-testid="scroll-to-bottom"
                onClick={() => {
                    const viewport = document.querySelector('[data-radix-scroll-area-viewport]');
                    if (viewport) {
                        viewport.scrollTop = viewport.scrollHeight;
                    }
                }}
            >
                scroll to bottom
            </button>{' '}
            <button
                data-testid="scroll-to-top"
                onClick={() => {
                    const viewport = document.querySelector('[data-radix-scroll-area-viewport]');
                    if (viewport) {
                        viewport.scrollTop = 0;
                    }
                }}
            >
                scroll to top
            </button>
        </>
    );
}
