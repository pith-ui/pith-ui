# MenubarSub

## React Signature

```typescript
interface MenubarSubProps {
  children?: React.ReactNode;
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?(open: boolean): void;
}

const MenubarSub: React.FC<MenubarSubProps> = (props) => { ... }
```

`MenubarSub` is a plain functional component (not `forwardRef`) -- it does not render a DOM element itself.

## Leptos Signature

```rust
pub fn MenubarSub(
    #[prop(into, optional)] open: MaybeProp<bool>,
    #[prop(into, optional)] default_open: MaybeProp<bool>,
    #[prop(into, optional)] on_open_change: Option<Callback<bool>>,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `open` | `open` | `boolean \| undefined` | `MaybeProp<bool>` | The controlled open state of the submenu. When set, the component becomes controlled. |
| `defaultOpen` | `default_open` | `boolean \| undefined` | `MaybeProp<bool>` | Whether the submenu should be open on initial render. Defaults to `false`. |
| `onOpenChange` | `on_open_change` | `(open: boolean) => void` | `Option<Callback<bool>>` | Called when the open state changes. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | The sub-trigger and sub-content. |

### Implicit behavior

- Manages the open/close state of the submenu via `useControllableState`.
- Wraps children in the underlying `MenuSub` component, passing the open state and change handler.
