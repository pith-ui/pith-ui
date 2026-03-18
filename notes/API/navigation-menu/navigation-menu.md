# NavigationMenu (Root)

## Anatomy

The expected component nesting structure:

```
NavigationMenu
├── NavigationMenuList
│   ├── NavigationMenuItem (one per top-level entry)
│   │   ├── NavigationMenuTrigger (for items with dropdown content)
│   │   ├── NavigationMenuContent (dropdown panel)
│   │   │   └── NavigationMenuLink (one per link inside content)
│   │   │   └── NavigationMenuSub (optional nested submenu)
│   │   │       ├── NavigationMenuList
│   │   │       │   └── NavigationMenuItem ...
│   │   │       └── NavigationMenuViewport (optional, for submenu)
│   │   └── NavigationMenuLink (for items without dropdown — direct link)
│   └── NavigationMenuIndicator (optional, animated active indicator)
├── NavigationMenuViewport (optional, renders content in a shared viewport)
```

### React

```tsx
<NavigationMenu.Root>
  <NavigationMenu.List>
    <NavigationMenu.Item>
      <NavigationMenu.Trigger>Products</NavigationMenu.Trigger>
      <NavigationMenu.Content>
        <NavigationMenu.Link href="/product-a">Product A</NavigationMenu.Link>
        <NavigationMenu.Link href="/product-b">Product B</NavigationMenu.Link>
      </NavigationMenu.Content>
    </NavigationMenu.Item>

    <NavigationMenu.Item>
      <NavigationMenu.Link href="/about">About</NavigationMenu.Link>
    </NavigationMenu.Item>

    <NavigationMenu.Indicator />
  </NavigationMenu.List>

  <NavigationMenu.Viewport />
</NavigationMenu.Root>
```

### Leptos

```rust
<NavigationMenu>
  <NavigationMenuList>
    <NavigationMenuItem>
      <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
      <NavigationMenuContent>
        <NavigationMenuLink attr:href="/product-a">"Product A"</NavigationMenuLink>
        <NavigationMenuLink attr:href="/product-b">"Product B"</NavigationMenuLink>
      </NavigationMenuContent>
    </NavigationMenuItem>

    <NavigationMenuItem>
      <NavigationMenuLink attr:href="/about">"About"</NavigationMenuLink>
    </NavigationMenuItem>

    <NavigationMenuIndicator />
  </NavigationMenuList>

  <NavigationMenuViewport />
</NavigationMenu>
```

## React Signature

```typescript
type NavigationMenuElement = React.ComponentRef<typeof Primitive.nav>;
type PrimitiveNavProps = React.ComponentPropsWithoutRef<typeof Primitive.nav>;

interface NavigationMenuProps extends PrimitiveNavProps {
  value?: string;
  defaultValue?: string;
  onValueChange?: (value: string) => void;
  dir?: Direction; // 'ltr' | 'rtl'
  orientation?: Orientation; // 'horizontal' | 'vertical'
  /**
   * The duration from when the pointer enters the trigger until the tooltip gets opened.
   * @defaultValue 200
   */
  delayDuration?: number;
  /**
   * How much time a user has to enter another trigger without incurring a delay again.
   * @defaultValue 300
   */
  skipDelayDuration?: number;
}

const NavigationMenu = React.forwardRef<NavigationMenuElement, NavigationMenuProps>(...)
```

## Leptos Signature

```rust
pub fn NavigationMenu(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))]
    orientation: MaybeProp<Orientation>,
    #[prop(into, optional, default = MaybeProp::from(200.0))] delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
```

## Prop Mapping

| React Prop | Leptos Prop | Type (React) | Type (Leptos) | Description |
|---|---|---|---|---|
| `value` | `value` | `string \| undefined` | `MaybeProp<String>` | The controlled value of the currently active/open item. An empty string `""` means no item is open. When set, the component becomes controlled. |
| `defaultValue` | `default_value` | `string \| undefined` | `MaybeProp<String>` | The value of the item that is open on initial render. Use when you do not need to control the menu state. Defaults to `""` (no item open). |
| `onValueChange` | `on_value_change` | `(value: string) => void` | `Option<Callback<String>>` | Callback fired when the active item changes. Receives the new value string (or `""` when all items close). |
| `dir` | `dir` | `'ltr' \| 'rtl'` | `MaybeProp<Direction>` | The reading direction. Affects horizontal arrow-key navigation and the `data-motion` animation direction on content. |
| `orientation` | `orientation` | `'horizontal' \| 'vertical'` (default `'horizontal'`) | `MaybeProp<Orientation>` (default `Horizontal`) | The layout axis. Controls which arrow keys navigate between triggers. Note: React defaults to `'horizontal'`; Leptos matches this. |
| `delayDuration` | `delay_duration` | `number` (default `200`) | `MaybeProp<f64>` (default `200.0`) | Milliseconds from pointer entering a trigger until the content opens. Set to `0` for instant open on hover. |
| `skipDelayDuration` | `skip_delay_duration` | `number` (default `300`) | `MaybeProp<f64>` (default `300.0`) | Milliseconds a user has to move to another trigger without incurring the delay again. Once a menu is open, subsequent triggers open instantly within this window. Set to `0` to always require the full delay. |
| `ref` | `node_ref` | `React.Ref` | `AnyNodeRef` | Ref to the root DOM element (`<nav>`). |
| `asChild` | `as_child` | `boolean` | `MaybeProp<bool>` | When `true`, renders the child directly instead of wrapping in a `<nav>`, merging props and refs. |
| *(spread)* | -- | `...PrimitiveNavProps` | -- | React allows spreading any `<nav>` HTML attribute. Leptos uses `attr:` directives instead. |

