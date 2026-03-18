# ContextMenu (Root)

## Anatomy

The expected component nesting structure:

```
ContextMenu
├── ContextMenuTrigger
└── ContextMenuPortal
    └── ContextMenuContent
        ├── ContextMenuItem
        ├── ContextMenuGroup
        │   ├── ContextMenuLabel
        │   └── ContextMenuItem
        ├── ContextMenuCheckboxItem
        │   └── ContextMenuItemIndicator
        ├── ContextMenuRadioGroup
        │   └── ContextMenuRadioItem
        │       └── ContextMenuItemIndicator
        ├── ContextMenuSeparator
        ├── ContextMenuArrow
        └── ContextMenuSub
            ├── ContextMenuSubTrigger
            └── ContextMenuPortal
                └── ContextMenuSubContent
```

### React

```tsx
<ContextMenu.Root>
  <ContextMenu.Trigger>Right click here</ContextMenu.Trigger>
  <ContextMenu.Portal>
    <ContextMenu.Content>
      <ContextMenu.Item>Undo</ContextMenu.Item>
      <ContextMenu.Item>Redo</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Sub>
        <ContextMenu.SubTrigger>More</ContextMenu.SubTrigger>
        <ContextMenu.Portal>
          <ContextMenu.SubContent>
            <ContextMenu.Item>One</ContextMenu.Item>
            <ContextMenu.Item>Two</ContextMenu.Item>
          </ContextMenu.SubContent>
        </ContextMenu.Portal>
      </ContextMenu.Sub>
    </ContextMenu.Content>
  </ContextMenu.Portal>
</ContextMenu.Root>
```

### Leptos

```rust
<ContextMenu>
  <ContextMenuTrigger>"Right click here"</ContextMenuTrigger>
  <ContextMenuPortal>
    <ContextMenuContent>
      <ContextMenuItem>"Undo"</ContextMenuItem>
      <ContextMenuItem>"Redo"</ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuSub>
        <ContextMenuSubTrigger>"More"</ContextMenuSubTrigger>
        <ContextMenuPortal>
          <ContextMenuSubContent>
            <ContextMenuItem>"One"</ContextMenuItem>
            <ContextMenuItem>"Two"</ContextMenuItem>
          </ContextMenuSubContent>
        </ContextMenuPortal>
      </ContextMenuSub>
    </ContextMenuContent>
  </ContextMenuPortal>
</ContextMenu>
```

## React Signature

```typescript
const ContextMenu: React.FC<ContextMenuProps> = (props) => { ... }

interface ContextMenuProps {
  children?: React.ReactNode;
  onOpenChange?(open: boolean): void;
  dir?: Direction; // 'ltr' | 'rtl'
  modal?: boolean;
}
```

`ContextMenu` is not a `forwardRef` component — it does not render a DOM element itself. It provides context and wraps `Menu.Root`.

## Leptos Signature

```rust
pub fn ContextMenu(
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] modal: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the context menu opens or closes. Receives the new open state. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects submenu opening direction: in RTL, submenus open to the left. |
| `modal` | `modal` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether the context menu is modal. When `true`, interaction with outside elements is blocked and only menu content is visible to screen readers. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The trigger and portal content. |

### Implicit behavior

- The open state is managed internally — there is no controlled `open` prop on `ContextMenu`. The menu opens on right-click (or long-press on touch) of the trigger and closes on item selection, escape, or outside interaction.
- Wraps `Menu` (the underlying menu primitive) and provides it with `open`, `onOpenChange`, `dir`, and `modal`.

## Usage Examples

### Basic usage

#### React

```tsx
<ContextMenu.Root>
  <ContextMenu.Trigger>Right click here</ContextMenu.Trigger>
  <ContextMenu.Portal>
    <ContextMenu.Content>
      <ContextMenu.Item onSelect={() => console.log('undo')}>Undo</ContextMenu.Item>
      <ContextMenu.Item onSelect={() => console.log('redo')}>Redo</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Item disabled onSelect={() => console.log('cut')}>Cut</ContextMenu.Item>
      <ContextMenu.Item onSelect={() => console.log('copy')}>Copy</ContextMenu.Item>
      <ContextMenu.Item onSelect={() => console.log('paste')}>Paste</ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Portal>
</ContextMenu.Root>
```

#### Leptos

```rust
<ContextMenu>
  <ContextMenuTrigger>"Right click here"</ContextMenuTrigger>
  <ContextMenuPortal>
    <ContextMenuContent>
      <ContextMenuItem on_select=Callback::new(move |_: ev::Event| { /* undo */ })>"Undo"</ContextMenuItem>
      <ContextMenuItem on_select=Callback::new(move |_: ev::Event| { /* redo */ })>"Redo"</ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuItem disabled=true>"Cut"</ContextMenuItem>
      <ContextMenuItem on_select=Callback::new(move |_: ev::Event| { /* copy */ })>"Copy"</ContextMenuItem>
      <ContextMenuItem on_select=Callback::new(move |_: ev::Event| { /* paste */ })>"Paste"</ContextMenuItem>
    </ContextMenuContent>
  </ContextMenuPortal>
