# Leptos vs React Radix Primitives — API Comparison

This document compares the public API of each Leptos Radix primitive component against its React counterpart in `reference/react-radix-primitives/`.

**Legend:**
- **Match** — The Leptos API matches the React API (accounting for idiomatic Rust/Leptos adaptations)
- **Missing** — A React prop/component that is not present in the Leptos implementation
- **Extra** — A Leptos prop/component not present in the React version
- **Different** — The prop exists in both but has a different name, type, or behavior

---

## Executive Summary

### Component Coverage

All **35 user-facing components** and **12 internal utility modules** from React Radix Primitives have Leptos equivalents. Every React-exported component has a corresponding Leptos export with matching names (snake_case adaptation of camelCase).

| Category               | Components                                                                                                                            | Component-level Match     | Assessment     |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- | -------------- |
| **Simple**             | AccessibleIcon, AspectRatio, Avatar, Label, Separator, VisuallyHidden, Progress, Toggle                                               | 8/8                       | Excellent      |
| **Form Controls**      | Checkbox, Switch, RadioGroup, Collapsible, Accordion                                                                                  | 5/5                       | Excellent      |
| **Overlays**           | Dialog, AlertDialog, Popover, HoverCard, Tooltip                                                                                      | 5/5                       | Excellent      |
| **Menus**              | Menu, DropdownMenu, ContextMenu, Menubar                                                                                              | 4/4 (65 components total) | Good           |
| **Navigation/Complex** | Tabs, Slider, ScrollArea, Select, Toolbar, ToggleGroup                                                                                | 6/6                       | Good–Excellent |
| **Other**              | Form, Toast, NavigationMenu, OTPField, PasswordToggleField                                                                            | 5/5                       | Excellent      |
| **Internal**           | Portal, Presence, Primitive, Direction, DismissableLayer, FocusScope, FocusGuards, RovingFocus, Popper, Arrow, Collection, ScrollLock | 12/12                     | Good           |

### Cross-Cutting Patterns (Leptos vs React)

These patterns recur across nearly every component. They are idiomatic Leptos adaptations, not bugs:

1. **`as_child: MaybeProp<bool>` + `node_ref: AnyNodeRef`** replace React's `asChild` and `ref` forwarding. Present on nearly all components.

2. **No arbitrary HTML attribute forwarding.** React spreads all element attributes via `PrimitiveDivProps`, `PrimitiveButtonProps`, etc. Leptos components only accept declared props; additional attributes use `attr:*` syntax.

3. **Explicit event handler props.** Where React composes `onClick`, `onKeyDown`, etc. from inherited HTML props, Leptos declares these as explicit component props (e.g., `on_click: Option<Callback<ev::MouseEvent>>`). This is necessary for event composition in Leptos.

4. **`forceMount: MaybeProp<bool>`** replaces React's `forceMount?: true` (literal type). Functionally equivalent.

5. **No `Root`/shorthand aliases.** React exports `Root`, `Trigger`, `Content`, etc. as aliases. Leptos uses only the full component names.

6. **No `createXScope` exports.** React uses `createContextScope` for context composition. Leptos uses built-in `Provider`/`expect_context`.

7. **`Signal<T>` / `MaybeProp<T>` wrappers.** Leptos wraps many props in reactive types. String unions become Rust enums (e.g., `Orientation`, `Direction`, `CheckedState`, `ActivationMode`).

8. **`container_ref: AnyNodeRef`** on all Portal components — a Leptos-specific addition since portals in Leptos break context chains (unlike React portals).

9. **Extra `class: MaybeProp<String>`** on components that render through `Presence`/`Show` boundaries (ScrollArea, Menu, Toast) — needed because Leptos conditional rendering boundaries lose attribute passthrough.

### Notable Gaps & Divergences

#### Missing Props (across multiple components)

| Missing Prop                                                                          | Affected Components                                                                                     | Impact                                                                        |
| ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `form`                                                                                | Checkbox, Switch, RadioGroup                                                                            | Cannot associate with a form by ID; only `closest("form")` detection works    |
| `loop`                                                                                | MenuContent, DropdownMenuContent, ContextMenuContent, MenubarContent, all SubContent                    | Keyboard navigation wrapping cannot be configured on these content components |
| `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached` | MenuContent, DropdownMenuContent, ContextMenuContent (NOT Menubar, Popover, HoverCard, Tooltip, Select) | Advanced Popper positioning props not forwarded through menu Content wrappers |
| `referrerPolicy`, `crossOrigin`                                                       | AvatarImage                                                                                             | CORS-restricted images may fail loading detection                             |

#### Behavioral Divergences

| Divergence                 | Component           | Detail                                                                                 |
| -------------------------- | ------------------- | -------------------------------------------------------------------------------------- |
| `position` default         | Select              | React defaults to `'item-aligned'`; Leptos defaults to `'popper'`                      |
| `align` default            | SelectContent       | React defaults to `'center'`; Leptos defaults to `Start`                               |
| `collisionPadding` default | SelectContent       | React defaults to `0`; Leptos defaults to `10.0`                                       |
| Value validation           | Progress            | React rejects invalid values (falls back to `null`); Leptos clamps to `[0, max]`       |
| Scroll locking             | Dialog, AlertDialog | React uses `react-remove-scroll` (full-featured); Leptos uses basic `overflow: hidden` |
| `placeholder` type         | SelectValue         | React accepts `ReactNode`; Leptos accepts `String` only                                |
| `Toast.type`               | Toast               | React uses typed `'foreground' \| 'background'`; Leptos uses raw `String`              |

#### Justified Leptos-Specific Additions

| Addition                                                    | Components          | Reason                                                                       |
| ----------------------------------------------------------- | ------------------- | ---------------------------------------------------------------------------- |
| `ScopedPortal`, `PortalContextValue`, `resolve_force_mount` | Portal              | Leptos portals break context chains; need manual re-provision                |
| `higher_layers_contain`                                     | DismissableLayer    | Leptos DOM events don't bubble through portals like React synthetic events   |
| Two-escape UX pattern                                       | DismissableLayer    | First Escape blurs text input, second dismisses (Rule 8 enhancement)         |
| `role` prop on DialogContent                                | Dialog, AlertDialog | AlertDialog needs to override role; can't use attribute spread in Leptos     |
| `content_style`, `id`, `aria_labelledby`                    | MenuContent         | Wrapper components need to set attributes that React handles via prop spread |
| `anchor_ref` on Popper                                      | Popper              | Parent components (Menu) can set anchor ref directly                         |
| `set_popper_virtual_ref()`                                  | Popper              | Replaces React's `virtualRef` prop with a standalone function                |

#### Consistent Naming Adaptations

| React                      | Leptos             | Notes                                            |
| -------------------------- | ------------------ | ------------------------------------------------ |
| `camelCase` props          | `snake_case` props | Standard Rust convention                         |
| `onXChange`                | `on_x_change`      | Callback naming                                  |
| `defaultX`                 | `default_x`        | Uncontrolled defaults                            |
| `loop`                     | `r#loop`           | Rust reserved keyword                            |
| `type`                     | `r#type`           | Rust reserved keyword                            |
| `forceMount`               | `force_mount`      | —                                                |
| `asChild`                  | `as_child`         | —                                                |
| `ref`                      | `node_ref`         | —                                                |
| `string \| 'value'` unions | Rust enums         | `Orientation`, `Direction`, `CheckedState`, etc. |

### Recommendations

1. **Forward missing `loop` prop** through menu Content wrappers — this is a prop that exists on the internal `MenuContentImpl` but is not exposed at the public API level for any of the four menu variants.

2. **Forward missing Popper props** (`collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached`) through DropdownMenuContent and ContextMenuContent — MenubarContent already forwards `align`, `align_offset`, `avoid_collisions` and should be used as the template.

3. **Fix Select defaults** — `position` should default to `'item-aligned'`, `align` to `'center'`, and `collisionPadding` to `0` to match React.

4. **Add `form` prop** to Checkbox, Switch, and RadioGroup for explicit form association by ID.

5. **Type `Toast.type`** as an enum (`ToastType::Foreground | ToastType::Background`) instead of raw `String`.

6. **Add `referrerPolicy` and `crossOrigin`** to AvatarImage's internal loading hook for CORS-restricted images.

7. **Consider upgrading scroll locking** from basic `overflow: hidden` to a more comprehensive solution (scroll bar compensation, touch event handling).

---

## AccessibleIcon

### React API

**Exported components:** `AccessibleIcon` (aliased as `Root`)

**AccessibleIcon** (renders a fragment: cloned child + VisuallyHidden)
- `children?: React.ReactNode` — expects a single child (icon element); clones it with `aria-hidden="true"` and `focusable="false"`
- `label: string` (required) — accessible label rendered inside a `VisuallyHidden`

### Leptos API

**Exported components:** `AccessibleIcon`

**AccessibleIcon**
- `label: Signal<String>` (required, `#[prop(into)]`) — accessible label rendered inside a `VisuallyHidden`
- `children: TypedChildren<impl IntoView + 'static>` (required) — the icon element; receives `aria-hidden="true"` and `focusable="false"` via `add_any_attr`

### Differences

1. **`label` type:** React uses `string`; Leptos uses `Signal<String>`, making it reactive. This means the label can be dynamically updated in Leptos, which React would require a re-render for anyway.
2. **No `Root` alias:** React exports both `AccessibleIcon` and `Root`. Leptos only exports `AccessibleIcon`. This is a convention difference -- Leptos modules do not use the `Root` aliasing pattern.
3. **No `as_child` or `node_ref` props:** Unlike most other Leptos primitives, `AccessibleIcon` does not expose `as_child` or `node_ref`. This matches React, which also does not use `Primitive` or `forwardRef` here.
4. **Child handling:** React uses `React.Children.only` + `React.cloneElement` to inject attributes into the single child. Leptos uses `TypedChildren` with `.add_any_attr()` to achieve the same effect idiomatically.

### Assessment

Excellent alignment. The component serves the same purpose and produces equivalent DOM output. The `Signal<String>` type for `label` is a minor Leptos-idiomatic difference that does not affect the public API contract. The lack of a `Root` alias is consistent across all Leptos primitives.

---

## AspectRatio

### React API

**Exported components:** `AspectRatio` (aliased as `Root`)

**AspectRatio** (renders a wrapper `div` + inner `Primitive.div`)
- `ratio?: number` (default `1`) — the desired width-to-height ratio
- Inherits all `div` HTML attributes via `PrimitiveDivProps` (applied to the inner div)
- `ref` (forwarded to the inner div)
- `style` — merged with absolute-positioning styles on the inner div

### Leptos API

**Exported components:** `AspectRatio`

**AspectRatio**
- `ratio: Signal<f64>` (optional, default `1.0`, `#[prop(into)]`) — the desired width-to-height ratio
- `as_child: MaybeProp<bool>` (optional) — render as child pattern
- `node_ref: AnyNodeRef` (optional) — forwarded ref
- `children: ChildrenFn` (required)

### Differences

1. **`ratio` type:** React uses `number` (default `1/1`); Leptos uses `Signal<f64>` (default `1.0`), making it reactive.
2. **No `Root` alias:** React exports both `AspectRatio` and `Root`. Leptos only exports `AspectRatio`.
3. **`as_child` is extra:** Leptos exposes `as_child` (standard Leptos Primitive pattern). React does not expose `asChild` on this component directly, but the inner element uses `Primitive.div` which implicitly supports it.
4. **No arbitrary HTML attribute forwarding:** React inherits all `div` attributes via `PrimitiveDivProps`. Leptos requires `attr:*` syntax.

### Assessment

Good alignment. The core `ratio` prop and wrapper/inner div structure match. The `as_child` prop in Leptos is a standard pattern across all Leptos primitives rather than a divergence. The reactive `Signal<f64>` for `ratio` is a Leptos idiom.

---

## Avatar

### React API

**Exported components:** `Avatar` (aliased as `Root`), `AvatarImage` (aliased as `Image`), `AvatarFallback` (aliased as `Fallback`), `createAvatarScope`

**Avatar** (renders `Primitive.span`)
- No component-specific props
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**AvatarImage** (renders `Primitive.img` when loaded, `null` otherwise)
- `src?: string` — image source (inherited from `PrimitiveImageProps`)
- `onLoadingStatusChange?: (status: ImageLoadingStatus) => void` — callback when loading status changes
- `referrerPolicy?: string` — passed to the internal image element for loading detection (inherited from `PrimitiveImageProps`)
- `crossOrigin?: string` — passed to the internal image element for loading detection (inherited from `PrimitiveImageProps`)
- Inherits all `img` HTML attributes via `PrimitiveImageProps`
- `ref` (forwarded)

**AvatarFallback** (renders `Primitive.span` when image not loaded)
- `delayMs?: number` — optional delay before showing the fallback
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**ImageLoadingStatus type:** `'idle' | 'loading' | 'loaded' | 'error'`

### Leptos API

**Exported components:** `Avatar`, `AvatarImage`, `AvatarFallback`

**Exported types:** `ImageLoadingStatus`

**Avatar**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AvatarImage**
- `src: MaybeProp<String>` (optional) — image source
- `on_loading_status_change: Option<Callback<ImageLoadingStatus>>` (optional) — callback when loading status changes
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**AvatarFallback**
- `delay_ms: MaybeProp<i32>` (optional) — delay in milliseconds before showing fallback
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**ImageLoadingStatus enum:** `Idle | Loading | Loaded | Error`

### Differences

1. **No `Root`/`Image`/`Fallback` aliases:** React exports short aliases; Leptos does not.
2. **No `createAvatarScope`:** React exports a scope creator for nested context composition. Leptos uses Leptos's built-in `Provider`/`expect_context` pattern and does not export a scope creator.
3. **Missing `referrerPolicy` and `crossOrigin` on AvatarImage:** React's `useImageLoadingStatus` hook reads `referrerPolicy` and `crossOrigin` from AvatarImage props and applies them to the internal detection image. Leptos's `use_image_loading_status` does not accept or use these props -- the internal image element used for load detection does not set referrer policy or cross-origin attributes.
4. **`delayMs` type:** React uses `number | undefined`; Leptos uses `MaybeProp<i32>`.
5. **`src` prop origin:** In React, `src` comes from inherited `PrimitiveImageProps` (HTML img attributes). In Leptos, it is an explicit prop.
6. **`ImageLoadingStatus` type:** React uses a string union type; Leptos uses an enum. Functionally equivalent.
7. **No arbitrary HTML attribute forwarding:** React inherits all `span`/`img` HTML attributes. Leptos requires `attr:*`.

### Assessment

Good alignment. All three components are present with the core API matching. The missing `referrerPolicy` and `crossOrigin` support in the image loading hook is a functional gap -- images served with CORS restrictions or referrer-policy requirements may fail to load correctly in the detection phase. The `ImageLoadingStatus` enum is a clean Rust adaptation of the React string union.

---

## Label

### React API

**Exported components:** `Label` (aliased as `Root`)

**Label** (renders `Primitive.label`)
- No component-specific props
- Inherits all `label` HTML attributes via `PrimitiveLabelProps` (includes `onMouseDown` via standard HTML events)
- `ref` (forwarded)
- Internal behavior: intercepts `onMouseDown` to prevent text selection on double-click (unless target is inside `button, input, select, textarea`)

### Leptos API

**Exported components:** `Label`

**Label**
- `on_mouse_down: MaybeCallback<MouseEvent>` (optional) — explicit mouse down handler, composed with internal behavior
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Explicit `on_mouse_down` prop:** Leptos declares `on_mouse_down` explicitly so it can compose it with the internal double-click prevention handler. React gets `onMouseDown` via inherited `PrimitiveLabelProps` and composes via `props.onMouseDown?.(event)`.
2. **No `Root` alias.**
3. **No arbitrary HTML attribute forwarding:** React inherits all `label` attributes. Leptos requires `attr:*`.

### Assessment

Excellent alignment. The component is simple and the APIs match closely. The explicit `on_mouse_down` prop is a necessary Leptos adaptation for event composition. The double-click prevention behavior is correctly implemented in both.

---

## Separator

### React API

**Exported components:** `Separator` (aliased as `Root`)

**Separator** (renders `Primitive.div`)
- `orientation?: 'horizontal' | 'vertical'` (default `'horizontal'`) — visual orientation
- `decorative?: boolean` — when true, renders with `role="none"` instead of `role="separator"` and omits `aria-orientation`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)
- Sets `data-orientation` attribute
- Sets `role="separator"` with `aria-orientation` (only when vertical, since horizontal is the default) when not decorative; `role="none"` when decorative

### Leptos API

**Exported components:** `Separator`

**Exported types:** `Orientation`

**Separator**
- `orientation: MaybeProp<Orientation>` (optional, default `Horizontal`) — enum `Horizontal | Vertical`
- `decorative: MaybeProp<bool>` (optional, default `false`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **`orientation` type:** React uses `'horizontal' | 'vertical'` string union with runtime validation (`isValidOrientation`). Leptos uses an `Orientation` enum, which provides compile-time type safety.
2. **No `Root` alias.**
3. **No arbitrary HTML attribute forwarding:** React inherits all `div` attributes. Leptos requires `attr:*`.
4. **No invalid orientation fallback:** React includes runtime validation that falls back to `'horizontal'` if an invalid orientation string is passed. Leptos's enum makes this unnecessary -- invalid values are a compile-time error.

### Assessment

Excellent alignment. Both props (`orientation`, `decorative`) match with correct behavior for ARIA attributes and data attributes. The `Orientation` enum is a strict improvement over React's string-based approach. The rendering logic (role, aria-orientation, data-orientation) is equivalent.

---

## VisuallyHidden

### React API

**Exported components:** `VisuallyHidden` (aliased as `Root`)

**Exported constants:** `VISUALLY_HIDDEN_STYLES` — the CSS properties object used for visual hiding

**VisuallyHidden** (renders `Primitive.span`)
- No component-specific props
- `style` — merged with `VISUALLY_HIDDEN_STYLES` (user styles override)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**VISUALLY_HIDDEN_STYLES:** `{ position: 'absolute', border: 0, width: 1, height: 1, padding: 0, margin: -1, overflow: 'hidden', clip: 'rect(0, 0, 0, 0)', whiteSpace: 'nowrap', wordWrap: 'normal' }`

### Leptos API

**Exported components:** `VisuallyHidden`

**VisuallyHidden**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

Styles are applied directly as individual `style:*` properties on the `Primitive` element.

### Differences

1. **No exported `VISUALLY_HIDDEN_STYLES` constant:** React exports this as a reusable constant. Leptos applies the styles inline and does not export them.
2. **No `Root` alias.**
3. **`style` merging behavior:** React uses `{ ...VISUALLY_HIDDEN_STYLES, ...props.style }`, allowing user styles to override the hiding styles. Leptos applies styles as individual `style:*` directives, which means user-supplied `style` attributes may not override them as easily (depends on how Leptos handles style merging with `attr:style` vs `style:*`).
4. **No arbitrary HTML attribute forwarding:** React inherits all `span` attributes. Leptos requires `attr:*`.
5. **Style values:** React uses numeric values (e.g., `width: 1`, `height: 1`, `border: 0`). Leptos uses string values with units (e.g., `"1px"`, `"0px"`).

### Assessment

Good alignment. The core purpose is identical and the same CSS hiding technique is used. The lack of an exported `VISUALLY_HIDDEN_STYLES` constant is a minor gap for advanced consumers who want to reuse the styles. The style override behavior difference could matter if a consumer needs to temporarily unhide a visually-hidden element via inline styles.

---

## Progress

### React API

**Exported components:** `Progress` (aliased as `Root`), `ProgressIndicator` (aliased as `Indicator`), `createProgressScope`

**Progress** (renders `Primitive.div`)
- `value?: number | null | undefined` (default `null`) — current progress value; `null` means indeterminate
- `max?: number` (default `100`) — maximum value; must be > 0
- `getValueLabel?: (value: number, max: number) => string` (default: percentage string) — custom label for accessibility
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)
- Sets: `role="progressbar"`, `aria-valuemax`, `aria-valuemin=0`, `aria-valuenow` (when value is a number), `aria-valuetext`, `data-state` (`'indeterminate' | 'complete' | 'loading'`), `data-value`, `data-max`

**ProgressIndicator** (renders `Primitive.div`)
- No component-specific props
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)
- Sets: `data-state`, `data-value`, `data-max` (read from context)

**ProgressState type:** `'indeterminate' | 'complete' | 'loading'`

### Leptos API

**Exported components:** `Progress`, `ProgressIndicator`

**Exported types:** `ProgressState`

**Progress**
- `value: MaybeProp<f64>` (optional) — current progress value; `None` means indeterminate
- `max: MaybeProp<f64>` (optional, default `100.0`) — maximum value
- `get_value_label: Option<Callback<(f64, f64), String>>` (optional) — custom label callback `(value, max) -> String`
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ProgressIndicator**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **`value` type:** React uses `number | null | undefined` (default `null`). Leptos uses `MaybeProp<f64>` where `None` represents indeterminate. Functionally equivalent.
2. **`max` type:** React uses `number`; Leptos uses `MaybeProp<f64>` making it reactive.
3. **`getValueLabel` signature:** React uses `(value: number, max: number) => string`. Leptos uses `Callback<(f64, f64), String>` (a tuple argument). Functionally equivalent.
4. **`max` validation:** React validates that `max > 0` and logs an error. Leptos treats `max == 0.0` as invalid (falls back to `DEFAULT_MAX`), but does not check for negative values explicitly -- a negative max would be used as-is.
5. **`value` validation:** React validates `0 <= value <= max` and logs an error, falling back to `null`. Leptos clamps value to `[0, max]` via `.min(max).max(0.0)` instead of rejecting invalid values.
6. **No `createProgressScope`:** React exports a scope creator. Leptos uses built-in context.
7. **No `Root`/`Indicator` aliases.**
8. **No arbitrary HTML attribute forwarding.**

### Assessment

Good alignment. Both components are present with matching data attributes and ARIA attributes. The core API (`value`, `max`, `getValueLabel`/`get_value_label`) maps well. The validation behavior differs subtly (clamping vs rejecting), which could matter for edge cases where invalid values are passed.

---

## Toggle

### React API

**Exported components:** `Toggle` (aliased as `Root`)

**Toggle** (renders `Primitive.button`)
- `pressed?: boolean` — controlled pressed state
- `defaultPressed?: boolean` (default `false`) — uncontrolled default pressed state
- `onPressedChange?: (pressed: boolean) => void` — callback when pressed state changes
- `disabled?: boolean` — inherited from `PrimitiveButtonProps`
- `onClick?: (event) => void` — inherited from `PrimitiveButtonProps`, composed with internal handler
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)
- Sets: `type="button"`, `aria-pressed`, `data-state` (`'on' | 'off'`), `data-disabled` (empty string when disabled)

### Leptos API

**Exported components:** `Toggle`

**Toggle**
- `pressed: MaybeProp<bool>` (optional) — controlled pressed state
- `default_pressed: MaybeProp<bool>` (optional) — uncontrolled default pressed state
- `on_pressed_change: Option<Callback<bool>>` (optional) — callback when pressed state changes
- `disabled: MaybeProp<bool>` (optional) — disables interaction
- `on_click: Option<Callback<ev::MouseEvent>>` (optional) — click handler, composed with internal toggle handler
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Explicit `disabled` and `on_click` props:** Leptos declares these explicitly. React gets them via inherited `PrimitiveButtonProps`. In Leptos, `disabled` is also used to set `attr:disabled` on the button (React relies on it being spread from `buttonProps`).
2. **No `Root` alias.**
3. **No arbitrary HTML attribute forwarding.**
4. **`data-disabled` rendering:** React sets `data-disabled=""` (empty string) when disabled, `undefined` when not. Leptos uses a `data_attr()` helper that should produce the same behavior.

### Assessment

Excellent alignment. The core toggle API (`pressed`, `defaultPressed`/`default_pressed`, `onPressedChange`/`on_pressed_change`) maps 1:1. The `disabled`, `on_click`, and controllable state patterns are all present and correct. The rendered attributes (`type="button"`, `aria-pressed`, `data-state`, `data-disabled`) match.

---

## Accordion

### React API

**Exported components:** `Accordion`, `AccordionItem`, `AccordionHeader`, `AccordionTrigger`, `AccordionContent`

