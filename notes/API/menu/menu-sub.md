# MenuSub

## React Signature

```typescript
interface MenuSubProps {
  children?: React.ReactNode;
  open?: boolean;
  onOpenChange?(open: boolean): void;
}

const MenuSub: React.FC<MenuSubProps> = (props) => { ... }
```

`MenuSub` is an `FC` (not `forwardRef`) — it does not render a DOM element. It is a context provider wrapping a Popper root.

## Leptos Signature

```rust
pub fn MenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | The controlled open state of the submenu. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Called when the submenu open state changes. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The submenu content tree (typically `MenuSubTrigger` and `MenuPortal > MenuSubContent`). |

### Implicit behavior

- Wraps children in a new `Popper` root, creating an independent positioning context for the submenu.
- Provides a new `MenuContextValue` (with its own `open` state) and a `MenuSubContextValue` (carrying auto-generated `contentId`, `triggerId`, and `triggerRef`) to descendants.
- Automatically closes when the parent menu closes — an Effect watches the parent `MenuContextValue.open` and calls `onOpenChange(false)` when it becomes `false`.
- Closes on cleanup (component unmount).
