---
react_location: "[[reference/react-radix-primitives/packages/react/navigation-menu/src/navigation-menu.tsx|navigation-menu]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/navigation-menu.stories.tsx|navigation-menu]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-dismissable-layer]]"
  - "[[leptos-id]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-visually-hidden]]"
ported: false
tested: false
---
## Intent

A hierarchical navigation menu with nested submenus, keyboard navigation, optional viewport for centered content display, and animated indicator showing the active menu position.

## React API

```ts
// 9 sub-components:
NavigationMenu, NavigationMenuSub, NavigationMenuList, NavigationMenuItem,
NavigationMenuTrigger, NavigationMenuLink, NavigationMenuIndicator,
NavigationMenuContent, NavigationMenuViewport
```

Props: `value`, `defaultValue`, `onValueChange`, `orientation` (default `'horizontal'`), `dir`, `delayDuration` (default 200ms), `skipDelayDuration` (default 300ms).

## React Implementation Notes

- ~1287 lines.
- Complex timer management for delayed menu opening (similar to tooltip skip-delay pattern).
- Nested menu support with parent-child context chains.
- Focus proxy elements for keyboard navigation between trigger and content.
- `NavigationMenuIndicator` animates to show active menu position, uses `ResizeObserver`.
- `NavigationMenuViewport` mounts content with size animation (CSS variables for width/height).
- Motion attributes (`data-motion`) for animation direction detection.
- Pointer-based interaction with mouse detection.
- Custom event for link selection (`navigationMenu.linkSelect`).
- `DismissableLayer` for outside interaction handling.