**Accordion** (renders `div`)
- `type: 'single' | 'multiple'` (required) — discriminated union; determines which additional props apply
- When `type = 'single'`:
  - `value?: string` — controlled open item value
  - `defaultValue?: string` — uncontrolled default value
  - `onValueChange?(value: string): void` — callback when value changes
  - `collapsible?: boolean` (default `false`) — whether the last open item can be closed
- When `type = 'multiple'`:
  - `value?: string[]` — controlled open items
  - `defaultValue?: string[]` — uncontrolled default values
  - `onValueChange?(value: string[]): void` — callback when values change
- `disabled?: boolean` (default `false`)
- `orientation?: 'horizontal' | 'vertical'` (default `'vertical'`)
- `dir?: 'ltr' | 'rtl'`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**AccordionItem** (renders Collapsible.Root)
- `value: string` (required) — unique identifier for the item
- `disabled?: boolean` (default `false`)
- Inherits Collapsible.Root props (minus `open`, `defaultOpen`, `onOpenChange`)
- `ref` (forwarded)

**AccordionHeader** (renders `h3`)
- No component-specific props
- Inherits all `h3` HTML attributes via `PrimitiveHeading3Props`
- `ref` (forwarded)

**AccordionTrigger** (renders Collapsible.Trigger / `button`)
- No component-specific props
- Inherits all `button` HTML attributes via `CollapsibleTriggerProps`
- `ref` (forwarded)

**AccordionContent** (renders Collapsible.Content / `div`)
- `forceMount?: true` — force mount for animation control (inherited from CollapsibleContentProps)
- Inherits all `div` HTML attributes via `CollapsibleContentProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `Accordion`, `AccordionItem`, `AccordionHeader`, `AccordionTrigger`, `AccordionContent`

**Accordion**
- `r#type: AccordionType` (required) — enum `Single | Multiple`
- `value: MaybeProp<String>` (optional) — single mode controlled value
- `default_value: MaybeProp<String>` (optional) — single mode default
- `on_value_change: Option<Callback<String>>` (optional) — single mode callback
- `collapsible: MaybeProp<bool>` (optional) — single mode collapsible
- `values: MaybeProp<Vec<String>>` (optional) — multiple mode controlled values
- `default_values: MaybeProp<Vec<String>>` (optional) — multiple mode default
- `on_values_change: Option<Callback<Vec<String>>>` (optional) — multiple mode callback
- `disabled: MaybeProp<bool>` (optional)
- `dir: MaybeProp<Direction>` (optional)
- `orientation: MaybeProp<Orientation>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AccordionItem**
- `value: String` (required)
- `disabled: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AccordionHeader**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AccordionTrigger**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AccordionContent**
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Type prop flattening:** React uses a discriminated union (`AccordionSingleProps | AccordionMultipleProps`) so only relevant props are valid per mode. Leptos uses a single `Accordion` component with all single/multiple props combined. The Leptos naming uses `values`/`default_values`/`on_values_change` (plural) for multiple mode, vs React which uses `value`/`defaultValue`/`onValueChange` with `string[]` types. This is a reasonable Leptos adaptation since Rust cannot do discriminated unions on component props.
2. **No arbitrary HTML attribute forwarding:** React inherits all `div`/`h3`/`button` HTML attributes via `PrimitiveDivProps`, etc. Leptos components only accept explicitly declared props. Arbitrary attributes must be passed via `attr:*` syntax on Leptos components.
3. **`as_child` and `node_ref` are Leptos-specific prop names** — these replace React's `asChild` and `ref` forwarding patterns.
4. **`forceMount` type:** React uses `forceMount?: true` (literal `true`), Leptos uses `MaybeProp<bool>`.
5. **`children` is `Option<ChildrenFn>` on AccordionContent** in Leptos but required in React (passed as `React.ReactNode`).

### Assessment

Good alignment. All five React components are present with matching names. The core props (type, value, defaultValue, onValueChange, collapsible, disabled, orientation, dir, forceMount) are all present. The flattened single/multiple prop approach is a reasonable Leptos adaptation. The main gap is that Leptos cannot forward arbitrary HTML attributes as React does through `PrimitiveDivProps` spread.

---

## Collapsible

### React API

**Exported components:** `Collapsible`, `CollapsibleTrigger`, `CollapsibleContent`

**Collapsible** (renders `div`)
- `open?: boolean` — controlled open state
- `defaultOpen?: boolean` — uncontrolled default open state
- `disabled?: boolean` — disables interaction
- `onOpenChange?(open: boolean): void` — callback when open state changes
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**CollapsibleTrigger** (renders `button`)
- No component-specific props
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)

**CollapsibleContent** (renders `div`)
- `forceMount?: true` — force mount for animation control
- Inherits all `div` HTML attributes via `PrimitiveDivProps` (minus `present`)
- `ref` (forwarded)

### Leptos API

**Exported components:** `Collapsible`, `CollapsibleTrigger`, `CollapsibleContent`

**Collapsible**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**CollapsibleTrigger**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**CollapsibleContent**
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **CollapsibleTrigger has explicit `on_click`:** Leptos declares `on_click` as an explicit prop so it can compose it with the internal toggle handler. In React this is part of the inherited `PrimitiveButtonProps` and composed via `composeEventHandlers(props.onClick, ...)`.
2. **No arbitrary HTML attribute forwarding:** Same as Accordion — React spreads all HTML attributes; Leptos requires `attr:*` syntax.
3. **`forceMount` type:** React `true` literal vs Leptos `MaybeProp<bool>`.
4. **`children` on CollapsibleContent is `Option<ChildrenFn>`** in Leptos (optional) vs implicitly optional `React.ReactNode` in React.
5. **Leptos CollapsibleTrigger adds a disabled guard** to the click handler (`if !context.disabled.get()`) that React does not have — React relies on the native `disabled` attribute on the button to prevent clicks.

### Assessment

Excellent alignment. All three React components are present with matching names and props. The core API (`open`, `defaultOpen`/`default_open`, `disabled`, `onOpenChange`/`on_open_change`, `forceMount`/`force_mount`) maps 1:1.

---

## Checkbox

### React API

**Exported stable components:** `Checkbox`, `CheckboxIndicator`
**Exported unstable components:** `CheckboxProvider`, `CheckboxTrigger`, `CheckboxBubbleInput` (exported with `unstable_` prefix)

**Checkbox** (renders `button`) — the combined root component
- `checked?: CheckedState` (`boolean | 'indeterminate'`)
- `defaultChecked?: CheckedState`
- `required?: boolean`
- `disabled?: boolean`
- `value?: string | number | readonly string[]` (default `'on'`)
- `name?: string`
- `form?: string`
- `onCheckedChange?(checked: CheckedState): void`
- Inherits all `button` HTML attributes via `PrimitiveButtonProps` (minus `checked`, `defaultChecked`)
- `ref` (forwarded)

**CheckboxIndicator** (renders `span`)
- `forceMount?: true`
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**CheckboxProvider** (unstable — headless provider)
- `checked?: CheckedState`
- `defaultChecked?: CheckedState`
- `required?: boolean`
- `disabled?: boolean`
- `value?: string | number | readonly string[]` (default `'on'`)
- `name?: string`
- `form?: string`
- `onCheckedChange?(checked: CheckedState): void`
- `children?: React.ReactNode`

**CheckboxTrigger** (unstable — button element)
- `children?: React.ReactNode`
- Inherits `button` HTML attributes minus `CheckboxProviderProps` keys
- `ref` (forwarded)

**CheckboxBubbleInput** (unstable — hidden input)
- Inherits `input` HTML attributes minus `checked`
- `ref` (forwarded)

**CheckedState type:** `boolean | 'indeterminate'`

### Leptos API

**Exported components:** `Checkbox`, `CheckboxIndicator`

