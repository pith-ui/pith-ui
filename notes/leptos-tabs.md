---
react_location: "[[reference/react-radix-primitives/packages/react/tabs/src/tabs.tsx|tabs]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/tabs.stories.tsx|tabs]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
tested_story: false
---
## Intent

A tabbed interface for organizing content into panels. Tabs navigate via roving focus; content panels show/hide with optional animation support.

## React API

```ts
// 4 sub-components:
Tabs, TabsList, TabsTrigger, TabsContent
```

Props: `value`, `defaultValue`, `onValueChange`, `orientation` (default `'horizontal'`), `activationMode` (`'automatic'` | `'manual'`), `dir`.

## React Implementation Notes

- ~299 lines.
- `RovingFocusGroup` for keyboard navigation between tab triggers.
- `Presence` for content mounting/unmounting with animation support.
- `activationMode`: `'automatic'` selects tab on focus, `'manual'` requires click/Enter.
- Generated IDs link triggers to content panels via `aria-controls`/`aria-labelledby`.
- RTL text direction support via `useDirection`.
- `data-state`: `active` | `inactive` on both trigger and content.
