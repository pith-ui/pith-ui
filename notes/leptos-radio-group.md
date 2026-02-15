---
react_location: "[[reference/react-radix-primitives/packages/react/radio-group/src/radio-group.tsx|radio-group]]"
rust_location: "[[packages/primitives/leptos/radio-group/src/radio_group.rs|radio-group]]"
react_story: "[[reference/react-radix-primitives/apps/storybook/stories/radio-group.stories.tsx|radio-group]]"
rust_story: "[[stories/leptos/src/primitives/radio_group.rs|radio-group]]"
dependencies:
  - "[[core-primitive]]"
  - "[[leptos-compose-refs]]"
  - "[[leptos-direction]]"
  - "[[leptos-presence]]"
  - "[[leptos-primitive]]"
  - "[[leptos-roving-focus]]"
  - "[[leptos-use-controllable-state]]"
  - "[[leptos-use-previous]]"
  - "[[leptos-use-size]]"
ported: true
tested: false
tested_story: true
---
## Intent

A group of mutually exclusive radio buttons with roving focus keyboard navigation, form integration, and flexible composition. Enforces single-selection semantics.

## React API

```ts
// From radio-group.tsx + radio.tsx:
const RadioGroup: React.ForwardRefExoticComponent<RadioGroupProps>
const RadioGroupItem: React.ForwardRefExoticComponent<RadioGroupItemProps>
const RadioGroupIndicator: React.ForwardRefExoticComponent<RadioGroupIndicatorProps>
```

Props: `name`, `required`, `disabled`, `value`, `defaultValue`, `onValueChange`, `orientation`, `dir`, `loop`.

## Leptos API

```rust
// From radio_group.rs:
fn RadioGroup(
    name: MaybeProp<String>,
    value: MaybeProp<String>,
    default_value: MaybeProp<String>,
    on_value_change: Option<Callback<String>>,
    required: MaybeProp<bool>,
    disabled: MaybeProp<bool>,
    orientation: MaybeProp<Orientation>,
    dir: MaybeProp<Direction>,
    r#loop: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
)

fn RadioGroupItem(
    value: String,
    disabled: MaybeProp<bool>,
    on_click: Option<Callback<ev::MouseEvent>>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
)

fn RadioGroupIndicator(
    force_mount: MaybeProp<bool>,
    as_child: MaybeProp<bool>,
    node_ref: AnyNodeRef,
    children: Option<ChildrenFn>,
)
```

Internal components (not exported):
```rust
fn RadioButton(...)      // Flat radio button (AttributeInterceptor → Primitive(button) → {..attrs})
fn RadioIndicator(...)   // Indicator with Presence support
fn RadioBubbleInput(...) // Hidden input for native form submission
```

## Keyboard Navigation (per Radix UI docs)