**Checkbox** (renders `button`)
- `name: MaybeProp<String>` (optional)
- `checked: MaybeProp<CheckedState>` (optional)
- `default_checked: MaybeProp<CheckedState>` (optional)
- `on_checked_change: Option<Callback<CheckedState>>` (optional)
- `required: MaybeProp<bool>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `value: MaybeProp<String>` (optional)
- `on_keydown: Option<Callback<ev::KeyboardEvent>>` (optional)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**CheckboxIndicator** (renders `span`)
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**CheckedState enum:** `False | True | Indeterminate`

### Differences

1. **Missing `form` prop:** React's `Checkbox` accepts `form?: string` to associate with a form by ID. Leptos does not have this prop. The Leptos form detection only uses `closest("form")`.
2. **Missing unstable components:** `CheckboxProvider`, `CheckboxTrigger`, `CheckboxBubbleInput` are not exported in Leptos (they are `unstable_` in React, so this is expected).
3. **`value` type difference:** React accepts `string | number | readonly string[]`; Leptos accepts only `MaybeProp<String>`.
4. **`CheckedState` type:** React uses `boolean | 'indeterminate'` (union type); Leptos uses an enum `CheckedState { False, True, Indeterminate }`. This is an idiomatic Rust adaptation.
5. **Explicit `on_keydown` and `on_click` props:** Leptos declares these explicitly for event composition. React gets them via inherited `PrimitiveButtonProps`.
6. **`forceMount` type:** React `true` literal vs Leptos `MaybeProp<bool>`.

### Assessment

Good alignment for the stable API. The two stable components (`Checkbox`, `CheckboxIndicator`) are present with equivalent props. The `CheckedState` adaptation from `boolean | 'indeterminate'` to a Rust enum is idiomatic and correct. The missing `form` prop is a minor gap. The unstable components are intentionally omitted.

---

## Switch

### React API

**Exported components:** `Switch`, `SwitchThumb`

**Switch** (renders `button`)
- `checked?: boolean` — controlled checked state
- `defaultChecked?: boolean` — uncontrolled default
- `required?: boolean`
- `disabled?: boolean`
- `value?: string` (default `'on'`)
- `name?: string` (extracted from props, passed to hidden input)
- `form?: string` (extracted from props, used for form association)
- `onCheckedChange?(checked: boolean): void`
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)

**SwitchThumb** (renders `span`)
- No component-specific props
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `Switch`, `SwitchThumb`

**Switch** (renders `button`)
- `name: MaybeProp<String>` (optional)
- `checked: MaybeProp<bool>` (optional)
- `default_checked: MaybeProp<bool>` (optional)
- `on_checked_change: Option<Callback<bool>>` (optional)
- `required: MaybeProp<bool>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `value: MaybeProp<String>` (optional)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**SwitchThumb** (renders `span`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Missing `form` prop:** React's `Switch` accepts `form?: string` for form association by ID. Leptos does not have this prop; it only detects forms via `closest("form")`.
2. **Explicit `on_click` prop:** Leptos declares `on_click` for event composition. React gets it via inherited `PrimitiveButtonProps`.
3. **No arbitrary HTML attribute forwarding:** React spreads all button/span attributes; Leptos requires explicit prop declarations or `attr:*`.

### Assessment

Excellent alignment. Both components match 1:1 with the React API. All core props (`checked`, `defaultChecked`, `required`, `disabled`, `value`, `name`, `onCheckedChange`) are present. The only functional gap is the missing `form` prop for explicit form association by ID.

---

## RadioGroup

### React API

**Exported components:** `RadioGroup`, `RadioGroupItem`, `RadioGroupIndicator`
**Internal (not exported):** `Radio`, `RadioIndicator`, `RadioBubbleInput`

**RadioGroup** (renders `div`, wrapped in `RovingFocusGroup.Root`)
- `name?: string` — name for the radio group (passed to hidden inputs)
- `value?: string | null` — controlled selected value
- `defaultValue?: string` — uncontrolled default value
- `onValueChange?(value: string): void` — callback when selection changes
- `required?: boolean` (default `false`)
- `disabled?: boolean` (default `false`)
- `orientation?: 'horizontal' | 'vertical'` — affects arrow key navigation and `aria-orientation`
- `dir?: 'ltr' | 'rtl'` — text direction for arrow key mapping
- `loop?: boolean` (default `true`) — whether keyboard navigation wraps
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**RadioGroupItem** (renders `button` via internal `Radio`)
- `value: string` (required) — value for this radio item
- `disabled?: boolean` — disables this item
- Inherits `Radio` props (minus `onCheck`, `name`)
- `ref` (forwarded)

**RadioGroupIndicator** (renders `span` via internal `RadioIndicator`)
- `forceMount?: true` — force mount for animation control
- Inherits all `span` HTML attributes via `RadioIndicatorProps`
- `ref` (forwarded)

**Internal Radio** (renders `button`)
- `checked?: boolean` (default `false`)
- `required?: boolean`
- `disabled?: boolean`
- `value?: string` (default `'on'`)
- `name?: string`
- `form?: string`
- `onCheck?(): void`
- Inherits all `button` HTML attributes
- `ref` (forwarded)

**Internal RadioIndicator** (renders `span`)
- `forceMount?: true`
- Inherits all `span` HTML attributes
- `ref` (forwarded)

### Leptos API

**Exported components:** `RadioGroup`, `RadioGroupItem`, `RadioGroupIndicator`
**Internal (pub(crate)):** `RadioIndicator`, `RadioBubbleInput`, `RadioButton`, `RadioContextValue`

**RadioGroup** (renders `div`, wrapped in `RovingFocusGroup`)
- `name: MaybeProp<String>` (optional)
- `value: MaybeProp<String>` (optional)
- `default_value: MaybeProp<String>` (optional)
- `on_value_change: Option<Callback<String>>` (optional)
- `required: MaybeProp<bool>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `orientation: MaybeProp<Orientation>` (optional)
- `dir: MaybeProp<Direction>` (optional)
- `r#loop: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**RadioGroupItem** (renders `button` via internal `RadioButton`)
- `value: String` (required)
- `disabled: MaybeProp<bool>` (optional)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**RadioGroupIndicator** (renders `span` via internal `RadioIndicator`)
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **`value` type on RadioGroup:** React accepts `string | null`; Leptos uses `MaybeProp<String>` where `None` represents no selection (equivalent to `null`).
2. **Explicit `on_click` on RadioGroupItem:** Leptos declares this explicitly; React gets it via inherited `PrimitiveButtonProps` on the internal `Radio` component.
3. **Missing `form` prop on internal Radio:** React's internal `Radio` accepts `form?: string` for explicit form association. Leptos's internal `RadioButton` does not expose this.
4. **No arbitrary HTML attribute forwarding:** React's `RadioGroupItem` inherits `Radio` props which inherit `PrimitiveButtonProps`. Leptos requires explicit `attr:*`.
5. **`forceMount` type:** React `true` literal vs Leptos `MaybeProp<bool>`.

### Assessment

Excellent alignment. All three public React components are present with matching names and equivalent props. The core API (`name`, `value`, `defaultValue`, `onValueChange`, `required`, `disabled`, `orientation`, `dir`, `loop`, `forceMount`) maps cleanly. The `string | null` vs `MaybeProp<String>` difference is idiomatic. The internal component structure differs (Leptos uses `RadioButton` instead of `Radio`) but the public API is equivalent.

---

## Form

### React API

**Exported components:** `Form` (alias `Root`), `FormField` (alias `Field`), `FormLabel` (alias `Label`), `FormControl` (alias `Control`), `FormMessage` (alias `Message`), `FormValidityState` (alias `ValidityState`), `FormSubmit` (alias `Submit`).

**Form (Root):**
- Renders `<form>` via `Primitive.form`
- `onClearServerErrors?: () => void`
- Inherits all `<form>` HTML props (via `PrimitiveFormProps`)
- `ref` forwarded

**FormField (Field):**
- Renders `<div>` via `Primitive.div`
- `name: string` (required)
- `serverInvalid?: boolean` (default `false`)
- Inherits all `<div>` HTML props
- `ref` forwarded

**FormLabel (Label):**
- Renders via `LabelPrimitive` (the Radix Label component)
- `htmlFor?: string`
- Inherits all Label props
- `ref` forwarded

**FormControl (Control):**
- Renders `<input>` via `Primitive.input`
- `id?: string`
- `name?: string`
- Inherits all `<input>` HTML props (including `onInvalid`, `onChange`)
- `ref` forwarded

**FormMessage (Message):**
- Renders `<span>` via `Primitive.span`
- `match?: ValidityMatcher | CustomMatcher`
- `forceMatch?: boolean`
- `name?: string`
- `id?: string`
- Inherits all `<span>` HTML props
- `ref` forwarded
- `ValidityMatcher` is one of: `'badInput'`, `'patternMismatch'`, `'rangeOverflow'`, `'rangeUnderflow'`, `'stepMismatch'`, `'tooLong'`, `'tooShort'`, `'typeMismatch'`, `'valid'`, `'valueMissing'`
- `CustomMatcher` is `(value: string, formData: FormData) => boolean | Promise<boolean>`

**FormValidityState (ValidityState):**
- Renderless (render prop pattern)
- `children: (validity: ValidityState | undefined) => ReactNode`
- `name?: string`
- No ref forwarding

**FormSubmit (Submit):**
- Renders `<button type="submit">` via `Primitive.button`
- Inherits all `<button>` HTML props
- `ref` forwarded

### Leptos API

**Exported components:** `Form` (alias `Root`), `FormField` (alias `Field`), `FormLabel` (alias `Label`), `FormControl` (alias `Control`), `FormMessage` (alias `Message`), `FormValidityState` (alias `ValidityState`), `FormSubmit` (alias `Submit`).

**Form (Root):**
- `on_clear_server_errors: Option<Callback<()>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**FormField (Field):**
- `name: String` (required, `#[prop(into)]`)
- `server_invalid: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**FormLabel (Label):**
- `html_for: Option<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**FormControl (Control):**
- `id: Option<String>`
- `name: Option<String>`
- `on_invalid: Option<Callback<web_sys::Event>>`
- `on_change: Option<Callback<web_sys::Event>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**FormMessage (Message):**
- `r#match: Option<Match>` (where `Match` is an enum wrapping `ValidityMatcher` or `CustomMatcher`)
- `force_match: MaybeProp<bool>`
- `name: Option<String>`
- `id: Option<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`
- `ValidityMatcher` enum: `BadInput`, `PatternMismatch`, `RangeOverflow`, `RangeUnderflow`, `StepMismatch`, `TooLong`, `TooShort`, `TypeMismatch`, `Valid`, `ValueMissing`
- `CustomMatcher` is a `Callback` accepting `(String, web_sys::FormData)` and returning `bool` (sync or async via `Pin<Box<dyn Future<Output = bool>>>`)

**FormValidityState (ValidityState):**
- `name: Option<String>`
- `children: Callback<Option<Validity>, AnyView>`

**FormSubmit (Submit):**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

### Differences

1. **Match prop type** (FormMessage): React uses a union type `ValidityMatcher | CustomMatcher` directly. Leptos wraps it in an `Option<Match>` enum with `Match::BuiltIn(ValidityMatcher)` and `Match::Custom(CustomMatcher)` variants. Functionally equivalent but structurally different.
2. **CustomMatcher async support**: React uses `Promise<boolean>` return type for async matchers. Leptos uses `Pin<Box<dyn Future<Output = bool>>>`, which is the idiomatic Rust equivalent.
3. **FormValidityState children**: React uses a render function `children(validity)`. Leptos uses `Callback<Option<Validity>, AnyView>`. Functionally equivalent pattern.
4. **Validity type**: React uses the browser's native `ValidityState`. Leptos uses a custom `Validity` struct that mirrors the same fields. This is needed because `web_sys::ValidityState` is not `Send`.
5. **HTML prop forwarding**: React inherits all HTML props via `extends PrimitiveFormProps`, etc. Leptos uses `as_child` pattern and attribute spreading via `{..attrs}` on the Primitive. Individual event handlers (`on_invalid`, `on_change`) are explicit props rather than inherited.
6. **Naming convention**: React uses camelCase (`onClearServerErrors`, `serverInvalid`, `htmlFor`, `forceMatch`). Leptos uses snake_case (`on_clear_server_errors`, `server_invalid`, `html_for`, `force_match`). This is expected.

### Assessment

Excellent alignment. All 7 exported components match. All significant props are present. The type differences are idiomatic adaptations (Rust enums instead of TS unions, `Callback` instead of render functions, `Validity` struct instead of browser `ValidityState`). No missing functionality.

---

## Toast

### React API

**Exported components:** `ToastProvider` (alias `Provider`), `ToastViewport` (alias `Viewport`), `Toast` (alias `Root`), `ToastTitle` (alias `Title`), `ToastDescription` (alias `Description`), `ToastAction` (alias `Action`), `ToastClose` (alias `Close`).

**ToastProvider (Provider):**
- Not a DOM element (renders only children + context)
- `children?: ReactNode`
- `label?: string` (default `'Notification'`)
- `duration?: number` (default `5000`)
- `swipeDirection?: 'up' | 'down' | 'left' | 'right'` (default `'right'`)
- `swipeThreshold?: number` (default `50`)

**ToastViewport (Viewport):**
- Renders `<ol>` via `Primitive.ol` wrapped in a `DismissableLayer.Branch` region
- `hotkey?: string[]` (default `['F8']`)
- `label?: string` (default `'Notifications ({hotkey})'`)
- Inherits all `<ol>` HTML props
- `ref` forwarded

**Toast (Root):**
- Renders `<li>` via `Primitive.li` (portaled into viewport)
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?: (open: boolean) => void`
- `forceMount?: true`
- `type?: 'foreground' | 'background'` (default `'foreground'`)
- `duration?: number`
- `onEscapeKeyDown?: (event: KeyboardEvent) => void`
- `onPause?: () => void`
- `onResume?: () => void`
- `onSwipeStart?: (event: SwipeEvent) => void`
- `onSwipeMove?: (event: SwipeEvent) => void`
- `onSwipeCancel?: (event: SwipeEvent) => void`
- `onSwipeEnd?: (event: SwipeEvent) => void`
- Inherits all `<li>` HTML props
- `ref` forwarded

**ToastTitle (Title):**
- Renders `<div>` via `Primitive.div`
- Inherits all `<div>` HTML props
- `ref` forwarded

**ToastDescription (Description):**
- Renders `<div>` via `Primitive.div`
- Inherits all `<div>` HTML props
- `ref` forwarded

**ToastAction (Action):**
- Renders `<button>` via `ToastClose` (which uses `Primitive.button`)
- `altText: string` (required)
- Inherits all `<button>` HTML props (via `ToastCloseProps`)
- `ref` forwarded

**ToastClose (Close):**
- Renders `<button type="button">` via `Primitive.button`
- Inherits all `<button>` HTML props
- `ref` forwarded

### Leptos API

**Exported components:** `ToastProvider`, `ToastViewport`, `Toast`, `ToastTitle`, `ToastDescription`, `ToastAction`, `ToastClose`.

**ToastProvider:**
- `label: String` (default `"Notification"`)
- `duration: Signal<i32>` (default `5000`)
- `swipe_direction: Signal<SwipeDirection>` (default `SwipeDirection::Right`)
- `swipe_threshold: Signal<f64>` (default `50.0`)
- `children: ChildrenFn`

**ToastViewport:**
- `hotkey: Option<Vec<String>>`
- `label: Option<String>`
- `node_ref: AnyNodeRef`
- `as_child: MaybeProp<bool>`
- `children: Option<ChildrenFn>`

**Toast:**
- `open: MaybeProp<bool>`
- `default_open: MaybeProp<bool>`
- `on_open_change: Option<Callback<bool>>`
- `force_mount: Option<bool>`
- `r#type: String` (default `"foreground"`)
- `duration: MaybeProp<i32>`
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>`
- `on_pause: Option<Callback<()>>`
- `on_resume: Option<Callback<()>>`
- `on_swipe_start: Option<Callback<SwipeEvent>>`
- `on_swipe_move: Option<Callback<SwipeEvent>>`
- `on_swipe_cancel: Option<Callback<SwipeEvent>>`
- `on_swipe_end: Option<Callback<SwipeEvent>>`
- `class: MaybeProp<String>` (Extra -- for portal class forwarding)
- `node_ref: AnyNodeRef`
- `as_child: MaybeProp<bool>`
- `children: ChildrenFn`

**ToastTitle:**
- `node_ref: AnyNodeRef`
- `as_child: MaybeProp<bool>`
- `children: ChildrenFn`

**ToastDescription:**
- `node_ref: AnyNodeRef`
- `as_child: MaybeProp<bool>`
- `children: ChildrenFn`

**ToastAction:**
- `alt_text: String` (required)
- `node_ref: AnyNodeRef`
- `as_child: MaybeProp<bool>`
- `children: ChildrenFn`

**ToastClose:**
- `node_ref: AnyNodeRef`
- `as_child: MaybeProp<bool>`
- `children: ChildrenFn`

### Differences

1. **Extra `class` prop on Toast**: Leptos adds a `class: MaybeProp<String>` prop not present in React. This is needed because Leptos portals create a separate rendering context and `attr:class` cannot cross the boundary. This is a framework-specific workaround, not an API divergence.
2. **`type` prop on Toast**: React uses `type?: 'foreground' | 'background'` (an enum-like string union). Leptos uses `r#type: String` with a string default of `"foreground"`. This could be a typed enum for better type safety.
3. **`duration` on ToastProvider**: React uses a plain `number` prop. Leptos uses `Signal<i32>`, making it reactive. This is a deliberate enhancement.
4. **`swipe_direction` on ToastProvider**: React uses a plain `SwipeDirection` prop. Leptos uses `Signal<SwipeDirection>`, making it reactive. Same pattern as duration.
5. **`swipe_threshold` on ToastProvider**: React uses `number`. Leptos uses `Signal<f64>`. Same reactive pattern.
6. **ToastClose missing explicit `on_click`**: React's `ToastClose` inherits `onClick` from `PrimitiveButtonProps`. The Leptos version does not expose an explicit `on_click` prop -- the close handler is wired internally. Users can still compose via `attr:on:click`.
7. **No re-export aliases**: React exports `Provider`, `Viewport`, `Root`, `Title`, `Description`, `Action`, `Close` as aliases. Leptos does not define these short aliases.

### Assessment

Good alignment. All 7 exported component types match. The core functionality (provider config, open/close control, swipe events, escape key, viewport hotkey) is fully present. The `class` prop is a justified Leptos-specific addition. The lack of an enum type for `Toast.type` is a minor type-safety gap. Making provider props reactive (`Signal<T>`) is an idiomatic improvement. Missing re-export aliases are cosmetic.

---

## NavigationMenu

### React API

**Exported components:** `NavigationMenu` (alias `Root`), `NavigationMenuSub` (alias `Sub`), `NavigationMenuList` (alias `List`), `NavigationMenuItem` (alias `Item`), `NavigationMenuTrigger` (alias `Trigger`), `NavigationMenuLink` (alias `Link`), `NavigationMenuIndicator` (alias `Indicator`), `NavigationMenuContent` (alias `Content`), `NavigationMenuViewport` (alias `Viewport`).

**NavigationMenu (Root):**
- Renders `<nav>` via `Primitive.nav`
- `value?: string`
- `defaultValue?: string`
- `onValueChange?: (value: string) => void`
- `dir?: 'ltr' | 'rtl'`
- `orientation?: 'vertical' | 'horizontal'` (default `'horizontal'`)
- `delayDuration?: number` (default `200`)
- `skipDelayDuration?: number` (default `300`)
- Inherits all `<nav>` HTML props
- `ref` forwarded

**NavigationMenuSub (Sub):**
- Renders `<div>` via `Primitive.div`
- `value?: string`
- `defaultValue?: string`
- `onValueChange?: (value: string) => void`
- `orientation?: 'vertical' | 'horizontal'` (default `'horizontal'`)
- Inherits all `<div>` HTML props
- `ref` forwarded

**NavigationMenuList (List):**
- Renders `<ul>` via `Primitive.ul` (inside a wrapper `<div>`)
- Inherits all `<ul>` HTML props
- `ref` forwarded

**NavigationMenuItem (Item):**
- Renders `<li>` via `Primitive.li`
- `value?: string`
- Inherits all `<li>` HTML props
- `ref` forwarded

**NavigationMenuTrigger (Trigger):**
- Renders `<button>` via `Primitive.button`
- `disabled?: boolean` (inherited from `PrimitiveButtonProps`)
- Inherits all `<button>` HTML props
- `ref` forwarded

**NavigationMenuLink (Link):**
- Renders `<a>` via `Primitive.a`
- `active?: boolean`
- `onSelect?: (event: Event) => void`
- Inherits all `<a>` HTML props (except `onSelect` which is custom)
- `ref` forwarded

**NavigationMenuIndicator (Indicator):**
- Renders `<div>` via `Primitive.div` (portaled into indicator track)
- `forceMount?: true`
- Inherits all `<div>` HTML props
- `ref` forwarded

**NavigationMenuContent (Content):**
- Renders via `DismissableLayer` (which renders a `<div>`)
- `forceMount?: true`
- Inherits `DismissableLayer` props (minus `onDismiss`, `disableOutsidePointerEvents`):
  - `onEscapeKeyDown?: (event: KeyboardEvent) => void`
  - `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void`
  - `onFocusOutside?: (event: FocusOutsideEvent) => void`
  - `onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void`
- Inherits all `<div>` HTML props
- `ref` forwarded

**NavigationMenuViewport (Viewport):**
- Renders `<div>` via `Primitive.div`
- `forceMount?: true`
- Inherits all `<div>` HTML props
- `ref` forwarded

### Leptos API

**Exported components:** `NavigationMenu`, `NavigationMenuSub`, `NavigationMenuList`, `NavigationMenuItem`, `NavigationMenuTrigger`, `NavigationMenuLink`, `NavigationMenuIndicator`, `NavigationMenuContent`, `NavigationMenuViewport`.

**NavigationMenu (Root):**
- `value: MaybeProp<String>`
- `default_value: MaybeProp<String>`
- `on_value_change: Option<Callback<String>>`
- `dir: MaybeProp<Direction>`
- `orientation: MaybeProp<Orientation>` (default `Orientation::Horizontal`)
- `delay_duration: MaybeProp<f64>` (default `200.0`)
- `skip_delay_duration: MaybeProp<f64>` (default `300.0`)
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuSub (Sub):**
- `value: MaybeProp<String>`
- `default_value: MaybeProp<String>`
- `on_value_change: Option<Callback<String>>`
- `orientation: MaybeProp<Orientation>` (default `Orientation::Horizontal`)
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuList (List):**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuItem (Item):**
- `value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuTrigger (Trigger):**
- `disabled: MaybeProp<bool>`
- `on_pointer_enter: Option<Callback<ev::PointerEvent>>`
- `on_pointer_move: Option<Callback<ev::PointerEvent>>`
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>`
- `on_click: Option<Callback<ev::MouseEvent>>`
- `on_key_down: Option<Callback<ev::KeyboardEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuLink (Link):**
- `active: MaybeProp<bool>`
- `on_select: Option<Callback<web_sys::Event>>`
- `on_click: Option<Callback<ev::MouseEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuIndicator (Indicator):**
- `force_mount: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuContent (Content):**
- `force_mount: MaybeProp<bool>`
- `on_pointer_enter: Option<Callback<ev::PointerEvent>>`
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>`
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**NavigationMenuViewport (Viewport):**
- `force_mount: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

### Differences

1. **Explicit event handler props on Trigger**: Leptos explicitly exposes `on_pointer_enter`, `on_pointer_move`, `on_pointer_leave`, `on_click`, `on_key_down` as named props on `NavigationMenuTrigger`. In React these are inherited from `PrimitiveButtonProps` and composed internally via `composeEventHandlers`. The Leptos approach makes these explicit for composition.
2. **Explicit `on_click` on Link**: Leptos adds an explicit `on_click` prop. React inherits it from `PrimitiveLinkProps`.
3. **Explicit event props on Content**: Leptos exposes `on_pointer_enter`, `on_pointer_leave` as explicit props alongside the DismissableLayer-inherited props. React inherits pointer events from HTML props and composes them internally.
4. **`delay_duration` and `skip_delay_duration` types**: React uses `number`. Leptos uses `MaybeProp<f64>`, making them reactive and optional.
5. **`forceMount` type**: React uses `forceMount?: true` (literal `true` type). Leptos uses `MaybeProp<bool>`. Functionally equivalent but Leptos allows `false` as a value.
6. **No re-export aliases**: React exports short aliases (`Root`, `Sub`, `List`, `Item`, `Trigger`, `Link`, `Indicator`, `Content`, `Viewport`). Leptos does not define these.

### Assessment

Excellent alignment. All 9 exported components match. All significant props are present (value control, direction, orientation, delay durations, force mount, dismiss layer callbacks). The explicit event handler props in Leptos are a framework-idiomatic pattern since Leptos components lack automatic HTML prop inheritance. No missing functionality.

---

## OneTimePasswordField

### React API

**Exported components:** `OneTimePasswordField` (alias `Root`), `OneTimePasswordFieldInput` (alias `Input`), `OneTimePasswordFieldHiddenInput` (alias `HiddenInput`).

**OneTimePasswordField (Root):**
- Renders `<div>` via `Primitive.Root.div` wrapped in `RovingFocusGroup.Root`
- `value?: string`
- `defaultValue?: string`
- `onValueChange?: (value: string) => void`
- `autoSubmit?: boolean` (default `false`)
- `onAutoSubmit?: (value: string) => void`
- `disabled?: boolean` (default `false`)
- `readOnly?: boolean` (default `false`)
- `autoComplete?: 'off' | 'one-time-code'` (default `'one-time-code'`)
- `autoFocus?: boolean` (default `false`)
- `form?: string`
- `name?: string`
- `placeholder?: string`
- `type?: 'password' | 'text'` (default `'text'`)
- `orientation?: 'horizontal' | 'vertical'` (default `'horizontal'`)
- `dir?: 'ltr' | 'rtl'`
- `validationType?: 'alpha' | 'numeric' | 'alphanumeric' | 'none'` (default `'numeric'`)
- `sanitizeValue?: (value: string) => string`
- `onPaste` (inherited from div HTML props)
- Inherits all `<div>` HTML props
- `ref` forwarded

**OneTimePasswordFieldInput (Input):**
- Renders `<input>` via `Primitive.Root.input` wrapped in `RovingFocusGroup.Item`
- `onInvalidChange?: (character: string) => void`
- `index?: number`
- Omits from HTML input props: `value`, `defaultValue`, `disabled`, `readOnly`, `autoComplete`, `autoFocus`, `form`, `name`, `placeholder`, `type`
- Inherits remaining `<input>` HTML props
- `ref` forwarded

**OneTimePasswordFieldHiddenInput (HiddenInput):**
- Renders native `<input type="hidden">`
- Omits: `value`, `defaultValue`, `type`, `onChange`, `readOnly`, `disabled`, `autoComplete`, `autoFocus`
- Inherits remaining `<input>` HTML props
- `ref` forwarded

### Leptos API

**Exported components:** `OneTimePasswordField`, `OneTimePasswordFieldInput`, `OneTimePasswordFieldHiddenInput`.

**OneTimePasswordField (Root):**
- `value: MaybeProp<String>`
- `default_value: MaybeProp<String>`
- `on_value_change: Option<Callback<String>>`
- `auto_submit: MaybeProp<bool>`
- `on_auto_submit: Option<Callback<String>>`
- `disabled: MaybeProp<bool>`
- `read_only: MaybeProp<bool>`
- `auto_complete: MaybeProp<AutoComplete>` (enum: `Off`, `OneTimeCode`, default `OneTimeCode`)
- `auto_focus: MaybeProp<bool>`
- `form: MaybeProp<String>`
- `name: MaybeProp<String>`
- `placeholder: MaybeProp<String>`
- `r#type: MaybeProp<InputType>` (enum: `Password`, `Text`, default `Text`)
- `orientation: MaybeProp<Orientation>` (enum: `Horizontal`, `Vertical`)
- `dir: MaybeProp<Direction>`
- `validation_type: MaybeProp<InputValidationType>` (enum: `Alpha`, `Numeric`, `Alphanumeric`, `None`, default `Numeric`)
- `sanitize_value: Option<Callback<String, String>>`
- `on_paste: Option<Callback<ev::ClipboardEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**OneTimePasswordFieldInput (Input):**
- `on_invalid_change: Option<Callback<String>>`
- `index: MaybeProp<usize>`
- `on_focus: Option<Callback<ev::FocusEvent>>`
- `on_cut: Option<Callback<ev::ClipboardEvent>>`
- `on_input: Option<Callback<ev::Event>>`
- `on_change: Option<Callback<ev::Event>>`
- `on_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_pointer_down: Option<Callback<ev::PointerEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<Children>`

**OneTimePasswordFieldHiddenInput (HiddenInput):**
- `name: MaybeProp<String>`
- `node_ref: AnyNodeRef`

### Differences

1. **String union types vs Rust enums**: React uses string literal unions (`'alpha' | 'numeric' | ...`). Leptos uses Rust enums (`InputValidationType`, `AutoComplete`, `InputType`, `Orientation`). This is the expected idiomatic adaptation.
2. **Explicit event handler props on Input**: Leptos explicitly exposes `on_focus`, `on_cut`, `on_input`, `on_change`, `on_key_down`, `on_pointer_down`. In React these are inherited from HTML input props. The Leptos approach makes the composable handlers explicit.
3. **`index` type**: React uses `number | undefined`. Leptos uses `MaybeProp<usize>`. Functionally equivalent.
4. **`on_paste` on Root**: Explicitly declared in both React (inherited from div props, composed) and Leptos (explicit prop).
5. **HiddenInput simplification**: Leptos exposes only `name` and `node_ref`. React allows all non-conflicting `<input>` HTML props via its Omit type. The Leptos version is more restrictive but covers the primary use case.
6. **No re-export aliases**: React exports `Root`, `Input`, `HiddenInput` as aliases. Leptos does not define these.

### Assessment

Excellent alignment. All 3 exported components match. Every significant prop from the React API is present in Leptos. The enum types are well-designed Rust equivalents of the React string unions. The explicit event handler props are a necessary Leptos adaptation. The HiddenInput is slightly more restrictive but covers the intended usage.

---

## PasswordToggleField

### React API

**Exported components:** `PasswordToggleField` (alias `Root`), `PasswordToggleFieldInput` (alias `Input`), `PasswordToggleFieldToggle` (alias `Toggle`), `PasswordToggleFieldSlot` (alias `Slot`), `PasswordToggleFieldIcon` (alias `Icon`).

**PasswordToggleField (Root):**
- Not a DOM element (renders only children + context)
- `id?: string`
- `visible?: boolean`
- `defaultVisible?: boolean`
- `onVisiblityChange?: (visible: boolean) => void` (note: React source has typo `onVisiblity` -- missing `i`)
- `children?: ReactNode`

**PasswordToggleFieldInput (Input):**
- Renders `<input>` via `Primitive.input`
- `autoComplete?: 'current-password' | 'new-password'` (default `'current-password'`)
- `autoCapitalize?: string` (default `'off'`)
- `spellCheck?: boolean` (default `false`)
- `id?: string`
- Omits `type` from inherited HTML input props (controlled internally)
- Inherits remaining `<input>` HTML props
- `ref` forwarded

**PasswordToggleFieldToggle (Toggle):**
- Renders `<button type="button">` via `Primitive.button`
- `onClick?: (event: MouseEvent) => void`
- `onPointerDown?: (event: PointerEvent) => void`
- `onPointerCancel?: (event: PointerEvent) => void`
- `onPointerUp?: (event: PointerEvent) => void`
- `onFocus?: (event: FocusEvent) => void`
- `aria-label?: string`
- `aria-controls?: string`
- `aria-hidden?: boolean`
- `tabIndex?: number`
- `children?: ReactNode`
- Omits `type` from inherited button props
- Inherits remaining `<button>` HTML props
- `ref` forwarded

**PasswordToggleFieldSlot (Slot):**
- Renderless (conditional rendering)
- Either declarative: `{ visible: ReactNode; hidden: ReactNode }` OR render prop: `{ render: (args: { visible: boolean }) => ReactElement }`

**PasswordToggleFieldIcon (Icon):**
- Renders `<svg>` via `Primitive.svg` with `asChild`
- `visible: ReactElement` (required)
- `hidden: ReactElement` (required)
- Omits `children` from SVG props
- Inherits remaining `<svg>` HTML props
- `ref` forwarded

### Leptos API

**Exported components:** `PasswordToggleField`, `PasswordToggleFieldInput`, `PasswordToggleFieldToggle`, `PasswordToggleFieldSlot`, `PasswordToggleFieldIcon`.

**PasswordToggleField (Root):**
- `id: MaybeProp<String>`
- `visible: MaybeProp<bool>`
- `default_visible: MaybeProp<bool>`
- `on_visibility_change: Option<Callback<bool>>`
- `children: ChildrenFn`

**PasswordToggleFieldInput (Input):**
- `auto_complete: MaybeProp<AutoComplete>` (enum: `CurrentPassword`, `NewPassword`, default `CurrentPassword`)
- `id: MaybeProp<String>`
- `on_blur: Option<Callback<ev::FocusEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**PasswordToggleFieldToggle (Toggle):**
- `on_click: Option<Callback<ev::MouseEvent>>`
- `on_pointer_down: Option<Callback<ev::PointerEvent>>`
- `on_pointer_cancel: Option<Callback<ev::PointerEvent>>`
- `on_pointer_up: Option<Callback<ev::PointerEvent>>`
- `aria_label: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**PasswordToggleFieldSlot (Slot):**
- `render: Option<Callback<bool, AnyView>>`
- `visible_content: Option<ChildrenFn>`
- `hidden_content: Option<ChildrenFn>`

**PasswordToggleFieldIcon (Icon):**
- `visible_icon: ViewFn` (required)
- `hidden_icon: ViewFn` (required)
- `node_ref: AnyNodeRef`

### Differences

1. **Callback naming**: React has `onVisiblityChange` (typo in source). Leptos fixes the spelling to `on_visibility_change`. This is a correction, not a divergence.
2. **Missing Input props**: Leptos does not expose `auto_capitalize` or `spell_check` as explicit props. These are hardcoded internally (`autocapitalize="off"`, `spellcheck="false"`). In React they are props with defaults, allowing override. Minor gap -- users could still use `attr:autocapitalize` in Leptos.
3. **Missing Toggle props**: Leptos does not expose `on_focus`, `aria-controls`, `aria-hidden`, or `tab_index` as explicit props. In React, `aria-controls` and `aria-hidden` have special hydration logic. `on_focus` is destructured from props. In Leptos, `aria-controls` is computed internally and always provided (no hydration gate needed in CSR). `aria-hidden` and `tabIndex` are not exposed because they were only needed for pre-hydration SSR state in React.
4. **Slot prop naming**: React uses `{ visible, hidden }` (declarative) or `{ render }` (render prop). Leptos uses `visible_content`, `hidden_content`, `render`. The naming differs but the pattern is equivalent.
5. **Icon prop naming**: React uses `visible` and `hidden` for the two icon elements. Leptos uses `visible_icon` and `hidden_icon`. The types differ: React uses `ReactElement`, Leptos uses `ViewFn`.
6. **AutoComplete type**: React uses string union `'current-password' | 'new-password'`. Leptos uses `AutoComplete` enum with `CurrentPassword`, `NewPassword` variants.
7. **No re-export aliases**: React exports `Root`, `Input`, `Toggle`, `Slot`, `Icon` as aliases. Leptos does not define these.

### Assessment

Good alignment. All 5 exported components match. The core toggle behavior (visibility control, input type switching, cursor position restoration, form reset/submit handling, aria-label auto-detection) is fully implemented. The missing explicit props (`auto_capitalize`, `spell_check`, `on_focus` on Toggle, `aria-hidden`, `tab_index`) are minor since most are hardcoded to their React defaults or are unnecessary in CSR mode. The `on_visibility_change` spelling fix is an improvement over React.

---

## Portal (Internal)

### React API

**Exported components:** `Portal` (aliased as `Root`)

**Portal** (renders `Primitive.div` inside `ReactDOM.createPortal`)
- `container?: Element | DocumentFragment | null` — target element where portaled content is appended; defaults to `document.body`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

### Leptos API

**Exported components/functions:** `Portal`, `ScopedPortal`, `PortalContextValue`, `resolve_force_mount`

**Portal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional) — target element for portal
- `container_ref: AnyNodeRef` (optional) — alternative target via node ref
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ScopedPortal** (wraps `Portal` with context re-provision)
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional)
- `force_mount: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**PortalContextValue** (context struct)
- `force_mount: Signal<bool>`

**resolve_force_mount(prop: MaybeProp<bool>) -> Signal<bool>** — utility to extract force_mount from prop or portal context

### Differences

1. **Extra `ScopedPortal` component:** Leptos exports a `ScopedPortal` component not present in React. This is a Leptos-specific wrapper that provides `PortalContextValue` (with `force_mount`) through the portal boundary. React does not need this because React context propagates through portals automatically; Leptos portals (via `mount_to`) create new reactive owner trees, breaking context chains.
2. **Extra `container_ref` prop:** Leptos adds a `container_ref: AnyNodeRef` prop as an alternative to `container` for specifying the mount target. React only has `container`.
3. **Extra `force_mount` infrastructure:** `PortalContextValue`, `resolve_force_mount()`, and the `force_mount` prop on `ScopedPortal` have no React equivalent. In React, `forceMount` is handled per-component. The Leptos approach centralizes it at the portal level.
4. **`container` type:** React accepts `Element | DocumentFragment | null`. Leptos accepts `SendWrapper<web_sys::Element>` (no `DocumentFragment` support).
5. **No `DocumentFragment` support:** React supports portaling into `DocumentFragment`; Leptos does not.
6. **Custom `LeptosPortal` implementation:** Leptos includes a custom portal implementation (internal module `leptos_portal`) based on `leptos::Portal` with reactive mount target support. React uses `ReactDOM.createPortal`.

### Assessment

The core `Portal` component matches the React API semantically. The extra `ScopedPortal`, `PortalContextValue`, and `resolve_force_mount` are necessary Leptos-specific infrastructure to work around the fact that Leptos portals break context chains (unlike React portals). These are well-designed internal utilities that consuming components depend on. The lack of `DocumentFragment` support is a minor gap.

---

## Presence (Internal)

### React API

**Exported components:** `Presence` (aliased as `Root`)

**Presence** (render prop component, not a DOM element)
- `present: boolean` (required) — controls mount/unmount with animation support
- `children: ReactElement | ((props: { present: boolean }) => ReactElement)` — either a single element or render function receiving `{ present }`

**usePresence(present: boolean)** — internal hook (not exported)
- Returns `{ isPresent: boolean, ref: (node: HTMLElement) => void }`
- Manages animation lifecycle via state machine (mounted/unmountSuspended/unmounted)
- Monitors `animationstart`, `animationcancel`, `animationend` events

### Leptos API

**Exported components/functions:** `Presence`, `use_presence`

**Presence** (component wrapping `<Show>`)
- `present: Signal<bool>` (required) — reactive signal controlling visibility
- `node_ref: AnyNodeRef` (optional) — ref to the animated element
- `children: ChildrenFn` (required) — rendered when present

**use_presence(present: Signal<bool>, node_ref: AnyNodeRef) -> Signal<bool>** — public hook
- Returns a `Signal<bool>` indicating whether the element should be rendered
- Same state machine logic as React (mounted/unmountSuspended/unmounted)

### Differences

1. **`present` type:** React uses `boolean`; Leptos uses `Signal<bool>` (reactive).
2. **No render function pattern:** React's `Presence` supports `children` as a render function `((props: { present: boolean }) => ReactElement)` that receives `{ present }` to enable force-mount behavior. Leptos does not support this pattern; force-mount is handled separately via the portal context.
3. **`use_presence` is public:** Leptos exports `use_presence` as a public function. React's `usePresence` is internal to the module.
4. **`node_ref` is a prop in Leptos:** React's `Presence` does not accept a ref prop; instead, `usePresence` returns a ref callback that must be composed onto the child. Leptos takes `node_ref` as a prop and uses it directly.
5. **Return type:** React's `usePresence` returns `{ isPresent: boolean, ref }`. Leptos's `use_presence` returns `Signal<bool>` (just the is_present state), since the ref is passed in rather than returned.
6. **CSS.escape usage:** React uses `CSS.escape(event.animationName)` to compare animation names safely. Leptos uses a simple `String::contains` comparison without escaping.

### Assessment

Good functional alignment. The core animation lifecycle state machine is faithfully ported. The key architectural difference is that Leptos passes `node_ref` in (as a prop) while React returns a ref from the hook. This is an idiomatic Leptos adaptation. The missing render function pattern is compensated by the separate `force_mount` infrastructure in the portal module. The public `use_presence` export is useful for consuming components.

---

## Primitive (Internal)

### React API

**Exported:** `Primitive` (object with element-keyed components), `dispatchDiscreteCustomEvent`, `PrimitivePropsWithRef` type

**Primitive** — An object mapping HTML element names to forwarded-ref components:
- Keys: `a`, `button`, `div`, `form`, `h2`, `h3`, `img`, `input`, `label`, `li`, `nav`, `ol`, `p`, `select`, `span`, `svg`, `ul`
- Each entry: `React.forwardRef` component accepting `{ asChild?: boolean, ...elementProps }`
- When `asChild` is true, renders a `Slot` instead of the native element

**dispatchDiscreteCustomEvent(target, event)** — utility to flush React's batched event updates for custom events dispatched from discrete event handlers

### Leptos API

**Exported components/functions:** `Primitive`, `VoidPrimitive`, `wrap_callback`, `open_closed_state`, `prop_or`, `prop_or_default`, `data_attr`, `compose_callbacks`

**Primitive** (generic component)
- `element: fn() -> HtmlElement<E, (), ()>` (required) — element constructor
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**VoidPrimitive** (generic component for void elements like `input`)
- Same props as `Primitive` but does not add children when `as_child` is false

**Utility functions:**
- `wrap_callback(cb: Option<Callback<T>>) -> Callback<T>` — converts Option to no-op callback
- `open_closed_state(open: bool) -> &'static str` — returns `"open"` or `"closed"`
- `prop_or(prop: MaybeProp<T>, default: T) -> Signal<T>` — prop with default value
- `prop_or_default(prop: MaybeProp<T>) -> Signal<T>` — prop with T::default()
- `data_attr(signal: Signal<bool>) -> impl Fn() -> Option<&'static str>` — boolean to data attribute
- `compose_callbacks(original, our, check_default_prevented) -> impl Fn(E)` — event handler composition

### Differences

1. **Generic vs element-keyed:** React's `Primitive` is a record of 17 named components (`Primitive.div`, `Primitive.button`, etc.). Leptos's `Primitive` is a single generic component parameterized by `element`. This is a cleaner Rust/Leptos approach.
2. **`VoidPrimitive`:** Leptos adds `VoidPrimitive` for void HTML elements (e.g., `<input>`). React does not distinguish because JSX allows self-closing any element.
3. **Missing `dispatchDiscreteCustomEvent`:** This React utility handles React's batched event system. Leptos does not need this because it does not batch events the way React 18+ does.
4. **Extra utility functions:** Leptos exports `wrap_callback`, `open_closed_state`, `prop_or`, `prop_or_default`, `data_attr`, and `compose_callbacks`. These are Leptos-specific utilities shared across components. React's equivalent of `compose_callbacks` lives in `@radix-ui/primitive` as `composeEventHandlers` (a separate package), not in the primitive module.
5. **No `Slot` implementation:** React's `Primitive` uses `createSlot` from `@radix-ui/react-slot` for `asChild` behavior. Leptos's `Primitive` uses `add_any_attr(any_node_ref(...))` to forward the ref to children when `as_child` is true.

### Assessment

Good conceptual alignment. The core purpose -- rendering either a native element or forwarding to a child via `asChild` -- is preserved. The generic approach is more idiomatic for Rust. The extra utility functions are practical helpers used throughout the codebase. The absence of `dispatchDiscreteCustomEvent` is correct since Leptos does not have React's event batching behavior.

---

## Direction (Internal)

### React API

**Exported:** `DirectionProvider` (aliased as `Provider`), `useDirection`

**DirectionProvider** (renders React context provider)
- `dir: Direction` (required) — `'ltr' | 'rtl'`
- `children?: React.ReactNode`

**useDirection(localDir?: Direction) -> Direction** — hook
- Returns `localDir || globalDir || 'ltr'`

### Leptos API

**Exported:** `Direction` (enum), `DirectionProvider`, `use_direction`

**Direction** (enum with `Ltr`, `Rtl` variants)
- Implements `Display`, `AttributeValue` (for use in Leptos attributes)

**DirectionProvider** (component)
- `direction: Signal<Direction>` (required, `#[prop(into)]`)
- `children: Children` (required)

**use_direction(local_dir: MaybeProp<Direction>) -> Signal<Direction>** — hook
- Returns a reactive signal with priority: local_dir > global context > `Direction::Ltr`

### Differences

1. **`Direction` type exported:** Leptos exports the `Direction` enum as part of this module. React defines it as a type alias `'ltr' | 'rtl'` (just a union of string literals, not separately exported from the direction package).
2. **`direction` prop type:** React's `DirectionProvider` takes `dir: Direction` (plain value). Leptos takes `direction: Signal<Direction>` (reactive signal). The prop name also differs: `dir` vs `direction`.
3. **`AttributeValue` implementation:** Leptos's `Direction` implements `AttributeValue` for direct use in Leptos view attributes. This is Leptos-specific infrastructure.
4. **Return type:** React's `useDirection` returns a plain `Direction` string. Leptos's `use_direction` returns `Signal<Direction>`.
5. **`local_dir` parameter type:** React takes `Direction | undefined`. Leptos takes `MaybeProp<Direction>` (reactive optional).

### Assessment

Excellent alignment. The API is functionally equivalent. The reactive types (`Signal`, `MaybeProp`) in Leptos are idiomatic adaptations. The `Direction` enum with `Display` and `AttributeValue` implementations is clean Rust design. The only naming difference (`dir` vs `direction`) is minor.

---

## DismissableLayer (Internal)

### React API

**Exported components:** `DismissableLayer` (aliased as `Root`), `DismissableLayerBranch` (aliased as `Branch`)
**Exported types:** `DismissableLayerProps`

**DismissableLayer** (renders `Primitive.div`)
- `disableOutsidePointerEvents?: boolean` (default `false`) — blocks outside pointer interactions
- `onEscapeKeyDown?: (event: KeyboardEvent) => void` — escape key handler
- `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void` — pointer-down-outside handler
- `onFocusOutside?: (event: FocusOutsideEvent) => void` — focus-outside handler
- `onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void` — any outside interaction handler
- `onDismiss?: () => void` — called when layer should dismiss
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**DismissableLayerBranch** (renders `Primitive.div`)
- No component-specific props
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**Custom event types:**
- `PointerDownOutsideEvent = CustomEvent<{ originalEvent: PointerEvent }>`
- `FocusOutsideEvent = CustomEvent<{ originalEvent: FocusEvent }>`

**Context:** React uses a `DismissableLayerContext` with `layers`, `layersWithOutsidePointerEventsDisabled`, and `branches` Sets.

### Leptos API

**Exported components/types:** `DismissableLayer`, `DismissableLayerBranch`, `PointerDownOutsideEvent`, `FocusOutsideEvent`

**DismissableLayer** (renders `Primitive.div`)
- `disable_outside_pointer_events: MaybeProp<bool>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `on_pointer_down_outside: Option<Callback<PointerDownOutsideEvent>>` (optional)
- `on_focus_outside: Option<Callback<FocusOutsideEvent>>` (optional)
- `on_interact_outside: Option<Callback<CustomEvent>>` (optional)
- `on_dismiss: Option<Callback<()>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**DismissableLayerBranch** (renders `Primitive.div`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**Event types:**
- `PointerDownOutsideEvent = CustomEvent` (type alias to `web_sys::CustomEvent`)
- `FocusOutsideEvent = CustomEvent` (type alias to `web_sys::CustomEvent`)

### Differences

1. **Context implementation:** React uses a React context with Sets. Leptos uses a global `static Lazy<Mutex<DismissableLayerContextValue>>`. This is because Leptos portals break reactive context chains, and DismissableLayer must track layers globally across portal boundaries.
2. **Extra `higher_layers_contain` method:** Leptos adds this to the context to handle the fact that Leptos DOM events do not bubble through the component tree like React synthetic events do through portals.
3. **Event type simplification:** React's custom events are `CustomEvent<{ originalEvent: PointerEvent }>` / `CustomEvent<{ originalEvent: FocusEvent }>`. Leptos simplifies both to `web_sys::CustomEvent`, losing the typed `detail` field.
4. **`on_interact_outside` event type:** React types this as `PointerDownOutsideEvent | FocusOutsideEvent`. Leptos uses `CustomEvent` (the base type).
5. **Two-escape UX pattern:** Leptos implements the text-input escape pattern (first Escape blurs text input, second Escape dismisses) via `is_text_input()` in the escape handler. React does not have this -- it was added as a Leptos-specific UX enhancement per Rule 8.

### Assessment

Excellent functional alignment. All props match 1:1. The global Mutex context is a necessary Leptos adaptation for cross-portal layer tracking. The two-escape UX enhancement is a documented addition. The event type simplification is pragmatic -- web_sys CustomEvent covers the use case without the typed detail wrapper.

---

## FocusScope (Internal)

### React API

**Exported components:** `FocusScope` (aliased as `Root`)
**Exported types:** `FocusScopeProps`

**FocusScope** (renders `Primitive.div` with `tabIndex={-1}`)
- `loop?: boolean` (default `false`) — wrap tab focus at edges
- `trapped?: boolean` (default `false`) — trap focus within scope
- `onMountAutoFocus?: (event: Event) => void` — auto-focus on mount callback
- `onUnmountAutoFocus?: (event: Event) => void` — auto-focus on unmount callback
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `FocusScope`

**FocusScope** (renders `Primitive.div` with `tabindex="-1"`)
- `r#loop: MaybeProp<bool>` (optional)
- `trapped: MaybeProp<bool>` (optional)
- `on_mount_auto_focus: Option<Callback<Event>>` (optional)
- `on_unmount_auto_focus: Option<Option<Callback<Event>>>` (optional) — double-wrapped Option
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Double-wrapped `on_unmount_auto_focus`:** Leptos uses `Option<Option<Callback<Event>>>`. The outer `Option` indicates whether the prop was passed; the inner `Option` allows explicitly passing `None` to disable the default unmount auto-focus behavior. React uses a simple `(event: Event) => void` where calling `event.preventDefault()` in the handler suppresses default behavior. The Leptos double-wrap is annotated with `// TODO: hopefully remove the double option`.
2. **`loop` keyword escaping:** Leptos uses `r#loop` due to `loop` being a Rust reserved keyword. React uses `loop` directly.
3. **Focus scope stack:** Both use a global stack. React uses a module-level closure-based approach. Leptos uses `static Lazy<Mutex<FocusScopeStack>>` with `AtomicBool`-based `FocusScopeAPI` structs.
4. **Auto-focus timing:** Leptos defers auto-focus to `requestAnimationFrame` to emulate React's `useEffect` timing. This is documented as necessary because Leptos Effects run in creation order (parent first) while React effects run children-first.

### Assessment

Good alignment. All four React props are present. The double-wrapped `on_unmount_auto_focus` is a known issue marked with a TODO. The `r#loop` naming is a necessary Rust adaptation. The auto-focus timing adjustment is a well-documented Leptos-specific fix for effect ordering differences.

---

## FocusGuards (Internal)

### React API

**Exported:** `FocusGuards` (aliased as `Root`), `useFocusGuards`

**FocusGuards** (wrapper component)
- `children?: React.ReactNode`
- Calls `useFocusGuards()` internally

**useFocusGuards()** — hook
- Injects a pair of focus guard `<span>` elements at the edges of `document.body`
- Reference-counted: guards are only removed when the last consumer unmounts
- Guards have `data-radix-focus-guard` attribute, `tabIndex=0`, and are visually hidden

### Leptos API

**Exported:** `FocusGuards`, `use_focus_guards`

**FocusGuards** (component)
- `children: ChildrenFn` (required)
- Calls `use_focus_guards()` internally

**use_focus_guards()** — function
- Same behavior: injects/removes focus guard spans at body edges
- Uses `AtomicU64` counter for reference counting

### Differences

1. **`children` type:** React uses `React.ReactNode` (optional). Leptos uses `ChildrenFn` (required).
2. **Counter type:** React uses a module-level `let count = 0`. Leptos uses `static COUNT: AtomicU64` (thread-safe atomic).
3. **No functional differences:** Both implementations create identical focus guard spans with the same attributes and behavior.

### Assessment

Excellent alignment. The API and behavior are functionally identical. The `AtomicU64` counter is a correct Rust adaptation of the module-level counter pattern.

---

## RovingFocus (Internal)

### React API

**Exported:** `createRovingFocusGroupScope`, `RovingFocusGroup` (aliased as `Root`), `RovingFocusGroupItem` (aliased as `Item`)
**Exported types:** `RovingFocusGroupProps`, `RovingFocusItemProps`

**RovingFocusGroup** (renders `Primitive.div` via `RovingFocusGroupImpl`, wrapped in `Collection.Provider` + `Collection.Slot`)
- `orientation?: Orientation` (`'horizontal' | 'vertical' | undefined`)
- `dir?: Direction` (`'ltr' | 'rtl'`)
- `loop?: boolean` (default `false`) — wrap keyboard navigation
- `currentTabStopId?: string | null` — controlled active tab stop
- `defaultCurrentTabStopId?: string` — uncontrolled default
- `onCurrentTabStopIdChange?: (tabStopId: string | null) => void`
- `onEntryFocus?: (event: Event) => void` — called when group receives keyboard focus
- `preventScrollOnEntryFocus?: boolean` (default `false`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps` (minus `dir`)
- `ref` (forwarded)

**RovingFocusGroupItem** (renders `Primitive.span` wrapped in `Collection.ItemSlot`)
- `tabStopId?: string` — custom tab stop ID (defaults to auto-generated ID)
- `focusable?: boolean` (default `true`)
- `active?: boolean` (default `false`) — marks this item as the "active" one
- `children?: React.ReactNode | ((props: { hasTabStop: boolean, isCurrentTabStop: boolean }) => React.ReactNode)` — supports render function
- Inherits all `span` HTML attributes via `PrimitiveSpanProps` (minus `children`)
- `ref` (forwarded)

### Leptos API

**Exported:** `RovingFocusGroup`, `RovingFocusGroupItem`, `Orientation`, `RovingFocusGroupContext`, `RovingFocusGroupItemContext`

**RovingFocusGroup** (renders `Primitive.div` via `RovingFocusGroupImpl`, wrapped in `CollectionProvider` + `CollectionSlot`)
- `orientation: MaybeProp<Orientation>` (optional)
- `dir: MaybeProp<Direction>` (optional)
- `r#loop: MaybeProp<bool>` (optional)
- `current_tab_stop_id: MaybeProp<String>` (optional)
- `default_current_tab_stop_id: MaybeProp<String>` (optional)
- `on_current_tab_stop_id_change: Option<Callback<Option<String>>>` (optional)
- `on_entry_focus: Option<Callback<ev::Event>>` (optional)
- `prevent_scroll_on_entry_focus: MaybeProp<bool>` (optional)
- `on_mouse_down: Option<Callback<ev::MouseEvent>>` (optional)
- `on_focus: Option<Callback<ev::FocusEvent>>` (optional)
- `on_blur: Option<Callback<ev::FocusEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**RovingFocusGroupItem** (renders `Primitive.span` wrapped in `CollectionItemSlot`)
- `tab_stop_id: MaybeProp<String>` (optional)
- `focusable: MaybeProp<bool>` (optional)
- `active: MaybeProp<bool>` (optional)
- `on_mouse_down: Option<Callback<ev::MouseEvent>>` (optional)
- `on_focus: Option<Callback<ev::FocusEvent>>` (optional)
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**RovingFocusGroupContext** (public context struct)
- `has_tab_stop: Signal<bool>`

**RovingFocusGroupItemContext** (public context struct)
- `is_current_tab_stop: Signal<bool>`

### Differences

1. **Explicit event handler props:** Leptos declares `on_mouse_down`, `on_focus`, `on_blur` on `RovingFocusGroup` and `on_mouse_down`, `on_focus`, `on_key_down` on `RovingFocusGroupItem`. React gets these via inherited `PrimitiveDivProps`/`PrimitiveSpanProps`.
2. **Missing render function on Item:** React's `RovingFocusGroupItem` supports `children` as a render function `((props: { hasTabStop, isCurrentTabStop }) => React.ReactNode)`. Leptos does not support this. Instead, Leptos exports `RovingFocusGroupContext` and `RovingFocusGroupItemContext` as public context structs that consuming components can read via `use_context`.
3. **Missing `createRovingFocusGroupScope`:** React exports a scope factory function for context composition. Leptos does not need this -- it uses Leptos context providers directly.
4. **`Orientation` type location:** Leptos exports `Orientation` from this module. React defines `Orientation` as a type alias for `React.AriaAttributes['aria-orientation']`.
5. **`preventScrollOnEntryFocus` partially implemented:** Leptos passes this to `focus_first` but the `focus_first` function has a `// TODO: focus options with prevent_scroll` comment -- the value is accepted but not yet used.
6. **Extra `items_initialized` workaround:** Leptos adds an `items_initialized` RwSignal to handle timing differences between React's synchronous effects and Leptos's async effects. This prevents tabindex from being `-1` before items register.
7. **Focus deferred to microtask:** Leptos defers focus updates on item focus to `queueMicrotask` to avoid WASM closure panics from reactive updates during synchronous focus events. React uses `setTimeout` only in the keydown handler.

### Assessment

Good alignment. All React props are present. The missing render function on `RovingFocusGroupItem` is compensated by the exported context structs. The explicit event handler props are needed because Leptos cannot inherit HTML attributes through the type system. The `preventScrollOnEntryFocus` TODO is a minor gap. The timing workarounds (items_initialized, microtask deferral) are well-documented Leptos-specific adaptations.

---

## Popper (Internal)

### React API

**Exported:** `createPopperScope`, `Popper` (aliased as `Root`), `PopperAnchor` (aliased as `Anchor`), `PopperContent` (aliased as `Content`), `PopperArrow` (aliased as `Arrow`), `SIDE_OPTIONS`, `ALIGN_OPTIONS`
**Exported types:** `PopperProps`, `PopperAnchorProps`, `PopperContentProps`, `PopperArrowProps`

**Popper** (provider component, no DOM output)
- `children?: React.ReactNode`

**PopperAnchor** (renders `Primitive.div` or nothing if `virtualRef` is set)
- `virtualRef?: React.RefObject<Measurable>` — anchor to a virtual element
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**PopperContent** (renders wrapper `<div>` + inner `Primitive.div`)
- `side?: Side` (default `'bottom'`)
- `sideOffset?: number` (default `0`)
- `align?: Align` (default `'center'`)
- `alignOffset?: number` (default `0`)
- `arrowPadding?: number` (default `0`)
- `avoidCollisions?: boolean` (default `true`)
- `collisionBoundary?: Boundary | Boundary[]` (default `[]`)
- `collisionPadding?: number | Partial<Record<Side, number>>` (default `0`)
- `sticky?: 'partial' | 'always'` (default `'partial'`)
- `hideWhenDetached?: boolean` (default `false`)
- `updatePositionStrategy?: 'optimized' | 'always'` (default `'optimized'`)
- `onPlaced?: () => void`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**PopperArrow** (renders wrapper `<span>` + `ArrowPrimitive.Root`)
- Inherits all Arrow primitive props (width, height, etc.)
- `ref` (forwarded)

### Leptos API

**Exported:** `Popper`, `PopperAnchor`, `PopperContent`, `PopperArrow`, `PopperScope`, `use_popper_scope`, `provide_popper_scope`, `set_popper_virtual_ref`, `Align`, `Sticky`, `UpdatePositionStrategy`, `PopperVirtualElement`, `ClientRectObject`, `Padding`, `Side`
Re-exported from floating-ui-leptos: `ClientRectObject`, `Padding`, `Side`, `VirtualElement as PopperVirtualElement`

**Popper** (provider component)
- `anchor_ref: Option<AnyNodeRef>` (optional) — externally-managed anchor ref
- `children: ChildrenFn` (required)

**PopperAnchor** (renders `Primitive.div`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**PopperContent** (renders wrapper `<div>` + inner `Primitive.div`)
- `side: Signal<Side>` (default `Side::Bottom`)
- `side_offset: Signal<f64>` (default `0.0`)
- `align: Signal<Align>` (default `Align::Center`)
- `align_offset: Signal<f64>` (default `0.0`)
- `arrow_padding: Signal<f64>` (default `0.0`)
- `avoid_collisions: Signal<bool>` (default `true`)
- `collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>` (default `vec![]`)
- `collision_padding: Signal<Padding>` (default `Padding::All(0.0)`)
- `sticky: Signal<Sticky>` (default `Sticky::Partial`)
- `hide_when_detached: Signal<bool>` (default `false`)
- `update_position_strategy: Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`)
- `on_placed: MaybeCallback<()>` (optional)
- `dir: MaybeProp<String>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**PopperArrow** (renders wrapper `<span>` + `ArrowPrimitive`)
- `width: MaybeProp<f64>` (optional)
- `height: MaybeProp<f64>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Extra `anchor_ref` prop on Popper:** Leptos adds an optional `anchor_ref` to `Popper` for parent components (like Menu) to set the anchor ref directly. React does not have this -- it always creates its own ref internally.
2. **Missing `virtualRef` on PopperAnchor:** React's `PopperAnchor` accepts `virtualRef`. Leptos handles virtual anchoring via the standalone `set_popper_virtual_ref()` function instead.
3. **Extra scope utilities:** Leptos exports `PopperScope`, `use_popper_scope()`, `provide_popper_scope()` for re-providing context across portal boundaries. React uses `createPopperScope` / `createContextScope` for scope composition.
4. **Missing `SIDE_OPTIONS` and `ALIGN_OPTIONS`:** React exports these as const arrays. Leptos does not export equivalent constants.
5. **Prop types use `Signal<T>` with defaults:** Leptos uses `Signal<T>` for most props (not `MaybeProp<T>`), with default values provided via `#[prop(default = ...)]`. React uses optional props with defaults destructured in the component body.
6. **`collision_boundary` type:** React accepts `Boundary | Boundary[]` where `Boundary = Element | null`. Leptos uses `Signal<SendWrapper<Vec<web_sys::Element>>>`.
7. **`collision_padding` type:** React accepts `number | Partial<Record<Side, number>>`. Leptos uses `Signal<Padding>` where `Padding` is a floating-ui-leptos enum.
8. **`dir` type:** React types this as inherited from `PrimitiveDivProps`. Leptos uses `MaybeProp<String>`.
9. **Extra type re-exports:** Leptos re-exports `Side`, `Padding`, `ClientRectObject`, `PopperVirtualElement` from floating-ui-leptos for consumer convenience.
10. **Style application workarounds:** Leptos uses Effects to apply positioning styles and transfer wrapper attributes to the inner element. React uses simple prop spreading. This is a Leptos-specific workaround for how attribute propagation works differently.

### Assessment

Good functional alignment. All four React components are present. All PopperContent positioning props are present with matching defaults. The main API differences are the virtual anchor approach (prop vs function), the scope utilities (needed for portal context), and the type differences for collision boundaries and padding. The `LimitShift` cross_axis workaround is well-documented.

---

## Arrow (Internal)

### React API

**Exported components:** `Arrow` (aliased as `Root`)
**Exported types:** `ArrowProps`

**Arrow** (renders `Primitive.svg`)
- `width?: number` (default `10`)
- `height?: number` (default `5`)
- `children?: React.ReactNode` — when `asChild` is true, uses children to replace the SVG
- Inherits all `svg` HTML attributes via `PrimitiveSvgProps`
- `ref` (forwarded)
- Renders `<polygon points="0,0 30,0 15,10" />` when not `asChild`
- `viewBox="0 0 30 10"`, `preserveAspectRatio="none"`

### Leptos API

**Exported components:** `Arrow`

**Arrow** (renders `Primitive` with `svg::svg`)
- `width: MaybeProp<f64>` (optional, default `10.0`)
- `height: MaybeProp<f64>` (optional, default `5.0`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **`width`/`height` types:** React uses `number`. Leptos uses `MaybeProp<f64>`.
2. **SVG rendering:** Both render the same default polygon (`0,0 30,0 15,10`) with `viewBox="0 0 30 10"` and `preserveAspectRatio="none"`.
3. **`asChild` behavior:** Both show children instead of the default polygon when `asChild` is true.
4. **`children` optionality:** React takes `children` as standard React children (optional via ReactNode). Leptos uses `Option<ChildrenFn>`.

### Assessment

Excellent alignment. The component is simple and maps 1:1 between React and Leptos. The SVG attributes and default polygon are identical.

---

## Collection (Internal)

### React API

**Exported:** `createCollection` factory function (legacy version is the stable export; new version exported as `unstable_createCollection`)

**createCollection\<ItemElement, ItemData\>(name)** returns `[{ Provider, Slot, ItemSlot }, useCollection, createCollectionScope]`:

**Provider** (provider component)
- `scope: any`
- `children?: React.ReactNode`
- Manages `collectionRef` and `itemMap`

**Slot** (renders a Slot element)
- `scope: any`
- `children?: React.ReactNode`
- Composes `collectionRef` from context

**ItemSlot** (renders a Slot element with `data-radix-collection-item`)
- `scope: any`
- `children: React.ReactNode`
- Spreads `ItemData` props
- Registers/unregisters item in `itemMap`

**useCollection(scope)** — hook returning `getItems()` function
- Returns items sorted by DOM order via `querySelectorAll('[data-radix-collection-item]')`

### Leptos API

**Exported:** `CollectionProvider`, `CollectionSlot`, `CollectionItemSlot`, `use_collection`, `CollectionItemValue`, `CollectionScope`, `use_collection_scope`, `provide_collection_scope`

**CollectionProvider\<ItemData\>** (component)
- `item_data_type: Option<PhantomData<ItemData>>` (optional) — type witness
- `children: ChildrenFn` (required)

**CollectionSlot\<ItemData\>** (component)
- `item_data_type: Option<PhantomData<ItemData>>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**CollectionItemSlot\<ItemData\>** (component)
- `item_data_type: Option<PhantomData<ItemData>>` (optional)
- `item_data: MaybeProp<ItemData>` (optional) — item-specific data
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**use_collection\<ItemData\>()** — returns a boxed closure `() -> Vec<CollectionItemValue<ItemData>>`

**CollectionItemValue\<ItemData\>** (public struct)
- `r#ref: AnyNodeRef`
- `data: ItemData`

**CollectionScope\<ItemData\>** — opaque handle for re-providing collection context across portals
**use_collection_scope()** / **provide_collection_scope()** — scope capture/re-provide utilities

### Differences

1. **No factory function:** React uses `createCollection()` to create a parameterized set of components. Leptos uses generic components parameterized by `ItemData` type with `PhantomData` as a type witness.
2. **`item_data_type` prop:** Leptos uses `PhantomData<ItemData>` as a prop to specify the generic type parameter. React passes `ItemData` as a type parameter to `createCollection`.
3. **`item_data` prop vs spread:** React's `ItemSlot` receives item data via prop spreading (the `ItemData` fields are spread as props). Leptos takes item data as a single `item_data: MaybeProp<ItemData>` prop.
4. **Extra scope utilities:** Leptos exports `CollectionScope`, `use_collection_scope`, `provide_collection_scope` for portal context re-provision. React uses `createCollectionScope` for scope composition.
5. **Missing `scope` prop:** React components take a `scope` prop for context isolation. Leptos uses Leptos's built-in context system directly.
6. **`CollectionItemValue` exposed:** Leptos exports the item value struct. React returns items from `useCollection` with `{ ref, ...itemData }` shape.
7. **Item storage:** React uses `Map<RefObject, ItemData>`. Leptos uses `HashMap<CollectionItemId, CollectionItemValue>` with nanoid-generated IDs.
8. **`createCollectionScope` not exported:** React exports this for scope composition. Leptos does not need it.

### Assessment

Good alignment with appropriate Rust/Leptos adaptations. The core pattern is the same: a provider manages a collection of items, a slot captures the collection root, item slots register items, and a hook returns items in DOM order. The generic component approach with `PhantomData` is idiomatic Rust. The scope utilities handle the portal context problem that React's context composition handles automatically.

---

## ScrollLock (Internal)

### React API

There is **no `scroll-lock` package** in `reference/react-radix-primitives/packages/react/`. Scroll locking in React Radix is handled inline within components (e.g., Dialog uses `react-remove-scroll` library or inline body style manipulation).

### Leptos API

**Exported:** `use_body_scroll_lock`

**use_body_scroll_lock()** — function (hook-like)
- Sets `overflow: hidden` on `<body>` when the calling component is mounted
- Restores the original `overflow` value on cleanup
- Uses `RwSignal<Option<String>>` to track the original value

### Differences

1. **Leptos-only module:** This module has no React counterpart as a separate package. React components handle scroll locking via `react-remove-scroll` or inline CSS manipulation within individual components.
2. **Simple implementation:** Leptos's `use_body_scroll_lock` is a minimal utility that only sets `overflow: hidden`. React's `react-remove-scroll` is significantly more sophisticated (handles scroll bars, touch events, scroll chaining, etc.).

### Assessment

This is a Leptos-specific utility module that provides basic scroll locking functionality. It is much simpler than what React achieves via `react-remove-scroll`. The basic approach (setting `overflow: hidden` on body) works for common cases but lacks advanced features like scroll bar compensation, touch scroll prevention, or nested scroll container support.

---

## Menu

### React API

**Exported components:** `Menu` (alias `Root`), `MenuAnchor` (alias `Anchor`), `MenuPortal` (alias `Portal`), `MenuContent` (alias `Content`), `MenuGroup` (alias `Group`), `MenuLabel` (alias `Label`), `MenuItem` (alias `Item`), `MenuCheckboxItem` (alias `CheckboxItem`), `MenuRadioGroup` (alias `RadioGroup`), `MenuRadioItem` (alias `RadioItem`), `MenuItemIndicator` (alias `ItemIndicator`), `MenuSeparator` (alias `Separator`), `MenuArrow` (alias `Arrow`), `MenuSub` (alias `Sub`), `MenuSubTrigger` (alias `SubTrigger`), `MenuSubContent` (alias `SubContent`).

**Menu (Root):**
- Not a DOM element (context-only wrapper)
- `children?: ReactNode`
- `open?: boolean` (default `false`)
- `onOpenChange?(open: boolean): void`
- `dir?: 'ltr' | 'rtl'`
- `modal?: boolean` (default `true`)

**MenuAnchor:**
- Renders via `PopperPrimitive.Anchor`
- Inherits all Popper anchor props
- `ref` forwarded

**MenuPortal:**
- Not a DOM element (portal wrapper)
- `children?: ReactNode`
- `container?: Element` (from Portal)
- `forceMount?: true`

**MenuContent:**
- `forceMount?: true`
- Inherits `MenuRootContentTypeProps` which extends `MenuContentImplProps` (minus private props)
- From `MenuContentImplProps`:
  - `onCloseAutoFocus?: (event: Event) => void`
  - `loop?: boolean` (default `false`)
  - `onEntryFocus?: (event: Event) => void`
  - `onEscapeKeyDown?: (event: KeyboardEvent) => void`
  - `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void`
  - `onFocusOutside?: (event: FocusOutsideEvent) => void`
  - `onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void`
- Inherits `PopperContentProps` (minus `dir`, `onPlaced`):
  - `side?: 'top' | 'right' | 'bottom' | 'left'` (default `'bottom'`)
  - `sideOffset?: number` (default `0`)
  - `align?: 'start' | 'center' | 'end'` (default `'center'`)
  - `alignOffset?: number` (default `0`)
  - `avoidCollisions?: boolean` (default `true`)
  - `collisionBoundary?: Element | null | Array<Element | null>`
  - `collisionPadding?: number | Partial<Record<Side, number>>`
  - `arrowPadding?: number`
  - `sticky?: 'partial' | 'always'`
  - `hideWhenDetached?: boolean`
- Inherits all `<div>` HTML props
- `ref` forwarded

**MenuGroup:**
- Renders `<div>` via `Primitive.div`
- `role="group"`
- Inherits all `<div>` HTML props
- `ref` forwarded

**MenuLabel:**
- Renders `<div>` via `Primitive.div`
- Inherits all `<div>` HTML props
- `ref` forwarded

**MenuItem:**
- Extends `MenuItemImplProps` (minus `onSelect`)
  - `disabled?: boolean` (default `false`)
  - `textValue?: string`
  - Inherits all `<div>` HTML props (`PrimitiveDivProps`)
- `onSelect?: (event: Event) => void`
- `ref` forwarded

**MenuCheckboxItem:**
- Extends `MenuItemProps`
- `checked?: CheckedState` (type: `boolean | 'indeterminate'`, default `false`)
- `onCheckedChange?: (checked: boolean) => void`
- `ref` forwarded

**MenuRadioGroup:**
- Extends `MenuGroupProps`
- `value?: string`
- `onValueChange?: (value: string) => void`
- `ref` forwarded

**MenuRadioItem:**
- Extends `MenuItemProps`
- `value: string` (required)
- `ref` forwarded

**MenuItemIndicator:**
- Renders `<span>` via `Primitive.span`
- `forceMount?: true`
- Inherits all `<span>` HTML props
- `ref` forwarded

**MenuSeparator:**
- Renders `<div>` via `Primitive.div`
- `role="separator"`, `aria-orientation="horizontal"`
- Inherits all `<div>` HTML props
- `ref` forwarded

**MenuArrow:**
- Renders via `PopperPrimitive.Arrow`
- Inherits all Popper arrow props
- `ref` forwarded

**MenuSub:**
- Not a DOM element (context-only wrapper)
- `children?: ReactNode`
- `open?: boolean` (default `false`)
- `onOpenChange?(open: boolean): void`

**MenuSubTrigger:**
- Extends `MenuItemImplProps` (`disabled?: boolean`, `textValue?: string`, inherits `PrimitiveDivProps`)
- `ref` forwarded

**MenuSubContent:**
- `forceMount?: true`
- Omits from `MenuContentImplProps`: private props, `onCloseAutoFocus`, `onEntryFocus`, `side`, `align`
- Remaining from `MenuContentImplProps`:
  - `loop?: boolean`
  - `onEscapeKeyDown?: (event: KeyboardEvent) => void`
  - `onPointerDownOutside?: (event) => void`
  - `onFocusOutside?: (event) => void`
  - `onInteractOutside?: (event) => void`
- Remaining from `PopperContentProps`:
  - `sideOffset?: number`
  - `alignOffset?: number`
  - `avoidCollisions?: boolean`
  - `collisionBoundary`
  - `collisionPadding`
  - `arrowPadding`
  - `sticky`
  - `hideWhenDetached`
- Inherits all `<div>` HTML props
- `ref` forwarded

### Leptos API

**Exported components:** `Menu`, `MenuAnchor`, `MenuPortal`, `MenuContent`, `MenuGroup`, `MenuLabel`, `MenuItem`, `MenuCheckboxItem`, `MenuRadioGroup`, `MenuRadioItem`, `MenuItemIndicator`, `MenuSeparator`, `MenuArrow`, `MenuSub`, `MenuSubTrigger`, `MenuSubContent`.

**Menu (Root):**
- `open: MaybeProp<bool>`
- `dir: MaybeProp<Direction>`
- `modal: MaybeProp<bool>` (default `true`)
- `on_open_change: Option<Callback<bool>>`
- `children: ChildrenFn`

**MenuAnchor:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuPortal:**
- `container: MaybeProp<SendWrapper<web_sys::Element>>`
- `container_ref: AnyNodeRef` (Extra)
- `force_mount: MaybeProp<bool>`
- `children: ChildrenFn`

**MenuContent:**
- `force_mount: MaybeProp<bool>`
- `class: MaybeProp<String>` (Extra)
- `on_close_auto_focus: Option<Callback<ev::Event>>`
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_pointer_down_outside: Option<Callback<CustomEvent>>`
- `on_focus_outside: Option<Callback<CustomEvent>>`
- `on_interact_outside: Option<Callback<CustomEvent>>`
- `on_entry_focus: Option<Callback<ev::Event>>`
- `on_key_down: Option<Callback<ev::KeyboardEvent>>`
- `side: MaybeProp<PopperSide>`
- `side_offset: MaybeProp<f64>`
- `align: MaybeProp<Align>`
- `align_offset: MaybeProp<f64>`
- `avoid_collisions: MaybeProp<bool>`
- `id: MaybeProp<String>` (Extra -- used by wrapper components to set content element id)
- `aria_labelledby: MaybeProp<String>` (Extra -- used by wrapper components for label association)
- `content_style: MaybeProp<String>` (Extra -- used by wrapper components for CSS custom property aliases)
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`
- Note: `loop` is NOT on the public `MenuContent` API; it is on the internal `MenuContentImpl` only.

**MenuGroup:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuLabel:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuItem:**
- `disabled: MaybeProp<bool>`
- `on_select: Option<Callback<ev::Event>>`
- `text_value: MaybeProp<String>`
- `role: MaybeProp<String>` (Extra -- explicit role prop for CheckboxItem/RadioItem to override)
- `on_click: Option<Callback<ev::MouseEvent>>` (Extra -- explicit event handler)
- `on_pointer_down: Option<Callback<ev::PointerEvent>>` (Extra -- explicit event handler)
- `on_pointer_up: Option<Callback<ev::PointerEvent>>` (Extra -- explicit event handler)
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (Extra -- explicit event handler)
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuCheckboxItem:**
- `checked: MaybeProp<CheckedState>` (enum: `False`, `True`, `Indeterminate`)
- `on_checked_change: Option<Callback<bool>>`
- `disabled: MaybeProp<bool>`
- `on_select: Option<Callback<ev::Event>>`
- `text_value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuRadioGroup:**
- `value: MaybeProp<String>`
- `on_value_change: Option<Callback<String>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuRadioItem:**
- `value: MaybeProp<String>` (required, `#[prop(into)]`)
- `disabled: MaybeProp<bool>`
- `on_select: Option<Callback<ev::Event>>`
- `text_value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuItemIndicator:**
- `force_mount: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**MenuSeparator:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**MenuArrow:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**MenuSub:**
- `open: MaybeProp<bool>`
- `on_open_change: Option<Callback<bool>>`
- `children: ChildrenFn`

**MenuSubTrigger:**
- `disabled: MaybeProp<bool>`
- `text_value: MaybeProp<String>`
- `on_click: Option<Callback<ev::MouseEvent>>` (Extra)
- `on_pointer_move: Option<Callback<ev::PointerEvent>>` (Extra)
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>` (Extra)
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (Extra)
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenuSubContent:**
- `force_mount: MaybeProp<bool>`
- `side_offset: MaybeProp<f64>`
- `align_offset: MaybeProp<f64>`
- `avoid_collisions: MaybeProp<bool>`
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_focus_outside: Option<Callback<CustomEvent>>`
- `on_key_down: Option<Callback<ev::KeyboardEvent>>`
- `class: MaybeProp<String>` (Extra)
- `content_style: MaybeProp<String>` (Extra -- used by wrapper components for CSS custom property aliases)
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

### Differences

1. **Missing `loop` on public `MenuContent`:** React's `MenuContentImplProps` exposes `loop?: boolean` (default `false`) for keyboard navigation wrapping. Leptos has `loop` on the internal `MenuContentImpl` but not on the public `MenuContent` API. Wrapper components (DropdownMenuContent, etc.) cannot pass this through.
2. **Extra `class`, `content_style`, `id`, `aria_labelledby` on MenuContent:** Leptos adds these props for framework-specific needs. `class` enables reactive CSS class updates. `content_style` passes CSS custom property aliases through from wrapper components. `id` and `aria_labelledby` are used by DropdownMenu/ContextMenu/Menubar to set content element attributes that React handles via prop spread.
3. **Extra `container_ref` on MenuPortal:** Leptos adds `container_ref: AnyNodeRef` for ref-based container targeting. React does not have this.
4. **Extra event handler props on MenuItem:** Leptos explicitly exposes `role`, `on_click`, `on_pointer_down`, `on_pointer_up`, `on_key_down` as named props. React gets these via `PrimitiveDivProps` prop spread.
5. **Extra event handler props on MenuSubTrigger:** Leptos explicitly exposes `on_click`, `on_pointer_move`, `on_pointer_leave`, `on_key_down`. React gets these via `PrimitiveDivProps` prop spread.
6. **`CheckedState` type:** React uses `boolean | 'indeterminate'`. Leptos uses an enum `CheckedState { False, True, Indeterminate }`.
7. **Missing PopperContent props on MenuContent:** Leptos does not expose `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached` on `MenuContent`. These are Popper-level props that are not forwarded.
8. **Missing props on MenuSubContent:** Leptos does not expose `on_pointer_down_outside`, `on_interact_outside`, or `loop`. React's `MenuSubContentProps` inherits these from `MenuContentImplProps`.
9. **`forceMount` type:** React uses `true` literal type. Leptos uses `MaybeProp<bool>`.

### Assessment

Good alignment. All 16 exported components are present with matching names. The core interactive behavior (open/close, keyboard navigation, typeahead, selection, dismiss, submenus) is fully implemented. The main gaps are: (1) `loop` not being exposed on the public `MenuContent` API, (2) several `PopperContentProps` not being forwarded (`collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached`), and (3) some missing DismissableLayer props on `MenuSubContent`. The extra props (`class`, `content_style`, `id`, `aria_labelledby`, `container_ref`) are justified Leptos-specific adaptations.

---

## DropdownMenu

### React API

**Exported components:** `DropdownMenu` (alias `Root`), `DropdownMenuTrigger` (alias `Trigger`), `DropdownMenuPortal` (alias `Portal`), `DropdownMenuContent` (alias `Content`), `DropdownMenuGroup` (alias `Group`), `DropdownMenuLabel` (alias `Label`), `DropdownMenuItem` (alias `Item`), `DropdownMenuCheckboxItem` (alias `CheckboxItem`), `DropdownMenuRadioGroup` (alias `RadioGroup`), `DropdownMenuRadioItem` (alias `RadioItem`), `DropdownMenuItemIndicator` (alias `ItemIndicator`), `DropdownMenuSeparator` (alias `Separator`), `DropdownMenuArrow` (alias `Arrow`), `DropdownMenuSub` (alias `Sub`), `DropdownMenuSubTrigger` (alias `SubTrigger`), `DropdownMenuSubContent` (alias `SubContent`).

**DropdownMenu (Root):**
- Not a DOM element (context-only wrapper)
- `children?: ReactNode`
- `dir?: Direction`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?(open: boolean): void`
- `modal?: boolean` (default `true`)

**DropdownMenuTrigger:**
- Renders `<button type="button">` via `Primitive.button` inside `MenuAnchor`
- `disabled?: boolean` (default `false`)
- Inherits all `<button>` HTML props (`PrimitiveButtonProps`)
- `ref` forwarded

**DropdownMenuPortal:**
- Same interface as `MenuPortalProps`
- `children?: ReactNode`
- `container?: Element`
- `forceMount?: true`

**DropdownMenuContent:**
- Extends `MenuContentProps` but Omits `onEntryFocus`
- All `MenuContentProps` minus `onEntryFocus`:
  - `forceMount?: true`
  - `onCloseAutoFocus?: (event: Event) => void`
  - `loop?: boolean`
  - `onEscapeKeyDown?: (event) => void`
  - `onPointerDownOutside?: (event) => void`
  - `onFocusOutside?: (event) => void`
  - `onInteractOutside?: (event) => void`
  - `side?: Side`, `sideOffset?: number`, `align?: Align`, `alignOffset?: number`
  - `avoidCollisions?: boolean`
  - `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached`
- Inherits all `<div>` HTML props
- `ref` forwarded

**DropdownMenuGroup:** Same as `MenuGroupProps` (inherits `PrimitiveDivProps`). `ref` forwarded.

**DropdownMenuLabel:** Same as `MenuLabelProps` (inherits `PrimitiveDivProps`). `ref` forwarded.

**DropdownMenuItem:** Same as `MenuItemProps` (`disabled?: boolean`, `onSelect?: (event) => void`, `textValue?: string`, inherits `PrimitiveDivProps`). `ref` forwarded.

**DropdownMenuCheckboxItem:** Same as `MenuCheckboxItemProps` (`checked?: CheckedState`, `onCheckedChange?: (checked: boolean) => void`, plus all `MenuItemProps`). `ref` forwarded.

**DropdownMenuRadioGroup:** Same as `MenuRadioGroupProps` (`value?: string`, `onValueChange?: (value: string) => void`, plus `MenuGroupProps`). `ref` forwarded.

**DropdownMenuRadioItem:** Same as `MenuRadioItemProps` (`value: string` required, plus all `MenuItemProps`). `ref` forwarded.

**DropdownMenuItemIndicator:** Same as `MenuItemIndicatorProps` (`forceMount?: true`, inherits `PrimitiveSpanProps`). `ref` forwarded.

**DropdownMenuSeparator:** Same as `MenuSeparatorProps` (inherits `PrimitiveDivProps`). `ref` forwarded.

**DropdownMenuArrow:** Same as `MenuArrowProps` (inherits `PopperArrowProps`). `ref` forwarded.

**DropdownMenuSub:**
- `children?: ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?(open: boolean): void`

**DropdownMenuSubTrigger:** Same as `MenuSubTriggerProps` (extends `MenuItemImplProps`). `ref` forwarded.

**DropdownMenuSubContent:** Same as `MenuSubContentProps`. `ref` forwarded.

### Leptos API

**Exported components:** `DropdownMenu`, `DropdownMenuTrigger`, `DropdownMenuPortal`, `DropdownMenuContent`, `DropdownMenuGroup`, `DropdownMenuLabel`, `DropdownMenuItem`, `DropdownMenuCheckboxItem`, `DropdownMenuRadioGroup`, `DropdownMenuRadioItem`, `DropdownMenuItemIndicator`, `DropdownMenuSeparator`, `DropdownMenuArrow`, `DropdownMenuSub`, `DropdownMenuSubTrigger`, `DropdownMenuSubContent`.

**DropdownMenu (Root):**
- `open: MaybeProp<bool>`
- `default_open: MaybeProp<bool>`
- `on_open_change: Option<Callback<bool>>`
- `dir: MaybeProp<Direction>`
- `modal: MaybeProp<bool>` (default `true`)
- `children: ChildrenFn`

**DropdownMenuTrigger:**
- `disabled: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuPortal:**
- `container: MaybeProp<SendWrapper<web_sys::Element>>`
- `container_ref: AnyNodeRef` (Extra)
- `force_mount: MaybeProp<bool>`
- `children: ChildrenFn`

**DropdownMenuContent:**
- `force_mount: MaybeProp<bool>`
- `class: MaybeProp<String>` (Extra)
- `side: MaybeProp<PopperSide>`
- `side_offset: MaybeProp<f64>`
- `align: MaybeProp<Align>`
- `on_close_auto_focus: Option<Callback<ev::Event>>`
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuGroup:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuLabel:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuItem:**
- `disabled: MaybeProp<bool>`
- `on_select: Option<Callback<ev::Event>>`
- `text_value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuCheckboxItem:**
- `checked: MaybeProp<CheckedState>`
- `on_checked_change: Option<Callback<bool>>`
- `disabled: MaybeProp<bool>`
- `on_select: Option<Callback<ev::Event>>`
- `text_value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuRadioGroup:**
- `value: MaybeProp<String>`
- `on_value_change: Option<Callback<String>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuRadioItem:**
- `value: MaybeProp<String>` (required, `#[prop(into)]`)
- `disabled: MaybeProp<bool>`
- `on_select: Option<Callback<ev::Event>>`
- `text_value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuItemIndicator:**
- `force_mount: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**DropdownMenuSeparator:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**DropdownMenuArrow:**
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: Option<ChildrenFn>`

**DropdownMenuSub:**
- `open: MaybeProp<bool>`
- `default_open: MaybeProp<bool>`
- `on_open_change: Option<Callback<bool>>`
- `children: ChildrenFn`

**DropdownMenuSubTrigger:**
- `disabled: MaybeProp<bool>`
- `text_value: MaybeProp<String>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**DropdownMenuSubContent:**
- `force_mount: MaybeProp<bool>`
- `side_offset: MaybeProp<f64>`
- `class: MaybeProp<String>` (Extra)
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

### Differences

1. **Missing `align_offset` on DropdownMenuContent:** React inherits `alignOffset` from `PopperContentProps` via `MenuContentProps`. Leptos does not forward this prop.
2. **Missing `avoid_collisions` on DropdownMenuContent:** React inherits `avoidCollisions` from `PopperContentProps`. Leptos does not forward this prop.
3. **Missing `loop` on DropdownMenuContent:** React inherits `loop` from `MenuContentImplProps` (via `MenuContentProps` minus `onEntryFocus`). Leptos does not forward this prop (also missing on the underlying `MenuContent`).
4. **Missing `on_entry_focus` omission is correct:** React explicitly omits `onEntryFocus` from `DropdownMenuContentProps`. Leptos also does not expose it. This is a match.
5. **Missing props on DropdownMenuSubContent:** React inherits `loop`, `on_pointer_down_outside`, `on_interact_outside`, `align_offset`, `avoid_collisions`, `on_key_down` from `MenuSubContentProps`. Leptos only exposes `side_offset`, `on_escape_key_down`, and `on_focus_outside`.
6. **Missing PopperContent advanced props:** Like Menu, `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached` are not forwarded.
7. **Extra `class` on DropdownMenuContent and DropdownMenuSubContent:** Framework-specific addition for reactive CSS class updates.
8. **Extra `container_ref` on DropdownMenuPortal:** Framework-specific addition.
9. **`CheckedState` type:** Same as Menu -- React `boolean | 'indeterminate'` vs Leptos enum.
10. **`forceMount` type:** React `true` literal vs Leptos `MaybeProp<bool>`.

### Assessment

Good alignment. All 16 exported components are present. The root, trigger, sub, and simple pass-through components (Group, Label, Item, CheckboxItem, RadioGroup, RadioItem, ItemIndicator, Separator, Arrow) all match well. The main gaps are in `DropdownMenuContent` (missing `align_offset`, `avoid_collisions`, `loop`) and `DropdownMenuSubContent` (missing several inherited props). These are inherited props that were not explicitly forwarded in the Leptos wrapper layer.

---

## ContextMenu

### React API

**Exported components:** `ContextMenu` (alias `Root`), `ContextMenuTrigger` (alias `Trigger`), `ContextMenuPortal` (alias `Portal`), `ContextMenuContent` (alias `Content`), `ContextMenuGroup` (alias `Group`), `ContextMenuLabel` (alias `Label`), `ContextMenuItem` (alias `Item`), `ContextMenuCheckboxItem` (alias `CheckboxItem`), `ContextMenuRadioGroup` (alias `RadioGroup`), `ContextMenuRadioItem` (alias `RadioItem`), `ContextMenuItemIndicator` (alias `ItemIndicator`), `ContextMenuSeparator` (alias `Separator`), `ContextMenuArrow` (alias `Arrow`), `ContextMenuSub` (alias `Sub`), `ContextMenuSubTrigger` (alias `SubTrigger`), `ContextMenuSubContent` (alias `SubContent`).

**ContextMenu (Root):**
- Not a DOM element (context-only wrapper)
- `children?: ReactNode`
- `onOpenChange?(open: boolean): void`
- `dir?: Direction`
- `modal?: boolean` (default `true`)
- Note: No `open` or `defaultOpen` props -- always uncontrolled.

**ContextMenuTrigger:**
- Renders `<span>` via `Primitive.span` (not `<button>`)
- `disabled?: boolean` (default `false`)
- Inherits all `<span>` HTML props (`PrimitiveSpanProps`)
- `ref` forwarded

**ContextMenuPortal:**
- Same as `MenuPortalProps`
- `children?: ReactNode`
- `container?: Element`
- `forceMount?: true`

**ContextMenuContent:**
- Extends `MenuContentProps` but Omits `onEntryFocus`, `side`, `sideOffset`, `align`
- Hardcoded: `side="right"`, `sideOffset={2}`, `align="start"`
- Remaining props from `MenuContentProps`:
  - `forceMount?: true`
  - `onCloseAutoFocus?: (event) => void`
  - `loop?: boolean`
  - `onEscapeKeyDown?: (event) => void`
  - `onPointerDownOutside?: (event) => void`
  - `onFocusOutside?: (event) => void`
  - `onInteractOutside?: (event) => void`
  - `alignOffset?: number`, `avoidCollisions?: boolean`
  - `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached`
- Inherits all `<div>` HTML props
- `ref` forwarded

**ContextMenuGroup:** Same as `MenuGroupProps`. `ref` forwarded.

**ContextMenuLabel:** Same as `MenuLabelProps`. `ref` forwarded.

**ContextMenuItem:** Same as `MenuItemProps`. `ref` forwarded.

**ContextMenuCheckboxItem:** Same as `MenuCheckboxItemProps`. `ref` forwarded.

**ContextMenuRadioGroup:** Same as `MenuRadioGroupProps`. `ref` forwarded.

**ContextMenuRadioItem:** Same as `MenuRadioItemProps`. `ref` forwarded.

**ContextMenuItemIndicator:** Same as `MenuItemIndicatorProps`. `ref` forwarded.

**ContextMenuSeparator:** Same as `MenuSeparatorProps`. `ref` forwarded.

**ContextMenuArrow:** Same as `MenuArrowProps`. `ref` forwarded.

**ContextMenuSub:**
- `children?: ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?(open: boolean): void`

**ContextMenuSubTrigger:** Same as `MenuSubTriggerProps`. `ref` forwarded.

**ContextMenuSubContent:** Same as `MenuSubContentProps`. `ref` forwarded.

### Leptos API

**Exported components:** `ContextMenu`, `ContextMenuTrigger`, `ContextMenuPortal`, `ContextMenuContent`, `ContextMenuGroup`, `ContextMenuLabel`, `ContextMenuItem`, `ContextMenuCheckboxItem`, `ContextMenuRadioGroup`, `ContextMenuRadioItem`, `ContextMenuItemIndicator`, `ContextMenuSeparator`, `ContextMenuArrow`, `ContextMenuSub`, `ContextMenuSubTrigger`, `ContextMenuSubContent`.

**ContextMenu (Root):**
- `on_open_change: Option<Callback<bool>>`
- `dir: MaybeProp<Direction>`
- `modal: MaybeProp<bool>` (default `true`)
- `children: ChildrenFn`
- Correctly omits `open` and `default_open` (always uncontrolled, matching React).

**ContextMenuTrigger:**
- `disabled: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**ContextMenuPortal:**
- `container: MaybeProp<SendWrapper<web_sys::Element>>`
- `container_ref: AnyNodeRef` (Extra)
- `force_mount: MaybeProp<bool>`
- `children: ChildrenFn`

**ContextMenuContent:**
- `force_mount: MaybeProp<bool>`
- `class: MaybeProp<String>` (Extra)
- `on_close_auto_focus: Option<Callback<ev::Event>>`
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`
- Correctly hardcodes: `side=PopperSide::Right`, `side_offset=2.0`, `align=Align::Start`

**ContextMenuGroup:** `as_child`, `node_ref`, `children`.

**ContextMenuLabel:** `as_child`, `node_ref`, `children`.

**ContextMenuItem:** `disabled`, `on_select`, `text_value`, `as_child`, `node_ref`, `children`.

**ContextMenuCheckboxItem:** `checked`, `on_checked_change`, `disabled`, `on_select`, `text_value`, `as_child`, `node_ref`, `children`.

**ContextMenuRadioGroup:** `value`, `on_value_change`, `as_child`, `node_ref`, `children`.

**ContextMenuRadioItem:** `value` (required), `disabled`, `on_select`, `text_value`, `as_child`, `node_ref`, `children`.

**ContextMenuItemIndicator:** `force_mount`, `as_child`, `node_ref`, `children: Option<ChildrenFn>`.

**ContextMenuSeparator:** `as_child`, `node_ref`, `children: Option<ChildrenFn>`.

**ContextMenuArrow:** `as_child`, `node_ref`, `children: Option<ChildrenFn>`.

**ContextMenuSub:**
- `open: MaybeProp<bool>`
- `default_open: MaybeProp<bool>`
- `on_open_change: Option<Callback<bool>>`
- `children: ChildrenFn`

**ContextMenuSubTrigger:** `disabled`, `text_value`, `as_child`, `node_ref`, `children`.

**ContextMenuSubContent:**
- `force_mount: MaybeProp<bool>`
- `side_offset: MaybeProp<f64>`
- `class: MaybeProp<String>` (Extra)
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

### Differences

1. **Correctly uncontrolled root:** Both React and Leptos ContextMenu omit `open`/`defaultOpen` props. The state is managed internally. Match.
2. **Missing `align_offset` and `avoid_collisions` on ContextMenuContent:** React inherits these from `PopperContentProps`. Leptos does not forward them.
3. **Missing `loop` on ContextMenuContent:** Same as DropdownMenu -- inherited from `MenuContentImplProps` but not forwarded.
4. **Missing advanced PopperContent props:** `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached` not forwarded.
5. **Missing props on ContextMenuSubContent:** Same pattern as DropdownMenu -- missing `on_pointer_down_outside`, `on_interact_outside`, `loop`, `align_offset`, `avoid_collisions`, `on_key_down`.
6. **Extra `class` on Content and SubContent:** Framework-specific.
7. **Extra `container_ref` on Portal:** Framework-specific.
8. **Hardcoded positioning matches:** Both React and Leptos hardcode `side="right"`, `sideOffset=2`, `align="start"`. Match.

### Assessment

Good alignment. All 16 exported components are present. The uncontrolled root pattern (no `open` prop) correctly matches React. The hardcoded positioning values match. The trigger correctly renders a `<span>` (via `html::span`). The main gaps mirror those in DropdownMenu: missing `align_offset`, `avoid_collisions`, `loop` on Content, and several missing inherited props on SubContent. The extra `class` and `container_ref` props are justified framework adaptations.

---

## Menubar

### React API

**Exported components:** `Menubar` (alias `Root`), `MenubarMenu` (alias `Menu`), `MenubarTrigger` (alias `Trigger`), `MenubarPortal` (alias `Portal`), `MenubarContent` (alias `Content`), `MenubarGroup` (alias `Group`), `MenubarLabel` (alias `Label`), `MenubarItem` (alias `Item`), `MenubarCheckboxItem` (alias `CheckboxItem`), `MenubarRadioGroup` (alias `RadioGroup`), `MenubarRadioItem` (alias `RadioItem`), `MenubarItemIndicator` (alias `ItemIndicator`), `MenubarSeparator` (alias `Separator`), `MenubarArrow` (alias `Arrow`), `MenubarSub` (alias `Sub`), `MenubarSubTrigger` (alias `SubTrigger`), `MenubarSubContent` (alias `SubContent`).

**Menubar (Root):**
- Renders `<div>` via `Primitive.div` with `role="menubar"`
- `value?: string`
- `defaultValue?: string`
- `onValueChange?: (value: string) => void`
- `loop?: boolean` (default `true`, passed to `RovingFocusGroup`)
- `dir?: Direction`
- Inherits all `<div>` HTML props (`PrimitiveDivProps`)
- `ref` forwarded

**MenubarMenu:**
- Not a DOM element (context-only wrapper)
- `value?: string` (auto-generated via `useId` if omitted)
- `children?: ReactNode`

**MenubarTrigger:**
- Renders `<button type="button">` via `Primitive.button` inside `MenuAnchor` and `RovingFocusGroup.Item`
- `disabled?: boolean` (default `false`)
- `role="menuitem"`, `aria-haspopup="menu"`, etc.
- Inherits all `<button>` HTML props (`PrimitiveButtonProps`)
- `ref` forwarded

**MenubarPortal:** Same as `MenuPortalProps`.

**MenubarContent:**
- Extends `MenuContentProps` but Omits `onEntryFocus`
- `align` defaults to `'start'` (overriding `PopperContentProps` default of `'center'`)
- All other `MenuContentProps` props available (minus `onEntryFocus`)
- Inherits all `<div>` HTML props
- `ref` forwarded

**MenubarGroup:** Same as `MenuGroupProps`. `ref` forwarded.

**MenubarLabel:** Same as `MenuLabelProps`. `ref` forwarded.

**MenubarItem:** Same as `MenuItemProps`. `ref` forwarded.

**MenubarCheckboxItem:** Same as `MenuCheckboxItemProps`. `ref` forwarded.

**MenubarRadioGroup:** Same as `MenuRadioGroupProps`. `ref` forwarded.

**MenubarRadioItem:** Same as `MenuRadioItemProps`. `ref` forwarded.

**MenubarItemIndicator:** Same as `MenuItemIndicatorProps`. `ref` forwarded.

**MenubarSeparator:** Same as `MenuSeparatorProps`. `ref` forwarded.

**MenubarArrow:** Same as `MenuArrowProps`. `ref` forwarded.

**MenubarSub:**
- `children?: ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?(open: boolean): void`

**MenubarSubTrigger:** Same as `MenuSubTriggerProps`. Adds `data-radix-menubar-subtrigger=""`. `ref` forwarded.

**MenubarSubContent:** Same as `MenuSubContentProps`. Adds `data-radix-menubar-content=""`. `ref` forwarded.

### Leptos API

**Exported components:** `Menubar`, `MenubarMenu`, `MenubarTrigger`, `MenubarPortal`, `MenubarContent`, `MenubarGroup`, `MenubarLabel`, `MenubarItem`, `MenubarCheckboxItem`, `MenubarRadioGroup`, `MenubarRadioItem`, `MenubarItemIndicator`, `MenubarSeparator`, `MenubarArrow`, `MenubarSub`, `MenubarSubTrigger`, `MenubarSubContent`.

**Menubar (Root):**
- `value: MaybeProp<String>`
- `default_value: MaybeProp<String>`
- `on_value_change: Option<Callback<String>>`
- `r#loop: MaybeProp<bool>` (default `true`)
- `dir: MaybeProp<Direction>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenubarMenu:**
- `value: Option<String>`
- `children: ChildrenFn`

**MenubarTrigger:**
- `disabled: MaybeProp<bool>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenubarPortal:**
- `container: MaybeProp<SendWrapper<web_sys::Element>>`
- `container_ref: AnyNodeRef` (Extra)
- `force_mount: MaybeProp<bool>`
- `children: ChildrenFn`

**MenubarContent:**
- `force_mount: MaybeProp<bool>`
- `class: MaybeProp<String>` (Extra)
- `align: MaybeProp<Align>` (default `Align::Start`)
- `align_offset: MaybeProp<f64>`
- `avoid_collisions: MaybeProp<bool>`
- `side: MaybeProp<PopperSide>`
- `side_offset: MaybeProp<f64>`
- `on_close_auto_focus: Option<Callback<ev::Event>>`
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>`
- `on_key_down: Option<Callback<ev::KeyboardEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

**MenubarGroup:** `as_child`, `node_ref`, `children`.

**MenubarLabel:** `as_child`, `node_ref`, `children`.

**MenubarItem:** `disabled`, `on_select`, `text_value`, `as_child`, `node_ref`, `children`.

**MenubarCheckboxItem:** `checked`, `on_checked_change`, `disabled`, `on_select`, `text_value`, `as_child`, `node_ref`, `children`.

**MenubarRadioGroup:** `value`, `on_value_change`, `as_child`, `node_ref`, `children`.

**MenubarRadioItem:** `value` (required), `disabled`, `on_select`, `text_value`, `as_child`, `node_ref`, `children`.

**MenubarItemIndicator:** `force_mount`, `as_child`, `node_ref`, `children: Option<ChildrenFn>`.

**MenubarSeparator:** `as_child`, `node_ref`, `children: Option<ChildrenFn>`.

**MenubarArrow:** `as_child`, `node_ref`, `children: Option<ChildrenFn>`.

**MenubarSub:**
- `open: MaybeProp<bool>`
- `default_open: MaybeProp<bool>`
- `on_open_change: Option<Callback<bool>>`
- `children: ChildrenFn`

**MenubarSubTrigger:** `disabled`, `text_value`, `as_child`, `node_ref`, `children`. Sets `data-radix-menubar-subtrigger` via Effect.

**MenubarSubContent:**
- `force_mount: MaybeProp<bool>`
- `side_offset: MaybeProp<f64>`
- `align_offset: MaybeProp<f64>`
- `avoid_collisions: MaybeProp<bool>`
- `class: MaybeProp<String>` (Extra)
- `on_escape_key_down: Option<Callback<ev::KeyboardEvent>>`
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>`
- `as_child: MaybeProp<bool>`
- `node_ref: AnyNodeRef`
- `children: ChildrenFn`

### Differences

1. **MenubarContent has the most complete prop surface:** Unlike DropdownMenuContent and ContextMenuContent, Leptos's `MenubarContent` forwards `align`, `align_offset`, `avoid_collisions`, `side`, `side_offset`, `on_key_down`, `on_focus_outside`, and `on_interact_outside`. This is the most complete of the four menu-family Content components.
2. **Missing `loop` on MenubarContent:** React inherits `loop` from `MenuContentImplProps` but Leptos does not forward it (same gap as the other menu components).
3. **Missing advanced PopperContent props on MenubarContent:** `collisionBoundary`, `collisionPadding`, `arrowPadding`, `sticky`, `hideWhenDetached` are not forwarded.
4. **Missing props on MenubarSubContent:** Missing `on_pointer_down_outside`, `on_interact_outside`, `loop`, `on_key_down` compared to React's `MenuSubContentProps`.
5. **Default `align` matches:** Both React and Leptos default `align` to `'start'`/`Align::Start` on MenubarContent.
6. **`data-radix-menubar-subtrigger` and `data-radix-menubar-content`:** React sets these via prop spread. Leptos sets them via Effect since the underlying Menu components don't support arbitrary data attribute passthrough. Functionally equivalent.
7. **Extra `class` on Content and SubContent:** Framework-specific.
8. **Extra `container_ref` on Portal:** Framework-specific.
9. **17 components (extra `MenubarMenu`):** Both React and Leptos export 17 components (16 like the other menu variants + `MenubarMenu`). Match.

### Assessment

Best alignment of the four menu-family components. All 17 exported components are present. MenubarContent has the most complete prop forwarding of any of the Content components in this family. The root Menubar correctly manages `value`/`defaultValue`/`onValueChange` and `loop`/`dir` for roving focus. The `data-radix-menubar-*` attributes are correctly applied via Effects. The main gaps are the consistently missing `loop` on Content and some inherited SubContent props.

---

## Dialog

### React API

**Exported components:** `Dialog`, `DialogTrigger`, `DialogPortal`, `DialogOverlay`, `DialogContent`, `DialogTitle`, `DialogDescription`, `DialogClose` (aliased as `Root`, `Trigger`, `Portal`, `Overlay`, `Content`, `Title`, `Description`, `Close`)

**Dialog** (provider component, no DOM output)
- `children?: React.ReactNode`
- `open?: boolean` -- controlled open state
- `defaultOpen?: boolean` -- uncontrolled default
- `onOpenChange?: (open: boolean) => void`
- `modal?: boolean` (default `true`)

**DialogTrigger** (renders `Primitive.button`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)
- Internally sets: `type="button"`, `aria-haspopup="dialog"`, `aria-expanded`, `aria-controls`, `data-state`, composes `onClick`

**DialogPortal** (FC, renders `Presence` + `PortalPrimitive` per child)
- `children?: React.ReactNode`
- `container?: PortalProps['container']` -- target element for portal
- `forceMount?: true` -- force mount for animation control

**DialogOverlay** (renders `Primitive.div` via `DialogOverlayImpl`, wrapped in `RemoveScroll`)
- `forceMount?: true`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)
- Internally sets: `data-state`, `style={{ pointerEvents: 'auto' }}`

**DialogContent** (renders modal or non-modal variant)
- `forceMount?: true`
- `trapFocus?: boolean` -- from FocusScopeProps (omitted from public type)
- `disableOutsidePointerEvents?: boolean` -- from DismissableLayerProps (omitted from public type)
- `onOpenAutoFocus?: (event: Event) => void`
- `onCloseAutoFocus?: (event: Event) => void`
- `onEscapeKeyDown?: (event: KeyboardEvent) => void`
- `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void`
- `onFocusOutside?: (event: FocusOutsideEvent) => void`
- `onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void`
- Inherits all `div` HTML attributes via `DismissableLayerProps` (extends `PrimitiveDivProps`)
- `ref` (forwarded)
- Internally sets: `role="dialog"`, `id`, `aria-describedby`, `aria-labelledby`, `data-state`

**DialogTitle** (renders `Primitive.h2`)
- Inherits all `h2` HTML attributes via `PrimitiveHeading2Props`
- `ref` (forwarded)
- Internally sets: `id`

**DialogDescription** (renders `Primitive.p`)
- Inherits all `p` HTML attributes via `PrimitiveParagraphProps`
- `ref` (forwarded)
- Internally sets: `id`

**DialogClose** (renders `Primitive.button`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)
- Internally sets: `type="button"`, composes `onClick`

### Leptos API

**Exported components:** `Dialog`, `DialogTrigger`, `DialogPortal`, `DialogOverlay`, `DialogContent`, `DialogTitle`, `DialogDescription`, `DialogClose`

**Dialog**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `modal: MaybeProp<bool>` (optional, defaults to `true` via `prop_or`)
- `children: ChildrenFn` (required)

**DialogTrigger**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**DialogPortal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional)
- `force_mount: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**DialogOverlay**
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**DialogContent**
- `force_mount: MaybeProp<bool>` (optional)
- `role: Option<String>` (optional, defaults to `"dialog"`)
- `on_open_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_close_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**DialogTitle**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**DialogDescription**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**DialogClose**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Extra `role` prop on DialogContent:** Leptos adds a `role: Option<String>` prop (defaults to `"dialog"`) that React does not expose. This is used internally by AlertDialog to override the role to `"alertdialog"`. In React, AlertDialog achieves this by passing `role="alertdialog"` as a prop spread on `DialogPrimitive.Content`, which works because DismissableLayer inherits all div props. In Leptos, the role must be an explicit prop because Leptos does not support arbitrary HTML attribute inheritance through component boundaries.
2. **Extra `container_ref` prop on DialogPortal:** Leptos adds `container_ref: AnyNodeRef` as an alternative to `container` for specifying the portal target via a node ref. React only has `container`.
3. **`forceMount` type:** React uses literal type `true` (`forceMount?: true`). Leptos uses `MaybeProp<bool>`. The React type restricts the value to only `true` (or undefined), while Leptos allows any boolean.
4. **Portal implementation:** React wraps each child individually in `<Presence><PortalPrimitive>`. Leptos always renders the portal and lets each child (Overlay, Content) handle their own Presence wrappers. This is documented in the code as necessary because Leptos cannot map over children the same way React does.
5. **Scroll lock implementation:** React uses `RemoveScroll` from `react-remove-scroll` (which wraps the overlay). Leptos uses `use_body_scroll_lock()` which sets `overflow: hidden` on body. The Leptos approach is simpler and lacks features like scroll bar compensation and pinch zoom handling.

### Assessment

Strong alignment. All 8 exported components match 1:1. All DialogContent callback props are present. The extra `role` prop is a practical adaptation for AlertDialog composition. The `container_ref` addition is a Leptos-specific convenience. The scroll lock simplification is a known gap.

---

## AlertDialog

### React API

**Exported components:** `AlertDialog`, `AlertDialogTrigger`, `AlertDialogPortal`, `AlertDialogOverlay`, `AlertDialogContent`, `AlertDialogAction`, `AlertDialogCancel`, `AlertDialogTitle`, `AlertDialogDescription` (aliased as `Root`, `Trigger`, `Portal`, `Overlay`, `Content`, `Action`, `Cancel`, `Title`, `Description`)

**AlertDialog** (wraps `DialogPrimitive.Root` with `modal={true}`)
- `children?: React.ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?: (open: boolean) => void`
- Note: `modal` is omitted from the interface and hardcoded to `true`

**AlertDialogTrigger** (wraps `DialogPrimitive.Trigger`)
- Inherits all `DialogTriggerProps`
- `ref` (forwarded)

**AlertDialogPortal** (wraps `DialogPrimitive.Portal`)
- Inherits all `DialogPortalProps` (`children`, `container`, `forceMount`)

**AlertDialogOverlay** (wraps `DialogPrimitive.Overlay`)
- Inherits all `DialogOverlayProps` (`forceMount`, plus all div attributes)
- `ref` (forwarded)

**AlertDialogContent** (wraps `DialogPrimitive.Content`)
- Inherits `DialogContentProps` MINUS `onPointerDownOutside` and `onInteractOutside` (these are `Omit`-ed)
- Remaining content props: `forceMount`, `onOpenAutoFocus`, `onCloseAutoFocus`, `onEscapeKeyDown`, `onFocusOutside`
- `ref` (forwarded)
- Internally: sets `role="alertdialog"`, prevents `onPointerDownOutside` and `onInteractOutside`, auto-focuses cancel button via `onOpenAutoFocus` override

**AlertDialogAction** (wraps `DialogPrimitive.Close`)
- Inherits all `DialogCloseProps`
- `ref` (forwarded)

**AlertDialogCancel** (wraps `DialogPrimitive.Close`)
- Inherits all `DialogCloseProps`
- `ref` (forwarded)
- Registers itself with content context for auto-focus

**AlertDialogTitle** (wraps `DialogPrimitive.Title`)
- Inherits all `DialogTitleProps`
- `ref` (forwarded)

**AlertDialogDescription** (wraps `DialogPrimitive.Description`)
- Inherits all `DialogDescriptionProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `AlertDialog`, `AlertDialogTrigger`, `AlertDialogPortal`, `AlertDialogOverlay`, `AlertDialogContent`, `AlertDialogAction`, `AlertDialogCancel`, `AlertDialogTitle`, `AlertDialogDescription`

**AlertDialog**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `children: ChildrenFn` (required)
- Internally passes `modal=true` to Dialog

**AlertDialogTrigger**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AlertDialogPortal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional)
- `force_mount: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**AlertDialogOverlay**
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**AlertDialogContent**
- `force_mount: MaybeProp<bool>` (optional)
- `on_open_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_close_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)
- Internally: passes `role="alertdialog"` to DialogContent, prevents pointer down outside and interact outside

**AlertDialogTitle**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AlertDialogDescription**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AlertDialogAction**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**AlertDialogCancel**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Missing `on_focus_outside` on AlertDialogContent:** React's AlertDialogContent inherits `onFocusOutside` from `DialogContentProps` (only `onPointerDownOutside` and `onInteractOutside` are omitted). Leptos does not expose `on_focus_outside` on AlertDialogContent. This is a minor gap -- in practice, AlertDialog always prevents focus-outside dismissal internally (via DialogContentModal's focus_outside handler), so the user callback would rarely be needed.
2. **Auto-focus strategy:** React overrides `onOpenAutoFocus` to call `event.preventDefault()` and directly focus `cancelRef.current`. Leptos instead relies on FocusScope's default `focus_first()` behavior, which auto-focuses the first tabbable element (typically the cancel button by convention). The comment in the code explains this avoids timing issues with `use_composed_refs` Effect propagation.
3. **Extra `container_ref` prop on AlertDialogPortal:** Inherited from DialogPortal -- same as the Dialog difference.

### Assessment

Excellent alignment. All 9 exported components match 1:1. The correctly omitted `onPointerDownOutside` and `onInteractOutside` from AlertDialogContent matches React's `Omit<>` pattern. The missing `on_focus_outside` is minor since AlertDialog always prevents focus-outside dismissal. The auto-focus strategy difference is well-documented and functionally equivalent.

---

## Popover

### React API

**Exported components:** `Popover`, `PopoverAnchor`, `PopoverTrigger`, `PopoverPortal`, `PopoverContent`, `PopoverClose`, `PopoverArrow` (aliased as `Root`, `Anchor`, `Trigger`, `Portal`, `Content`, `Close`, `Arrow`)

**Popover** (provider component wrapping `PopperPrimitive.Root`)
- `children?: React.ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?: (open: boolean) => void`
- `modal?: boolean` (default `false`)

**PopoverAnchor** (renders `PopperPrimitive.Anchor`)
- Inherits all `PopperAnchorProps` (including `virtualRef`)
- `ref` (forwarded)

**PopoverTrigger** (renders `Primitive.button`, optionally wrapped in `PopperPrimitive.Anchor`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)
- Internally sets: `type="button"`, `aria-haspopup="dialog"`, `aria-expanded`, `aria-controls`, `data-state`, composes `onClick`

**PopoverPortal** (FC, renders `Presence` + `PortalPrimitive`)
- `children?: React.ReactNode`
- `container?: PortalProps['container']`
- `forceMount?: true`

**PopoverContent** (renders modal or non-modal variant via `PopoverContentImpl`)
- `forceMount?: true`
- `onOpenAutoFocus?: (event: Event) => void`
- `onCloseAutoFocus?: (event: Event) => void`
- `onEscapeKeyDown?: (event: KeyboardEvent) => void`
- `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void`
- `onFocusOutside?: (event: FocusOutsideEvent) => void`
- `onInteractOutside?: (event: PointerDownOutsideEvent | FocusOutsideEvent) => void`
- All `PopperContentProps`: `side` (default `'bottom'`), `sideOffset` (default `0`), `align` (default `'center'`), `alignOffset` (default `0`), `arrowPadding` (default `0`), `avoidCollisions` (default `true`), `collisionBoundary` (default `[]`), `collisionPadding` (default `0`), `sticky` (default `'partial'`), `hideWhenDetached` (default `false`), `updatePositionStrategy` (default `'optimized'`)
- Inherits `DismissableLayerProps` (minus `onDismiss`)
- `ref` (forwarded)
- Internally sets: `data-state`, `role="dialog"`, `id`, custom CSS properties

**PopoverClose** (renders `Primitive.button`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)
- Internally sets: `type="button"`, composes `onClick`

**PopoverArrow** (renders `PopperPrimitive.Arrow`)
- Inherits all `PopperArrowProps` (width, height, SVG attributes)
- `ref` (forwarded)

### Leptos API

**Exported components:** `Popover`, `PopoverAnchor`, `PopoverTrigger`, `PopoverPortal`, `PopoverContent`, `PopoverClose`, `PopoverArrow`

**Popover**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `modal: MaybeProp<bool>` (optional, defaults to `false` via `prop_or_default`)
- `children: ChildrenFn` (required)

**PopoverAnchor**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**PopoverTrigger**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**PopoverPortal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional)
- `force_mount: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**PopoverContent**
- `force_mount: MaybeProp<bool>` (optional)
- `on_open_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_close_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `side: Signal<Side>` (default `Side::Bottom`)
- `side_offset: Signal<f64>` (default `0.0`)
- `align: Signal<Align>` (default `Align::Center`)
- `align_offset: Signal<f64>` (default `0.0`)
- `arrow_padding: Signal<f64>` (default `0.0`)
- `avoid_collisions: Signal<bool>` (default `true`)
- `collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>` (default `vec![]`)
- `collision_padding: Signal<Padding>` (default `Padding::All(0.0)`)
- `sticky: Signal<Sticky>` (default `Sticky::Partial`)
- `hide_when_detached: Signal<bool>` (default `false`)
- `update_position_strategy: Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**PopoverClose**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**PopoverArrow**
- `width: MaybeProp<f64>` (optional)
- `height: MaybeProp<f64>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Extra `container_ref` prop on PopoverPortal:** Same as Dialog -- Leptos adds `container_ref: AnyNodeRef` as an alternative to `container`. React only has `container`.
2. **Positioning props use `Signal<T>`:** React uses plain values (`side?: Side`, `sideOffset?: number`, etc.) with defaults in the component body. Leptos uses `Signal<T>` for all positioning props, enabling reactive updates. The defaults match React.
3. **`collision_padding` type:** React accepts `number | Partial<Record<Side, number>>`. Leptos uses `Signal<Padding>` where `Padding` is a floating-ui-leptos enum (`Padding::All(f64)` or per-side).
4. **`collision_boundary` type:** React accepts `Boundary | Boundary[]` where `Boundary = Element | null`. Leptos uses `Signal<SendWrapper<Vec<web_sys::Element>>>`.

