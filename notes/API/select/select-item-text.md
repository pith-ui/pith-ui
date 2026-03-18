# SelectItemText

## React Signature

```typescript
const SelectItemText = React.forwardRef<SelectItemTextElement, SelectItemTextProps>(...)

type SelectItemTextElement = React.ComponentRef<typeof Primitive.span>;

interface SelectItemTextProps extends PrimitiveSpanProps {}
```

React ignores `className` and `style` on this component since the text should not be styled independently from the item.

## Leptos Signature

```rust
pub fn SelectItemText(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the rendered DOM element (`<span>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<span>`, merging props and refs. |
| `children` | `children` | `React.ReactNode` | `Option<ChildrenFn>` | The text content of the item. This text is used for typeahead matching and is displayed in `SelectValue` when the item is selected. |
| *(spread)* | -- | `...PrimitiveSpanProps` | -- | React allows spreading any `<span>` HTML attribute. Leptos uses `attr:` directives instead. |

### Implicit behavior

- Sets its own `id` (auto-generated from the parent `SelectItem`'s `text_id`), which `SelectItem` uses for `aria-labelledby`.
- Notifies the parent `SelectItem` of its text content for typeahead search via `on_item_text_change`.
- When the item is selected and `SelectValue` has no static children, the text content is automatically displayed in the `SelectValue` span.
  - In React, this is done via `ReactDOM.createPortal` -- the actual children nodes are portalled into the `SelectValue` span.
  - In Leptos, this is done via an `Effect` that copies the text content string into the `SelectValue` span. This means only plain text is transferred; if `SelectItemText` contains complex markup, only the `textContent` will appear in the trigger.
- In React, the component also creates a native `<option>` element registered with the hidden `<select>` for form autofill. The Leptos implementation handles native option management differently (via the `SelectBubbleInput` internal component).
