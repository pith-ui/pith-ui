# Menu (Root)

> **Note:** Menu is a low-level building block for constructing custom menu components. End users should use [DropdownMenu](../dropdown-menu/), [ContextMenu](../context-menu/), or [Menubar](../menubar/) instead — they provide the trigger, open/close state management, and accessibility wiring out of the box. Menu is publicly exported for library authors who need full control.

## Anatomy

The expected component nesting structure:

```
Menu
├── MenuAnchor
├── MenuPortal
│   └── MenuContent
│       ├── MenuLabel
│       ├── MenuGroup
│       │   └── MenuItem
│       ├── MenuCheckboxItem
│       │   └── MenuItemIndicator
│       ├── MenuRadioGroup
│       │   └── MenuRadioItem
│       │       └── MenuItemIndicator
│       ├── MenuSeparator
│       ├── MenuArrow
│       └── MenuSub
│           ├── MenuSubTrigger
│           └── MenuPortal
│               └── MenuSubContent
└── ...
```

### React

```tsx
<Menu.Root open={open} onOpenChange={setOpen}>
  <Menu.Anchor />
  <Menu.Portal>
    <Menu.Content>
      <Menu.Label>Label</Menu.Label>
      <Menu.Group>
        <Menu.Item>Item</Menu.Item>
      </Menu.Group>
      <Menu.Separator />
      <Menu.CheckboxItem checked={checked} onCheckedChange={setChecked}>
        Checkbox
        <Menu.ItemIndicator>✓</Menu.ItemIndicator>
      </Menu.CheckboxItem>
      <Menu.RadioGroup value={value} onValueChange={setValue}>
        <Menu.RadioItem value="a">
          A
          <Menu.ItemIndicator>✓</Menu.ItemIndicator>
        </Menu.RadioItem>
      </Menu.RadioGroup>
      <Menu.Sub open={subOpen} onOpenChange={setSubOpen}>
        <Menu.SubTrigger>More →</Menu.SubTrigger>
        <Menu.Portal>
          <Menu.SubContent>
            <Menu.Item>Sub item</Menu.Item>
          </Menu.SubContent>
        </Menu.Portal>
      </Menu.Sub>
      <Menu.Arrow />
    </Menu.Content>
  </Menu.Portal>
</Menu.Root>
```

### Leptos

```rust
<Menu open=open on_open_change=Callback::new(move |v| set_open.set(v))>
  <MenuAnchor />
  <MenuPortal>
    <MenuContent>
      <MenuLabel>"Label"</MenuLabel>
      <MenuGroup>
        <MenuItem>"Item"</MenuItem>
      </MenuGroup>
      <MenuSeparator />
      <MenuCheckboxItem
        checked=checked
        on_checked_change=Callback::new(move |v: bool| set_checked.set(v.into()))
      >
        "Checkbox"
        <MenuItemIndicator>"✓"</MenuItemIndicator>
      </MenuCheckboxItem>
      <MenuRadioGroup value=value on_value_change=Callback::new(move |v: String| set_value.set(v))>
        <MenuRadioItem value="a">
          "A"
          <MenuItemIndicator>"✓"</MenuItemIndicator>
        </MenuRadioItem>
      </MenuRadioGroup>
      <MenuSub open=sub_open on_open_change=Callback::new(move |v| set_sub_open.set(v))>
        <MenuSubTrigger>"More →"</MenuSubTrigger>
        <MenuPortal>
          <MenuSubContent>
            <MenuItem>"Sub item"</MenuItem>
          </MenuSubContent>
        </MenuPortal>
      </MenuSub>
      <MenuArrow />
    </MenuContent>
  </MenuPortal>
</Menu>
```

## React Signature

```typescript
interface MenuProps {
  children?: React.ReactNode;
  open?: boolean;
  onOpenChange?(open: boolean): void;
  dir?: Direction; // 'ltr' | 'rtl'
  modal?: boolean;
}

const Menu: React.FC<MenuProps> = (props) => { ... }
```

`Menu` is an `FC` (not `forwardRef`) — it does not render a DOM element. It is a context provider wrapping a Popper root.

## Leptos Signature