### Assessment

Excellent alignment. All 7 exported components match 1:1. All callback props on PopoverContent are present. All PopperContent positioning props are present with matching defaults. The `Signal<T>` wrapping is an idiomatic Leptos adaptation for reactivity. The type differences for `collision_padding` and `collision_boundary` are pragmatic Rust adaptations.

---

## HoverCard

### React API

**Exported components:** `HoverCard`, `HoverCardTrigger`, `HoverCardPortal`, `HoverCardContent`, `HoverCardArrow` (aliased as `Root`, `Trigger`, `Portal`, `Content`, `Arrow`)

**HoverCard** (provider component wrapping `PopperPrimitive.Root`)
- `children?: React.ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?: (open: boolean) => void`
- `openDelay?: number` (default `700`)
- `closeDelay?: number` (default `300`)

**HoverCardTrigger** (renders `Primitive.a` inside `PopperPrimitive.Anchor`)
- Inherits all `a` HTML attributes via `PrimitiveLinkProps`
- `ref` (forwarded)
- Internally sets: `data-state`, composes `onPointerEnter` (excludeTouch), `onPointerLeave` (excludeTouch), `onFocus`, `onBlur`, `onTouchStart` (preventDefault)

**HoverCardPortal** (FC, renders `Presence` + `PortalPrimitive`)
- `children?: React.ReactNode`
- `container?: PortalProps['container']`
- `forceMount?: true`

