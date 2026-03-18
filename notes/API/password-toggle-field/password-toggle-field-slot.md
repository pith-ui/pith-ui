# PasswordToggleFieldSlot

## React Signature

```typescript
const PasswordToggleFieldSlot: React.FC<PasswordToggleFieldSlotProps>

interface PasswordToggleFieldSlotDeclarativeProps {
  visible: React.ReactNode;
  hidden: React.ReactNode;
}

interface PasswordToggleFieldSlotRenderProps {
  render: (args: { visible: boolean }) => React.ReactElement;
}

type PasswordToggleFieldSlotProps =
  | PasswordToggleFieldSlotDeclarativeProps
  | PasswordToggleFieldSlotRenderProps;
```

`PasswordToggleFieldSlot` is a plain `React.FC` (no `forwardRef`) since it renders no wrapping DOM element -- it returns one of its children directly.

## Leptos Signature

```rust
pub fn PasswordToggleFieldSlot(
    #[prop(into, optional)] render: Option<Callback<bool, AnyView>>,
    #[prop(into, optional)] visible_content: Option<ChildrenFn>,
    #[prop(into, optional)] hidden_content: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `visible` | `visible_content` | `React.ReactNode` | `Option<ChildrenFn>` | Content to render when the password is visible (shown as plain text). Note: Leptos renames from `visible` to `visible_content` to avoid collision with the context's `visible` signal. |
| `hidden` | `hidden_content` | `React.ReactNode` | `Option<ChildrenFn>` | Content to render when the password is hidden (masked). Note: Leptos renames from `hidden` to `hidden_content`. |
| `render` | `render` | `(args: { visible: boolean }) => React.ReactElement` | `Option<Callback<bool, AnyView>>` | Render callback for full control. Receives the visibility state and returns the content to render. When provided, the `visible`/`hidden` (or `visible_content`/`hidden_content`) props are ignored. In Leptos, the callback receives a plain `bool` rather than a struct with a `visible` field. |

### Implicit behavior

- Renders no DOM element of its own. It acts as a conditional switch, rendering `visible_content` or `hidden_content` based on the current visibility state from context.
- When `render` is provided, it takes precedence over the declarative `visible_content`/`hidden_content` props.
- React accepts a discriminated union type (either `{visible, hidden}` or `{render}`). Leptos accepts all three as optional props; the component checks `render` first, then falls back to the declarative pair.
