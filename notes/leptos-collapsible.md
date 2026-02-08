---
react_location: "[[reference/react-radix-primitives/packages/react/collapsible/src/collapsible.tsx|collapsible]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/collapsible.stories.tsx|collapsible]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-presence]]"
  - "[[leptos-id]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A controlled, composable component for managing expandable/collapsible content sections. Supports toggle states with optional animation integration and smart dimension tracking for height/width animations.

## React API

```ts
const Collapsible: React.ForwardRefExoticComponent<CollapsibleProps>
const CollapsibleTrigger: React.ForwardRefExoticComponent<CollapsibleTriggerProps>
const CollapsibleContent: React.ForwardRefExoticComponent<CollapsibleContentProps>
```

Props: `open`, `defaultOpen`, `disabled`, `onOpenChange`. Content supports `forceMount` for animation libraries.

## React Implementation Notes

- ~247 lines.
- Uses `Presence` component for conditional rendering (animation-friendly).
- Tracks content dimensions via `getBoundingClientRect()` and exposes as CSS variables: `--radix-collapsible-content-height`, `--radix-collapsible-content-width`.
- Blocks animations temporarily while measuring to get full dimensions.
- Manages animation state with `isMountAnimationPreventedRef` via `requestAnimationFrame`.
- `data-state`: `open` | `closed`. `data-disabled` when disabled.
- Accessibility: Sets proper `aria-controls` and `aria-expanded` on trigger.
