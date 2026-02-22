import {useState} from 'react';
import * as ContextMenu from '@radix-ui/react-context-menu';
import '../../../shared/context-menu.css';

export default function ContextMenuPage() {
    const [lastAction, setLastAction] = useState('');
    const [checked, setChecked] = useState(false);
    const [radioValue, setRadioValue] = useState('radio1');
    const [disabled, setDisabled] = useState(false);

    return (
        <>
            <ContextMenu.Root>
                <ContextMenu.Trigger className="context-trigger" data-testid="context-trigger">
                    Right click here
                </ContextMenu.Trigger>
                <ContextMenu.Portal>
                    <ContextMenu.Content className="context-content">
                        <ContextMenu.Label className="context-label">Actions</ContextMenu.Label>
                        <ContextMenu.Item
                            className="context-item"
                            onSelect={() => setLastAction('Item 1')}
                        >
                            Item 1
                        </ContextMenu.Item>
                        <ContextMenu.Item
                            className="context-item"
                            disabled={disabled}
                            onSelect={() => setLastAction('Item 2')}
                        >
                            Item 2
                        </ContextMenu.Item>
                        <ContextMenu.Item
                            className="context-item"
                            onSelect={() => setLastAction('Item 3')}
                        >
                            Item 3
                        </ContextMenu.Item>

                        <ContextMenu.Separator className="context-separator" />

                        <ContextMenu.CheckboxItem
                            className="context-item"
                            checked={checked}
                            onCheckedChange={setChecked}
                        >
                            <ContextMenu.ItemIndicator className="context-indicator">
                                ✓
                            </ContextMenu.ItemIndicator>
                            Check me
                        </ContextMenu.CheckboxItem>

                        <ContextMenu.Separator className="context-separator" />

                        <ContextMenu.RadioGroup value={radioValue} onValueChange={setRadioValue}>
                            <ContextMenu.RadioItem className="context-item" value="radio1">
                                <ContextMenu.ItemIndicator className="context-indicator">
                                    ●
                                </ContextMenu.ItemIndicator>
                                Radio 1
                            </ContextMenu.RadioItem>
                            <ContextMenu.RadioItem className="context-item" value="radio2">
                                <ContextMenu.ItemIndicator className="context-indicator">
                                    ●
                                </ContextMenu.ItemIndicator>
                                Radio 2
                            </ContextMenu.RadioItem>
                        </ContextMenu.RadioGroup>

                        <ContextMenu.Separator className="context-separator" />

                        <ContextMenu.Sub>
                            <ContextMenu.SubTrigger className="context-item context-sub-trigger">
                                Submenu →
                            </ContextMenu.SubTrigger>
                            <ContextMenu.Portal>
                                <ContextMenu.SubContent className="context-content" sideOffset={2}>
                                    <ContextMenu.Item
                                        className="context-item"
                                        onSelect={() => setLastAction('Sub item 1')}
                                    >
                                        Sub item 1
                                    </ContextMenu.Item>
                                    <ContextMenu.Item
                                        className="context-item"
                                        onSelect={() => setLastAction('Sub item 2')}
                                    >
                                        Sub item 2
                                    </ContextMenu.Item>
                                </ContextMenu.SubContent>
                            </ContextMenu.Portal>
                        </ContextMenu.Sub>
                    </ContextMenu.Content>
                </ContextMenu.Portal>
            </ContextMenu.Root>

            <br />
            <br />

            <label>
                <input type="checkbox" checked={disabled} onChange={(e) => setDisabled(e.target.checked)} />{' '}
                disabled
            </label>

            <br />
            <br />

            <span data-testid="last-action">{lastAction}</span>
            <br />
            <span data-testid="checkbox-state">{checked ? 'true' : 'false'}</span>
            <br />
            <span data-testid="radio-value">{radioValue}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>
            <input data-testid="outside-input" placeholder="name" />
        </>
    );
}
