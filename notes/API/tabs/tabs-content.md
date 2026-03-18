# TabsContent

## React Signature

```typescript
const TabsContent = React.forwardRef<TabsContentElement, TabsContentProps>(...)

type TabsContentElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface TabsContentProps extends PrimitiveDivProps {
  value: string;
  /**
   * Used to force mounting when more control is needed. Useful when
   * controlling animation with React animation libraries.
   */
  forceMount?: true;
}
```

## Leptos Signature

```rust
pub fn TabsContent(
    /// A unique value matching the corresponding TabsTrigger.
    value: String,
    /// Force mount the content even when inactive (for animation control).
    #[prop(into, optional)]
    force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string` (required) | `String` (required) | A unique string matching the corresponding `TabsTrigger`'s `value`. The content is shown when this value matches the active tab. **Note:** `value` does not have `#[prop(into)]`, so it requires `.to_string()` at the call site for string literals. |
| `forceMount` | `force_mount` | `true \| undefined` | `MaybeProp<bool>` | Forces the content to stay mounted in the DOM even when its tab is inactive. Useful when controlling show/hide animations with CSS or animation libraries -- without this, the element is removed from the DOM when inactive and animations cannot run. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The tab panel content. Leptos wraps this in `Option` to allow an empty content region. |
| *(spread)* | -- | `...PrimitiveDivProps` | -- | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Renders with `role="tabpanel"` and `aria-labelledby` pointing to the corresponding `TabsTrigger`'s auto-generated `id` (`{baseId}-trigger-{value}`).
- The content's own `id` is auto-generated as `{baseId}-content-{value}` and is referenced by the trigger's `aria-controls`.
- `tabindex="0"` is set to make the panel focusable, allowing keyboard users to `Tab` into the content area.
- On initial mount, if the content is selected, `animation-duration` is set to `0s` for one animation frame (via `requestAnimationFrame`) to prevent entry animations from firing on the initially visible panel. This is cleared after the first frame.
- React sets `hidden={!present}` on the content element when using `forceMount`, so inactive panels are hidden via the HTML `hidden` attribute. The Leptos implementation does not set the `hidden` attribute; it relies on the `Presence` component for mount/unmount behavior.

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-state` | `"active" \| "inactive"` | Whether this content panel is currently visible. |
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the parent `Tabs` component's `orientation`. |