### Data attributes (rendered on DOM)

| Attribute | Value | Description |
|---|---|---|
| `data-orientation` | `"horizontal" \| "vertical"` | Reflects the `orientation` prop. |

### Implicit behavior

- Renders with `aria-label="Main"` hardcoded on the `<nav>` element.
- Provides context for all descendant parts (value state, timers, direction, orientation).
- The value `""` (empty string) represents the "closed" state -- no item is open.

## Usage Examples

### Basic (uncontrolled, inline content)

#### React

```tsx
<NavigationMenu.Root>
  <NavigationMenu.List>
    <NavigationMenu.Item>
      <NavigationMenu.Trigger>Products</NavigationMenu.Trigger>
      <NavigationMenu.Content>
        <NavigationMenu.Link href="/widgets">Widgets</NavigationMenu.Link>
        <NavigationMenu.Link href="/gadgets">Gadgets</NavigationMenu.Link>
      </NavigationMenu.Content>
    </NavigationMenu.Item>

    <NavigationMenu.Item>
      <NavigationMenu.Link href="/about">About</NavigationMenu.Link>
    </NavigationMenu.Item>
  </NavigationMenu.List>
</NavigationMenu.Root>
```

#### Leptos

```rust
<NavigationMenu>
  <NavigationMenuList>
    <NavigationMenuItem>
      <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
      <NavigationMenuContent>
        <NavigationMenuLink attr:href="/widgets">"Widgets"</NavigationMenuLink>
        <NavigationMenuLink attr:href="/gadgets">"Gadgets"</NavigationMenuLink>
      </NavigationMenuContent>
    </NavigationMenuItem>

    <NavigationMenuItem>
      <NavigationMenuLink attr:href="/about">"About"</NavigationMenuLink>
    </NavigationMenuItem>
  </NavigationMenuList>
</NavigationMenu>
```

### Controlled

#### React

```tsx
const [value, setValue] = React.useState('');

<NavigationMenu.Root value={value} onValueChange={setValue}>
  {/* ...list and items */}
</NavigationMenu.Root>
```

#### Leptos

```rust
let (value, set_value) = signal(String::new());

<NavigationMenu
  value=value
  on_value_change=Callback::new(move |v: String| set_value.set(v))
>
  // ...list and items
</NavigationMenu>
```

### With Viewport (shared content rendering area)

#### React

```tsx
<NavigationMenu.Root>
  <NavigationMenu.List>
    <NavigationMenu.Item>
      <NavigationMenu.Trigger>Products</NavigationMenu.Trigger>
      <NavigationMenu.Content style={{ width: 600 }}>
        {/* Content renders inside the viewport below */}
      </NavigationMenu.Content>
    </NavigationMenu.Item>

    <NavigationMenu.Indicator>
      <div className="arrow" />
    </NavigationMenu.Indicator>
  </NavigationMenu.List>

  <NavigationMenu.Viewport />
</NavigationMenu.Root>
```

#### Leptos

```rust
<NavigationMenu>
  <NavigationMenuList>
    <NavigationMenuItem>
      <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
      <NavigationMenuContent attr:style="width: 600px;">
        // Content renders inside the viewport below
      </NavigationMenuContent>
    </NavigationMenuItem>

    <NavigationMenuIndicator>
      <div class="arrow" />
    </NavigationMenuIndicator>
  </NavigationMenuList>

  <NavigationMenuViewport />
</NavigationMenu>
```

### Custom delay durations

#### React

```tsx
<NavigationMenu.Root delayDuration={0} skipDelayDuration={0}>
  {/* Instant open, never skip delay */}
</NavigationMenu.Root>
```

#### Leptos

```rust
<NavigationMenu delay_duration=0.0 skip_delay_duration=0.0>
  // Instant open, never skip delay
</NavigationMenu>
```

### With submenu

#### React

