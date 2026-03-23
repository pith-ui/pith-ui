import {useState, useMemo} from 'react';
import {Routes, Route, Link, useLocation} from 'react-router-dom';
import {BrowserRouter} from 'react-router-dom';
import {Combobox} from '@base-ui/react/combobox';
import './combobox.css';

const FRUITS = ['Apple', 'Avocado', 'Banana', 'Blueberry', 'Cherry', 'Grape', 'Kiwi', 'Lemon', 'Mango', 'Orange', 'Peach', 'Pear', 'Strawberry'];
const VEGETABLES = ['Artichoke', 'Broccoli', 'Carrot', 'Celery', 'Eggplant', 'Lettuce', 'Potato', 'Spinach', 'Tomato', 'Zucchini'];

function filter(items, query) {
    if (!query) return items;
    const q = query.toLowerCase();
    return items.filter((i) => i.toLowerCase().includes(q));
}

function TickIcon() {
    return (
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32" width="12" height="12"
            fill="none" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="3">
            <path d="M2 20 L12 28 30 4" />
        </svg>
    );
}

function Item({value, disabled = false, children}) {
    return (
        <Combobox.Item className="item" value={value} disabled={disabled}>
            <Combobox.ItemIndicator className="indicator"><TickIcon /></Combobox.ItemIndicator>
            {children || value}
        </Combobox.Item>
    );
}

/* ── Styled ─────────────────────────────────────────────── */

function Styled() {
    const [value, setValue] = useState(null);
    const [inputValue, setInputValue] = useState('');
    const filtered = useMemo(() => filter(FRUITS, inputValue), [inputValue]);

    return (
        <div className="root">
            <h2>Combobox</h2>
            <Combobox.Root
                value={value}
                onValueChange={(v) => { setValue(v); setInputValue(v ?? ''); }}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filtered}
            >
                <div className="anchor">
                    <Combobox.Input className="input" placeholder="Select a fruit..." />
                    <Combobox.Clear className="clear" aria-label="Clear">✕</Combobox.Clear>
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.Empty className="empty">No results found</Combobox.Empty>
                            <Combobox.List className="viewport">
                                {filtered.map((item) => (
                                    <Item key={item} value={item} disabled={item === 'Cherry'} />
                                ))}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
            <p>Selected: {value || '(none)'}</p>
        </div>
    );
}

/* ── WithGroups ──────────────────────────────────────────── */

function WithGroups() {
    const [value, setValue] = useState(null);
    const [inputValue, setInputValue] = useState('');
    const filteredFruits = useMemo(() => filter(FRUITS, inputValue), [inputValue]);
    const filteredVegs = useMemo(() => filter(VEGETABLES, inputValue), [inputValue]);
    const allFiltered = useMemo(() => [...filteredFruits, ...filteredVegs], [filteredFruits, filteredVegs]);

    return (
        <div className="root">
            <h2>With Groups</h2>
            <Combobox.Root
                value={value}
                onValueChange={(v) => { setValue(v); setInputValue(v ?? ''); }}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={allFiltered}
            >
                <div className="anchor">
                    <Combobox.Input className="input" placeholder="Search produce..." />
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.Empty className="empty">No results found</Combobox.Empty>
                            <Combobox.List className="viewport">
                                {filteredFruits.length > 0 && (
                                    <Combobox.Group>
                                        <Combobox.GroupLabel className="label">Fruits</Combobox.GroupLabel>
                                        {filteredFruits.map((item) => <Item key={item} value={item} />)}
                                    </Combobox.Group>
                                )}
                                {filteredFruits.length > 0 && filteredVegs.length > 0 && (
                                    <div className="separator" role="separator" />
                                )}
                                {filteredVegs.length > 0 && (
                                    <Combobox.Group>
                                        <Combobox.GroupLabel className="label">Vegetables</Combobox.GroupLabel>
                                        {filteredVegs.map((item) => <Item key={item} value={item} />)}
                                    </Combobox.Group>
                                )}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
            <p>Selected: {value || '(none)'}</p>
        </div>
    );
}

