# DropdownMenu (Root)

## Anatomy

The expected component nesting structure:

```
DropdownMenu
├── DropdownMenuTrigger
├── DropdownMenuPortal
│   └── DropdownMenuContent
│       ├── DropdownMenuItem
│       ├── DropdownMenuGroup
│       │   └── DropdownMenuItem
│       ├── DropdownMenuLabel
│       ├── DropdownMenuCheckboxItem
│       │   └── DropdownMenuItemIndicator
│       ├── DropdownMenuRadioGroup
│       │   └── DropdownMenuRadioItem
│       │       └── DropdownMenuItemIndicator
│       ├── DropdownMenuSeparator
│       ├── DropdownMenuSub
│       │   ├── DropdownMenuSubTrigger
│       │   └── DropdownMenuPortal
│       │       └── DropdownMenuSubContent
│       └── DropdownMenuArrow
```

### React

```tsx
<DropdownMenu.Root>
  <DropdownMenu.Trigger>Open</DropdownMenu.Trigger>
  <DropdownMenu.Portal>
    <DropdownMenu.Content>
      <DropdownMenu.Item>Undo</DropdownMenu.Item>
      <DropdownMenu.Item>Redo</DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.CheckboxItem checked={checked} onCheckedChange={setChecked}>
        <DropdownMenu.ItemIndicator>✓</DropdownMenu.ItemIndicator>
        Show Minimap
      </DropdownMenu.CheckboxItem>
      <DropdownMenu.Sub>
        <DropdownMenu.SubTrigger>More</DropdownMenu.SubTrigger>
        <DropdownMenu.Portal>
          <DropdownMenu.SubContent>
            <DropdownMenu.Item>Save As...</DropdownMenu.Item>
          </DropdownMenu.SubContent>
        </DropdownMenu.Portal>
      </DropdownMenu.Sub>
      <DropdownMenu.Arrow />
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
```

### Leptos

```rust
<DropdownMenu>
  <DropdownMenuTrigger>"Open"</DropdownMenuTrigger>
  <DropdownMenuPortal>
    <DropdownMenuContent>
      <DropdownMenuItem>"Undo"</DropdownMenuItem>
      <DropdownMenuItem>"Redo"</DropdownMenuItem>
      <DropdownMenuSeparator />
      <DropdownMenuCheckboxItem checked=checked on_checked_change=set_checked>
        <DropdownMenuItemIndicator>"✓"</DropdownMenuItemIndicator>
        "Show Minimap"
      </DropdownMenuCheckboxItem>
      <DropdownMenuSub>
        <DropdownMenuSubTrigger>"More"</DropdownMenuSubTrigger>
        <DropdownMenuPortal>
          <DropdownMenuSubContent>
            <DropdownMenuItem>"Save As..."</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuPortal>
      </DropdownMenuSub>
      <DropdownMenuArrow />
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
```

## React Signature

`DropdownMenu` is a plain `React.FC` (not `forwardRef`) since it is a context-only wrapper with no DOM element of its own.

```typescript
interface DropdownMenuProps {
  children?: React.ReactNode;
  dir?: Direction; // 'ltr' | 'rtl'
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
  modal?: boolean; // default true
}

const DropdownMenu: React.FC<DropdownMenuProps> = (props) => { ... };
```

## Leptos Signature

```rust
pub fn DropdownMenu(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects submenu open/close arrow key direction. |
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state. When set, the component becomes controlled. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | The open state on initial render. Use when you do not need to control the open state. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the open state changes. |
| `modal` | `modal` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether the dropdown menu is modal. When `true`, interaction with outside elements is blocked, only menu content is visible to screen readers, and outside scroll is disabled. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The menu trigger and portal/content. |

### Implicit behavior

- Provides `DropdownMenuContext` to descendants containing: auto-generated `trigger_id`, `trigger_ref`, auto-generated `content_id`, `open` state, `on_open_change`, `on_open_toggle`, and `modal` signal.
- Renders an inner `Menu` (from the menu primitive) that manages roving focus, typeahead, and submenu coordination.

## Usage Examples

### Basic (uncontrolled)

#### React

```tsx
<DropdownMenu.Root>
  <DropdownMenu.Trigger>Options</DropdownMenu.Trigger>
  <DropdownMenu.Portal>
    <DropdownMenu.Content sideOffset={5}>
      <DropdownMenu.Item onSelect={() => console.log('cut')}>Cut</DropdownMenu.Item>
      <DropdownMenu.Item onSelect={() => console.log('copy')}>Copy</DropdownMenu.Item>
      <DropdownMenu.Item onSelect={() => console.log('paste')}>Paste</DropdownMenu.Item>
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
```

#### Leptos

```rust
<DropdownMenu>
  <DropdownMenuTrigger>"Options"</DropdownMenuTrigger>
  <DropdownMenuPortal>
    <DropdownMenuContent side_offset=5.0>
      <DropdownMenuItem on_select=Callback::new(|_| log!("cut"))>"Cut"</DropdownMenuItem>
      <DropdownMenuItem on_select=Callback::new(|_| log!("copy"))>"Copy"</DropdownMenuItem>
      <DropdownMenuItem on_select=Callback::new(|_| log!("paste"))>"Paste"</DropdownMenuItem>
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
```

### Controlled

#### React

```tsx
const [open, setOpen] = React.useState(false);