**HoverCardContent** (renders `HoverCardContentImpl` via `DismissableLayer` + `PopperPrimitive.Content`)
- `forceMount?: true`
- `onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown']`
- `onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside']`
- `onFocusOutside?: DismissableLayerProps['onFocusOutside']`
- `onInteractOutside?: DismissableLayerProps['onInteractOutside']`
- All `PopperContentProps` (same as Popover: side, sideOffset, align, alignOffset, etc.) minus `onPlaced`
- Inherits remaining `PopperContentProps` (all div attributes)
- `ref` (forwarded)
- Internally: composes `onPointerEnter` (excludeTouch), `onPointerLeave` (excludeTouch), sets `data-state`, manages text selection, suppresses tabbable nodes, custom CSS properties

**HoverCardArrow** (renders `PopperPrimitive.Arrow`)
- Inherits all `PopperArrowProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `HoverCard`, `HoverCardTrigger`, `HoverCardPortal`, `HoverCardContent`, `HoverCardArrow`

**HoverCard**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `open_delay: MaybeProp<f64>` (optional, default `700.0`)
- `close_delay: MaybeProp<f64>` (optional, default `300.0`)
- `children: ChildrenFn` (required)

**HoverCardTrigger**
- `on_pointer_enter: Option<Callback<ev::PointerEvent>>` (optional)
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>` (optional)
- `on_focus: Option<Callback<ev::FocusEvent>>` (optional)
- `on_blur: Option<Callback<ev::FocusEvent>>` (optional)
- `on_touch_start: Option<Callback<ev::TouchEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**HoverCardPortal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional)
- `force_mount: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**HoverCardContent**
- `force_mount: MaybeProp<bool>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_focus_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_interact_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `on_pointer_enter: Option<Callback<ev::PointerEvent>>` (optional)
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>` (optional)
- `side: Signal<Side>` (default `Side::Bottom`)
- `side_offset: Signal<f64>` (default `0.0`)
- `align: Signal<Align>` (default `Align::Center`)
- `align_offset: Signal<f64>` (default `0.0`)
- `arrow_padding: Signal<f64>` (default `0.0`)
- `avoid_collisions: Signal<bool>` (default `true`)
- `collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>` (default `vec![]`)
- `collision_padding: Signal<Padding>` (default `Padding::All(0.0)`)
- `sticky: Signal<Sticky>` (default `Sticky::Partial`)
- `hide_when_detached: Signal<bool>` (default `false`)
- `update_position_strategy: Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**HoverCardArrow**
- `width: MaybeProp<f64>` (optional)
- `height: MaybeProp<f64>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Explicit event handler props on HoverCardTrigger:** Leptos explicitly declares `on_pointer_enter`, `on_pointer_leave`, `on_focus`, `on_blur`, `on_touch_start` as component props. React gets these via inherited `PrimitiveLinkProps` and destructures them from the spread props. The behavior is identical -- both compose user callbacks with internal handlers.
2. **Explicit event handler props on HoverCardContent:** Leptos explicitly declares `on_pointer_enter` and `on_pointer_leave` as component props. React composes these from `props.onPointerEnter` / `props.onPointerLeave` which come via inherited PopperContentProps.
3. **Extra `container_ref` prop on HoverCardPortal:** Same as Dialog and Popover.
4. **Delay prop types:** React uses `number`. Leptos uses `MaybeProp<f64>`. Both have the same default values (700 and 300 respectively).
5. **Nested hover card coordination:** Leptos captures parent `HoverCardContextValue` context and chains `on_open` / `on_close` calls up the parent hierarchy to keep ancestor hover cards open. This is a Leptos-specific adaptation because Leptos portals use native DOM events (which fire `pointerleave` at portal boundaries), unlike React where synthetic events bubble through the component tree across portals.

