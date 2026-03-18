# ScrollArea (Root)

## Anatomy

The expected component nesting structure:

```
ScrollArea
├── ScrollAreaViewport
│   └── (scrollable content)
├── ScrollAreaScrollbar (vertical)
│   └── ScrollAreaThumb
├── ScrollAreaScrollbar (horizontal)
│   └── ScrollAreaThumb
└── ScrollAreaCorner
```

### React

```tsx
<ScrollArea.Root type="hover" scrollHideDelay={600}>
  <ScrollArea.Viewport>
    {/* scrollable content */}
  </ScrollArea.Viewport>
  <ScrollArea.Scrollbar orientation="vertical">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
  <ScrollArea.Scrollbar orientation="horizontal">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
  <ScrollArea.Corner />
</ScrollArea.Root>
```

### Leptos

```rust
<ScrollArea r#type=ScrollAreaType::Hover scroll_hide_delay=600u32>
  <ScrollAreaViewport>
    // scrollable content
  </ScrollAreaViewport>
  <ScrollAreaScrollbar orientation=Orientation::Vertical>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
  <ScrollAreaScrollbar orientation=Orientation::Horizontal>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
  <ScrollAreaCorner>""</ScrollAreaCorner>
</ScrollArea>
```

## React Signature

```typescript
const ScrollArea = React.forwardRef<ScrollAreaElement, ScrollAreaProps>(...)

type ScrollAreaElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface ScrollAreaProps extends PrimitiveDivProps {
  type?: 'auto' | 'always' | 'scroll' | 'hover';
  dir?: 'ltr' | 'rtl';
  scrollHideDelay?: number;
}
```

## Leptos Signature

```rust
pub fn ScrollArea(
    #[prop(into, optional)] r#type: Option<ScrollAreaType>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] scroll_hide_delay: Option<u32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `type` | `r#type` | `'auto' \| 'always' \| 'scroll' \| 'hover'` (default `'hover'`) | `Option<ScrollAreaType>` (default `Hover`) | Controls scrollbar visibility behavior. `hover`: visible on pointer hover over the scroll area. `scroll`: visible when the user scrolls. `auto`: visible when content overflows. `always`: always visible. |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal scrollbar positioning (left/right) and drag-scroll direction. Inherited from a `DirectionProvider` if not set. |
| `scrollHideDelay` | `scroll_hide_delay` | `number` (default `600`) | `Option<u32>` (default `600`) | Delay in milliseconds before scrollbars hide after the user stops interacting. Only applies to `type="hover"` and `type="scroll"`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<div>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<div>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives instead. |

### Leptos-only: `ScrollAreaType` enum

```rust
#[derive(Default)]
pub enum ScrollAreaType {
    Auto,
    Always,
    Scroll,
    #[default]
    Hover,
}
```

### Data attributes (rendered on DOM)

ScrollArea does not render its own `data-*` attributes. It sets CSS custom properties on the root element (see CSS Custom Properties below).

### Implicit behavior

- Sets `position: relative` on the root element.
- Sets `dir` attribute on the root element reflecting the resolved direction.
- Provides a context to all child components (`ScrollAreaViewport`, `ScrollAreaScrollbar`, `ScrollAreaThumb`, `ScrollAreaCorner`) with shared state: scrollbar type, direction, hide delay, and element refs.

## Usage Examples

### Basic (vertical scrollbar, hover visibility)

#### React

```tsx
<ScrollArea.Root style={{ width: 200, height: 200 }}>
  <ScrollArea.Viewport>
    <p>Long scrollable content...</p>
  </ScrollArea.Viewport>
  <ScrollArea.Scrollbar orientation="vertical">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
</ScrollArea.Root>
```

#### Leptos

```rust
<ScrollArea attr:style="width: 200px; height: 200px;">
  <ScrollAreaViewport>
    <p>"Long scrollable content..."</p>
  </ScrollAreaViewport>
  <ScrollAreaScrollbar orientation=Orientation::Vertical>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
</ScrollArea>
```

### Both axes with always-visible scrollbars

#### React

```tsx
<ScrollArea.Root type="always" style={{ width: 400, height: 400 }}>
  <ScrollArea.Viewport>{children}</ScrollArea.Viewport>
  <ScrollArea.Scrollbar orientation="vertical">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
  <ScrollArea.Scrollbar orientation="horizontal">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
  <ScrollArea.Corner />
</ScrollArea.Root>
```

#### Leptos

