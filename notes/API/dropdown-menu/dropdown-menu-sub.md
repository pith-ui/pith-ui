# DropdownMenuSub

## React Signature

```typescript
interface DropdownMenuSubProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
}

const DropdownMenuSub: React.FC<DropdownMenuSubProps> = (props) => { ... };
```

`DropdownMenuSub` is a plain `React.FC` (not `forwardRef`) since it is a context-only wrapper with no DOM element of its own.

## Leptos Signature

```rust
pub fn DropdownMenuSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state of the submenu. When set, the submenu becomes controlled. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | The open state of the submenu on initial render. Use when you do not need to control the submenu state. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Callback fired when the submenu open state changes. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | Must include a `DropdownMenuSubTrigger` and a portal/content pair. |

### Implicit behavior

- Wraps children in a `MenuSub` primitive that manages submenu open/close state.
- Uses `useControllableState` to support both controlled and uncontrolled modes.
- In React, closing the parent menu also closes any open submenus.
