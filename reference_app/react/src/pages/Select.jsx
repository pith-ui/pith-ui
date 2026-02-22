import {useState} from 'react';
import * as Select from '@radix-ui/react-select';
import '../../../shared/select.css';

export default function SelectPage() {
    const [value, setValue] = useState('');
    const [disabled, setDisabled] = useState(false);
    const [controlledValue, setControlledValue] = useState('uk');

    return (
        <>
            {/* ── Main select (controlled, with groups) ── */}
            <Select.Root value={value} onValueChange={setValue} disabled={disabled}>
                <Select.Trigger className="select-trigger" data-testid="select-trigger">
                    <Select.Value placeholder="Select a fruit..." />
                    <Select.Icon className="select-icon">▼</Select.Icon>
                </Select.Trigger>
                <Select.Portal>
                    <Select.Content className="select-content" position="popper" sideOffset={4}>
                        <Select.ScrollUpButton className="select-scroll-button">▲</Select.ScrollUpButton>
                        <Select.Viewport className="select-viewport">
                            <Select.Group>
                                <Select.Label className="select-label">Fruits</Select.Label>
                                <Select.Item className="select-item" value="apple">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Apple</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="avocado">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Avocado</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="banana">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Banana</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="cherry" disabled>
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Cherry</Select.ItemText>
                                </Select.Item>
                            </Select.Group>

                            <Select.Separator className="select-separator" />

                            <Select.Group>
                                <Select.Label className="select-label">Vegetables</Select.Label>
                                <Select.Item className="select-item" value="carrot">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Carrot</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="potato">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Potato</Select.ItemText>
                                </Select.Item>
                            </Select.Group>
                        </Select.Viewport>
                        <Select.ScrollDownButton className="select-scroll-button">▼</Select.ScrollDownButton>
                    </Select.Content>
                </Select.Portal>
            </Select.Root>

            <br />
            <br />

            <label>
                <input type="checkbox" checked={disabled} onChange={(e) => setDisabled(e.target.checked)} />{' '}
                disabled
            </label>

            <br />
            <br />

            <span data-testid="select-value">{value || '(none)'}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>
            <input data-testid="outside-input" placeholder="name" />

            <hr />

            {/* ── Default value select ── */}
            <h3>Default Value</h3>
            <Select.Root defaultValue="banana">
                <Select.Trigger className="select-trigger" data-testid="default-trigger">
                    <Select.Value />
                    <Select.Icon className="select-icon">▼</Select.Icon>
                </Select.Trigger>
                <Select.Portal>
                    <Select.Content className="select-content" position="popper" sideOffset={4}>
                        <Select.Viewport className="select-viewport">
                            <Select.Item className="select-item" value="apple">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Apple</Select.ItemText>
                            </Select.Item>
                            <Select.Item className="select-item" value="banana">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Banana</Select.ItemText>
                            </Select.Item>
                            <Select.Item className="select-item" value="cherry">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Cherry</Select.ItemText>
                            </Select.Item>
                        </Select.Viewport>
                    </Select.Content>
                </Select.Portal>
            </Select.Root>

            <hr />

            {/* ── Controlled dual selects (shared state) ── */}
            <h3>Controlled Pair</h3>
            <div style={{display: 'flex', gap: 20}}>
                <label>
                    Select A:
                    <Select.Root value={controlledValue} onValueChange={setControlledValue}>
                        <Select.Trigger className="select-trigger" data-testid="controlled-trigger-a">
                            <Select.Value />
                            <Select.Icon className="select-icon">▼</Select.Icon>
                        </Select.Trigger>
                        <Select.Portal>
                            <Select.Content className="select-content" position="popper" sideOffset={4}>
                                <Select.Viewport className="select-viewport">
                                    <Select.Item className="select-item" value="fr">
                                        <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                        <Select.ItemText>France</Select.ItemText>
                                    </Select.Item>
                                    <Select.Item className="select-item" value="uk">
                                        <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                        <Select.ItemText>United Kingdom</Select.ItemText>
                                    </Select.Item>
                                    <Select.Item className="select-item" value="es">
                                        <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                        <Select.ItemText>Spain</Select.ItemText>
                                    </Select.Item>
                                </Select.Viewport>
                            </Select.Content>
                        </Select.Portal>
                    </Select.Root>
                </label>

                <label>
                    Select B:
                    <Select.Root value={controlledValue} onValueChange={setControlledValue}>
                        <Select.Trigger className="select-trigger" data-testid="controlled-trigger-b">
                            <Select.Value />
                            <Select.Icon className="select-icon">▼</Select.Icon>
                        </Select.Trigger>
                        <Select.Portal>
                            <Select.Content className="select-content" position="popper" sideOffset={4}>
                                <Select.Viewport className="select-viewport">
                                    <Select.Item className="select-item" value="fr">
                                        <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                        <Select.ItemText>France</Select.ItemText>
                                    </Select.Item>
                                    <Select.Item className="select-item" value="uk">
                                        <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                        <Select.ItemText>United Kingdom</Select.ItemText>
                                    </Select.Item>
                                    <Select.Item className="select-item" value="es">
                                        <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                        <Select.ItemText>Spain</Select.ItemText>
                                    </Select.Item>
                                </Select.Viewport>
                            </Select.Content>
                        </Select.Portal>
                    </Select.Root>
                </label>
            </div>
            <span data-testid="controlled-value">{controlledValue}</span>

            <hr />

            {/* ── Form integration ── */}
            <h3>Form Integration</h3>
            <FormSection />

            <hr />

            {/* ── Item-aligned positioning (default) ── */}
            <h3>Item Aligned</h3>
            <Select.Root defaultValue="banana">
                <Select.Trigger className="select-trigger" data-testid="aligned-trigger">
                    <Select.Value />
                    <Select.Icon className="select-icon">▼</Select.Icon>
                </Select.Trigger>
                <Select.Portal>
                    <Select.Content className="select-content" data-testid="aligned-content">
                        <Select.Viewport className="select-viewport">
                            <Select.Item className="select-item" value="apple">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Apple</Select.ItemText>
                            </Select.Item>
                            <Select.Item className="select-item" value="banana">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Banana</Select.ItemText>
                            </Select.Item>
                            <Select.Item className="select-item" value="cherry">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Cherry</Select.ItemText>
                            </Select.Item>
                            <Select.Item className="select-item" value="date">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Date</Select.ItemText>
                            </Select.Item>
                            <Select.Item className="select-item" value="elderberry">
                                <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                <Select.ItemText>Elderberry</Select.ItemText>
                            </Select.Item>
                        </Select.Viewport>
                    </Select.Content>
                </Select.Portal>
            </Select.Root>
        </>
    );
}

