# AspectRatio

AspectRatio is a single-part component -- it has no sub-parts. The root element is the only part.

## Anatomy

```
AspectRatio
└── (children)
```

### React

```tsx
<AspectRatio.Root ratio={16 / 9}>
  <img src="..." alt="..." style={{ objectFit: 'cover', width: '100%', height: '100%' }} />
</AspectRatio.Root>
```

### Leptos

```rust
<AspectRatio ratio=16.0 / 9.0>
    <img src="..." alt="..." style:object-fit="cover" style:width="100%" style:height="100%" />
</AspectRatio>
```

## React Signature

```typescript
type AspectRatioElement = React.ComponentRef<typeof Primitive.div>;
type PrimitiveDivProps = React.ComponentPropsWithoutRef<typeof Primitive.div>;

interface AspectRatioProps extends PrimitiveDivProps {
  ratio?: number;
}

const AspectRatio = React.forwardRef<AspectRatioElement, AspectRatioProps>(
  (props, forwardedRef) => {
    const { ratio = 1 / 1, style, ...aspectRatioProps } = props;
    // ...
  }
);
```

## Leptos Signature

```rust
pub fn AspectRatio(
    #[prop(into, optional, default = 1.0.into())] ratio: Signal<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ratio` | `ratio` | `number` (default `1/1`) | `Signal<f64>` (default `1.0`) | The desired width-to-height ratio. For example, `16/9` produces a widescreen aspect ratio. The component uses a padding-bottom CSS trick to enforce the ratio: `padding-bottom = 100% / ratio`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the inner DOM element (`<div>`). Note: this refs the inner content `<div>`, not the outer wrapper. |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, the inner element renders its child directly instead of wrapping in a `<div>`, merging props and refs onto the child. The outer wrapper `<div>` is always rendered regardless of this prop. |
| `style` | — | `React.CSSProperties` | — | React merges the user-provided `style` onto the inner element alongside the component's absolute-positioning styles. In Leptos, use `style:` directives on the `<AspectRatio>` element instead. |
| *(spread)* | — | `...PrimitiveDivProps` | — | React allows spreading any `<div>` HTML attribute. Leptos uses `attr:` directives on the call site instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-radix-aspect-ratio-wrapper` | `""` | Set on the outer wrapper `<div>` (not the inner content element). Can be used for styling the wrapper. |

### Implicit behavior

- **Two-element DOM structure:** The component always renders two nested `<div>` elements. The outer wrapper has `position: relative`, `width: 100%`, and a computed `padding-bottom` based on the ratio. The inner element has `position: absolute` with `top/right/bottom/left: 0`, causing children to fill the aspect-ratio-constrained space.
- **Wrapper is not replaceable:** The outer wrapper `<div>` cannot be replaced or removed via `as_child`. The `as_child` prop only affects the inner content element. This is the same behavior as the React reference.
- **Reactive ratio:** In Leptos, `ratio` is a `Signal<f64>`, so the padding-bottom recalculates reactively when the ratio changes.

## Usage Examples

### Default ratio (1:1)

#### React

```tsx
<div style={{ width: 300 }}>
  <AspectRatio.Root>
    <p>Square content (1:1)</p>
  </AspectRatio.Root>
</div>
```

#### Leptos

```rust
<div style:width="300px">
    <AspectRatio>
        <p>"Square content (1:1)"</p>
    </AspectRatio>
</div>
```

### Custom ratio with image

#### React

```tsx
<div style={{ width: 400 }}>
  <AspectRatio.Root ratio={16 / 9}>
    <img
      src="https://example.com/photo.jpg"
      alt="Landscape"
      style={{ objectFit: 'cover', width: '100%', height: '100%' }}
    />
  </AspectRatio.Root>
</div>
```

#### Leptos

```rust
<div style:width="400px">
    <AspectRatio ratio=16.0 / 9.0>
        <img
            src="https://example.com/photo.jpg"
            alt="Landscape"
            style:object-fit="cover"
            style:width="100%"
            style:height="100%"
        />
    </AspectRatio>
</div>
```

### Multiple ratios side by side

#### React

```tsx
<div style={{ display: 'flex', gap: 20 }}>
  <div style={{ width: 200 }}>
    <AspectRatio.Root ratio={1 / 2}>{image}</AspectRatio.Root>
  </div>
  <div style={{ width: 200 }}>
    <AspectRatio.Root>{image}</AspectRatio.Root>
  </div>
  <div style={{ width: 200 }}>
    <AspectRatio.Root ratio={16 / 9}>{image}</AspectRatio.Root>
  </div>
  <div style={{ width: 200 }}>
    <AspectRatio.Root ratio={2 / 1}>{image}</AspectRatio.Root>
  </div>
</div>
```

#### Leptos

```rust
<div style:display="flex" style:gap="20px">
    <div style:width="200px">
        <AspectRatio ratio=1.0 / 2.0><Image /></AspectRatio>
    </div>
    <div style:width="200px">
        <AspectRatio><Image /></AspectRatio>
    </div>
    <div style:width="200px">
        <AspectRatio ratio=16.0 / 9.0><Image /></AspectRatio>
    </div>
    <div style:width="200px">
        <AspectRatio ratio=2.0 / 1.0><Image /></AspectRatio>
    </div>
</div>
```

## Accessibility

AspectRatio is a purely presentational layout component. It does not implement any WAI-ARIA pattern and does not add ARIA attributes, roles, or keyboard interactions. It renders plain `<div>` elements.

Accessibility considerations are delegated to the content placed inside the component (e.g., ensuring images have appropriate `alt` text).