</ContextMenu>
```

### With checkbox and radio items

#### React

```tsx
const [bold, setBold] = React.useState(false);
const [file, setFile] = React.useState('index.js');

<ContextMenu.Root>
  <ContextMenu.Trigger>Right click here</ContextMenu.Trigger>
  <ContextMenu.Portal>
    <ContextMenu.Content>
      <ContextMenu.CheckboxItem checked={bold} onCheckedChange={setBold}>
        Bold
        <ContextMenu.ItemIndicator><TickIcon /></ContextMenu.ItemIndicator>
      </ContextMenu.CheckboxItem>
      <ContextMenu.Separator />
      <ContextMenu.RadioGroup value={file} onValueChange={setFile}>
        <ContextMenu.RadioItem value="readme.md">
          README.md
          <ContextMenu.ItemIndicator><TickIcon /></ContextMenu.ItemIndicator>
        </ContextMenu.RadioItem>
        <ContextMenu.RadioItem value="index.js">
          index.js
          <ContextMenu.ItemIndicator><TickIcon /></ContextMenu.ItemIndicator>
        </ContextMenu.RadioItem>
      </ContextMenu.RadioGroup>
    </ContextMenu.Content>
  </ContextMenu.Portal>
</ContextMenu.Root>
```

#### Leptos

```rust
let (bold, set_bold) = signal(false);
let (file, set_file) = signal("index.js".to_string());

<ContextMenu>
  <ContextMenuTrigger>"Right click here"</ContextMenuTrigger>
  <ContextMenuPortal>
    <ContextMenuContent>
      <ContextMenuCheckboxItem
        checked=Signal::derive(move || if bold.get() { CheckedState::True } else { CheckedState::False })
        on_checked_change=Callback::new(move |v: bool| set_bold.set(v))
      >
        "Bold"
        <ContextMenuItemIndicator>"✓"</ContextMenuItemIndicator>
      </ContextMenuCheckboxItem>
      <ContextMenuSeparator />
      <ContextMenuRadioGroup value=file on_value_change=Callback::new(move |v: String| set_file.set(v))>
        <ContextMenuRadioItem value="readme.md">
          "README.md"
          <ContextMenuItemIndicator>"✓"</ContextMenuItemIndicator>
        </ContextMenuRadioItem>
        <ContextMenuRadioItem value="index.js">
          "index.js"
          <ContextMenuItemIndicator>"✓"</ContextMenuItemIndicator>
        </ContextMenuRadioItem>
      </ContextMenuRadioGroup>
    </ContextMenuContent>
  </ContextMenuPortal>
</ContextMenu>
```

### With submenus

#### React

```tsx
<ContextMenu.Root>
  <ContextMenu.Trigger>Right click here</ContextMenu.Trigger>
  <ContextMenu.Portal>
    <ContextMenu.Content>
      <ContextMenu.Item>New Tab</ContextMenu.Item>
      <ContextMenu.Separator />
      <ContextMenu.Sub>
        <ContextMenu.SubTrigger>Bookmarks →</ContextMenu.SubTrigger>
        <ContextMenu.Portal>
          <ContextMenu.SubContent sideOffset={12}>
            <ContextMenu.Item>Inbox</ContextMenu.Item>
            <ContextMenu.Item>Calendar</ContextMenu.Item>
            <ContextMenu.Arrow />
          </ContextMenu.SubContent>
        </ContextMenu.Portal>
      </ContextMenu.Sub>
    </ContextMenu.Content>
  </ContextMenu.Portal>
</ContextMenu.Root>
```

#### Leptos

```rust
<ContextMenu>
  <ContextMenuTrigger>"Right click here"</ContextMenuTrigger>
  <ContextMenuPortal>
    <ContextMenuContent>
      <ContextMenuItem>"New Tab"</ContextMenuItem>
      <ContextMenuSeparator />
      <ContextMenuSub>
        <ContextMenuSubTrigger>"Bookmarks →"</ContextMenuSubTrigger>
        <ContextMenuPortal>
          <ContextMenuSubContent side_offset=12.0>
            <ContextMenuItem>"Inbox"</ContextMenuItem>
            <ContextMenuItem>"Calendar"</ContextMenuItem>
            <ContextMenuArrow />
          </ContextMenuSubContent>
        </ContextMenuPortal>
      </ContextMenuSub>
    </ContextMenuContent>
  </ContextMenuPortal>
</ContextMenu>
```

### Non-modal

#### React

```tsx
<ContextMenu.Root modal={false}>
  <ContextMenu.Trigger>Right click here</ContextMenu.Trigger>
  <ContextMenu.Portal>
    <ContextMenu.Content>
      <ContextMenu.Item>Undo</ContextMenu.Item>
      <ContextMenu.Item>Redo</ContextMenu.Item>
    </ContextMenu.Content>
  </ContextMenu.Portal>