/* ── MultiSelect ─────────────────────────────────────────── */

function MultiSelect() {
    const [values, setValues] = useState([]);
    const [inputValue, setInputValue] = useState('');
    const allItems = useMemo(() => [...FRUITS, ...VEGETABLES], []);
    const filtered = useMemo(() => filter(allItems, inputValue), [allItems, inputValue]);

    return (
        <div className="root">
            <h2>Multi-Select</h2>
            <Combobox.Root
                multiple
                value={values}
                onValueChange={setValues}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filtered}
            >
                <div className="anchor">
                    <Combobox.Chips className="chips">
                        <Combobox.Value>
                            {(selected) => (
                                <>
                                    {selected.map((val) => (
                                        <Combobox.Chip key={val} className="chip" aria-label={val}>
                                            {val}
                                            <Combobox.ChipRemove className="chipRemove" aria-label="Remove">
                                                ✕
                                            </Combobox.ChipRemove>
                                        </Combobox.Chip>
                                    ))}
                                    <Combobox.Input
                                        className="input"
                                        placeholder={selected.length > 0 ? '' : 'Search...'}
                                    />
                                </>
                            )}
                        </Combobox.Value>
                    </Combobox.Chips>
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.Empty className="empty">No results found</Combobox.Empty>
                            <Combobox.List className="viewport">
                                {filtered.map((item) => <Item key={item} value={item} />)}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
            <p>Selected: {values.length > 0 ? values.join(', ') : '(none)'}</p>
            <div style={{display: 'flex', gap: '8px', marginTop: '8px'}}>
                <button>Before</button>
                <input placeholder="Tab target" style={{padding: '4px 8px'}} />
                <button>After</button>
            </div>
        </div>
    );
}

/* ── Controlled ──────────────────────────────────────────── */

function Controlled() {
    const [value, setValue] = useState('Banana');
    const [inputValue, setInputValue] = useState('Banana');
    const filtered = useMemo(() => filter(FRUITS, inputValue), [inputValue]);

    return (
        <div className="root">
            <h2>Controlled</h2>
            <p>Use the buttons below to set the value externally.</p>
            <div style={{display: 'flex', gap: '8px', marginBottom: '8px'}}>
                <button onClick={() => { setValue('Apple'); setInputValue('Apple'); }}>Set Apple</button>
                <button onClick={() => { setValue('Mango'); setInputValue('Mango'); }}>Set Mango</button>
                <button onClick={() => { setValue(null); setInputValue(''); }}>Clear</button>
            </div>
            <Combobox.Root
                value={value}
                onValueChange={(v) => { setValue(v); setInputValue(v ?? ''); }}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filtered}
            >
                <div className="anchor">
                    <Combobox.Input className="input" placeholder="Select a fruit..." />
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.List className="viewport">
                                {filtered.map((item) => <Item key={item} value={item} />)}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
            <p>Selected: {value || '(none)'}</p>
        </div>
    );
}

/* ── Disabled ────────────────────────────────────────────── */

function Disabled() {
    return (
        <div className="root">
            <h2>Disabled</h2>
            <Combobox.Root disabled items={FRUITS}>
                <div className="anchor">
                    <Combobox.Input className="input" placeholder="Cannot interact..." />
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.List className="viewport">
                                {FRUITS.map((item) => <Item key={item} value={item} />)}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
        </div>
    );
}

/* ── WithEmpty ───────────────────────────────────────────── */

