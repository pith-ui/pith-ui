# Leptos Porting Status Inventory

Last updated: 2026-02-22

## Summary

| Metric                                            | Count            |
| ------------------------------------------------- | ---------------- |
| React Radix Primitive packages                    | 59               |
| Leptos package directories                        | 46               |
| Ported to modern Leptos 0.8 API                   | 44               |
| Ported but on old Leptos API (not in workspace)   | 2 (avatar, menu) |
| Not ported (public UI components)                 | 5                |
| React packages with no Leptos equivalent needed   | 7                |
| Stories verified (tested_story)                   | 32               |
| Stories needing verification                      | 4                |
| E2E tests passing both React & Leptos             | 20               |
| E2E tests written (React only, Leptos not ported) | 4                |

## Public UI Components

| Component               | Ported | In Workspace | API     | Story         | Story Verified | E2E (React) | E2E (Leptos) | Notes                                                                       |
| ----------------------- | ------ | ------------ | ------- | ------------- | -------------- | ----------- | ------------ | --------------------------------------------------------------------------- |
| accessible-icon         | yes    | yes          | 0.8     | active        | no             | -           | -            |                                                                             |
| accordion               | yes    | yes          | 0.8     | active        | yes            | 33/33       | 33/33        |                                                                             |
| alert-dialog            | yes    | yes          | 0.8     | active        | yes            | 30/30       | 30/30        |                                                                             |
| announce                | yes    | yes          | 0.8     | n/a           | n/a            | -           | -            | No visual story                                                             |
| arrow                   | yes    | yes          | 0.8     | active        | yes            | -           | -            |                                                                             |
| aspect-ratio            | yes    | yes          | 0.8     | active        | yes            | -           | -            |                                                                             |
| avatar                  | yes    | **no**       | **old** | commented out | no             | -           | -            | Uses `create_signal`, `NodeRef<AnyElement>`, `#[prop(attrs)]`, `on_cleanup` |
| checkbox                | yes    | yes          | 0.8     | active        | yes            | 23/23       | 23/23        |                                                                             |
| collapsible             | yes    | yes          | 0.8     | active        | yes            | 18/18       | 18/18        |                                                                             |
| context-menu            | **no** | -            | -       | -             | -              | 31/31       | -            | Depends on menu                                                             |
| dialog                  | yes    | yes          | 0.8     | active        | yes            | 31/31       | 31/31        |                                                                             |
| dropdown-menu           | **no** | -            | -       | -             | -              | 42/42       | -            | Depends on menu                                                             |
| form                    | yes    | yes          | 0.8     | active        | yes            | 16/16       | 16/16        |                                                                             |
| hover-card              | yes    | yes          | 0.8     | active        | yes            | 12/12       | 12/12        |                                                                             |
| label                   | yes    | yes          | 0.8     | active        | yes            | -           | -            |                                                                             |
| menu                    | yes    | **no**       | **old** | commented out | no             | -           | -            | Internal; 7 stub components; old API                                        |
| menubar                 | **no** | -            | -       | -             | -              | -           | -            | Depends on menu                                                             |
| navigation-menu         | yes    | yes          | 0.8     | active        | yes            | 42/42       | 42/42        | See [[leptos-navigation-menu#Known issues]]                                 |
| one-time-password-field | yes    | yes          | 0.8     | active        | no             | -           | -            | Unstable/preview                                                            |
| password-toggle-field   | yes    | yes          | 0.8     | active        | yes            | -           | -            |                                                                             |
| popover                 | yes    | yes          | 0.8     | active        | yes            | 26/26       | 26/26        |                                                                             |
| progress                | yes    | yes          | 0.8     | active        | yes            | 26/26       | 26/26        |                                                                             |
| radio-group             | yes    | yes          | 0.8     | active        | yes            | 29/29       | 29/29        |                                                                             |
| scroll-area             | yes    | yes          | 0.8     | active        | yes            | 22/22       | 22/22        |                                                                             |
| select                  | **no** | -            | -       | -             | -              | 39/39       | -            | Large dependency list                                                       |
| separator               | yes    | yes          | 0.8     | active        | yes            | 9/9         | 9/9          |                                                                             |
| slider                  | yes    | yes          | 0.8     | active        | yes            | 45/45       | 45/45        |                                                                             |
| switch                  | yes    | yes          | 0.8     | active        | yes            | 21/21       | 21/21        |                                                                             |
| tabs                    | yes    | yes          | 0.8     | active        | yes            | 41/41       | 41/41        |                                                                             |
| toast                   | **no** | -            | -       | -             | -              | 19/19       | -            |                                                                             |
| toggle                  | yes    | yes          | 0.8     | active        | yes            | 15/15       | 15/15        |                                                                             |
| toggle-group            | yes    | yes          | 0.8     | active        | yes            | 41/41       | 41/41        |                                                                             |
| toolbar                 | yes    | yes          | 0.8     | active        | yes            | 32/32       | 32/32        |                                                                             |
| tooltip                 | yes    | yes          | 0.8     | active        | yes            | 17/17       | 17/17        |                                                                             |
| visually-hidden         | yes    | yes          | 0.8     | active        | yes            | -           | -            |                                                                             |

## Internal Infrastructure / Utilities

| Package                | Leptos Equivalent             | Ported | In Workspace | Notes                                                 |
| ---------------------- | ----------------------------- | ------ | ------------ | ----------------------------------------------------- |
| collection             | leptos-collection             | yes    | yes          |                                                       |
| compose-refs           | leptos-compose-refs           | yes    | yes          |                                                       |
| context                | Native Leptos context         | yes    | n/a          | No separate package needed                            |
| direction              | leptos-direction              | yes    | yes          |                                                       |
| dismissable-layer      | leptos-dismissable-layer      | yes    | yes          |                                                       |
| focus-guards           | leptos-focus-guards           | yes    | yes          |                                                       |
| focus-scope            | leptos-focus-scope            | yes    | yes          |                                                       |
| id                     | leptos-id                     | yes    | yes          |                                                       |
| popper                 | leptos-popper                 | yes    | yes          |                                                       |
| portal                 | leptos-portal                 | yes    | yes          |                                                       |
| presence               | leptos-presence               | yes    | yes          |                                                       |
| primitive              | leptos-primitive              | yes    | yes          | Includes slot functionality                           |
| roving-focus           | leptos-roving-focus           | yes    | yes          |                                                       |
| slot                   | Part of leptos-primitive      | yes    | n/a          | No separate package needed                            |
| use-callback-ref       | leptos-use-callback-ref       | yes    | n/a          | No separate directory; implemented inline or as crate |
| use-controllable-state | leptos-use-controllable-state | yes    | yes          |                                                       |
| use-effect-event       | leptos-use-effect-event       | yes    | n/a          | No separate directory                                 |
| use-escape-keydown     | leptos-use-escape-keydown     | yes    | yes          |                                                       |
| use-is-hydrated        | leptos-use-is-hydrated        | yes    | n/a          | No separate directory                                 |
| use-layout-effect      | leptos-use-layout-effect      | yes    | n/a          | No separate directory                                 |
| use-previous           | leptos-use-previous           | yes    | yes          |                                                       |
| use-rect               | leptos-use-rect               | yes    | yes          |                                                       |
| use-size               | leptos-use-size               | yes    | yes          |                                                       |

## Core Packages (Framework-Agnostic)

| Package        | Ported | Notes |
| -------------- | ------ | ----- |
| core-number    | yes    |       |
| core-primitive | yes    |       |
| core-rect      | yes    |       |

## Aggregate Package

| Package  | Status                                              |
| -------- | --------------------------------------------------- |
| radix-ui | N/A — re-exports everything; not applicable to Rust |

## Blocking Issues

### 1. Avatar and Menu on Old Leptos API

Both `avatar` and `menu` are ported but use the pre-0.8 Leptos API (`create_signal`, `NodeRef<AnyElement>`, `#[prop(attrs)]`, `on_cleanup`). They are **excluded from the workspace** and their stories are **commented out**. These need to be migrated to the 0.8 API before they can be re-enabled.

### 2. Menu Has Stub Components

`menu` has 7 components returning empty `view! {}`:
- MenuCheckboxItem
- MenuRadioGroup
- MenuRadioItem
- MenuItemIndicator
- MenuSub
- MenuSubTrigger
- MenuSubContent

Additionally, `MenuPortal` passes children through without actual portal behavior (has a `// TODO: portal` comment).

### 3. Components Blocked by Menu

Three components depend on a fully functional `menu` and cannot be ported until menu is complete:
- **context-menu** — depends on menu
- **dropdown-menu** — depends on menu
- **menubar** — depends on menu and dropdown-menu patterns

### 4. Select and Toast Not Yet Ported

These have no blocking dependency on menu but are not yet implemented:
- **select** — large dependency list (17 internal deps); has React E2E tests (39/39)
- **toast** — has React E2E tests (19/19)

### 5. Stories Needing Verification

Four components have stories that have not been manually verified (`tested_story: false`):
- **accessible-icon** — story active, needs manual check
- **avatar** — story commented out (blocked by old API)
- **menu** — story commented out (blocked by old API + stubs)
- **one-time-password-field** — story active, needs manual check (unstable)

## E2E Test Coverage

### Passing Both React & Leptos (20 components)

| Component      | Tests |
| -------------- | ----- |
| Accordion      | 33/33 |
| AlertDialog    | 30/30 |
| Checkbox       | 21/21 |
| Collapsible    | 18/18 |
| Dialog         | 31/31 |
| Form           | 16/16 |
| HoverCard      | 12/12 |
| NavigationMenu | 35/35 |
| Popover        | 26/26 |
| Progress       | 26/26 |
| RadioGroup     | 29/29 |
| ScrollArea     | 22/22 |
| Separator      | 9/9   |
| Slider         | 45/45 |
| Switch         | 21/21 |
| Tabs           | 41/41 |
| Toggle         | 15/15 |
| ToggleGroup    | 41/41 |
| Toolbar        | 32/32 |
| Tooltip        | 17/17 |

**Total: 518/518 tests passing across both frameworks**

### React-Only E2E Tests (Leptos not yet ported)

| Component    | React Tests | Notes           |
| ------------ | ----------- | --------------- |
| ContextMenu  | 31/31       | Blocked on menu |
| DropdownMenu | 42/42       | Blocked on menu |
| Select       | 39/39       | Not yet ported  |
| Toast        | 19/19       | Not yet ported  |

### No E2E Tests Yet

accessible-icon, announce, arrow, aspect-ratio, avatar, label, one-time-password-field, password-toggle-field, visually-hidden

These are generally simpler components where E2E behavioral testing may be lower priority than unit tests or story verification.