```rust
pub fn Menu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | The controlled open state of the menu. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects submenu open/close key bindings. |
| `modal` | `modal` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | When `true`, interaction with outside elements is disabled, focus is trapped, and scrolling outside is prevented. When `false`, none of those constraints apply. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The menu content tree. |

### Implicit behavior

- Wraps children in a `Popper` root (positioning context for `MenuContent`).
- Provides `MenuContextValue` and `MenuRootContextValue` to descendants via context.
- Tracks whether the user is navigating via keyboard (`isUsingKeyboard`) by listening to global `keydown` and `pointerdown`/`pointermove` events in the capture phase. This drives focus behavior in `MenuContent` — keyboard users get the first item focused, pointer users get the content container focused.

## Usage Examples

### Basic menu (always open, non-modal)

#### React

```tsx
<Menu.Root open={true} onOpenChange={() => {}} modal={false}>
  <Menu.Anchor style={{ display: 'inline-block' }} />
  <Menu.Portal>
    <Menu.Content align="start">
      <Menu.Item onSelect={() => alert('undo')}>Undo</Menu.Item>
      <Menu.Item onSelect={() => alert('redo')}>Redo</Menu.Item>
      <Menu.Separator />
      <Menu.Item disabled>Cut</Menu.Item>
      <Menu.Item onSelect={() => alert('copy')}>Copy</Menu.Item>
    </Menu.Content>
  </Menu.Portal>
</Menu.Root>
```

#### Leptos

```rust
<Menu open=true modal=false>
  <MenuAnchor attr:style="display: inline-block" />
  <MenuPortal>
    <MenuContent align=Align::Start>
      <MenuItem on_select=Callback::new(|_| window().alert_with_message("undo").ok())>"Undo"</MenuItem>
      <MenuItem on_select=Callback::new(|_| window().alert_with_message("redo").ok())>"Redo"</MenuItem>
      <MenuSeparator />
      <MenuItem disabled=true>"Cut"</MenuItem>
      <MenuItem on_select=Callback::new(|_| window().alert_with_message("copy").ok())>"Copy"</MenuItem>
    </MenuContent>
  </MenuPortal>
</Menu>
```

### Checkbox items

#### React

```tsx
const [bold, setBold] = React.useState(false);

<Menu.CheckboxItem checked={bold} onCheckedChange={setBold}>
  Bold
  <Menu.ItemIndicator><TickIcon /></Menu.ItemIndicator>
</Menu.CheckboxItem>
```

#### Leptos

```rust
let (bold, set_bold) = signal(false);

<MenuCheckboxItem
  checked=Signal::derive(move || CheckedState::from(bold.get()))
  on_checked_change=Callback::new(move |v: bool| set_bold.set(v))
>
  "Bold"
  <MenuItemIndicator>"✓"</MenuItemIndicator>
</MenuCheckboxItem>
```

### Radio items

#### React

```tsx
const [file, setFile] = React.useState('index.js');

<Menu.RadioGroup value={file} onValueChange={setFile}>
  <Menu.RadioItem value="README.md">
    README.md
    <Menu.ItemIndicator><TickIcon /></Menu.ItemIndicator>
  </Menu.RadioItem>
  <Menu.RadioItem value="index.js">
    index.js
    <Menu.ItemIndicator><TickIcon /></Menu.ItemIndicator>
  </Menu.RadioItem>
</Menu.RadioGroup>
```

#### Leptos

```rust
let (file, set_file) = signal("index.js".to_string());

<MenuRadioGroup value=file on_value_change=Callback::new(move |v: String| set_file.set(v))>
  <MenuRadioItem value="README.md">
    "README.md"
    <MenuItemIndicator>"✓"</MenuItemIndicator>
  </MenuRadioItem>
  <MenuRadioItem value="index.js">
    "index.js"
    <MenuItemIndicator>"✓"</MenuItemIndicator>
  </MenuRadioItem>
</MenuRadioGroup>
```

### Submenus

#### React

```tsx
const [subOpen, setSubOpen] = React.useState(false);

<Menu.Sub open={subOpen} onOpenChange={setSubOpen}>
  <Menu.SubTrigger>More →</Menu.SubTrigger>
  <Menu.Portal>
    <Menu.SubContent>
      <Menu.Item onSelect={() => alert('one')}>One</Menu.Item>
      <Menu.Item onSelect={() => alert('two')}>Two</Menu.Item>
    </Menu.SubContent>
  </Menu.Portal>
