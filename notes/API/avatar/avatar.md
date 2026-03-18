# Avatar (Root)

## Anatomy

The expected component nesting structure:

```
Avatar
├── AvatarImage
└── AvatarFallback
```

### React

```tsx
<Avatar.Root>
  <Avatar.Image src="..." alt="..." />
  <Avatar.Fallback>AB</Avatar.Fallback>
</Avatar.Root>
```

### Leptos

```rust
<Avatar>
  <AvatarImage src="..." attr:alt="..." />
  <AvatarFallback>"AB"</AvatarFallback>
</Avatar>
```

## React Signature

```typescript
const Avatar = React.forwardRef<AvatarElement, AvatarProps>(...)

type AvatarElement = React.ComponentRef<typeof Primitive.span>;
type PrimitiveSpanProps = React.ComponentPropsWithoutRef<typeof Primitive.span>;
interface AvatarProps extends PrimitiveSpanProps {}
```

## Leptos Signature

```rust
pub fn Avatar(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| *(spread)* | — | `...PrimitiveSpanProps` | — | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Provides an `AvatarContextValue` to descendant components via Leptos context (`Provider`). The context holds:
  - `image_loading_status: ReadSignal<ImageLoadingStatus>` — the current loading state of the image.
  - `on_image_loading_status_change: Callback<ImageLoadingStatus>` — callback to update the loading status (called by `AvatarImage`).
- Initializes `image_loading_status` to `ImageLoadingStatus::Idle`.

## Usage Examples

### Without image (fallback only)

#### React

```tsx
<Avatar.Root>
  <Avatar.Fallback>JS</Avatar.Fallback>
</Avatar.Root>
```

#### Leptos

```rust
<Avatar>
  <AvatarFallback>"JS"</AvatarFallback>
</Avatar>
```

### With image and fallback

#### React

```tsx
<Avatar.Root>
  <Avatar.Image src="https://example.com/photo.jpg" alt="John Smith" />
  <Avatar.Fallback delayMs={300}>JS</Avatar.Fallback>
</Avatar.Root>
```

#### Leptos

```rust
<Avatar>
  <AvatarImage
    src="https://example.com/photo.jpg"
    attr:alt="John Smith"
  />
  <AvatarFallback delay_ms=300>"JS"</AvatarFallback>
</Avatar>
```

### With broken image and icon fallback

#### React

```tsx
<Avatar.Root>
  <Avatar.Image
    src="https://broken.link.com/pic.jpg"
    alt="John Smith"
    onLoadingStatusChange={console.log}
  />
  <Avatar.Fallback>
    <MyIcon />
  </Avatar.Fallback>
</Avatar.Root>
```

#### Leptos

```rust
<Avatar>
  <AvatarImage
    src="https://broken.link.com/pic.jpg"
    on_loading_status_change=Callback::new(move |status: ImageLoadingStatus| {
      log::info!("{:?}", status);
    })
    attr:alt="John Smith"
  />
  <AvatarFallback>
    <MyIcon />
  </AvatarFallback>
</Avatar>
```

### Dynamic image source

#### React

```tsx
const [src, setSrc] = React.useState('https://example.com/photo1.jpg');

<Avatar.Root>
  <Avatar.Image src={src} alt="John Smith" />
  <Avatar.Fallback delayMs={300}>JS</Avatar.Fallback>
</Avatar.Root>
```

#### Leptos

```rust
let (src, set_src) = signal("https://example.com/photo1.jpg".to_string());

<Avatar>
  <AvatarImage
    src=Signal::derive(move || Some(src.get()))
    attr:alt="John Smith"
  />
  <AvatarFallback delay_ms=300>"JS"</AvatarFallback>
</Avatar>
```

## Accessibility

The Avatar component is a presentational component that does not implement a specific WAI-ARIA pattern. It provides an accessible image with automatic fallback behavior.

### Keyboard Interactions

No keyboard interactions. Avatar is not an interactive component.

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `AvatarImage` | `alt` | User-provided string | Standard `<img>` alt text. Must be provided by the consumer via `attr:alt` in Leptos or the `alt` prop in React for accessibility. |

### Behavioral Notes

- The `AvatarImage` renders as an `<img>` element only after the image has successfully loaded. Before that, nothing is rendered by `AvatarImage`.
- The `AvatarFallback` renders when the image has not loaded (status is not `'loaded'`). If `delayMs` is specified, the fallback waits that many milliseconds before appearing, avoiding a flash of fallback content for images that load quickly.
- When the `src` changes at runtime, the image loading cycle restarts: the image is hidden, the fallback reappears (if applicable), and the new image is loaded in the background.
- The image loading status is tracked via a hidden `Image()` element (not the rendered `<img>`), so loading detection works even before the image is added to the DOM.
