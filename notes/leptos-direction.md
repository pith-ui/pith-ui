---
react_location: "[[reference/react-radix-primitives/packages/react/direction/src/direction.tsx|direction]]"
rust_location: "[[packages/primitives/leptos/direction/src/direction.rs|direction]]"
dependencies: []
ported: true
tested: false
---
## Intent

Provides a global reading direction (`ltr`/`rtl`) via context. Components consume it with `use_direction`, which falls back through local prop > global context > `ltr` default.

## React API

```ts
type Direction = 'ltr' | 'rtl';

const DirectionProvider: React.FC<{ dir: Direction; children?: ReactNode }>
function useDirection(localDir?: Direction): Direction
```

## Leptos API

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction { Ltr, Rtl }

#[component]
fn DirectionProvider(
    #[prop(into)] direction: Signal<Direction>,
    children: Children,
) -> impl IntoView

fn use_direction(local_dir: MaybeProp<Direction>) -> Signal<Direction>
```

## React Implementation Notes

- Simple React context with a string union type.
- `useDirection` priority: `localDir` > context value > `'ltr'`.

## Leptos Implementation Notes

- `Direction` enum implements `Display`, `AttributeValue` (for use as an HTML attribute directly in Leptos templates).
- The `AttributeValue` impl is verbose — handles hydration, build, and rebuild with diffing against previous value.
- `DirectionProvider` uses Leptos `Provider` (context). `use_direction` reads context with `use_context`.
- `direction` prop is reactive (`Signal<Direction>`), unlike React's static string.
- No external dependencies beyond `leptos`.
