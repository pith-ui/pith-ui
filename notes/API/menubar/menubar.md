# Menubar (Root)

## Anatomy

The expected component nesting structure:

```
Menubar
└── MenubarMenu (one per menu)
    ├── MenubarTrigger
    └── MenubarPortal (optional)
        └── MenubarContent
            ├── MenubarItem
            ├── MenubarGroup
            │   ├── MenubarLabel
            │   └── MenubarItem
            ├── MenubarCheckboxItem
            │   └── MenubarItemIndicator
            ├── MenubarRadioGroup
            │   └── MenubarRadioItem
            │       └── MenubarItemIndicator
            ├── MenubarSeparator
            ├── MenubarSub
            │   ├── MenubarSubTrigger
            │   └── MenubarPortal (optional)
            │       └── MenubarSubContent
            │           └── MenubarArrow (optional)
            └── MenubarArrow (optional)
```

### React

```tsx
<Menubar.Root>
  <Menubar.Menu>
    <Menubar.Trigger>File</Menubar.Trigger>
    <Menubar.Portal>
      <Menubar.Content>
        <Menubar.Item>New Tab</Menubar.Item>
        <Menubar.Separator />
        <Menubar.Sub>
          <Menubar.SubTrigger>Share</Menubar.SubTrigger>
          <Menubar.Portal>
            <Menubar.SubContent>
              <Menubar.Item>Email</Menubar.Item>
            </Menubar.SubContent>
          </Menubar.Portal>
        </Menubar.Sub>
      </Menubar.Content>
    </Menubar.Portal>
  </Menubar.Menu>
</Menubar.Root>
```

### Leptos

```rust
<Menubar>
  <MenubarMenu>
    <MenubarTrigger>"File"</MenubarTrigger>
    <MenubarPortal>
      <MenubarContent>
        <MenubarItem>"New Tab"</MenubarItem>
        <MenubarSeparator />
        <MenubarSub>
          <MenubarSubTrigger>"Share"</MenubarSubTrigger>
          <MenubarPortal>
            <MenubarSubContent>
              <MenubarItem>"Email"</MenubarItem>
            </MenubarSubContent>
          </MenubarPortal>
        </MenubarSub>
      </MenubarContent>
    </MenubarPortal>
  </MenubarMenu>
</Menubar>
```

## React Signature

```typescript
const Menubar = React.forwardRef<MenubarElement, MenubarProps>(...)

type MenubarElement = React.ComponentRef<typeof Primitive.div>;
type RovingFocusGroupProps = React.ComponentPropsWithoutRef<typeof RovingFocusGroup.Root>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface MenubarProps extends PrimitiveDivProps {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  loop?: RovingFocusGroupProps['loop'];
  dir?: RovingFocusGroupProps['dir'];
}
```

## Leptos Signature

```rust
pub fn Menubar(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The controlled value of the currently open menu. The value corresponds to the `value` prop of the `MenubarMenu` that should be open. When set, the component becomes controlled. An empty string `""` means no menu is open. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The value of the menu that should be open on initial render. Use when you do not need to control which menu is open. Defaults to `""` (no menu open). |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the open menu changes. Receives the new value string (or `""` when all menus close). |
| `loop` | `r#loop` | `boolean` (default `true`) | `MaybeProp<bool>` (default `true`) | Whether keyboard navigation should loop from the last trigger back to the first (and vice versa). Also controls looping within `MenubarContent` when navigating between menus via arrow keys. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal arrow-key navigation between menus: in RTL, `ArrowRight` moves to the previous menu and `ArrowLeft` to the next. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>` with `role="menubar"`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

The root `<div>` renders with `role="menubar"`. No custom `data-*` attributes are set on the root element itself.

### Implicit behavior

- Internally wraps children in a `RovingFocusGroup` configured with `orientation="horizontal"` for left/right arrow key navigation between triggers.
- Manages a `currentTabStopId` signal independently of `RovingFocusGroup`'s default focus tracking, since triggers may never receive focus directly (e.g., click-to-open then click-outside-to-close).
- The `value` prop represents which menu is currently open. An empty string means no menu is open. When a menu opens, `value` is set to that menu's value; when it closes, `value` is set to `""`.

## Usage Examples

### Basic (uncontrolled)

#### React

```tsx
<Menubar.Root>
  <Menubar.Menu>
    <Menubar.Trigger>File</Menubar.Trigger>
    <Menubar.Portal>
      <Menubar.Content sideOffset={2}>
        <Menubar.Item>New Tab</Menubar.Item>
        <Menubar.Item>New Window</Menubar.Item>
        <Menubar.Separator />
        <Menubar.Item>Print...</Menubar.Item>
      </Menubar.Content>
    </Menubar.Portal>
  </Menubar.Menu>

  <Menubar.Menu>
    <Menubar.Trigger>Edit</Menubar.Trigger>
    <Menubar.Portal>
      <Menubar.Content sideOffset={2}>
        <Menubar.Item>Undo</Menubar.Item>
        <Menubar.Item>Redo</Menubar.Item>
      </Menubar.Content>
    </Menubar.Portal>
  </Menubar.Menu>
</Menubar.Root>
```