function WithEmpty() {
    const [inputValue, setInputValue] = useState('zzzzz');
    const filtered = useMemo(() => filter(FRUITS, inputValue), [inputValue]);

    return (
        <div className="root">
            <h2>Empty State</h2>
            <p>Pre-filled with a query that matches nothing.</p>
            <Combobox.Root
                defaultOpen
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filtered}
            >
                <div className="anchor">
                    <Combobox.Input className="input" placeholder="Type to search..." />
                    <Combobox.Clear className="clear" aria-label="Clear">✕</Combobox.Clear>
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.Empty className="empty">No results found</Combobox.Empty>
                            <Combobox.List className="viewport">
                                {filtered.map((item) => <Item key={item} value={item} />)}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
        </div>
    );
}

/* ── WithClear ───────────────────────────────────────────── */

function WithClear() {
    const [value, setValue] = useState('Orange');
    const [inputValue, setInputValue] = useState('Orange');
    const filtered = useMemo(() => filter(FRUITS, inputValue), [inputValue]);

    return (
        <div className="root">
            <h2>With Clear Button</h2>
            <p>Select a value, then use the ✕ button to clear it.</p>
            <Combobox.Root
                value={value}
                onValueChange={(v) => { setValue(v); setInputValue(v ?? ''); }}
                inputValue={inputValue}
                onInputValueChange={setInputValue}
                items={filtered}
            >
                <div className="anchor">
                    <Combobox.Input className="input" placeholder="Select a fruit..." />
                    <Combobox.Clear className="clear" aria-label="Clear">✕</Combobox.Clear>
                    <Combobox.Trigger className="trigger" aria-label="Toggle">▼</Combobox.Trigger>
                </div>
                <Combobox.Portal>
                    <Combobox.Positioner sideOffset={4} align="start">
                        <Combobox.Popup className="content">
                            <Combobox.List className="viewport">
                                {filtered.map((item) => <Item key={item} value={item} />)}
                            </Combobox.List>
                        </Combobox.Popup>
                    </Combobox.Positioner>
                </Combobox.Portal>
            </Combobox.Root>
            <p>Selected: {value || '(none)'}</p>
        </div>
    );
}

/* ── Nav + Router ────────────────────────────────────────── */

function Nav() {
    const location = useLocation();
    const stories = [
        ['/styled', 'Styled'],
        ['/with-groups', 'With Groups'],
        ['/multi-select', 'Multi-Select'],
        ['/controlled', 'Controlled'],
        ['/disabled', 'Disabled'],
        ['/with-empty', 'With Empty'],
        ['/with-clear', 'With Clear'],
    ];

    return (
        <nav style={{padding: '16px', borderRight: '1px solid #e4e4e7', minWidth: '180px'}}>
            <h3 style={{margin: '0 0 12px', fontSize: '14px', color: '#71717a'}}>Base UI Combobox</h3>
            <ul style={{listStyle: 'none', margin: 0, padding: 0}}>
                {stories.map(([path, label]) => (
                    <li key={path} style={{marginBottom: '4px'}}>
                        <Link
                            to={path}
                            style={{
                                color: location.pathname === path ? '#3b82f6' : '#18181b',
                                textDecoration: 'none',
                                fontSize: '14px',
                                fontWeight: location.pathname === path ? 600 : 400,
                            }}
                        >{label}</Link>
                    </li>
                ))}
            </ul>
        </nav>
    );
}

export default function App() {
    return (
        <BrowserRouter>
            <div style={{display: 'flex', minHeight: '100vh', fontFamily: '-apple-system, BlinkMacSystemFont, sans-serif'}}>
                <Nav />
                <main style={{flex: 1}}>
                    <Routes>
                        <Route path="/" element={<Styled />} />
                        <Route path="/styled" element={<Styled />} />
                        <Route path="/with-groups" element={<WithGroups />} />
                        <Route path="/multi-select" element={<MultiSelect />} />
                        <Route path="/controlled" element={<Controlled />} />
                        <Route path="/disabled" element={<Disabled />} />
                        <Route path="/with-empty" element={<WithEmpty />} />
                        <Route path="/with-clear" element={<WithClear />} />
                    </Routes>
                </main>
            </div>
        </BrowserRouter>
    );
}