```tsx
<NavigationMenu.Root>
  <NavigationMenu.List>
    <NavigationMenu.Item>
      <NavigationMenu.Trigger>Products</NavigationMenu.Trigger>
      <NavigationMenu.Content>
        <NavigationMenu.Sub defaultValue="widgets">
          <NavigationMenu.List>
            <NavigationMenu.Item value="widgets">
              <NavigationMenu.Trigger>Widgets</NavigationMenu.Trigger>
              <NavigationMenu.Content>...</NavigationMenu.Content>
            </NavigationMenu.Item>
          </NavigationMenu.List>
          <NavigationMenu.Viewport />
        </NavigationMenu.Sub>
      </NavigationMenu.Content>
    </NavigationMenu.Item>
  </NavigationMenu.List>
</NavigationMenu.Root>
```

#### Leptos

```rust
<NavigationMenu>
  <NavigationMenuList>
    <NavigationMenuItem>
      <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
      <NavigationMenuContent>
        <NavigationMenuSub default_value="widgets">
          <NavigationMenuList>
            <NavigationMenuItem value="widgets">
              <NavigationMenuTrigger>"Widgets"</NavigationMenuTrigger>
              <NavigationMenuContent>"..."</NavigationMenuContent>
            </NavigationMenuItem>
          </NavigationMenuList>
          <NavigationMenuViewport />
        </NavigationMenuSub>
      </NavigationMenuContent>
    </NavigationMenuItem>
  </NavigationMenuList>
</NavigationMenu>
```

## Accessibility

Implements a navigation pattern based on the [WAI-ARIA Disclosure Navigation Menu with Top-Level Links](https://www.w3.org/WAI/ARIA/apd/patterns/disclosure-navigation/) pattern, with hover-intent behavior.

### Keyboard Interactions

| Key | Description |
|---|---|
| `Enter` / `Space` | When focus is on a trigger, toggles the associated content open/closed. When focus is on a link, activates the link. |
| `ArrowDown` | When `orientation="horizontal"` and a trigger is focused and open: moves focus into the content area (first tabbable element). When `orientation="vertical"`: moves focus to the next trigger/link. |
| `ArrowUp` | When `orientation="vertical"`: moves focus to the previous trigger/link. |
| `ArrowRight` | When `orientation="horizontal"` (LTR): moves focus to the next trigger/link. When `orientation="vertical"` and trigger is open: moves focus into content. In RTL, direction is reversed. |
| `ArrowLeft` | When `orientation="horizontal"` (LTR): moves focus to the previous trigger/link. In RTL, direction is reversed. |
| `Home` | Moves focus to the first trigger/link in the list. |
| `End` | Moves focus to the last trigger/link in the list. |
| `Escape` | Closes the currently open content and returns focus to its trigger. |
| `Tab` | When inside content, moves focus through tabbable elements. At content edges, moves focus to the focus proxy to allow natural tab flow out of the menu. |

### ARIA Attributes

| Element | Attribute | Value | Notes |
|---|---|---|---|
| `NavigationMenu` (root) | `aria-label` | `"Main"` | Hardcoded label on the `<nav>` element identifying it as the main navigation. |
| `NavigationMenuTrigger` | `aria-expanded` | `true` / `false` | Whether the trigger's associated content is open. |
| `NavigationMenuTrigger` | `aria-controls` | `string` | Auto-generated ID pointing to the associated content element. |
| `NavigationMenuContent` | `aria-labelledby` | `string` | Auto-generated ID pointing to the associated trigger element. |
| `NavigationMenuLink` | `aria-current` | `"page"` / absent | Set to `"page"` when the `active` prop is `true`. |
| `NavigationMenuIndicator` | `aria-hidden` | `"true"` | The indicator is decorative and hidden from the accessibility tree. |

### Behavioral Notes

- The navigation menu uses hover-intent with configurable delays (`delayDuration`, `skipDelayDuration`) to avoid accidental opens.
- Only mouse pointer events trigger hover behavior; touch events use click/tap instead.
- A 150ms close timer prevents the menu from closing when the pointer briefly leaves the trigger/content area (e.g., moving between trigger and content).
- When a viewport is present, content is portaled into it; otherwise content renders inline adjacent to the trigger.
- The indicator is portaled into the list's wrapper `<div>` (the indicator track) and positioned absolutely to track the active trigger.
- Focus management between trigger and content uses a visually hidden focus proxy element to maintain natural tab order.
- When content is open and rendered in a viewport, an `aria-owns` span restructures the accessibility tree so screen readers associate the content with the trigger.

## CSS Custom Properties

| Property | Source | Description |
|---|---|---|
| `--radix-navigation-menu-viewport-width` | Set on `NavigationMenuViewport` | The measured width (in px) of the currently active content inside the viewport. Use for width animations/transitions. |
| `--radix-navigation-menu-viewport-height` | Set on `NavigationMenuViewport` | The measured height (in px) of the currently active content inside the viewport. Use for height animations/transitions. |
