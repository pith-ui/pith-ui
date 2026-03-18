# API Reference Notes

Instructions for creating and maintaining API reference documentation in `notes/API/`.

## Purpose

These notes capture the public API of every component in the library — both the React Radix original and our Leptos port — as the source material for the future documentation site. They are not the final docs; they are structured research notes that make doc-site authoring straightforward.

## Directory Structure

```
notes/API/
├── CLAUDE.md          ← this file
├── TEMPLATE.md        ← per-part template
├── accordion/
│   ├── accordion.md           ← root part (includes component-level sections)
│   ├── accordion-item.md
│   ├── accordion-header.md
│   ├── accordion-trigger.md
│   └── accordion-content.md
├── collapsible/
│   ├── collapsible.md
│   ├── collapsible-trigger.md
│   └── collapsible-content.md
└── ...
```

- One directory per component (e.g. `accordion/`, `dialog/`, `tooltip/`).
- One markdown file per component **part** (e.g. `accordion-item.md`, `dialog-trigger.md`).
- The **root part** file (named after the component itself, e.g. `accordion.md`) includes the component-level sections: Anatomy, Usage Examples, Accessibility, and CSS Custom Properties.

## How to Create Notes for a Component

### Step 1: Identify all parts

Read the React source at `reference/react-radix-primitives/packages/react/<component>/src/` and the Leptos source at `packages/primitives/leptos/src/components/<component>/`. List every publicly exported component part.

### Step 2: Create the directory and files

```bash
mkdir -p notes/API/<component>
```

Create one file per part using `TEMPLATE.md` as the starting point.

### Step 3: Fill in each part file

For each part, read both the React and Leptos source to populate:

1. **React Signature** — Copy the `forwardRef` signature and full TypeScript prop interface(s), including any inherited/extended types. Include JSDoc comments from the source.

2. **Leptos Signature** — Copy the full `#[component]` function signature with all `#[prop(...)]` attributes.

3. **Prop Mapping Table** — One row per prop. Columns:
   - `React Prop` — the React prop name (camelCase)
   - `Leptos Prop` — the Leptos prop name (snake_case)
   - `Type (React)` — the TypeScript type, including defaults
   - `Type (Leptos)` — the Rust type, including defaults
   - `Description` — what the prop does, when to use it, and any behavioral nuance

   Include rows for `ref`/`node_ref`, `asChild`/`as_child`, and `*(spread)*` where applicable.

4. **Data Attributes** — Table of `data-*` attributes rendered on the DOM element.

5. **Implicit Behavior** — Only if the part has auto-generated IDs, inherited context, conditional ARIA attributes, or other behavior not visible through props.

### Step 4: Fill in root-only sections

The root part file (e.g. `accordion.md`) additionally includes:

1. **Anatomy** — Tree diagram of the expected nesting structure, plus minimal skeleton code in both React and Leptos.

2. **Usage Examples** — Side-by-side React/Leptos snippets covering at minimum:
   - Basic uncontrolled usage
   - Controlled usage
   - Any component-specific patterns (animation, disabled items, etc.)

   Keep examples minimal — just enough to show the API shape.

   **Leptos prop syntax rules:**
   - Props with `#[prop(into)]` accept `&str` directly for `MaybeProp<String>`
   - Props without `#[prop(into)]` (bare `String`) need `.to_string()`
   - `Vec<String>` values need `.into()` per element

3. **Accessibility** — Document:
   - Which WAI-ARIA pattern is implemented (link to spec)
   - Keyboard interaction table (key → behavior)
   - ARIA attributes table (element, attribute, value, notes)
   - Behavioral notes (gotchas, edge cases)

4. **CSS Custom Properties** — Only if the component exposes any. Table with property name, source, and description.

### Step 5: Review

Verify accuracy by cross-referencing:
- React source for prop interfaces and JSDoc
- Leptos source for `#[component]` signatures and `#[prop(...)]` annotations
- React storybook for usage patterns
- Leptos stories for idiomatic Leptos usage
- React test file for accessibility and keyboard behavior

## Reference Material Locations

| What | Where |
|---|---|
| React source | `reference/react-radix-primitives/packages/react/<component>/src/` |
| Leptos source | `packages/primitives/leptos/src/components/<component>/` |
| React storybook | `reference/react-radix-primitives/apps/storybook/stories/<component>.stories.tsx` |
| Leptos stories | `stories/leptos/src/primitives/<component>.rs` |
| React tests | `reference/react-radix-primitives/packages/react/<component>/src/<component>.test.tsx` |
| CSS modules | `stories/leptos/src/primitives/<component>.stories.module.css` |

## Flagging Issues

During research, if you notice something that seems off, inconsistent, or might need addressing — for example:

- A React prop that exists but has no Leptos equivalent
- A Leptos prop whose behavior diverges from React in a way that may be unintentional
- Missing ARIA attributes or keyboard interactions
- A data attribute that React sets but Leptos does not (or vice versa)
- A CSS custom property that isn't being forwarded correctly

**Add a task to `tasks.json`** so it can be investigated later. Follow the existing ID scheme (`<component>-<principle-abbrev>-<n>`) and use `"principle": "api-docs"` for issues found during API documentation. Example:

```json
{
  "id": "tooltip-api-docs-1",
  "component": "tooltip",
  "principle": "api-docs",
  "description": "React TooltipContent accepts `side` and `align` props but the Leptos version only has `side`. Check if `align` was intentionally omitted or missed during porting.",
  "status": "open",
  "blockers": []
}
```

Include enough context in the description for someone to investigate without re-reading the full source.

## Worked Example

See `notes/API/accordion/` for a complete example covering all five parts. Use it as a reference for tone, detail level, and structure.