#### Leptos

```rust
<Menubar>
  <MenubarMenu>
    <MenubarTrigger>"File"</MenubarTrigger>
    <MenubarPortal>
      <MenubarContent side_offset=2.0>
        <MenubarItem>"New Tab"</MenubarItem>
        <MenubarItem>"New Window"</MenubarItem>
        <MenubarSeparator />
        <MenubarItem>"Print..."</MenubarItem>
      </MenubarContent>
    </MenubarPortal>
  </MenubarMenu>

  <MenubarMenu>
    <MenubarTrigger>"Edit"</MenubarTrigger>
    <MenubarPortal>
      <MenubarContent side_offset=2.0>
        <MenubarItem>"Undo"</MenubarItem>
        <MenubarItem>"Redo"</MenubarItem>
      </MenubarContent>
    </MenubarPortal>
  </MenubarMenu>
</Menubar>
```

### Controlled

#### React

```tsx
const [value, setValue] = React.useState('');

<Menubar.Root value={value} onValueChange={setValue}>
  <Menubar.Menu value="file">
    <Menubar.Trigger>File</Menubar.Trigger>
    <Menubar.Portal>
      <Menubar.Content>
        <Menubar.Item>New Tab</Menubar.Item>
      </Menubar.Content>
    </Menubar.Portal>
  </Menubar.Menu>
</Menubar.Root>
```

#### Leptos

```rust
let (value, set_value) = signal(String::new());

<Menubar
  value=value
  on_value_change=Callback::new(move |v: String| set_value.set(v))
>
  <MenubarMenu value="file".to_string()>
    <MenubarTrigger>"File"</MenubarTrigger>
    <MenubarPortal>
      <MenubarContent>
        <MenubarItem>"New Tab"</MenubarItem>
      </MenubarContent>
    </MenubarPortal>
  </MenubarMenu>
</Menubar>
```

### With checkbox and radio items

#### React

```tsx
const [bold, setBold] = React.useState(false);
const [file, setFile] = React.useState('index.js');

<Menubar.Root>
  <Menubar.Menu>
    <Menubar.Trigger>View</Menubar.Trigger>
    <Menubar.Portal>
      <Menubar.Content>
        <Menubar.CheckboxItem checked={bold} onCheckedChange={setBold}>
          Bold
          <Menubar.ItemIndicator>
            <TickIcon />
          </Menubar.ItemIndicator>
        </Menubar.CheckboxItem>
        <Menubar.Separator />
        <Menubar.RadioGroup value={file} onValueChange={setFile}>
          <Menubar.RadioItem value="index.js">
            index.js
            <Menubar.ItemIndicator>
              <TickIcon />
            </Menubar.ItemIndicator>
          </Menubar.RadioItem>
          <Menubar.RadioItem value="page.css">
            page.css
            <Menubar.ItemIndicator>
              <TickIcon />
            </Menubar.ItemIndicator>
          </Menubar.RadioItem>
        </Menubar.RadioGroup>
      </Menubar.Content>
    </Menubar.Portal>
  </Menubar.Menu>
</Menubar.Root>
```

#### Leptos

```rust
let (bold, set_bold) = signal(false);
let (file, set_file) = signal("index.js".to_string());

<Menubar>
  <MenubarMenu>
    <MenubarTrigger>"View"</MenubarTrigger>
    <MenubarPortal>
      <MenubarContent>
        <MenubarCheckboxItem
          checked=Signal::derive(move || bold.get().into())
          on_checked_change=Callback::new(move |v: bool| set_bold.set(v))
        >
          "Bold"
          <MenubarItemIndicator>
            {tick_icon()}
          </MenubarItemIndicator>
        </MenubarCheckboxItem>
        <MenubarSeparator />
        <MenubarRadioGroup
          value=file
          on_value_change=Callback::new(move |v: String| set_file.set(v))
        >
          <MenubarRadioItem value="index.js">
            "index.js"
            <MenubarItemIndicator>
              {tick_icon()}
            </MenubarItemIndicator>
          </MenubarRadioItem>
          <MenubarRadioItem value="page.css">
            "page.css"
            <MenubarItemIndicator>
              {tick_icon()}
            </MenubarItemIndicator>
          </MenubarRadioItem>
        </MenubarRadioGroup>
      </MenubarContent>
    </MenubarPortal>
  </MenubarMenu>
</Menubar>
```

### With submenus

#### React