<DropdownMenu.Root open={open} onOpenChange={setOpen}>
  <DropdownMenu.Trigger>Options</DropdownMenu.Trigger>
  <DropdownMenu.Portal>
    <DropdownMenu.Content>
      {/* ...items */}
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
```

#### Leptos

```rust
let (open, set_open) = signal(false);

<DropdownMenu open=open on_open_change=Callback::new(move |v: bool| set_open.set(v))>
  <DropdownMenuTrigger>"Options"</DropdownMenuTrigger>
  <DropdownMenuPortal>
    <DropdownMenuContent>
      // ...items
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
```

### With checkbox and radio items

#### React

```tsx
const [showMinimap, setShowMinimap] = React.useState(true);
const [color, setColor] = React.useState('blue');

<DropdownMenu.Root>
  <DropdownMenu.Trigger>Settings</DropdownMenu.Trigger>
  <DropdownMenu.Portal>
    <DropdownMenu.Content>
      <DropdownMenu.CheckboxItem checked={showMinimap} onCheckedChange={setShowMinimap}>
        <DropdownMenu.ItemIndicator>✓</DropdownMenu.ItemIndicator>
        Show Minimap
      </DropdownMenu.CheckboxItem>
      <DropdownMenu.Separator />
      <DropdownMenu.RadioGroup value={color} onValueChange={setColor}>
        <DropdownMenu.RadioItem value="blue">
          <DropdownMenu.ItemIndicator>●</DropdownMenu.ItemIndicator>
          Blue
        </DropdownMenu.RadioItem>
        <DropdownMenu.RadioItem value="red">
          <DropdownMenu.ItemIndicator>●</DropdownMenu.ItemIndicator>
          Red
        </DropdownMenu.RadioItem>
      </DropdownMenu.RadioGroup>
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
```

#### Leptos

```rust
let (show_minimap, set_show_minimap) = signal(true);
let (color, set_color) = signal("blue".to_string());

<DropdownMenu>
  <DropdownMenuTrigger>"Settings"</DropdownMenuTrigger>
  <DropdownMenuPortal>
    <DropdownMenuContent>
      <DropdownMenuCheckboxItem
        checked=Signal::derive(move || CheckedState::from(show_minimap.get()))
        on_checked_change=Callback::new(move |v: bool| set_show_minimap.set(v))
      >
        <DropdownMenuItemIndicator>"✓"</DropdownMenuItemIndicator>
        "Show Minimap"
      </DropdownMenuCheckboxItem>
      <DropdownMenuSeparator />
      <DropdownMenuRadioGroup
        value=color
        on_value_change=Callback::new(move |v: String| set_color.set(v))
      >
        <DropdownMenuRadioItem value="blue">
          <DropdownMenuItemIndicator>"●"</DropdownMenuItemIndicator>
          "Blue"
        </DropdownMenuRadioItem>
        <DropdownMenuRadioItem value="red">
          <DropdownMenuItemIndicator>"●"</DropdownMenuItemIndicator>
          "Red"
        </DropdownMenuRadioItem>
      </DropdownMenuRadioGroup>
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
```

### With submenus

#### React

```tsx
<DropdownMenu.Root>
  <DropdownMenu.Trigger>Edit</DropdownMenu.Trigger>
  <DropdownMenu.Portal>
    <DropdownMenu.Content>
      <DropdownMenu.Item>Undo</DropdownMenu.Item>
      <DropdownMenu.Sub>
        <DropdownMenu.SubTrigger>More Tools</DropdownMenu.SubTrigger>
        <DropdownMenu.Portal>
          <DropdownMenu.SubContent>
            <DropdownMenu.Item>Save Page As...</DropdownMenu.Item>
            <DropdownMenu.Item>Create Shortcut...</DropdownMenu.Item>
          </DropdownMenu.SubContent>
        </DropdownMenu.Portal>
      </DropdownMenu.Sub>
    </DropdownMenu.Content>
  </DropdownMenu.Portal>
</DropdownMenu.Root>
```

#### Leptos

```rust
<DropdownMenu>
  <DropdownMenuTrigger>"Edit"</DropdownMenuTrigger>
  <DropdownMenuPortal>
    <DropdownMenuContent>
      <DropdownMenuItem>"Undo"</DropdownMenuItem>
      <DropdownMenuSub>
        <DropdownMenuSubTrigger>"More Tools"</DropdownMenuSubTrigger>
        <DropdownMenuPortal>
          <DropdownMenuSubContent>
            <DropdownMenuItem>"Save Page As..."</DropdownMenuItem>
            <DropdownMenuItem>"Create Shortcut..."</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuPortal>
      </DropdownMenuSub>
    </DropdownMenuContent>
  </DropdownMenuPortal>