</ContextMenu.Root>
```

#### Leptos

```rust
<ContextMenu modal=false>
  <ContextMenuTrigger>"Right click here"</ContextMenuTrigger>
  <ContextMenuPortal>
    <ContextMenuContent>
      <ContextMenuItem>"Undo"</ContextMenuItem>
      <ContextMenuItem>"Redo"</ContextMenuItem>
    </ContextMenuContent>
  </ContextMenuPortal>
</ContextMenu>
```

### Prevent closing on select

#### React

```tsx
<ContextMenu.Item
  onSelect={(event) => {
    event.preventDefault();
    window.alert('This action keeps the menu open');
  }}
>
  I won't close
</ContextMenu.Item>
```

#### Leptos

```rust
<ContextMenuItem
  on_select=Callback::new(move |event: ev::Event| {
    event.prevent_default();
    web_sys::window().unwrap().alert_with_message("This action keeps the menu open").ok();
  })
>
  "I won't close"
</ContextMenuItem>
```

## Accessibility

Implements the [WAI-ARIA Menu pattern](https://www.w3.org/WAI/ARIA/apd/patterns/menu/) triggered by right-click context.

### Keyboard Interactions

| Key | Description |
|---|---|
| `ArrowDown` | Moves focus to the next item in the menu. Wraps from last to first. |
| `ArrowUp` | Moves focus to the previous item in the menu. Wraps from first to last. |
| `ArrowRight` | When focus is on a `SubTrigger`, opens the submenu. In RTL, this closes the submenu instead. |
| `ArrowLeft` | When inside a submenu, closes it and returns focus to the parent `SubTrigger`. In RTL, this opens the submenu instead. |
| `Enter` / `Space` | Activates the focused item. If the item has `onSelect`, it fires and the menu closes (unless `event.preventDefault()` is called). |
| `Escape` | Closes the context menu and returns focus appropriately. |
| `Home` | Moves focus to the first item. |
| `End` | Moves focus to the last item. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `ContextMenuContent` | `role` | `"menu"` | Identifies the content as a menu. Inherited from `MenuContent`. |
| `ContextMenuContent` | `aria-orientation` | `"vertical"` | Indicates the menu is vertically oriented. |
| `ContextMenuItem` | `role` | `"menuitem"` | Identifies each item as a menu item. |
| `ContextMenuItem` | `aria-disabled` | `true` / absent | Set when the item is disabled. |
| `ContextMenuCheckboxItem` | `role` | `"menuitemcheckbox"` | Identifies the item as a checkbox menu item. |
| `ContextMenuCheckboxItem` | `aria-checked` | `true` / `false` / `"mixed"` | Reflects the checked state. |
| `ContextMenuRadioItem` | `role` | `"menuitemradio"` | Identifies the item as a radio menu item. |
| `ContextMenuRadioItem` | `aria-checked` | `true` / `false` | Reflects whether this radio item is selected. |
| `ContextMenuSeparator` | `role` | `"separator"` | Identifies the element as a separator. |
| `ContextMenuSeparator` | `aria-orientation` | `"horizontal"` | Indicates horizontal separator orientation. |
| `ContextMenuSubTrigger` | `aria-haspopup` | `"menu"` | Indicates the trigger opens a submenu. |
| `ContextMenuSubTrigger` | `aria-expanded` | `true` / `false` | Reflects whether the submenu is open. |
| `ContextMenuSubTrigger` | `aria-controls` | `string` | Points to the submenu content's `id`. |

### Behavioral Notes

- Right-clicking the trigger area opens the context menu at the cursor position. The menu is positioned using a virtual anchor at the click coordinates (zero-size rect).
- On touch devices, a long press (700ms) triggers the context menu.
- When `disabled` is set on the trigger, the native browser context menu fires instead.
- In modal mode (the default), a backdrop layer prevents interaction with other page elements and hides them from screen readers.
- In non-modal mode, the page remains interactive. Clicking outside the menu closes it, but focus is not forcibly restored to the trigger (since context menus are position-based, not element-based).
- The trigger renders as a `<span>` element (not a `<button>`), since it responds to right-click/long-press rather than primary click.
- `-webkit-touch-callout: none` is applied to the trigger to prevent the iOS native context menu from appearing.

## CSS Custom Properties

These properties are set on `ContextMenuContent` and `ContextMenuSubContent`, aliased from the underlying popper (floating-ui) values.

| Property | Source | Description |
|---|---|---|
| `--radix-context-menu-content-transform-origin` | `var(--radix-popper-transform-origin)` | The CSS transform origin computed by the positioning engine. Useful for scale/rotate animations anchored to the menu's attachment point. |
| `--radix-context-menu-content-available-width` | `var(--radix-popper-available-width)` | The available width between the virtual anchor and the viewport edge. |
| `--radix-context-menu-content-available-height` | `var(--radix-popper-available-height)` | The available height between the virtual anchor and the viewport edge. |
| `--radix-context-menu-trigger-width` | `var(--radix-popper-anchor-width)` | The width of the virtual anchor (always `0` for context menus since the anchor is a point). |
| `--radix-context-menu-trigger-height` | `var(--radix-popper-anchor-height)` | The height of the virtual anchor (always `0` for context menus since the anchor is a point). |
