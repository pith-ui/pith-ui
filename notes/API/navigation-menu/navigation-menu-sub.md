# NavigationMenuSub

## React Signature

```typescript
type NavigationMenuSubElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface NavigationMenuSubProps extends PrimitiveDivProps {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  orientation?: Orientation; // 'horizontal' | 'vertical'
}

const NavigationMenuSub = React.forwardRef<NavigationMenuSubElement, NavigationMenuSubProps>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenuSub(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))]
    orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The controlled value of the active sub-item. When set, the sub-menu becomes controlled. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The value of the sub-item that is active on initial render. Defaults to `""` (none active). |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the active sub-item changes. |
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>` (default `Horizontal`) | The layout axis for the sub-menu. Controls arrow-key navigation direction within the sub-menu. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the sub-menu's `orientation` prop. |

### Implicit behavior

- Inherits `dir` and `rootNavigationMenu` from the parent `NavigationMenu` context. The React interface also omits `dir` from the public props (it is inherited from the parent context).
- Does not support `delayDuration` or `skipDelayDuration` -- sub-menus open instantly on trigger enter/select (no hover delay).
- Does not fire `onTriggerLeave`, `onContentEnter`, or `onContentLeave` callbacks -- these are no-ops in sub-menus (only relevant for root-level hover behavior).
- Sets `isRootMenu=false` in its context provider, which affects how content dismissal and pointer-events-on-close behave in descendant parts.