</Menu.Sub>
```

#### Leptos

```rust
let (sub_open, set_sub_open) = signal(false);

<MenuSub open=sub_open on_open_change=Callback::new(move |v| set_sub_open.set(v))>
  <MenuSubTrigger>"More →"</MenuSubTrigger>
  <MenuPortal>
    <MenuSubContent>
      <MenuItem on_select=Callback::new(|_| window().alert_with_message("one").ok())>"One"</MenuItem>
      <MenuItem on_select=Callback::new(|_| window().alert_with_message("two").ok())>"Two"</MenuItem>
    </MenuSubContent>
  </MenuPortal>
</MenuSub>
```

## Accessibility

Implements the [WAI-ARIA Menu pattern](https://www.w3.org/WAI/ARIA/apd/patterns/menu/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | Activates the focused menu item (fires `onSelect`). |
| `ArrowDown` | Moves focus to the next item. Wraps from last to first. |
| `ArrowUp` | Moves focus to the previous item. Wraps from first to last. |
| `ArrowRight` | In LTR: opens a focused submenu and focuses its first item. In RTL: closes the current submenu and focuses its trigger. |
| `ArrowLeft` | In LTR: closes the current submenu and focuses its trigger. In RTL: opens a focused submenu and focuses its first item. |
| `Home` / `PageUp` | Moves focus to the first item. |
| `End` / `PageDown` | Moves focus to the last item. |
| `Escape` | Closes the menu (in modal mode, closes the entire menu tree). |
| `Tab` | Prevented — menus should not be navigated with Tab. |
| Any printable character | Typeahead: focuses the next item whose text content starts with the typed character(s). Repeated single characters cycle through matches. Search resets after 1 second of inactivity. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `MenuContent` | `role` | `"menu"` | Identifies the element as a menu. |
| `MenuContent` | `aria-orientation` | `"vertical"` | Always vertical for menus. |
| `MenuItem` | `role` | `"menuitem"` | Default role for menu items. |
| `MenuItem` | `aria-disabled` | `"true"` / absent | Present when the item is disabled. |
| `MenuCheckboxItem` | `role` | `"menuitemcheckbox"` | Identifies the item as a checkbox. |
| `MenuCheckboxItem` | `aria-checked` | `"true"` / `"false"` / `"mixed"` | Reflects the checked state. `"mixed"` for indeterminate. |
| `MenuRadioItem` | `role` | `"menuitemradio"` | Identifies the item as a radio button. |
| `MenuRadioItem` | `aria-checked` | `"true"` / `"false"` | Reflects whether this radio item is selected. |
| `MenuGroup` | `role` | `"group"` | Groups related menu items. |
| `MenuSeparator` | `role` | `"separator"` | Visual and semantic separator between groups. |
| `MenuSeparator` | `aria-orientation` | `"horizontal"` | Always horizontal. |
| `MenuSubTrigger` | `aria-haspopup` | `"menu"` | Indicates the trigger opens a submenu. |
| `MenuSubTrigger` | `aria-expanded` | `"true"` / `"false"` | Whether the submenu is currently open. |
| `MenuSubTrigger` | `aria-controls` | `string` | Points to the submenu content's auto-generated `id`. |
| `MenuSubContent` | `aria-labelledby` | `string` | Points to the sub-trigger's auto-generated `id`. |

### Behavioral Notes

- In modal mode (`modal=true`, the default), `aria-hidden` is applied to all DOM elements outside the menu content using the `aria-hidden` library, ensuring screen readers only see the menu.
- Disabled items are skipped during keyboard navigation (roving focus) but remain visible.
- Pointer interaction uses `pointermove` (not `mouseover`) for item highlighting to match native menu behavior — re-entering a previously focused item re-focuses it.
- Submenus open on pointer hover after a 100ms delay and close when the pointer leaves, with a grace area polygon to prevent accidental closure when moving the pointer diagonally toward the submenu.
- When opening, keyboard users get the first non-disabled item focused; pointer users get the content container focused.

## CSS Custom Properties

Menu does not expose any CSS custom properties. Content dimensions are determined by the PopperContent positioning layer.
