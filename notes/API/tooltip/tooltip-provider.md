# TooltipProvider

## React Signature

```typescript
const TooltipProvider: React.FC<TooltipProviderProps> = (props) => { ... }

interface TooltipProviderProps {
  children: React.ReactNode;
  /**
   * The duration from when the pointer enters the trigger until the tooltip gets opened.
   * @defaultValue 700
   */
  delayDuration?: number;
  /**
   * How much time a user has to enter another trigger without incurring a delay again.
   * @defaultValue 300
   */
  skipDelayDuration?: number;
  /**
   * When `true`, trying to hover the content will result in the tooltip closing as the pointer leaves the trigger.
   * @defaultValue false
   */
  disableHoverableContent?: boolean;
}
```

Note: `TooltipProvider` is an `FC` (not `forwardRef`) because it does not render a DOM element — it is a pure context provider.

## Leptos Signature

```rust
pub fn TooltipProvider(
    #[prop(into, optional, default = MaybeProp::from(DEFAULT_DELAY_DURATION))]
    delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))]
    skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] disable_hoverable_content: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView
```

Where `DEFAULT_DELAY_DURATION` is `700.0`.

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `delayDuration` | `delay_duration` | `number` (default `700`) | `MaybeProp<f64>` (default `700.0`) | The duration in milliseconds from when the pointer enters a trigger until the tooltip opens. Individual `Tooltip` instances can override this value. |
| `skipDelayDuration` | `skip_delay_duration` | `number` (default `300`) | `MaybeProp<f64>` (default `300.0`) | The duration in milliseconds after a tooltip closes during which moving to another trigger opens it instantly (without the delay). Set to `0` to always require the full delay. |
| `disableHoverableContent` | `disable_hoverable_content` | `boolean` (default `false`) | `MaybeProp<bool>` (default `false`) | When `true`, the tooltip closes immediately when the pointer leaves the trigger, rather than allowing the user to hover the tooltip content. Individual `Tooltip` instances can override this value. |
| `children` | `children` | `React.ReactNode` | `ChildrenFn` | One or more `Tooltip` instances (and any other content). |

### Implicit behavior

- Provides a `TooltipProviderContextValue` to all descendant `Tooltip` components, containing:
  - `isOpenDelayed` / `is_open_delayed` — a flag indicating whether the next tooltip open should be delayed. Initially `true`. Set to `false` when a tooltip opens (via `onOpen`). After a tooltip closes, a timer of `skipDelayDuration` runs before resetting it to `true`.
  - `isPointerInTransit` / `is_pointer_in_transit` — whether the pointer is currently moving through the grace area between trigger and content.
  - Default `delayDuration` and `disableHoverableContent` values that individual `Tooltip` instances inherit.
- The skip-delay timer is cleaned up on unmount.