### Assessment

Strong alignment. All 5 exported components match 1:1. All DismissableLayer callback props on HoverCardContent are present. The explicit event handler props are needed because Leptos cannot inherit HTML attributes through component types. The nested hover card coordination is a well-designed Leptos adaptation to compensate for portal event bubbling differences.

---

## Tooltip

### React API

**Exported components:** `TooltipProvider`, `Tooltip`, `TooltipTrigger`, `TooltipPortal`, `TooltipContent`, `TooltipArrow` (aliased as `Provider`, `Root`, `Trigger`, `Portal`, `Content`, `Arrow`)

**TooltipProvider** (provider component, cross-tooltip coordination)
- `children: React.ReactNode` (required)
- `delayDuration?: number` (default `700`)
- `skipDelayDuration?: number` (default `300`)
- `disableHoverableContent?: boolean` (default `false`)

**Tooltip** (provider component wrapping `PopperPrimitive.Root`)
- `children?: React.ReactNode`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?: (open: boolean) => void`
- `delayDuration?: number` -- overrides provider's delay
- `disableHoverableContent?: boolean` -- overrides provider's setting

**TooltipTrigger** (renders `Primitive.button` inside `PopperPrimitive.Anchor`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)
- Internally sets: `aria-describedby` (conditional on open), `data-state` (uses `stateAttribute`: `'closed'`/`'delayed-open'`/`'instant-open'`), composes `onPointerMove`, `onPointerLeave`, `onPointerDown`, `onFocus`, `onBlur`, `onClick`
- Note: does NOT set `type="button"` (intentional -- tooltip triggers may be anchors)

**TooltipPortal** (FC, renders `Presence` + `PortalPrimitive`)
- `children?: React.ReactNode`
- `container?: PortalProps['container']`
- `forceMount?: true`

**TooltipContent** (renders `TooltipContentImpl` or `TooltipContentHoverable`)
- `forceMount?: true`
- `'aria-label'?: string` -- accessible label for the tooltip
- `onEscapeKeyDown?: DismissableLayerProps['onEscapeKeyDown']`
- `onPointerDownOutside?: DismissableLayerProps['onPointerDownOutside']`
- All `PopperContentProps` minus `onPlaced`: `side` (default `'top'`), `sideOffset`, `align`, `alignOffset`, `arrowPadding`, `avoidCollisions`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`, `updatePositionStrategy`
- `ref` (forwarded)
- Internally: sets `data-state` (uses `stateAttribute`), custom CSS properties, includes `VisuallyHidden` with `role="tooltip"` and `id={contentId}` containing `ariaLabel || children`
- Note: does NOT expose `onFocusOutside` or `onInteractOutside` (these are handled internally)

