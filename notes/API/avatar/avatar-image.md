# AvatarImage

## React Signature

```typescript
const AvatarImage = React.forwardRef<AvatarImageElement, AvatarImageProps>(...)

type AvatarImageElement = React.ComponentRef<typeof Primitive.img>;
type PrimitiveImageProps = React.ComponentPropsWithoutRef<typeof Primitive.img>;

interface AvatarImageProps extends PrimitiveImageProps {
  onLoadingStatusChange?: (status: ImageLoadingStatus) => void;
}
```

`ImageLoadingStatus` is defined as:

```typescript
type ImageLoadingStatus = 'idle' | 'loading' | 'loaded' | 'error';
```

## Leptos Signature

```rust
pub fn AvatarImage(
    #[prop(into, optional)] src: MaybeProp<String>,
    /// The referrer policy to use when fetching the image for loading status detection.
    #[prop(into, optional)]
    referrer_policy: MaybeProp<String>,
    /// The CORS setting to use when fetching the image for loading status detection.
    #[prop(into, optional)]
    cross_origin: MaybeProp<String>,
    #[prop(into, optional)] on_loading_status_change: Option<Callback<ImageLoadingStatus>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

`ImageLoadingStatus` is defined as:

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageLoadingStatus {
    Idle,
    Loading,
    Loaded,
    Error,
}
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `src` | `src` | `string \| undefined` (from `<img>` props) | `MaybeProp<String>` | The image URL. When `src` is `undefined` or empty, the image is not rendered and the loading status is set to `'error'`, causing the fallback to display. |
| `onLoadingStatusChange` | `on_loading_status_change` | `(status: ImageLoadingStatus) => void` | `Option<Callback<ImageLoadingStatus>>` | Callback fired when the image loading status changes. Receives one of `'idle'`, `'loading'`, `'loaded'`, or `'error'` (React) / `ImageLoadingStatus::Idle`, `Loading`, `Loaded`, `Error` (Leptos). |
| `referrerPolicy` | `referrer_policy` | `React.HTMLAttributeReferrerPolicy` (from `<img>` spread) | `MaybeProp<String>` | The referrer policy for image fetch. In React this is passed via spread on `<img>` props; Leptos exposes it as a named prop because `use_image_loading_status` needs it to configure the hidden image loader. |
| `crossOrigin` | `cross_origin` | `string` (from `<img>` spread) | `MaybeProp<String>` | The CORS setting for image fetch. In React this is passed via spread on `<img>` props; Leptos exposes it as a named prop for the same reason as `referrer_policy`. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered `<img>` DOM element. Only available when the image is in the `'loaded'` state (since the element is not rendered otherwise). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in an `<img>`, merging props and refs. |
| — | `children` | `React.ReactNode` (from spread) | `Option<ChildrenFn>` | Optional children. Leptos explicitly accepts this as a prop; in React, children are part of the standard JSX spread. |
| *(spread)* | — | `...PrimitiveImageProps` | — | React allows spreading any `<img>` HTML attribute (e.g., `alt`, `width`, `height`). Leptos uses `attr:` directives instead (e.g., `attr:alt="..."`, `attr:width=42`). |

### Implicit behavior

- Only renders the `<img>` element when `imageLoadingStatus` is `'loaded'` / `ImageLoadingStatus::Loaded`. When loading, idle, or errored, nothing is rendered.
- On mount and whenever `src` changes, creates a hidden `Image` element to detect loading status. Listens for `load` and `error` events to track status.
- Notifies the parent `Avatar` context of status changes so that `AvatarFallback` can react accordingly.
- The `referrerPolicy` and `crossOrigin` values are applied to the hidden image loader, not just the rendered `<img>`. This ensures that loading status detection uses the same fetch configuration as the final image.
