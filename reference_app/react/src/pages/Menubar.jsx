import {useState} from 'react';
import * as Menubar from '@radix-ui/react-menubar';
import '../../../shared/menubar.css';

export default function MenubarPage() {
    const [lastAction, setLastAction] = useState('');
    const [bookmarks, setBookmarks] = useState(true);
    const [urls, setUrls] = useState(false);
    const [zoom, setZoom] = useState('normal');
    const [disabled, setDisabled] = useState(false);
    const [rtl, setRtl] = useState(false);
    const dir = rtl ? 'rtl' : 'ltr';

    return (
        <>
            <div dir={dir}>
            <Menubar.Root className="menubar-root" dir={dir}>
                <Menubar.Menu value="file">
                    <Menubar.Trigger className="menubar-trigger">File</Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content className="menubar-content" sideOffset={5} align="start" avoidCollisions={!rtl}>
                            <Menubar.Item
                                className="menubar-item"
                                onSelect={() => setLastAction('New Tab')}
                            >
                                New Tab
                            </Menubar.Item>
                            <Menubar.Item
                                className="menubar-item"
                                onSelect={() => setLastAction('New Window')}
                            >
                                New Window
                            </Menubar.Item>
                            <Menubar.Item
                                className="menubar-item"
                                onSelect={() => setLastAction('Print')}
                            >
                                Print
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>

                <Menubar.Menu value="edit">
                    <Menubar.Trigger className="menubar-trigger">Edit</Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content className="menubar-content" sideOffset={5} align="start" avoidCollisions={!rtl}>
                            <Menubar.Item
                                className="menubar-item"
                                disabled={disabled}
                                onSelect={() => setLastAction('Undo')}
                            >
                                Undo
                            </Menubar.Item>
                            <Menubar.Item
                                className="menubar-item"
                                onSelect={() => setLastAction('Redo')}
                            >
                                Redo
                            </Menubar.Item>
                            <Menubar.Separator className="menubar-separator" />
                            <Menubar.Sub>
                                <Menubar.SubTrigger className="menubar-item menubar-sub-trigger">
                                    Find →
                                </Menubar.SubTrigger>
                                <Menubar.Portal>
                                    <Menubar.SubContent className="menubar-content" sideOffset={2}>
                                        <Menubar.Item
                                            className="menubar-item"
                                            onSelect={() => setLastAction('Search the web…')}
                                        >
                                            Search the web…
                                        </Menubar.Item>
                                        <Menubar.Item
                                            className="menubar-item"
                                            onSelect={() => setLastAction('Find…')}
                                        >
                                            Find…
                                        </Menubar.Item>
                                        <Menubar.Item
                                            className="menubar-item"
                                            onSelect={() => setLastAction('Find Next')}
                                        >
                                            Find Next
                                        </Menubar.Item>
                                    </Menubar.SubContent>
                                </Menubar.Portal>
                            </Menubar.Sub>
                            <Menubar.Separator className="menubar-separator" />
                            <Menubar.Item
                                className="menubar-item"
                                onSelect={() => setLastAction('Cut')}
                            >
                                Cut
                            </Menubar.Item>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>

                <Menubar.Menu value="view">
                    <Menubar.Trigger className="menubar-trigger">View</Menubar.Trigger>
                    <Menubar.Portal>
                        <Menubar.Content className="menubar-content" sideOffset={5} align="start" avoidCollisions={!rtl}>
                            <Menubar.CheckboxItem
                                className="menubar-item"
                                checked={bookmarks}
                                onCheckedChange={setBookmarks}
                            >
                                <Menubar.ItemIndicator className="menubar-indicator">✓</Menubar.ItemIndicator>
                                Always Show Bookmarks Bar
                            </Menubar.CheckboxItem>
                            <Menubar.CheckboxItem
                                className="menubar-item"
                                checked={urls}
                                onCheckedChange={setUrls}
                            >
                                <Menubar.ItemIndicator className="menubar-indicator">✓</Menubar.ItemIndicator>
                                Always Show Full URLs
                            </Menubar.CheckboxItem>
                            <Menubar.Separator className="menubar-separator" />
                            <Menubar.Label className="menubar-label">Zoom</Menubar.Label>
                            <Menubar.RadioGroup value={zoom} onValueChange={setZoom}>
                                <Menubar.RadioItem className="menubar-item" value="compact">
                                    <Menubar.ItemIndicator className="menubar-indicator">●</Menubar.ItemIndicator>
                                    Compact
                                </Menubar.RadioItem>
                                <Menubar.RadioItem className="menubar-item" value="normal">
                                    <Menubar.ItemIndicator className="menubar-indicator">●</Menubar.ItemIndicator>
                                    Normal
                                </Menubar.RadioItem>
                            </Menubar.RadioGroup>
                        </Menubar.Content>
                    </Menubar.Portal>
                </Menubar.Menu>
            </Menubar.Root>
            </div>

            <br />
            <br />

            <label>
                <input type="checkbox" checked={disabled} onChange={(e) => setDisabled(e.target.checked)} />{' '}
                disabled
            </label>
            <br />
            <label>
                <input type="checkbox" checked={rtl} onChange={(e) => setRtl(e.target.checked)} />{' '}
                rtl
            </label>

            <br />
            <br />

            <span data-testid="last-action">{lastAction}</span>
            <br />
            <span data-testid="checkbox-bookmarks">{bookmarks ? 'true' : 'false'}</span>
            <br />
            <span data-testid="checkbox-urls">{urls ? 'true' : 'false'}</span>
            <br />
            <span data-testid="radio-size">{zoom}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>
            <input data-testid="outside-input" placeholder="name" />
        </>
    );
}