function FormSection() {
    const [data, setData] = useState({});

    function handleChange(event) {
        const formData = new FormData(event.currentTarget);
        setData(Object.fromEntries(formData.entries()));
    }

    return (
        <form
            data-testid="select-form"
            onSubmit={(event) => {
                handleChange(event);
                event.preventDefault();
            }}
            onChange={handleChange}
        >
            <label>
                Country:{' '}
                <Select.Root name="country" defaultValue="fr">
                    <Select.Trigger className="select-trigger" data-testid="form-trigger">
                        <Select.Value />
                        <Select.Icon className="select-icon">▼</Select.Icon>
                    </Select.Trigger>
                    <Select.Portal>
                        <Select.Content className="select-content" position="popper" sideOffset={4}>
                            <Select.Viewport className="select-viewport">
                                <Select.Item className="select-item" value="fr">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>France</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="uk">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>United Kingdom</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="es">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Spain</Select.ItemText>
                                </Select.Item>
                            </Select.Viewport>
                        </Select.Content>
                    </Select.Portal>
                </Select.Root>
            </label>
            <button type="submit">Submit</button>
            <pre data-testid="form-data">{JSON.stringify(data)}</pre>
        </form>
    );
}

export function SelectForcedOpenPage() {
    return (
        <>
            {/* ── Forced open, no default value, item-aligned ── */}
            <h3>Forced Open No Value (Item Aligned)</h3>
            <div style={{position: 'relative', minHeight: 200}}>
                <Select.Root open>
                    <Select.Trigger className="select-trigger" data-testid="forced-novalue-trigger">
                        <Select.Value placeholder="Pick an option" />
                        <Select.Icon className="select-icon">▼</Select.Icon>
                    </Select.Trigger>
                    <Select.Portal>
                        <Select.Content className="select-content" data-testid="forced-novalue-content" style={{opacity: 0.7}}>
                            <Select.Viewport className="select-viewport">
                                <Select.Item className="select-item" value="apple">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Apple</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="banana">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Banana</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="cherry">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Cherry</Select.ItemText>
                                </Select.Item>
                            </Select.Viewport>
                        </Select.Content>
                    </Select.Portal>
                </Select.Root>
            </div>

            <hr />

            {/* ── Forced open, popper (controlled open=true) ── */}
            <h3>Forced Open (Popper)</h3>
            <div style={{position: 'relative', minHeight: 200}}>
                <Select.Root defaultValue="banana" open>
                    <Select.Trigger className="select-trigger" data-testid="forced-trigger">
                        <Select.Value />
                        <Select.Icon className="select-icon">▼</Select.Icon>
                    </Select.Trigger>
                    <Select.Portal>
                        <Select.Content className="select-content" data-testid="forced-content" position="popper" sideOffset={4}>
                            <Select.Viewport className="select-viewport">
                                <Select.Item className="select-item" value="apple">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Apple</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="banana">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Banana</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="cherry">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Cherry</Select.ItemText>
                                </Select.Item>
                            </Select.Viewport>
                        </Select.Content>
                    </Select.Portal>
                </Select.Root>
            </div>

            <hr />

            {/* ── Forced open, item-aligned (controlled open=true, no position="popper") ── */}
            <h3>Forced Open (Item Aligned)</h3>
            <div style={{position: 'relative', minHeight: 200}}>
                <Select.Root defaultValue="banana" open>
                    <Select.Trigger className="select-trigger" data-testid="forced-aligned-trigger">
                        <Select.Value />
                        <Select.Icon className="select-icon">▼</Select.Icon>
                    </Select.Trigger>
                    <Select.Portal>
                        <Select.Content className="select-content" data-testid="forced-aligned-content">
                            <Select.Viewport className="select-viewport">
                                <Select.Item className="select-item" value="apple">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Apple</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="banana">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Banana</Select.ItemText>
                                </Select.Item>
                                <Select.Item className="select-item" value="cherry">
                                    <Select.ItemIndicator className="select-indicator">✓</Select.ItemIndicator>
                                    <Select.ItemText>Cherry</Select.ItemText>
                                </Select.Item>
                            </Select.Viewport>
                        </Select.Content>
                    </Select.Portal>
                </Select.Root>
            </div>
        </>
    );
}