```rust
<ScrollArea r#type=ScrollAreaType::Always attr:style="width: 400px; height: 400px;">
  <ScrollAreaViewport>
    {children()}
  </ScrollAreaViewport>
  <ScrollAreaScrollbar orientation=Orientation::Vertical>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
  <ScrollAreaScrollbar orientation=Orientation::Horizontal>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
  <ScrollAreaCorner>""</ScrollAreaCorner>
</ScrollArea>
```

### RTL direction

#### React

```tsx
<ScrollArea.Root type="always" dir="rtl">
  <ScrollArea.Viewport>{children}</ScrollArea.Viewport>
  <ScrollArea.Scrollbar orientation="vertical">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
  <ScrollArea.Scrollbar orientation="horizontal">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
  <ScrollArea.Corner />
</ScrollArea.Root>
```

#### Leptos

```rust
<ScrollArea r#type=ScrollAreaType::Always dir=Direction::Rtl>
  <ScrollAreaViewport>
    {children()}
  </ScrollAreaViewport>
  <ScrollAreaScrollbar orientation=Orientation::Vertical>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
  <ScrollAreaScrollbar orientation=Orientation::Horizontal>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
  <ScrollAreaCorner>""</ScrollAreaCorner>
</ScrollArea>
```

### Custom hide delay (scroll type)

#### React

```tsx
<ScrollArea.Root type="scroll" scrollHideDelay={1000}>
  <ScrollArea.Viewport>{children}</ScrollArea.Viewport>
  <ScrollArea.Scrollbar orientation="vertical">
    <ScrollArea.Thumb />
  </ScrollArea.Scrollbar>
</ScrollArea.Root>
```

#### Leptos

```rust
<ScrollArea r#type=ScrollAreaType::Scroll scroll_hide_delay=1000u32>
  <ScrollAreaViewport>
    {children()}
  </ScrollAreaViewport>
  <ScrollAreaScrollbar orientation=Orientation::Vertical>
    <ScrollAreaThumb>""</ScrollAreaThumb>
  </ScrollAreaScrollbar>
</ScrollArea>
```

## Accessibility

ScrollArea is a presentational component that provides custom-styled scrollbars while hiding native browser scrollbars. It does not implement a specific WAI-ARIA pattern — scrolling behavior is provided by the native viewport, and the custom scrollbars are purely visual overlays driven by pointer interaction.

### Keyboard Interactions

| Key | Description |
|---|---|
| Standard scroll keys | All keyboard scrolling (arrow keys, Page Up/Down, Home/End, Space) is handled natively by the viewport element. The custom scrollbars reflect the scroll position but do not intercept keyboard input. |

### Behavioral Notes

- Native scrollbars are hidden using CSS (`scrollbar-width: none`, `-ms-overflow-style: none`, and `::-webkit-scrollbar { display: none }`). A `<style>` element is injected by `ScrollAreaViewport` to apply these styles.
- The viewport uses `overflow: scroll` (not `auto`) when a scrollbar is present for the given axis, and `overflow: hidden` when no scrollbar is present. This prevents the browser from trying to decide whether to show native scrollbars.
- Touch scrolling uses `-webkit-overflow-scrolling: touch` for momentum scroll on iOS.
- The custom scrollbars do not receive focus and are not keyboard-accessible — all scrolling is via the native viewport. This is intentional: the custom scrollbars are a visual overlay, and keyboard users scroll the viewport directly.

## CSS Custom Properties

These properties are set on the `ScrollArea` root element. They reflect the measured dimensions of the corner (the intersection of horizontal and vertical scrollbars) and are used internally by the scrollbar positioning CSS.

| Property | Source | Description |
|---|---|---|
| `--radix-scroll-area-corner-width` | Measured from `ScrollAreaCorner` | The width of the corner element in pixels. Used by the vertical scrollbar to set its `bottom` offset and by the horizontal scrollbar to set its `right`/`left` offset, so the scrollbars do not overlap the corner. |
| `--radix-scroll-area-corner-height` | Measured from `ScrollAreaCorner` | The height of the corner element in pixels. Used by the vertical scrollbar to set its `bottom` offset. |
| `--radix-scroll-area-thumb-width` | Calculated from viewport/content ratio | The computed width of the horizontal scrollbar thumb in pixels. Set on the horizontal scrollbar element. Use in CSS to size the thumb. |
| `--radix-scroll-area-thumb-height` | Calculated from viewport/content ratio | The computed height of the vertical scrollbar thumb in pixels. Set on the vertical scrollbar element. Use in CSS to size the thumb. |