**TooltipArrow** (renders `PopperPrimitive.Arrow`, hidden inside `VisuallyHidden`)
- Inherits all `PopperArrowProps`
- `ref` (forwarded)
- Returns null if inside `VisuallyHiddenContentContext` (prevents duplicate arrow)

### Leptos API

**Exported components:** `TooltipProvider`, `Tooltip`, `TooltipTrigger`, `TooltipPortal`, `TooltipContent`, `TooltipArrow`

**TooltipProvider**
- `delay_duration: MaybeProp<f64>` (optional, default `700.0`)
- `skip_delay_duration: MaybeProp<f64>` (optional, default `300.0`)
- `disable_hoverable_content: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**Tooltip**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `delay_duration: MaybeProp<f64>` (optional)
- `disable_hoverable_content: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**TooltipTrigger**
- `on_pointer_move: Option<Callback<ev::PointerEvent>>` (optional)
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>` (optional)
- `on_pointer_down: Option<Callback<ev::PointerEvent>>` (optional)
- `on_focus: Option<Callback<ev::FocusEvent>>` (optional)
- `on_blur: Option<Callback<ev::FocusEvent>>` (optional)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**TooltipPortal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional)
- `force_mount: MaybeProp<bool>` (optional)
- `children: ChildrenFn` (required)

**TooltipContent**
- `force_mount: MaybeProp<bool>` (optional)
- `aria_label: MaybeProp<String>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `side: Signal<Side>` (default `Side::Top`)
- `side_offset: Signal<f64>` (default `0.0`)
- `align: Signal<Align>` (default `Align::Center`)
- `align_offset: Signal<f64>` (default `0.0`)
- `arrow_padding: Signal<f64>` (default `0.0`)
- `avoid_collisions: Signal<bool>` (default `true`)
- `collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>` (default `vec![]`)
- `collision_padding: Signal<Padding>` (default `Padding::All(0.0)`)
- `sticky: Signal<Sticky>` (default `Sticky::Partial`)
- `hide_when_detached: Signal<bool>` (default `false`)
- `update_position_strategy: Signal<UpdatePositionStrategy>` (default `UpdatePositionStrategy::Optimized`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**TooltipArrow**
- `width: MaybeProp<f64>` (optional)
- `height: MaybeProp<f64>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)
- Returns nothing if inside `VisuallyHiddenContentContextValue` with `is_inside: true`

### Differences

1. **`aria_label` prop naming:** React uses `'aria-label'` (kebab-case, matching the HTML attribute). Leptos uses `aria_label` (snake_case, matching Rust naming conventions). Both serve the same purpose.
2. **Explicit event handler props on TooltipTrigger:** Leptos explicitly declares `on_pointer_move`, `on_pointer_leave`, `on_pointer_down`, `on_focus`, `on_blur`, `on_click` as component props. React gets these via inherited `PrimitiveButtonProps`. The behavior is identical -- both compose user callbacks with internal handlers.
3. **Extra `container_ref` prop on TooltipPortal:** Same as Dialog, Popover, and HoverCard.
4. **`side` default is `Side::Top`:** Both React and Leptos default to `top` for tooltip content (unlike Popover which defaults to `bottom`). This matches correctly.
5. **Geometry utilities location:** React defines `getExitSideFromRect`, `getPaddedExitPoints`, `getPointsFromRect`, `isPointInPolygon`, `getHull` inline in the tooltip module. Leptos extracts `Point`, `get_hull`, `is_point_in_polygon` to a shared `pith_ui_utils` crate, while `get_exit_side_from_rect`, `get_padded_exit_points`, `get_points_from_rect` remain in the tooltip module. This is a reasonable code organization choice.
6. **Pointer-down deferred to microtask:** Leptos defers the tooltip close on `pointerdown` to `queue_microtask` to avoid closure invalidation during Leptos's delegated event dispatch. React does not need this because React 18+ batches state updates during event handlers.

### Assessment

Excellent alignment. All 6 exported components match 1:1. The `side` default of `top` correctly matches React (unlike Popover's `bottom`). The `aria_label` naming is an idiomatic Rust adaptation. The explicit event handler props on TooltipTrigger are needed because Leptos cannot inherit HTML attributes through component types. The `TooltipContentHoverable` pattern (grace area polygon math for pointer transit between trigger and content) is fully implemented. The `VisuallyHidden` + `role="tooltip"` accessibility pattern is correctly ported.

---

## Cross-Component Summary (Dialog, AlertDialog, Popover, HoverCard, Tooltip)

| Component   | React Exports | Leptos Exports | API Match | Notable Gaps                           |
| ----------- | ------------- | -------------- | --------- | -------------------------------------- |
| Dialog      | 8             | 8              | Complete  | Extra `role` prop, `container_ref`     |
| AlertDialog | 9             | 9              | Complete  | Missing `on_focus_outside` (minor)     |
| Popover     | 7             | 7              | Complete  | `container_ref`, Signal wrapping       |
| HoverCard   | 5             | 5              | Complete  | Explicit event handler props           |
| Tooltip     | 6             | 6              | Complete  | `aria_label` naming, explicit handlers |

All five components have complete API coverage. The recurring patterns across all five are:

1. **`container_ref` on Portal:** All portal components add a Leptos-specific `container_ref: AnyNodeRef` prop.
2. **`forceMount` type:** All use `MaybeProp<bool>` instead of React's literal `true` type.
3. **Explicit event handler props:** Components that compose event handlers (Trigger, Content) declare them as explicit props rather than relying on inherited HTML attribute types.
4. **`Signal<T>` for positioning props:** All Popper-based components wrap positioning props in `Signal<T>` for reactivity.
5. **Context re-provision across portals:** All portal components explicitly capture and re-provide contexts, compensating for Leptos portals breaking reactive context chains.

---

## Tabs

### React API

**Exported components:** `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent` (aliased as `Root`, `List`, `Trigger`, `Content`)

**Tabs** (renders `Primitive.div`)
- `value?: string` — controlled active tab
- `defaultValue?: string` — default active tab (uncontrolled)
- `onValueChange?: (value: string) => void`
- `orientation?: 'horizontal' | 'vertical'` (default `'horizontal'`)
- `dir?: 'ltr' | 'rtl'`
- `activationMode?: 'automatic' | 'manual'` (default `'automatic'`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**TabsList** (renders `Primitive.div` inside `RovingFocusGroup.Root`)
- `loop?: boolean` (default `true`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**TabsTrigger** (renders `Primitive.button` inside `RovingFocusGroup.Item`)
- `value: string` (required)
- `disabled?: boolean` (default `false`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)

**TabsContent** (renders `Primitive.div` inside `Presence`)
- `value: string` (required)
- `forceMount?: true`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent` (plus internal `TabsContentImpl`)

**Tabs**
- `value: MaybeProp<String>` (optional)
- `default_value: MaybeProp<String>` (optional)
- `on_value_change: Option<Callback<String>>` (optional)
- `orientation: MaybeProp<Orientation>` (optional, default `Horizontal`)
- `dir: MaybeProp<Direction>` (optional)
- `activation_mode: MaybeProp<ActivationMode>` (optional, default `Automatic`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**TabsList**
- `r#loop: MaybeProp<bool>` (optional, default `true`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**TabsTrigger**
- `value: String` (required)
- `disabled: MaybeProp<bool>` (optional)
- `on_mouse_down: Option<Callback<ev::MouseEvent>>` (optional, Extra)
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (optional, Extra)
- `on_focus: Option<Callback<ev::FocusEvent>>` (optional, Extra)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**TabsContent**
- `value: String` (required)
- `force_mount: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Activation mode type**: React uses string literal `'automatic' | 'manual'`. Leptos uses `ActivationMode` enum with `Automatic`, `Manual` variants.
2. **Orientation type**: React uses string literal `'horizontal' | 'vertical'`. Leptos uses `Orientation` enum (re-exported from `roving_focus`).
3. **Explicit event handler props on TabsTrigger**: Leptos declares `on_mouse_down`, `on_key_down`, and `on_focus` as explicit props for event handler composition. React composes these from the spread `...triggerProps`. Functionally equivalent — Leptos requires explicit declaration because it cannot spread arbitrary props.
4. **forceMount type**: React uses `true` literal type. Leptos uses `MaybeProp<bool>`.
5. **TabsContent children**: Leptos uses `Option<ChildrenFn>` (optional), React requires children via `PrimitiveDivProps`.

### Assessment

Excellent alignment. All 4 exported components match. All props are present with idiomatic Leptos types. The explicit event handler props on TabsTrigger are a necessary adaptation for Leptos's lack of prop spreading, not a divergence. The `ActivationMode` and `Orientation` enums are cleaner than React's string literals.

---

## Slider

### React API

**Exported components:** `Slider`, `SliderTrack`, `SliderRange`, `SliderThumb` (aliased as `Root`, `Track`, `Range`, `Thumb`)

**Slider** (renders `SliderHorizontal` or `SliderVertical` depending on orientation, wrapping `Primitive.span`)
- `name?: string`
- `disabled?: boolean` (default `false`)
- `orientation?: 'horizontal' | 'vertical'` (default `'horizontal'`)
- `dir?: 'ltr' | 'rtl'`
- `min?: number` (default `0`)
- `max?: number` (default `100`)
- `step?: number` (default `1`)
- `minStepsBetweenThumbs?: number` (default `0`)
- `value?: number[]`
- `defaultValue?: number[]` (default `[min]`)
- `onValueChange?: (value: number[]) => void`
- `onValueCommit?: (value: number[]) => void`
- `inverted?: boolean` (default `false`)
- `form?: string`
- Inherits HTML attributes via `SliderOrientationProps` chain (eventually `PrimitiveDivProps`)
- `ref` (forwarded)

**SliderTrack** (renders `Primitive.span`)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**SliderRange** (renders `Primitive.span`)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**SliderThumb** (renders `Primitive.span` via `SliderThumbImpl`)
- `name?: string` (per-thumb name override)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `Slider`, `SliderTrack`, `SliderRange`, `SliderThumb` (plus internal `SliderThumbImpl`, `SliderHorizontal`, `SliderVertical`, `SliderImpl`)

**Slider**
- `name: MaybeProp<String>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `orientation: MaybeProp<Orientation>` (optional, default `Horizontal`)
- `dir: MaybeProp<Direction>` (optional)
- `min: MaybeProp<f64>` (optional, default `0.0`)
- `max: MaybeProp<f64>` (optional, default `100.0`)
- `step: MaybeProp<f64>` (optional, default `1.0`)
- `min_steps_between_thumbs: MaybeProp<f64>` (optional, default `0.0`)
- `value: MaybeProp<Vec<f64>>` (optional)
- `default_value: MaybeProp<Vec<f64>>` (optional)
- `on_value_change: Option<Callback<Vec<f64>>>` (optional)
- `on_value_commit: Option<Callback<Vec<f64>>>` (optional)
- `inverted: MaybeProp<bool>` (optional)
- `form: MaybeProp<String>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**SliderTrack**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**SliderRange**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SliderThumb**
- `name: MaybeProp<String>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **Numeric types**: React uses `number`. Leptos uses `f64` for all numeric props.
2. **Value arrays**: React uses `number[]`. Leptos uses `Vec<f64>`.
3. **Orientation type**: React uses string literal `'horizontal' | 'vertical'`. Leptos uses `Orientation` enum.
4. **minStepsBetweenThumbs type**: React uses `number` (integer semantics). Leptos uses `f64`.

### Assessment

Excellent alignment. All 4 exported components match. Every prop is present. The `f64` vs `number` difference is a natural Rust/JS type mapping. The `form` prop is correctly implemented for form integration via hidden input elements.

---

## ScrollArea

### React API

**Exported components:** `ScrollArea`, `ScrollAreaViewport`, `ScrollAreaScrollbar`, `ScrollAreaThumb`, `ScrollAreaCorner` (aliased as `Root`, `Viewport`, `Scrollbar`, `Thumb`, `Corner`)

**ScrollArea** (renders `Primitive.div`)
- `type?: 'auto' | 'always' | 'scroll' | 'hover'` (default `'hover'`)
- `dir?: 'ltr' | 'rtl'`
- `scrollHideDelay?: number` (default `600`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**ScrollAreaViewport** (renders `Primitive.div`)
- `nonce?: string`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**ScrollAreaScrollbar** (renders `ScrollAreaScrollbarVisible` inside `Presence`, delegated per scroll type)
- `orientation?: 'horizontal' | 'vertical'` (default `'vertical'`)
- `forceMount?: true`
- Inherits all `div` HTML attributes via visible scrollbar chain
- `ref` (forwarded)

**ScrollAreaThumb** (renders `Primitive.div` inside `Presence`)
- `forceMount?: true`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**ScrollAreaCorner** (renders `Primitive.div` conditionally)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

### Leptos API

**Exported components:** `ScrollArea`, `ScrollAreaViewport`, `ScrollAreaScrollbar`, `ScrollAreaThumb`, `ScrollAreaCorner` (plus internal `ScrollAreaThumbImpl`, `ScrollAreaCornerImpl`, and various scrollbar strategy components)

**ScrollArea**
- `r#type: Option<ScrollAreaType>` (optional, default `Hover`)
- `dir: MaybeProp<Direction>` (optional)
- `scroll_hide_delay: Option<u32>` (optional, default `600`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ScrollAreaViewport**
- `nonce: Option<String>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ScrollAreaScrollbar**
- `orientation: Option<Orientation>` (optional, default `Vertical`)
- `force_mount: Option<bool>` (optional, default `false`)
- `class: MaybeProp<String>` (optional, Extra)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ScrollAreaThumb**
- `force_mount: Option<bool>` (optional, default `false`)
- `class: MaybeProp<String>` (optional, Extra)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ScrollAreaCorner**
- `class: MaybeProp<String>` (optional, Extra)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **type prop**: React uses string union `'auto' | 'always' | 'scroll' | 'hover'`. Leptos uses `ScrollAreaType` enum with `Auto`, `Always`, `Scroll`, `Hover` variants.
2. **scrollHideDelay type**: React uses `number`. Leptos uses `u32`.
3. **Extra `class` prop on Scrollbar, Thumb, Corner**: Leptos adds a `class: MaybeProp<String>` prop that is forwarded via context to bypass `Presence`/`Show` boundaries. This is a framework-specific necessity — React can spread `className` through Presence, but Leptos components with conditional rendering boundaries (Presence, Show) lose attribute passthrough. The `class` prop is forwarded via context (`ForwardedScrollbarClass`, `ForwardedThumbClass`, `ForwardedCornerClass`) to the inner implementation components.
4. **Orientation type**: React uses string literal `'horizontal' | 'vertical'`. Leptos uses `Orientation` enum.
5. **forceMount type**: React uses `true` literal type. Leptos uses `Option<bool>`.

### Assessment

Excellent alignment. All 5 exported components match. All React props are present. The extra `class` props on Scrollbar, Thumb, and Corner are a well-motivated framework adaptation to handle CSS class forwarding across Presence/Show rendering boundaries. The `nonce` prop on ScrollAreaViewport is correctly implemented.

---

## Toolbar

### React API

**Exported components:** `Toolbar`, `ToolbarSeparator`, `ToolbarButton`, `ToolbarLink`, `ToolbarToggleGroup`, `ToolbarToggleItem` (aliased as `Root`, `Separator`, `Button`, `Link`, `ToggleGroup`, `ToggleItem`)

**Toolbar** (renders `Primitive.div` inside `RovingFocusGroup.Root`)
- `orientation?: 'horizontal' | 'vertical'` (default `'horizontal'`)
- `loop?: boolean` (default `true`)
- `dir?: 'ltr' | 'rtl'`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**ToolbarSeparator** (renders `SeparatorPrimitive.Root`)
- Inherits all Separator props via `SeparatorProps`
- `ref` (forwarded)

**ToolbarButton** (renders `Primitive.button` inside `RovingFocusGroup.Item`)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)

**ToolbarLink** (renders `Primitive.a` inside `RovingFocusGroup.Item`)
- Inherits all `a` HTML attributes via `PrimitiveLinkProps`
- `ref` (forwarded)

**ToolbarToggleGroup** (renders `ToggleGroupPrimitive.Root` with `rovingFocus={false}`)
- Inherits all `ToggleGroupSingleProps | ToggleGroupMultipleProps` — the discriminated union from ToggleGroup
- `ref` (forwarded)

**ToolbarToggleItem** (renders `ToggleGroupPrimitive.Item` inside `ToolbarButton`)
- Inherits all `ToggleGroupItemProps` (includes `value: string`)
- `ref` (forwarded)

### Leptos API

**Exported components:** `Toolbar`, `ToolbarSeparator`, `ToolbarButton`, `ToolbarLink`, `ToolbarToggleGroup`, `ToolbarToggleItem`

**Toolbar**
- `orientation: Option<Orientation>` (optional, default `Horizontal`)
- `dir: MaybeProp<Direction>` (optional)
- `r#loop: MaybeProp<bool>` (optional, default `true`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ToolbarSeparator**
- `decorative: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**ToolbarButton**
- `disabled: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ToolbarLink**
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ToolbarToggleGroup**
- `r#type: ToggleGroupType` (required)
- `value: MaybeProp<Vec<String>>` (optional)
- `default_value: MaybeProp<Vec<String>>` (optional)
- `on_value_change: Option<Callback<Vec<String>>>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ToolbarToggleItem**
- `value: Signal<String>` (required)
- `disabled: MaybeProp<bool>` (optional)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Orientation type**: React uses string literal `'horizontal' | 'vertical'`. Leptos uses `Orientation` enum. Leptos stores it as a non-reactive value (`Orientation`, not `Signal<Orientation>`), matching React which also doesn't make it dynamic.
2. **ToolbarToggleGroup value type**: React uses a discriminated union — `type: 'single'` accepts `value?: string` and `onValueChange?: (value: string) => void`, while `type: 'multiple'` accepts `value?: string[]` and `onValueChange?: (value: string[]) => void`. Leptos uses a unified `Vec<String>` for both modes. For single mode, the vector contains at most one element.
3. **ToolbarToggleGroup missing props**: Leptos does not expose `orientation`, `dir`, or `loop` on `ToolbarToggleGroup`. React inherits these from `ToggleGroupImplProps`. In practice, the toolbar internally passes `roving_focus=false` and sets `orientation` and `dir` from context, so these props would be redundant, but React does expose them.
4. **ToolbarLink explicit event handler**: Leptos declares `on_key_down` explicitly for event handler composition. React composes from `...linkProps`.
5. **Extra `data-orientation` on Toolbar root**: Leptos sets `attr:data-orientation` on the root Toolbar div. React does not set this attribute.

### Assessment

Good alignment. All 6 exported components match. The unified `Vec<String>` value type on ToolbarToggleGroup is a deliberate simplification that avoids Rust's lack of discriminated union prop types. The missing `orientation`/`dir`/`loop` on ToolbarToggleGroup is harmless since the toolbar manages these internally via context and `roving_focus=false`.

---

## ToggleGroup

### React API

**Exported components:** `ToggleGroup`, `ToggleGroupItem` (aliased as `Root`, `Item`)

**ToggleGroup** (discriminated union on `type`)

When `type: 'single'`:
- `type: 'single'` (required)
- `value?: string` — controlled pressed item
- `defaultValue?: string` — default pressed item (uncontrolled)
- `onValueChange?: (value: string) => void`

When `type: 'multiple'`:
- `type: 'multiple'` (required)
- `value?: string[]` — controlled pressed items
- `defaultValue?: string[]` — default pressed items (uncontrolled)
- `onValueChange?: (value: string[]) => void`

Common props (via `ToggleGroupImplProps`):
- `disabled?: boolean` (default `false`)
- `rovingFocus?: boolean` (default `true`)
- `loop?: boolean` (default `true`)
- `orientation?: 'horizontal' | 'vertical'`
- `dir?: 'ltr' | 'rtl'`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**ToggleGroupItem** (renders `Toggle` inside optional `RovingFocusGroup.Item`)
- `value: string` (required)
- `disabled?: boolean`
- Inherits all `Toggle` props (excluding `defaultPressed`, `onPressedChange`)
- `ref` (forwarded)

### Leptos API

**Exported components:** `ToggleGroup`, `ToggleGroupItem` (plus internal `ToggleGroupImpl`, `ToggleGroupItemImpl`)

**ToggleGroup**
- `r#type: ToggleGroupType` (required) — `ToggleGroupType::Single` or `ToggleGroupType::Multiple`
- `value: MaybeProp<Vec<String>>` (optional)
- `default_value: MaybeProp<Vec<String>>` (optional)
- `on_value_change: Option<Callback<Vec<String>>>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `roving_focus: MaybeProp<bool>` (optional, default `true`)
- `r#loop: MaybeProp<bool>` (optional, default `true`)
- `orientation: MaybeProp<Orientation>` (optional)
- `dir: MaybeProp<Direction>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

**ToggleGroupItem**
- `value: Signal<String>` (required)
- `disabled: MaybeProp<bool>` (optional)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: ChildrenFn` (required)

### Differences

1. **Unified value type**: React uses a discriminated union — single mode has `value?: string` and multiple mode has `value?: string[]`. Leptos uses `Vec<String>` for both modes. For single mode, `toggle_group_activate` always produces a one-element vector, and `toggle_group_deactivate` produces an empty vector.
2. **ToggleGroupType enum**: React uses string literal `'single' | 'multiple'`. Leptos uses `ToggleGroupType` enum.
3. **onValueChange callback type**: React's single mode fires `(value: string) => void`, multiple mode fires `(value: string[]) => void`. Leptos always fires `Callback<Vec<String>>`.
4. **ToggleGroupItem value type**: React uses `value: string` (plain string). Leptos uses `value: Signal<String>` (reactive signal), allowing dynamic value changes.
5. **ToggleGroupItemImpl renders button directly**: React's `ToggleGroupItemImpl` wraps the `Toggle` component and overrides `aria-pressed` to `undefined` for single mode. Leptos renders the button directly with correct ARIA attributes per mode, avoiding the need to override attributes from a child component.
6. **Extra `on_click` on ToggleGroupItem**: Leptos declares `on_click` explicitly for event composition. React receives it via `...itemProps` spread.

### Assessment

Good alignment. Both exported components match. The unified `Vec<String>` value type is a pragmatic adaptation — Rust does not support discriminated union props in the same way TypeScript does. The `ToggleGroupItemImpl` rendering the button directly instead of wrapping `Toggle` is a well-documented design decision that avoids attribute-override complications in Leptos's component model. The `Signal<String>` on ToggleGroupItem's `value` prop enables reactivity that React's plain string does not.

---

## Select

### React API

**Exported components (16):** `Select`, `SelectTrigger`, `SelectValue`, `SelectIcon`, `SelectPortal`, `SelectContent`, `SelectViewport`, `SelectGroup`, `SelectLabel`, `SelectItem`, `SelectItemText`, `SelectItemIndicator`, `SelectScrollUpButton`, `SelectScrollDownButton`, `SelectSeparator`, `SelectArrow` (aliased as `Root`, `Trigger`, `Value`, `Icon`, `Portal`, `Content`, `Viewport`, `Group`, `Label`, `Item`, `ItemText`, `ItemIndicator`, `ScrollUpButton`, `ScrollDownButton`, `Separator`, `Arrow`)

**Select** (functional component, no DOM element)
- `value?: string`
- `defaultValue?: string`
- `onValueChange?: (value: string) => void`
- `open?: boolean`
- `defaultOpen?: boolean`
- `onOpenChange?: (open: boolean) => void`
- `dir?: 'ltr' | 'rtl'`
- `name?: string`
- `autoComplete?: string`
- `disabled?: boolean`
- `required?: boolean`
- `form?: string`
- `children?: ReactNode`

**SelectTrigger** (renders `Primitive.button` inside `PopperPrimitive.Anchor`)
- `disabled?: boolean` (defaults to context disabled)
- Inherits all `button` HTML attributes via `PrimitiveButtonProps`
- `ref` (forwarded)

**SelectValue** (renders `Primitive.span`)
- `placeholder?: ReactNode`
- Inherits all `span` HTML attributes (excluding native `placeholder`) via `PrimitiveSpanProps`
- `ref` (forwarded)

**SelectIcon** (renders `Primitive.span`)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**SelectPortal** (renders `PortalPrimitive`)
- `container?: Element | DocumentFragment | null`
- `children?: ReactNode`

**SelectContent** (renders `SelectContentImpl` when open, DocumentFragment when closed)
- `position?: 'item-aligned' | 'popper'` (default `'item-aligned'`)
- `onCloseAutoFocus?: (event: Event) => void`
- `onEscapeKeyDown?: (event: KeyboardEvent) => void`
- `onPointerDownOutside?: (event: PointerDownOutsideEvent) => void`
- When `position='popper'`, inherits `PopperContentProps`: `side`, `sideOffset`, `align`, `alignOffset`, `arrowPadding`, `avoidCollisions`, `collisionBoundary`, `collisionPadding`, `sticky`, `hideWhenDetached`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectViewport** (renders `Primitive.div`)
- `nonce?: string`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectGroup** (renders `Primitive.div`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectLabel** (renders `Primitive.div`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectItem** (renders `Primitive.div`)
- `value: string` (required)
- `disabled?: boolean` (default `false`)
- `textValue?: string`
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectItemText** (renders `Primitive.span`)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**SelectItemIndicator** (renders `Primitive.span`)
- Inherits all `span` HTML attributes via `PrimitiveSpanProps`
- `ref` (forwarded)

**SelectScrollUpButton** (renders `Primitive.div`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectScrollDownButton** (renders `Primitive.div`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectSeparator** (renders `Primitive.div`)
- Inherits all `div` HTML attributes via `PrimitiveDivProps`
- `ref` (forwarded)

**SelectArrow** (renders `PopperPrimitive.Arrow`)
- Inherits all `PopperArrowProps` (includes `width`, `height`)
- `ref` (forwarded)

### Leptos API

**Exported components (16):** `Select`, `SelectTrigger`, `SelectValue`, `SelectIcon`, `SelectPortal`, `SelectContent`, `SelectViewport`, `SelectGroup`, `SelectLabel`, `SelectItem`, `SelectItemText`, `SelectItemIndicator`, `SelectScrollUpButton`, `SelectScrollDownButton`, `SelectSeparator`, `SelectArrow` (plus internal `SelectContentImpl`, `SelectBubbleInput`, `SelectScrollButtonImpl`, etc.)

**Select**
- `open: MaybeProp<bool>` (optional)
- `default_open: MaybeProp<bool>` (optional)
- `on_open_change: Option<Callback<bool>>` (optional)
- `value: MaybeProp<String>` (optional)
- `default_value: MaybeProp<String>` (optional)
- `on_value_change: Option<Callback<String>>` (optional)
- `dir: MaybeProp<Direction>` (optional)
- `name: MaybeProp<String>` (optional)
- `auto_complete: MaybeProp<String>` (optional)
- `disabled: MaybeProp<bool>` (optional)
- `required: MaybeProp<bool>` (optional)
- `form: MaybeProp<String>` (optional)
- `children: ChildrenFn` (required)

**SelectTrigger**
- `on_click: Option<Callback<ev::MouseEvent>>` (optional, Extra)
- `on_pointer_down: Option<Callback<ev::PointerEvent>>` (optional, Extra)
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (optional, Extra)
- `disabled: MaybeProp<bool>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectValue**
- `placeholder: MaybeProp<String>` (optional)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectIcon**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectPortal**
- `container: MaybeProp<SendWrapper<web_sys::Element>>` (optional)
- `container_ref: AnyNodeRef` (optional, Extra)
- `force_mount: MaybeProp<bool>` (optional, Extra)
- `children: ChildrenFn` (required)

**SelectContent**
- `position: MaybeProp<String>` (optional)
- `on_close_auto_focus: Option<Callback<web_sys::Event>>` (optional)
- `on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>` (optional)
- `on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>` (optional)
- `side: Signal<Side>` (optional, default `Side::Bottom`)
- `side_offset: Signal<f64>` (optional, default `0.0`)
- `align: Signal<Align>` (optional, default `Align::Start`)
- `align_offset: Signal<f64>` (optional, default `0.0`)
- `arrow_padding: Signal<f64>` (optional, default `0.0`)
- `avoid_collisions: Signal<bool>` (optional, default `true`)
- `collision_boundary: Signal<SendWrapper<Vec<web_sys::Element>>>` (optional)
- `collision_padding: Signal<Padding>` (optional, default `Padding::All(10.0)`)
- `sticky: Signal<Sticky>` (optional, default `Sticky::Partial`)
- `hide_when_detached: Signal<bool>` (optional, default `false`)
- `update_position_strategy: Signal<UpdatePositionStrategy>` (optional, default `Optimized`)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectViewport**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectGroup**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectLabel**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectItem**
- `value: String` (required)
- `disabled: MaybeProp<bool>` (optional)
- `text_value: MaybeProp<String>` (optional)
- `on_pointer_up: Option<Callback<ev::PointerEvent>>` (optional, Extra)
- `on_pointer_down: Option<Callback<ev::PointerEvent>>` (optional, Extra)
- `on_pointer_move: Option<Callback<ev::PointerEvent>>` (optional, Extra)
- `on_pointer_leave: Option<Callback<ev::PointerEvent>>` (optional, Extra)
- `on_key_down: Option<Callback<ev::KeyboardEvent>>` (optional, Extra)
- `on_focus: Option<Callback<ev::FocusEvent>>` (optional, Extra)
- `on_blur: Option<Callback<ev::FocusEvent>>` (optional, Extra)
- `on_click: Option<Callback<ev::MouseEvent>>` (optional, Extra)
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectItemText**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectItemIndicator**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectScrollUpButton**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectScrollDownButton**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectSeparator**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `children: Option<ChildrenFn>` (optional)

**SelectArrow**
- `as_child: MaybeProp<bool>` (optional)
- `node_ref: AnyNodeRef` (optional)
- `width: Signal<f64>` (optional, default `10.0`)
- `height: Signal<f64>` (optional, default `5.0`)
- `children: Option<ChildrenFn>` (optional)

### Differences

1. **SelectValue placeholder type**: React uses `ReactNode` (any renderable content). Leptos uses `MaybeProp<String>` (string only). This means Leptos cannot render complex placeholder content (e.g., styled fragments).
2. **SelectContent position type**: React uses string literal `'item-aligned' | 'popper'` (default `'item-aligned'`). Leptos uses `MaybeProp<String>` (no enum, stored as string). Leptos has no dedicated `SelectItemAlignedPosition` component -- the `position` value is checked as a string in the implementation.
3. **SelectContent default position**: React defaults to `'item-aligned'`. Leptos defaults to `'popper'` (the `hidden_content_context` provides `"popper"` as the default position string). This is a behavioral divergence.
4. **SelectViewport missing nonce prop**: Leptos's `SelectViewport` does not expose a `nonce` prop, though `ScrollAreaViewport` does. The style tag in SelectViewport is inline without nonce support.
5. **SelectPortal extra props**: Leptos adds `container_ref: AnyNodeRef` and `force_mount: MaybeProp<bool>` which React's SelectPortal does not have. These come from the underlying `ScopedPortal` component.
6. **Explicit event handler props on SelectTrigger and SelectItem**: Leptos declares many explicit event handler props (`on_click`, `on_pointer_down`, `on_key_down`, `on_focus`, `on_blur`, `on_pointer_up`, `on_pointer_move`, `on_pointer_leave`) for event composition. React receives these via prop spreading. SelectItem has the most extensive set of explicit handlers.
7. **SelectContent popper props**: Leptos exposes all `PopperContent` props directly on `SelectContent` (`side`, `side_offset`, `align`, `align_offset`, `arrow_padding`, `avoid_collisions`, `collision_boundary`, `collision_padding`, `sticky`, `hide_when_detached`, `update_position_strategy`). React inherits these through the type system via `SelectPopperPositionProps extends PopperContentProps`. Both expose the same props; the mechanism differs.
8. **SelectContent default align**: Leptos defaults `align` to `Align::Start`. React's PopperContent defaults `align` to `'center'`. This may cause visual differences in popper-positioned select content.
9. **SelectContent default collision_padding**: Leptos defaults to `Padding::All(10.0)`. React's PopperContent defaults `collisionPadding` to `0`. This padding difference affects boundary collision detection.

### Assessment

Good alignment. All 16 exported components match. The most significant divergences are: (a) the `position` default differing between `'item-aligned'` (React) and `'popper'` (Leptos), (b) `SelectValue` placeholder being string-only in Leptos vs ReactNode in React, (c) `SelectViewport` missing the `nonce` prop, and (d) different defaults for `align` and `collisionPadding` on SelectContent. The explicit event handler props on SelectTrigger and SelectItem are a framework necessity, not a divergence. The form integration (hidden native select via `SelectBubbleInput`) is correctly implemented with `name`, `autoComplete`, `form`, `disabled`, and `required` support.

---
