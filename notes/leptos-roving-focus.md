---
react_location: "[[reference/react-radix-primitives/packages/react/roving-focus/src/roving-focus-group.tsx|roving-focus-group]]"
rust_location: "[[packages/primitives/leptos/roving-focus/src/roving_focus_group.rs|roving_focus_group]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/roving-focus-group.stories.tsx|roving-focus-group]]"
rust_story: "[[stories/leptos/src/primitives/roving_focus.rs|roving_focus]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-collection]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-id]]"
  - "[[leptos-primitive]]"
  - "[[leptos-use-callback-ref]]"
  - "[[leptos-use-controllable-state]]"
ported: true
tested: false
tested_story: true
---
## Intent

Implements roving tabindex for composite widgets (toolbars, menus, radio groups). Only one item in the group is tabbable at a time; arrow keys move focus between items. Supports horizontal, vertical, and both orientations, with optional wrapping.

## React API

```ts
const RovingFocusGroup: React.ForwardRefExoticComponent<RovingFocusGroupProps>
const RovingFocusGroupItem: React.ForwardRefExoticComponent<RovingFocusItemProps>
```

Props: `orientation`, `dir`, `loop`, `currentTabStopId`, `defaultCurrentTabStopId`, `onCurrentTabStopIdChange`, `preventScrollOnEntryFocus`, `onEntryFocus`.

## Leptos API

```rust
#[component] fn RovingFocusGroup(...) -> impl IntoView
#[component] fn RovingFocusGroupItem(focusable: MaybeProp<bool>, active: MaybeProp<bool>, ...) -> impl IntoView
```

## React Implementation Notes

- Uses `Collection` to track items in DOM order.
- `currentTabStopId` tracks which item is tabbable (via `useControllableState`).
- Arrow key navigation uses `Collection.getItems()` to find the next focusable item.
- Dispatches custom `rovingFocusGroup.onEntryFocus` event on group focus.
- Respects `dir` (LTR/RTL) for horizontal arrow key mapping.

## Leptos Implementation Notes

- Uses `CollectionProvider`/`CollectionSlot`/`CollectionItemSlot` for item tracking.
- Arrow key handling mirrors React: filters focusable items, wraps if `loop` is set.
- Custom events via `web_sys::CustomEvent` for entry focus.
- Dependencies: `leptos`, `leptos-node-ref`, `radix-leptos-collection`, `radix-leptos-compose-refs`, `radix-leptos-direction`, `radix-leptos-id`, `radix-leptos-primitive`, `radix-leptos-use-controllable-state`, `send_wrapper`, `web-sys`.

### Migration decisions (Leptos 0.8)

- **Removed `IntoAttribute` impl for `Orientation`**: No longer needed; `attr:data-orientation` uses `.to_string()` via closure.
- **Replaced `#[prop(attrs)]` with `AttributeInterceptor`**: Both `RovingFocusGroup` and `RovingFocusGroupItem` now use `<AttributeInterceptor let:attrs>` + `{..attrs}` spread, matching the pattern used by all migrated Leptos 0.8 components.
- **`NodeRef<AnyElement>` → `AnyNodeRef`**: All node refs use `leptos_node_ref::AnyNodeRef`.
- **`ChildrenFn` → `TypedChildrenFn`**: With `.into_inner()` stored in `StoredValue`.
- **`create_signal` → `signal`**: Leptos 0.8 API.
- **`on_cleanup` → `Owner::on_cleanup`**: Leptos 0.8 API.
- **`.call()` → `.run()`**: On `Callback` instances.
- **`Rc<Closure<dyn Fn(Event)>>` → `SendWrapper<Closure<dyn Fn(Event)>>`**: Stored in `StoredValue` for the entry focus event listener. `SendWrapper` is required because `Closure` is `!Send` but Leptos 0.8 requires `Send` for stored values.
- **`"tab-index"` → `"tabindex"`**: Corrected to use the proper HTML attribute name.
- **Bug fix: `get_focus_intent` orientation filtering was swapped**: The old code had horizontal and vertical swapped relative to the React source. Fixed to match React: vertical filters left/right, horizontal filters up/down.
- **Bug fix: RTL `ArrowRight` mapped to `"Arrowleft"` (lowercase L)**: Fixed to `"ArrowLeft"`.
- **`on:mouseup` added to reset `is_click_focus`**: React's `onFocus` maps to `focusin` (bubbles), so clicking a child resets `isClickFocusRef` when the child's `focusin` bubbles to the group. In Leptos, using `on:focusin` causes re-entrant closure invocation (tachys wraps handlers in `Rc<RefCell<dyn FnMut>>` which panics on re-entry when `focus_first` synchronously focuses a child whose `focusin` bubbles back). Instead, we use `on:focus` (doesn't bubble, no re-entrancy) and add `on:mouseup` (bubbles) to reset `is_click_focus` after a child click. This prevents `is_click_focus` from getting stuck at `true`.
- **`on:blur` → `on:focusout` for `is_tabbing_back_out` reset**: React's `onBlur` maps to `focusout` (bubbles). Using native `blur` (doesn't bubble), `is_tabbing_back_out` was never reset when a child button lost focus after Shift+Tab, permanently locking the group at `tabindex="-1"`. `on:focusout` bubbles from children and has no re-entrancy risk (the handler doesn't call `.focus()`).
- **Effect cleanup for focusable item tracking**: React's `useEffect` runs its cleanup before re-running. Leptos `Effect::new` doesn't, so the focusable item add/remove Effect uses the `was_focusable` return-value pattern (`Effect::new(move |was_focusable: Option<bool>| { ... })`) to detect transitions and call `on_focusable_item_remove` when focusable changes from `true` to `false`.
- **Omitted `style` prop spreading from React**: React spreads `{outline: 'none', ...props.style}` on the group div. In Leptos, the `outline: none` style is set directly via `attr:style`. User-provided styles can be passed through `AttributeInterceptor` attrs.