| Key | Behavior |
|-----|----------|
| **Tab** | Moves focus to the checked radio item, or the first item if none is checked. |
| **Space** | Checks the focused radio item (if unchecked). |
| **ArrowDown / ArrowRight** | Moves focus to the next item AND checks it. |
| **ArrowUp / ArrowLeft** | Moves focus to the previous item AND checks it. |
| **Enter** | Prevented (per WAI-ARIA spec, radio groups don't activate on Enter). |

The component uses "roving tabindex" — only one item in the group receives tab focus at a time. Arrow key navigation both moves focus and selects simultaneously.

### How arrow-key auto-check works (React implementation)

1. `RadioGroupItem` tracks arrow key state via document-level `keydown`/`keyup` listeners on `isArrowKeyPressedRef`.
2. `RovingFocusGroup` handles arrow key navigation — it moves DOM focus to the next/previous item.
3. When the `Radio` button receives focus, the `onFocus` handler checks `isArrowKeyPressedRef.current`. If true, it calls `ref.current?.click()`.
4. The click triggers `on_check` → `context.onValueChange` → the radio becomes checked.

This approach fires the radio change event through the click path (rather than setting value directly), ensuring form events and native validation work correctly.

## React Implementation Notes

- ~441 lines across two files (`radio-group.tsx` + `radio.tsx`).
- Two-file structure: `radio-group.tsx` orchestrates, `radio.tsx` provides the Radio button primitive.
- `RovingFocusGroup` for keyboard navigation.
- Form integration via `RadioBubbleInput` — hidden input behind button, uses property descriptor trick to set checked state.
- Arrow key tracking: On focus from arrow key, auto-clicks the radio to check it.
- Enter key explicitly prevented per WAI-ARIA spec.
- Accessibility: `aria-checked`, `data-state`, `role="radio"`.
- `RadioGroupIndicator` supports `forceMount` via `Presence`.

## Leptos Implementation Notes

### Structure

Two-file structure: `radio_group.rs` (RadioGroup, RadioGroupItem, RadioButton, RadioGroupIndicator) and `radio.rs` (RadioContextValue, RadioIndicator, RadioBubbleInput). Internal types are `pub(crate)` only; only group-level components are publicly exported.

Unlike React's two-file split (`radio-group.tsx` + `radio.tsx`), the Leptos port eliminates the standalone `Radio` component. React's `Radio` is a full component with its own context provision, ref forwarding, and event composition. In Leptos, this was replaced with a flat `RadioButton` internal component that uses the `AttributeInterceptor → Primitive(button) → {..attrs}` pattern (matching `ToggleGroupItemImpl`). This was necessary because any wrapper components (Provider, Show, Fragment) between `AttributeInterceptor` and the `Primitive` break attribute forwarding from `RovingFocusGroupItem`, which prevents keyboard navigation from working.

### Omissions

1. **Scoped context** (`__scopeRadioGroup`, `__scopeRadio`): Leptos uses `provide_context`/`expect_context` instead of React's `createContextScope`. No scope parameter is needed.

2. **`composeRefs`/`useComposedRefs`**: Used via `radix-leptos-compose-refs::use_composed_refs` where multiple refs are needed (RadioGroupItem has local ref + forwarded ref, RadioIndicator has forwarded ref + presence ref).

3. **BubbleInput property descriptor trick**: The React impl uses `Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'checked')` to programmatically set `checked` via the native setter. In Leptos, we use `input.set_checked()` directly, which is equivalent (same as checkbox pattern).

4. **`on_focus`/`on_keydown` props on RadioButton**: In React, `RadioGroupItem` passes `onFocus` and `onKeyDown` to `Radio`, which composes them on the button. In Leptos, these handlers are instead passed as props to `RovingFocusGroupItem` (`on_focus`, `on_key_down`), which composes them with its own arrow-key navigation and focus-tracking handlers. This avoids conflicting `on:` event bindings — in Leptos, having both an explicit `on:keydown` and `{..attrs}` containing another `on:keydown` on the same element causes one to override the other instead of composing.

5. **`form` prop on Radio**: React's Radio accepts a `form` attribute for associating with forms outside the DOM tree. Omitted as this is a rarely-used HTML feature and the `closest("form")` detection covers the standard case.

6. **`isPropagationStopped` tracking**: React's synthetic event system has `isPropagationStopped()`. In the Leptos port, `has_consumer_stopped_propagation` uses `event.cancel_bubble()` — the native DOM property that returns `true` after `stopPropagation()` was called. This is used to conditionally suppress the BubbleInput's change event bubbling when the consumer has already stopped propagation.

7. **BubbleInput event type**: React dispatches a `"click"` event because React's synthetic event system maps `onChange` to `click` for radio/checkbox inputs. In native DOM (Leptos), we dispatch `"change"` so that native form change handlers fire correctly.

8. **BubbleInput fires only on check**: The BubbleInput only dispatches a change event when the radio becomes checked (`checked.get() == true`). Radios cannot be unchecked by the user, so only the newly checked radio should fire. Without this guard, both the newly checked and previously checked radios would fire change events, and the form handler would see the wrong (stale) value from the unchecking radio last.

### Key Decisions

- **Flat RadioButton pattern**: `RadioButton` uses `<AttributeInterceptor let:attrs>` → `<Primitive element=html::button {..attrs}>` with NO Provider, Show, or Fragment wrappers between them. This ensures attributes from `RovingFocusGroupItem` (tabindex for roving focus, on:keydown for arrow key navigation, on:focus, on:mousedown) are properly forwarded to the button element. This follows the `ToggleGroupItemImpl` pattern from toggle-group.

- **Provider for RadioContextValue**: Each `RadioGroupItem` wraps its view in `<Provider value=radio_context>` to provide a scoped `RadioContextValue`. Using bare `provide_context` at the component level doesn't scope correctly — all RadioIndicators end up reading from the last item's context because `provide_context` sets it at the component's Owner scope, which is shared across siblings rendered in the same parent view. The `<Provider>` component creates a proper nested Owner, scoping the context to each item's subtree.

- **BubbleInput as sibling, not child**: `RadioBubbleInput` is rendered as a sibling after the `Provider`/`RovingFocusGroupItem` block (inside a `<Show>` guard), NOT inside the `RovingFocusGroupItem` chain. This prevents the hidden input from participating in roving focus and avoids it intercepting tab focus.

- **Capture phase for arrow key listeners**: Document-level `keydown`/`keyup` listeners use capture phase (`add_event_listener_with_callback_and_bool("keydown", callback, true)`). This ensures the `is_arrow_key_pressed` flag is set BEFORE element-level keydown handlers fire. The event flow is: capture phase document listener → sets flag → element keydown handler (RovingFocusGroupItem) → moves focus → on_focus handler → checks flag → calls click(). Without capture phase, the flag would be set too late.

- **Arrow key event listeners**: Use `Arc<SendWrapper<Closure<...>>>` pattern (same as `use_escape_keydown`) for document-level event listeners that need to be both registered in `Effect` and cleaned up in `on_cleanup`. This satisfies Leptos 0.8's `Send + Sync` requirement for `on_cleanup`.

- **`RadioGroupIndicator` children**: Made children `Option<ChildrenFn>` instead of required `TypedChildrenFn` because the indicator is typically used as a self-closing tag (`<RadioGroupIndicator />`). Using `TypedChildrenFn` with `Option` causes type inference issues when no children are provided.

- **Value type**: `use_controllable_state` is parameterized with `T = String`, yielding `Signal<Option<String>>` for the value. This maps cleanly to the React `string | null` type.

- **Controlled null value**: React's `value={null}` creates a controlled component with no selection. In Leptos, `MaybeProp<String>` maps `None` to uncontrolled (since `use_controllable_state` uses `prop.get().is_some()` to determine controlledness). The equivalent is `value=""` — controlled with an empty string that matches no item. Without `on_value_change`, clicking does nothing.

- **Arrow-key auto-check on focus**: Follows the React pattern: `RadioGroupItem` tracks arrow key state via document-level listeners (`Arc<SendWrapper<Closure<...>>>`), passes an `on_focus` callback to `RovingFocusGroupItem` that calls `element.click()` when `is_arrow_key_pressed` is true. `RovingFocusGroupItem` composes this with its own focus handler via `compose_callbacks`. This ensures arrow key navigation both moves focus AND checks the item.

- **Presence ref for RadioIndicator**: `RadioIndicator` creates a separate `presence_ref` and composes it with the forwarded `node_ref` via `use_composed_refs`. The `presence_ref` is passed to `Presence` (for animation event observation) while the composed ref is passed to the `Primitive`. This ensures Presence can detect CSS animations for exit transitions.

- **Event handler composition via props, not duplicate `on:` bindings**: In Leptos, having both an explicit `on:keydown=handler1` and `{..attrs}` (containing `on:keydown=handler2`) on the same element causes one to override the other — they do not compose. Therefore, `RadioGroupItem` passes its `on_keydown` and `on_focus` handlers as props to `RovingFocusGroupItem` (which composes them internally via `compose_callbacks`), rather than passing them to `RadioButton` where they would conflict with the same handlers arriving through `{..attrs}`. `RadioButton`'s Primitive only has `on:click` (which doesn't conflict since `RovingFocusGroupItem` doesn't set `on:click`).
