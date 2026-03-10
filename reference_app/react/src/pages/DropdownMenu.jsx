import {useState} from 'react';
import * as DropdownMenu from '@radix-ui/react-dropdown-menu';
import '../../../shared/dropdown-menu.css';

export default function DropdownMenuPage() {
    const [lastAction, setLastAction] = useState('');
    const [checked, setChecked] = useState(false);
    const [radioValue, setRadioValue] = useState('radio1');
    const [disabled, setDisabled] = useState(false);
    const [controlledOpen, setControlledOpen] = useState(false);
    const [triggerClickCount, setTriggerClickCount] = useState(0);

    return (
        <>
            <DropdownMenu.Root>
                <DropdownMenu.Trigger
                    className="dropdown-trigger"
                    onClick={() => setTriggerClickCount((c) => c + 1)}
                >
                    open
                </DropdownMenu.Trigger>
                <DropdownMenu.Portal>
                    <DropdownMenu.Content className="dropdown-content" sideOffset={5}>
                        <DropdownMenu.Label className="dropdown-label">Actions</DropdownMenu.Label>
                        <DropdownMenu.Item
                            className="dropdown-item"
                            onSelect={() => setLastAction('Item 1')}
                        >
                            Item 1
                        </DropdownMenu.Item>
                        <DropdownMenu.Item
                            className="dropdown-item"
                            disabled={disabled}
                            onSelect={() => setLastAction('Item 2')}
                        >
                            Item 2
                        </DropdownMenu.Item>
                        <DropdownMenu.Item
                            className="dropdown-item"
                            onSelect={() => setLastAction('Item 3')}
                        >
                            Item 3
                        </DropdownMenu.Item>

                        <DropdownMenu.Separator className="dropdown-separator" />

                        <DropdownMenu.CheckboxItem
                            className="dropdown-item"
                            checked={checked}
                            onCheckedChange={setChecked}
                        >
                            <DropdownMenu.ItemIndicator className="dropdown-indicator">
                                ✓
                            </DropdownMenu.ItemIndicator>
                            Check me
                        </DropdownMenu.CheckboxItem>

                        <DropdownMenu.Separator className="dropdown-separator" />

                        <DropdownMenu.RadioGroup value={radioValue} onValueChange={setRadioValue}>
                            <DropdownMenu.RadioItem className="dropdown-item" value="radio1">
                                <DropdownMenu.ItemIndicator className="dropdown-indicator">
                                    ●
                                </DropdownMenu.ItemIndicator>
                                Radio 1
                            </DropdownMenu.RadioItem>
                            <DropdownMenu.RadioItem className="dropdown-item" value="radio2">
                                <DropdownMenu.ItemIndicator className="dropdown-indicator">
                                    ●
                                </DropdownMenu.ItemIndicator>
                                Radio 2
                            </DropdownMenu.RadioItem>
                        </DropdownMenu.RadioGroup>

                        <DropdownMenu.Separator className="dropdown-separator" />

                        <DropdownMenu.Sub>
                            <DropdownMenu.SubTrigger className="dropdown-item dropdown-sub-trigger">
                                Submenu →
                            </DropdownMenu.SubTrigger>
                            <DropdownMenu.Portal>
                                <DropdownMenu.SubContent className="dropdown-content" sideOffset={2}>
                                    <DropdownMenu.Item
                                        className="dropdown-item"
                                        onSelect={() => setLastAction('Sub item 1')}
                                    >
                                        Sub item 1
                                    </DropdownMenu.Item>
                                    <DropdownMenu.Item
                                        className="dropdown-item"
                                        onSelect={() => setLastAction('Sub item 2')}
                                    >
                                        Sub item 2
                                    </DropdownMenu.Item>
                                </DropdownMenu.SubContent>
                            </DropdownMenu.Portal>
                        </DropdownMenu.Sub>

                        <DropdownMenu.Arrow className="dropdown-arrow" />
                    </DropdownMenu.Content>
                </DropdownMenu.Portal>
            </DropdownMenu.Root>

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

            <span data-testid="trigger-click-count">{triggerClickCount}</span>
            <br />
            <button data-testid="outside-button">outside</button>
            <input data-testid="outside-input" placeholder="name" />

            <br />
            <br />
            <hr />

            <h3>Controlled</h3>

            <label>
                <input
                    type="checkbox"
                    data-testid="controlled-open-checkbox"
                    checked={controlledOpen}
                    onChange={(e) => setControlledOpen(e.target.checked)}
                />{' '}
                open
            </label>
            <button
                type="button"
                data-testid="controlled-external-close"
                onClick={() => setControlledOpen(false)}
            >
                external close
            </button>
            <span data-testid="controlled-open-state">{controlledOpen ? 'open' : 'closed'}</span>

            <br />
            <br />

            <DropdownMenu.Root open={controlledOpen} onOpenChange={setControlledOpen}>
                <DropdownMenu.Trigger className="controlled-dropdown-trigger" data-testid="controlled-dropdown-trigger">
                    controlled open
                </DropdownMenu.Trigger>
                <DropdownMenu.Portal>
                    <DropdownMenu.Content className="dropdown-content" data-testid="controlled-dropdown-content" sideOffset={5}>
                        <DropdownMenu.Item className="dropdown-item">Controlled Item 1</DropdownMenu.Item>
                        <DropdownMenu.Item className="dropdown-item">Controlled Item 2</DropdownMenu.Item>
                    </DropdownMenu.Content>
                </DropdownMenu.Portal>
            </DropdownMenu.Root>
        </>
    );
}