</DropdownMenu>
```

## Accessibility

Implements the [WAI-ARIA Menu pattern](https://www.w3.org/WAI/ARIA/apd/patterns/menu/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on the trigger, toggles the menu. When focus is on a menu item, activates it (fires `onSelect`). |
| `ArrowDown` | When focus is on the trigger, opens the menu. When focus is inside the menu, moves focus to the next item. |
| `ArrowUp` | When focus is inside the menu, moves focus to the previous item. |
| `ArrowRight` | When focus is on a sub-trigger (LTR), opens the submenu. When focus is inside a submenu (RTL), closes it and returns focus to the sub-trigger. |
| `ArrowLeft` | When focus is on a sub-trigger (RTL), opens the submenu. When focus is inside a submenu (LTR), closes it and returns focus to the sub-trigger. |
| `Escape` | Closes the menu and returns focus to the trigger. In a submenu, closes the entire menu tree. |
| `Home` / `PageUp` | Moves focus to the first menu item. |
| `End` / `PageDown` | Moves focus to the last menu item. |
| `Tab` | Prevented inside the menu to keep focus trapped (in modal mode). |
| Any printable character | Typeahead: focuses the next item whose text value starts with the typed character(s). |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `DropdownMenuTrigger` | `aria-haspopup` | `"menu"` | Indicates the trigger opens a menu. |
| `DropdownMenuTrigger` | `aria-expanded` | `"true" \| "false"` | Reflects the menu open state. |
| `DropdownMenuTrigger` | `aria-controls` | `string \| undefined` | Points to the content element's auto-generated `id`. Only set when menu is open. |
| `DropdownMenuContent` | `role` | `"menu"` | Applied by the underlying `MenuContent`. |
| `DropdownMenuContent` | `aria-orientation` | `"vertical"` | Menus are always vertically oriented. |
| `DropdownMenuContent` | `aria-labelledby` | `string` | Points to the trigger's auto-generated `id`. |
| `DropdownMenuItem` | `role` | `"menuitem"` | Standard menu item role. |
| `DropdownMenuItem` | `aria-disabled` | `"true" \| undefined` | Set when the item is disabled. |
| `DropdownMenuCheckboxItem` | `role` | `"menuitemcheckbox"` | Checkbox menu item role. |
| `DropdownMenuCheckboxItem` | `aria-checked` | `"true" \| "false" \| "mixed"` | Reflects the checked state. `"mixed"` when indeterminate. |
| `DropdownMenuRadioItem` | `role` | `"menuitemradio"` | Radio menu item role. |
| `DropdownMenuRadioItem` | `aria-checked` | `"true" \| "false"` | Whether this radio item is selected. |
| `DropdownMenuSubTrigger` | `aria-haspopup` | `"menu"` | Indicates the sub-trigger opens a submenu. |
| `DropdownMenuSubTrigger` | `aria-expanded` | `"true" \| "false"` | Reflects the submenu open state. |
| `DropdownMenuSubTrigger` | `aria-controls` | `string` | Points to the sub-content's auto-generated `id`. |
| `DropdownMenuGroup` | `role` | `"group"` | Groups related menu items. |
| `DropdownMenuSeparator` | `role` | `"separator"` | Visual separator between menu item groups. |
| `DropdownMenuSeparator` | `aria-orientation` | `"horizontal"` | Separators are always horizontal. |

### Behavioral Notes

- When `modal=true` (default), opening the menu hides all other content from assistive technology via `aria-hidden` on sibling elements, and traps focus within the content.
- When `modal=false`, focus is not trapped and outside elements remain accessible. The menu closes on outside interaction.
- On close, focus returns to the trigger unless the user interacted outside the menu (clicked/tapped outside), in which case the browser's natural focus handling is used.
- Right-clicking outside a modal dropdown menu does not dismiss it (to avoid interfering with context menus).
- Pointer-based submenu navigation uses a "grace area" polygon to prevent accidental submenu closure when the pointer moves diagonally between a sub-trigger and its content.
- Typeahead search resets after 1 second of inactivity. Repeated characters cycle through matches starting with that character.
- Disabled items are skipped during keyboard navigation but remain visible.

## CSS Custom Properties

These properties are set on `DropdownMenuContent` and `DropdownMenuSubContent` and alias the underlying Popper values. Use them in CSS for positioning-aware styling.

| Property | Source | Description |
|---|---|---|
| `--dropdown-menu-content-transform-origin` | `var(--popper-transform-origin)` | The CSS transform origin computed from the content's position relative to the trigger. Useful for scale/rotate animations. |
| `--dropdown-menu-content-available-width` | `var(--popper-available-width)` | The available width between the trigger and the viewport edge. |
| `--dropdown-menu-content-available-height` | `var(--popper-available-height)` | The available height between the trigger and the viewport edge. |
| `--dropdown-menu-trigger-width` | `var(--popper-anchor-width)` | The width of the trigger element. Useful for matching content width to trigger width. |
| `--dropdown-menu-trigger-height` | `var(--popper-anchor-height)` | The height of the trigger element. |
