# ContextMenuSub

## React Signature

```typescript
const ContextMenuSub: React.FC<ContextMenuSubProps> = (props) => { ... }

interface ContextMenuSubProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
}
```

`ContextMenuSub` is not a `forwardRef` component — it does not render a DOM element itself. It manages submenu open state and wraps `Menu.Sub`.

## Leptos Signature

```rust
pub fn ContextMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean` | `MaybeProp<bool>` | The controlled open state of the submenu. When set, the component becomes controlled. |
| `defaultOpen` | `default_open` | `boolean` (default `false`) | `MaybeProp<bool>` | The open state on initial render when uncontrolled. Defaults to `false`. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Called when the submenu opens or closes. Receives the new open state. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The sub-trigger and portal content. |

### Implicit behavior

- Uses `useControllableState` to manage the open state, supporting both controlled and uncontrolled modes.
- Provides the open state to child `ContextMenuSubTrigger` and `ContextMenuSubContent` components via context (through the underlying `Menu.Sub`).