```tsx
<Menubar.Root>
  <Menubar.Menu>
    <Menubar.Trigger>Edit</Menubar.Trigger>
    <Menubar.Portal>
      <Menubar.Content sideOffset={2}>
        <Menubar.Item>Undo</Menubar.Item>
        <Menubar.Separator />
        <Menubar.Sub>
          <Menubar.SubTrigger>Find</Menubar.SubTrigger>
          <Menubar.Portal>
            <Menubar.SubContent alignOffset={-6}>
              <Menubar.Item>Find...</Menubar.Item>
              <Menubar.Item>Find Next</Menubar.Item>
            </Menubar.SubContent>
          </Menubar.Portal>
        </Menubar.Sub>
      </Menubar.Content>
    </Menubar.Portal>
  </Menubar.Menu>
</Menubar.Root>
```

#### Leptos

```rust
<Menubar>
  <MenubarMenu>
    <MenubarTrigger>"Edit"</MenubarTrigger>
    <MenubarPortal>
      <MenubarContent side_offset=2.0>
        <MenubarItem>"Undo"</MenubarItem>
        <MenubarSeparator />
        <MenubarSub>
          <MenubarSubTrigger>"Find"</MenubarSubTrigger>
          <MenubarPortal>
            <MenubarSubContent align_offset=-6.0>
              <MenubarItem>"Find..."</MenubarItem>
              <MenubarItem>"Find Next"</MenubarItem>
            </MenubarSubContent>
          </MenubarPortal>
        </MenubarSub>
      </MenubarContent>
    </MenubarPortal>
  </MenubarMenu>
</Menubar>
```

## Accessibility

Implements the [WAI-ARIA Menu Bar pattern](https://www.w3.org/WAI/ARIA/apd/patterns/menubar/).

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on a trigger, toggles the associated menu open/closed. When focus is on a menu item, activates it. |
| `ArrowDown` | When focus is on a trigger, opens the associated menu. When focus is inside a menu, moves focus to the next item. |
| `ArrowUp` | When focus is inside a menu, moves focus to the previous item. |
| `ArrowRight` | When focus is on a trigger, moves focus to the next trigger (LTR). When focus is inside a menu content, opens the next menu in the menubar (LTR) or opens a submenu if the focused item is a sub-trigger. |
| `ArrowLeft` | When focus is on a trigger, moves focus to the previous trigger (LTR). When focus is inside a menu content, opens the previous menu in the menubar (LTR) or closes a submenu if inside one. |
| `Escape` | Closes the currently open menu and returns focus to the trigger. |
| `Home` | When focus is inside a menu, moves focus to the first item. |
| `End` | When focus is inside a menu, moves focus to the last item. |

Arrow key directions are reversed in RTL mode.

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `Menubar` root | `role` | `"menubar"` | Identifies the element as a menubar. |
| `MenubarTrigger` | `role` | `"menuitem"` | Each trigger is a menuitem within the menubar. |
| `MenubarTrigger` | `aria-haspopup` | `"menu"` | Indicates the trigger opens a menu. |
| `MenubarTrigger` | `aria-expanded` | `"true"` / `"false"` | Reflects whether the associated menu is open. |
| `MenubarTrigger` | `aria-controls` | `string` | Points to the content element's `id` when the menu is open. |
| `MenubarContent` | `aria-labelledby` | `string` | Points to the trigger's auto-generated `id`. |

### Behavioral Notes

- The menubar operates in non-modal mode -- menus do not trap focus or block interaction with the rest of the page.
- Hovering a trigger while another menu is already open immediately opens the hovered menu (menu switching behavior).
- When a menu is opened via pointer, focus is not moved to the first item; when opened via keyboard (`Enter`, `Space`, `ArrowDown`), focus moves to the first item.
- Clicking a trigger toggles the menu; pressing `Enter`/`Space` on a trigger also toggles.
- Disabled triggers are skipped during roving focus keyboard navigation.
- Arrow key navigation between menus from within content skips disabled items and respects the `loop` prop.
- When a submenu's sub-trigger has focus and the "next" arrow key is pressed, it opens the submenu rather than navigating to the next menubar menu.

## CSS Custom Properties

These properties are set on `MenubarContent` and `MenubarSubContent` and alias the underlying popper values. Use them in CSS for positioning-aware styling.

| Property | Source | Description |
|---|---|---|
| `--radix-menubar-content-transform-origin` | `var(--radix-popper-transform-origin)` | The transform origin computed by the popper for entry/exit animations. |
| `--radix-menubar-content-available-width` | `var(--radix-popper-available-width)` | The available width between the trigger and the viewport edge. |
| `--radix-menubar-content-available-height` | `var(--radix-popper-available-height)` | The available height between the trigger and the viewport edge. |
| `--radix-menubar-trigger-width` | `var(--radix-popper-anchor-width)` | The width of the trigger (anchor) element. |
| `--radix-menubar-trigger-height` | `var(--radix-popper-anchor-height)` | The height of the trigger (anchor) element. |
