import {useState, useMemo} from 'react';
import {Combobox} from '@base-ui/react/combobox';
import '../../../shared/combobox.css';

const fruits = ['Apple', 'Avocado', 'Banana', 'Cherry', 'Grape', 'Mango', 'Orange', 'Pear'];
const vegetables = ['Carrot', 'Celery', 'Lettuce', 'Potato', 'Spinach', 'Tomato'];

function filterItems(items, query) {
    if (!query) return items;
    return items.filter((item) => item.toLowerCase().includes(query.toLowerCase()));
}

export default function ComboboxPage() {
    const [value, setValue] = useState(null);
    const [inputValue, setInputValue] = useState('');
    const [disabled, setDisabled] = useState(false);

    const filteredFruits = useMemo(() => filterItems(fruits, inputValue), [inputValue]);
    const filteredVegetables = useMemo(() => filterItems(vegetables, inputValue), [inputValue]);
    const allFiltered = useMemo(
        () => [...filteredFruits, ...filteredVegetables],
        [filteredFruits, filteredVegetables]
    );

    return (
        <>
            {/* ── Single-select combobox (with groups) ── */}
            <h3>Single Select</h3>
            <Combobox.Root
                value={value}
                onValueChange={(val) => {
                    setValue(val);
                    setInputValue(val ?? '');
                }}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                disabled={disabled}
                items={allFiltered}
            >
                <div className="combobox-anchor" data-testid="combobox-anchor" data-disabled={disabled || undefined}>
                    <Combobox.Input
                        className="combobox-input"
                        data-testid="combobox-input"
                        placeholder="Search..."
                    />
                    <Combobox.Clear className="combobox-clear" data-testid="combobox-clear" aria-label="Clear">
                        ✕
                    </Combobox.Clear>
                    <Combobox.Trigger className="combobox-trigger" data-testid="combobox-trigger" aria-label="Toggle">
                        ▼
                    </Combobox.Trigger>
                </div>

                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4}>
                        <Combobox.Popup className="combobox-content" data-testid="combobox-content">
                            <Combobox.Empty className="combobox-empty" data-testid="combobox-empty">
                                No results found
                            </Combobox.Empty>
                            <Combobox.List className="combobox-viewport" data-testid="combobox-viewport">
                                {filteredFruits.length > 0 && (
                                    <Combobox.Group className="combobox-group">
                                        <Combobox.GroupLabel className="combobox-label">
                                            Fruits
                                        </Combobox.GroupLabel>
                                        {filteredFruits.map((item) => (
                                            <Combobox.Item
                                                key={item}
                                                value={item}
                                                className="combobox-item"
                                                disabled={item === 'Cherry'}
                                            >
                                                <Combobox.ItemIndicator className="combobox-indicator">
                                                    ✓
                                                </Combobox.ItemIndicator>
                                                {item}
                                            </Combobox.Item>
                                        ))}
                                    </Combobox.Group>
                                )}

                                {filteredFruits.length > 0 && filteredVegetables.length > 0 && (
                                    <div className="combobox-separator" role="separator" />
                                )}

                                {filteredVegetables.length > 0 && (
                                    <Combobox.Group className="combobox-group">
                                        <Combobox.GroupLabel className="combobox-label">
                                            Vegetables
                                        </Combobox.GroupLabel>
                                        {filteredVegetables.map((item) => (
                                            <Combobox.Item
                                                key={item}
                                                value={item}
                                                className="combobox-item"
                                            >
                                                <Combobox.ItemIndicator className="combobox-indicator">
                                                    ✓
                                                </Combobox.ItemIndicator>
                                                {item}
                                            </Combobox.Item>
                                        ))}
                                    </Combobox.Group>
                                )}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>

            <br />
            <br />

            <label>
                <input
                    type="checkbox"
                    checked={disabled}
                    onChange={(e) => setDisabled(e.target.checked)}
                />{' '}
                disabled
            </label>

            <br />
            <br />

            <span data-testid="combobox-value">{value || '(none)'}</span>

            <br />
            <br />

            <button data-testid="outside-button">outside</button>
            <input data-testid="outside-input" placeholder="name" />

            <hr />

            {/* ── Multi-select combobox ── */}
            <MultiSelectCombobox />

            <hr />

            {/* ── Default value (uncontrolled) ── */}
            <DefaultValueCombobox />
        </>
    );
}

