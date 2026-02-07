---
react_location: "[[reference/react-radix-primitives/packages/react/collection/src/collection-legacy.tsx|collection]]"
rust_location: "[[packages/primitives/leptos/collection/src/collection.rs|collection]]"
dependencies:
  - "[[leptos-compose-refs]]"
  - "[[leptos-slot]]"
ported: true
tested: false
---
## Intent

Manages an ordered collection of items within a DOM subtree. Tracks item refs and associated data, returning items sorted by DOM order. Used by composite widgets (menus, listboxes, roving focus) that need to know the order and data of their items.

## React API

```ts
function createCollection<ItemElement, ItemData>(name: string):
  [{ Provider, Slot, ItemSlot }, useCollection, createCollectionScope]
```

Factory function. `Provider` wraps the collection, `Slot` marks the collection root, `ItemSlot` marks each item. `useCollection` returns a `getItems()` function that returns items sorted by DOM position. Items are identified by `data-radix-collection-item` attribute.

React has both a legacy and new version — the new version uses `OrderedDict` and `MutationObserver`.

## Leptos API

```rust
#[component] fn CollectionProvider<ItemData>(...) -> impl IntoView
#[component] fn CollectionSlot<ItemData>(...) -> impl IntoView
#[component] fn CollectionItemSlot<ItemData>(...) -> impl IntoView
fn use_collection<ItemData>() -> Rc<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>
```

Generic over `ItemData`. Uses `PhantomData` props for type inference.

**Note:** Uses old Leptos API. Needs migration.

## React Implementation Notes

- Legacy version: items stored in a `Map<RefObject, ItemData>`, sorted by querying `[data-radix-collection-item]` in DOM order.
- New version: uses `OrderedDict` (custom ordered map), `MutationObserver` for re-sorting, `compareDocumentPosition` for ordering.
- `createCollection` is a factory that creates scoped context + components for a specific collection.

## Leptos Implementation Notes

- Based on the legacy React version.
- Items stored in `RwSignal<HashMap<CollectionItemId, CollectionItemValue<ItemData>>>`.
- `CollectionItemId` uses `nanoid` for unique IDs (React uses ref identity).
- `use_collection` sorts by querying `[data-radix-collection-item]` and comparing DOM positions — same approach as React legacy.
- Uses `Slot` component (old Leptos pattern) for collection root and items.
- Uses old Leptos API — needs migration.
- Dependencies: `leptos`, `nanoid`, `radix-leptos-compose-refs`, `web-sys`.
