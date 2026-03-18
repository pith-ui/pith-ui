# MenubarMenu

## React Signature

```typescript
interface MenubarMenuProps {
  value?: string;
  children?: React.ReactNode;
}

const MenubarMenu: React.FC<MenubarMenuProps> = (props) => { ... }
```

`MenubarMenu` is not a `forwardRef` component -- it is a plain functional component that does not render a DOM element itself. It wraps `MenuPrimitive.Root` and provides context.

## Leptos Signature

```rust
pub fn MenubarMenu(
    #[prop(into, optional)] value: MaybeProp<String>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | A unique string identifying this menu within the menubar. This value is compared against the menubar's `value` prop to determine if this menu is open. If omitted, an auto-generated ID is used. Must be provided when using controlled mode on the parent `Menubar`. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The menu's trigger and content. |

### Implicit behavior

- Generates auto IDs for its trigger and content elements, used for `aria-controls` / `aria-labelledby` wiring.
- Provides a `MenubarMenuContextValue` context containing the menu's `value`, `trigger_id`, `trigger_ref`, `content_id`, and a `was_keyboard_trigger_open_ref` flag.
- Wraps children in a `Menu` (from the menu primitive) with `modal=false` and wires `on_open_change` so that closing the underlying menu also clears the menubar's value.
- When the menu is not open, `was_keyboard_trigger_open_ref` is reset to `false`.
- If no `value` is provided, React uses `useId()` and Leptos uses `use_id()` to generate one. React has a fallback `'LEGACY_REACT_AUTO_VALUE'` for legacy React versions; Leptos uses the ID directly.