function MultiSelectCombobox() {
    const [values, setValues] = useState([]);
    const [inputValue, setInputValue] = useState('');

    const allItems = useMemo(() => [...fruits, ...vegetables], []);
    const filteredItems = useMemo(() => filterItems(allItems, inputValue), [allItems, inputValue]);

    return (
        <>
            <h3>Multi Select</h3>
            <Combobox.Root
                multiple
                value={values}
                onValueChange={setValues}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filteredItems}
            >
                <div className="combobox-anchor" data-testid="multi-anchor">
                    <div className="combobox-chips" data-testid="multi-chips">
                        {values.map((val) => (
                            <span key={val} className="combobox-chip" data-testid="multi-chip">
                                {val}
                                <button
                                    className="combobox-chip-remove"
                                    data-testid="multi-chip-remove"
                                    aria-label={`Remove ${val}`}
                                    onClick={() => setValues(values.filter((v) => v !== val))}
                                >
                                    ✕
                                </button>
                            </span>
                        ))}
                    </div>
                    <Combobox.Input
                        className="combobox-input"
                        data-testid="multi-input"
                        placeholder="Search..."
                    />
                    <Combobox.Trigger className="combobox-trigger" data-testid="multi-trigger" aria-label="Toggle">
                        ▼
                    </Combobox.Trigger>
                </div>

                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4}>
                        <Combobox.Popup className="combobox-content" data-testid="multi-content">
                            <Combobox.Empty className="combobox-empty" data-testid="multi-empty">
                                No results found
                            </Combobox.Empty>
                            <Combobox.List className="combobox-viewport" data-testid="multi-viewport">
                                {filteredItems.map((item) => (
                                    <Combobox.Item
                                        key={item}
                                        value={item}
                                        className="combobox-item"
                                    >
                                        <Combobox.ItemIndicator className="combobox-indicator">
                                            ✓
                                        </Combobox.ItemIndicator>
                                        {item}
                                    </Combobox.Item>
                                ))}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>

            <br />

            <span data-testid="multi-value">{values.length > 0 ? values.join(', ') : '(none)'}</span>
        </>
    );
}

function DefaultValueCombobox() {
    const [inputValue, setInputValue] = useState('Banana');

    const filteredItems = useMemo(() => filterItems(fruits, inputValue), [inputValue]);

    return (
        <>
            <h3>Default Value</h3>
            <Combobox.Root
                defaultValue="Banana"
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filteredItems}
            >
                <div className="combobox-anchor" data-testid="default-anchor">
                    <Combobox.Input
                        className="combobox-input"
                        data-testid="default-input"
                        placeholder="Search..."
                    />
                    <Combobox.Trigger
                        className="combobox-trigger"
                        data-testid="default-trigger"
                        aria-label="Toggle"
                    >
                        ▼
                    </Combobox.Trigger>
                </div>

                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4}>
                        <Combobox.Popup className="combobox-content" data-testid="default-content">
                            <Combobox.List className="combobox-viewport" data-testid="default-viewport">
                                {filteredItems.map((item) => (
                                    <Combobox.Item
                                        key={item}
                                        value={item}
                                        className="combobox-item"
                                    >
                                        <Combobox.ItemIndicator className="combobox-indicator">
                                            ✓
                                        </Combobox.ItemIndicator>
                                        {item}
                                    </Combobox.Item>
                                ))}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
        </>
    );
}
