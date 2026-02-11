---
react_location: "[[reference/react-radix-primitives/packages/react/collection/src/collection-legacy.tsx|collection]]"
rust_location: "[[packages/primitives/leptos/collection/src/collection.rs|collection]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/collection.stories.tsx|collection]]"
rust_story: "[[stories/leptos/src/primitives/collection.rs|collection]]"
dependencies:
  - "[[leptos-compose-refs]]"
ported: true
tested: false
tested_story: true
---
## Intent

Manages an ordered collection of items within a DOM subtree. Tracks item refs and associated data, returning items sorted by DOM order. Used by composite widgets (menus, listboxes, roving focus, accordion) that need to know the order and data of their items.

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
fn use_collection<ItemData>() -> SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>>
```

Generic over `ItemData: Clone + Debug + PartialEq + Send + Sync + 'static`. Uses `PhantomData` props for type inference.

## React Implementation Notes

- Legacy version: items stored in a `Map<RefObject, ItemData>`, sorted by querying `[data-radix-collection-item]` in DOM order.
- New version: uses `OrderedDict` (custom ordered map), `MutationObserver` for re-sorting, `compareDocumentPosition` for ordering.
- `createCollection` is a factory that creates scoped context + components for a specific collection.

## Leptos Implementation Notes

- Based on the legacy React version.
- Items stored in `StoredValue<HashMap<CollectionItemId, CollectionItemValue<ItemData>>>`.
- `CollectionItemId` uses `nanoid` for unique IDs (React uses ref identity).
- `use_collection` sorts by querying `[data-radix-collection-item]` and comparing DOM positions — same approach as React legacy.
- Updated to Leptos 0.8 API: `TypedChildrenFn`, `AnyNodeRef` (from `leptos-node-ref`), `provide_context`/`expect_context`, `StoredValue`, `Owner::on_cleanup`.
- `CollectionSlot` and `CollectionItemSlot` use `children().add_any_attr(any_node_ref(...))` pattern instead of old `Slot` component.
- `CollectionItemSlot` adds `data-radix-collection-item` attribute via `leptos::attr::custom::custom_attribute()`.
- `use_collection` returns `SendWrapper<Box<dyn Fn()>>` instead of `Rc` because Leptos 0.8's `StoredValue` requires `Send + Sync`.
- All `ItemData` generics require `Send + Sync` bounds for Leptos 0.8 compatibility.
- **Omissions:**
  - `createCollectionScope` — React scoped context pattern not applicable in Leptos.
  - `Slot` component removed — Leptos 0.8 uses `add_any_attr` for ref/attribute merging.
- Dependencies: `leptos`, `leptos-node-ref`, `nanoid`, `radix-leptos-compose-refs`, `send_wrapper`, `web-sys`.

### Review Notes

- it would be preferable for `data-radix-collection-item` to be the first attribute, but currently the story lists the styles first when they are present.
- this has to do with the spreading of attributes in the item. could use attribute intercepter to spread them after the data-radix props
