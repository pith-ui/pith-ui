---
react_location: "[[reference/react-radix-primitives/packages/react/announce/src/announce.tsx|announce]]"
rust_location:
dependencies:
  - "[[leptos-compose-refs]]"
  - "[[leptos-primitive]]"
ported: false
tested: false
---
## Intent

Accessible ARIA live region announcements. Renders visible content while also portaling it to hidden live regions for screen reader announcement with configurable politeness levels.

## React API

```ts
const Announce: React.ForwardRefExoticComponent<AnnounceProps>
```

Props: `type` (`'polite'` | `'assertive'` | `'off'`), `role`, `aria-atomic`, `aria-relevant`, `regionIdentifier`, `children`.

## React Implementation Notes

- ~236 lines.
- Manages live region elements at document root.
- Deduplicates live regions by configuration (type + role + atomic + relevant).
- Uses `visibilitychange` listener to suppress announcements in background tabs.
- Caches live regions per configuration to reuse across multiple `Announce` instances.
- Handles owner document switching for dynamic contexts.
- Portals content into invisible live regions via `ReactDOM.createPortal`.
