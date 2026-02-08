---
react_location: "[[reference/react-radix-primitives/packages/react/accordion/src/accordion.tsx|accordion]]"
rust_location:
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/accordion.stories.tsx|accordion]]"
rust_story: ""
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collapsible]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-controllable-state]]"
ported: false
tested: false
---
## Intent

A composable accordion managing multiple expandable sections. Supports single and multiple open items, keyboard navigation (Home/End/ArrowKeys), optional collapsibility, and RTL support.

## React API

```ts
// 5 sub-components:
Accordion, AccordionItem, AccordionHeader, AccordionTrigger, AccordionContent
```

`type: 'single' | 'multiple'` (required). Single mode: `value?: string`, `collapsible?: boolean`. Multiple mode: `value?: string[]` (always collapsible).

## React Implementation Notes

- ~538 lines.
- Built on Collapsible primitives — each item is a `Collapsible`.
- Uses `Collection` pattern to track trigger elements for keyboard navigation.
- Comprehensive keyboard navigation: Home/End jump to first/last, ArrowDown/Up for vertical, ArrowLeft/Right for horizontal (respects RTL).
- Context tree: `AccordionValueProvider` (state) + `AccordionCollapsibleProvider` (collapsibility config) + `AccordionImplProvider` (shared config).
- Exposes CSS variables through Collapsible: `--radix-accordion-content-height`, `--radix-accordion-content-width`.
- Direction-aware via `useDirection(dir)`.
