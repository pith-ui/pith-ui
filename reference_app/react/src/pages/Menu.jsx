import {useState} from 'react';
import * as Menu from '@radix-ui/react-menu';
import '../../../shared/menu.css';

export default function MenuPage() {
    const [lastAction, setLastAction] = useState('');
    const [boldChecked, setBoldChecked] = useState(false);
    const [italicChecked, setItalicChecked] = useState(true);
    const [fontSize, setFontSize] = useState('Medium');
    const [subOpen, setSubOpen] = useState(false);
    const [animated, setAnimated] = useState(false);

    return (
        <>
            <label>
                <input
                    type="checkbox"
                    checked={animated}
                    onChange={(e) => setAnimated(e.target.checked)}
                    data-testid="animated-toggle"
                />
                {' '}Animated
            </label>
            <br /><br />
            <Menu.Root open={true} modal={false}>
                <Menu.Anchor className="menu-anchor">Anchor</Menu.Anchor>
                <Menu.Content className="menu-content" sideOffset={5}>
                    <Menu.Group>
                        <Menu.Label className="menu-label">Fruits</Menu.Label>
                        <Menu.Item className="menu-item" onSelect={() => setLastAction('Apple')}>
                            Apple
                        </Menu.Item>
                        <Menu.Item className="menu-item" onSelect={() => setLastAction('Banana')}>
                            Banana
                        </Menu.Item>
                        <Menu.Item className="menu-item" onSelect={() => setLastAction('Blueberry')}>
                            Blueberry
                        </Menu.Item>
                    </Menu.Group>

                    <Menu.Separator className="menu-separator" />

                    <Menu.Group>
                        <Menu.Label className="menu-label">Vegetables</Menu.Label>
                        <Menu.Item className="menu-item" onSelect={() => setLastAction('Broccoli')}>
                            Broccoli
                        </Menu.Item>
                        <Menu.Item className="menu-item" disabled onSelect={() => setLastAction('Carrot')}>
                            Carrot
                        </Menu.Item>
                        <Menu.Item className="menu-item" onSelect={() => setLastAction('Courgette')}>
                            Courgette
                        </Menu.Item>
                    </Menu.Group>

                    <Menu.Separator className="menu-separator" />

                    <Menu.Item className="menu-item" onSelect={() => setLastAction('Undo')}>
                        Undo
                    </Menu.Item>
                    <Menu.Item className="menu-item" onSelect={() => setLastAction('Redo')}>
                        Redo
                    </Menu.Item>

                    <Menu.Separator className="menu-separator" />

                    <Menu.CheckboxItem className="menu-item" checked={boldChecked} onCheckedChange={setBoldChecked}>
                        Bold
                        <Menu.ItemIndicator>✓</Menu.ItemIndicator>
                    </Menu.CheckboxItem>
                    <Menu.CheckboxItem className="menu-item" checked={italicChecked} onCheckedChange={setItalicChecked}>
                        Italic
                        <Menu.ItemIndicator>✓</Menu.ItemIndicator>
                    </Menu.CheckboxItem>

                    <Menu.Separator className="menu-separator" />

                    <Menu.RadioGroup value={fontSize} onValueChange={setFontSize}>
                        <Menu.RadioItem className="menu-item" value="Small">
                            Small
                            <Menu.ItemIndicator>✓</Menu.ItemIndicator>
                        </Menu.RadioItem>
                        <Menu.RadioItem className="menu-item" value="Medium">
                            Medium
                            <Menu.ItemIndicator>✓</Menu.ItemIndicator>
                        </Menu.RadioItem>
                        <Menu.RadioItem className="menu-item" value="Large">
                            Large
                            <Menu.ItemIndicator>✓</Menu.ItemIndicator>
                        </Menu.RadioItem>
                    </Menu.RadioGroup>

                    <Menu.Separator className="menu-separator" />

                    <Menu.Group>
                        <Menu.Label className="menu-label">Suits (textValue)</Menu.Label>
                        <Menu.Item className="menu-item" textValue="Hearts">
                            <span role="img" aria-label="Hearts">♥️</span> Hearts
                        </Menu.Item>
                        <Menu.Item className="menu-item" textValue="Spades">
                            <span role="img" aria-label="Spades">♠️</span> Spades
                        </Menu.Item>
                        <Menu.Item className="menu-item" textValue="Diamonds">
                            <span role="img" aria-label="Diamonds">♦️</span> Diamonds
                        </Menu.Item>
                        <Menu.Item className="menu-item" textValue="Clubs">
                            <span role="img" aria-label="Clubs">♣️</span> Clubs
                        </Menu.Item>
                    </Menu.Group>

                    <Menu.Separator className="menu-separator" />

                    <Menu.Sub open={subOpen} onOpenChange={setSubOpen}>
                        <Menu.SubTrigger className="menu-item">More Options...</Menu.SubTrigger>
                        <Menu.Portal>
                            <Menu.SubContent className={`menu-content${animated ? ' menu-content-animated' : ''}`} sideOffset={2} alignOffset={-5}>
                                <Menu.Item className="menu-item" onSelect={() => setLastAction('Option A')}>Option A</Menu.Item>
                                <Menu.Item className="menu-item" onSelect={() => setLastAction('Option B')}>Option B</Menu.Item>
                            </Menu.SubContent>
                        </Menu.Portal>
                    </Menu.Sub>
                </Menu.Content>
            </Menu.Root>

            <br />
            <br />

            <span data-testid="last-action">{lastAction}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>

            <br />
            <br />

            <span data-testid="checkbox-state">{[boldChecked && 'Bold', italicChecked && 'Italic'].filter(Boolean).join(',')}</span>

            <br />

            <span data-testid="radio-value">{fontSize}</span>
        </>
    );
}
