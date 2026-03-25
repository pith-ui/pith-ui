# Calendar, Date Picker, and Time Picker — Research & Implementation Plans

## Executive Summary

Radix UI has **no Calendar/DatePicker primitives** (explicitly closed as "not planned" in Jan 2025). This means we're designing from scratch rather than porting. This research surveys base-ui, shadcn/react-day-picker, React Aria, Ark UI/Zag.js, Melt UI, Corvu, and W3C ARIA specs to inform three implementation plans of varying scope.

---

## 1. Cross-Library Component Anatomy

### Calendar (standalone grid)

Every library treats Calendar as an independent primitive. Common parts:

| Concept        | Base-UI                               | react-day-picker                    | React Aria                                  | Ark UI                                        | Corvu                  |
| -------------- | ------------------------------------- | ----------------------------------- | ------------------------------------------- | --------------------------------------------- | ---------------------- |
| Root           | `Calendar.Root`                       | `Root` (div)                        | `Calendar`                                  | `Root`                                        | `Root`                 |
| Header/Nav     | `IncrementMonth` / `DecrementMonth`   | `Nav` + `PrevButton` / `NextButton` | `Button[slot=prev/next]`                    | `ViewControl` + `PrevTrigger` / `NextTrigger` | `Nav(action)`          |
| Heading        | (user-rendered)                       | `CaptionLabel`                      | `Heading`                                   | `RangeText` + `ViewTrigger`                   | `Label`                |
| Grid           | `DayGrid` (table)                     | `MonthGrid` (table)                 | `CalendarGrid` (table)                      | `Table`                                       | `Table`                |
| Grid Header    | `DayGridHeader` + `DayGridHeaderCell` | `Weekdays` + `Weekday`              | `CalendarGridHeader` + `CalendarHeaderCell` | `TableHead` + `TableHeader`                   | `HeadCell`             |
| Grid Body      | `DayGridBody` + `DayGridRow`          | `Weeks` + `Week`                    | `CalendarGridBody`                          | `TableBody` + `TableRow`                      | (implicit)             |
| Day Cell       | `DayGridCell` + `DayButton`           | `Day` (td) + `DayButton`            | `CalendarCell`                              | `TableCell` + `TableCellTrigger`              | `Cell` + `CellTrigger` |
| View Switching | —                                     | Dropdown nav                        | —                                           | `ViewTrigger` (day→month→year)                | —                      |

**Key observation:** Most libraries separate the cell container (`<td>`) from the interactive trigger (`<button>`) inside it. This gives styling flexibility — the cell handles grid layout, the trigger handles interaction states and focus.

### DatePicker (composed primitive)

DatePicker = Input + Trigger + Popover + Calendar. Two sub-patterns for the input:

1. **Plain text input** (shadcn, Ark UI): `<input type="text">` with date parsing
2. **Segmented field** (React Aria, Melt UI): Each date part (month/day/year) is an independent `role="spinbutton"` element

### TimePicker / TimeField

Always uses the segmented spinbutton pattern: hour, minute, optional second, optional AM/PM. Each segment is independently focusable and editable.

---

## 2. ARIA Accessibility Requirements

### Calendar Grid Pattern

- `<table role="grid">` with `aria-labelledby` referencing the month/year heading
- `<th scope="col">` for weekday headers (some libs mark thead `aria-hidden` since cells are self-describing)
- `<td role="gridcell">` containing `<button>` for each day
- **Roving tabindex:** One cell has `tabindex="0"`, all others `tabindex="-1"`
- `aria-selected="true"` on selected date(s)
- `aria-disabled="true"` on dates outside valid range
- `aria-current="date"` on today's date (base-ui pattern)
- Month/year heading: `aria-live="polite"` for announcing navigation changes

### Keyboard Navigation (Grid)

| Key                | Action                       |
| ------------------ | ---------------------------- |
| Arrow Left/Right   | Previous/next day            |
| Arrow Up/Down      | Same day previous/next week  |
| Home/End           | First/last day of week       |
| Page Up/Down       | Same day previous/next month |
| Shift+Page Up/Down | Same day previous/next year  |
| Enter/Space        | Select focused date          |
| Escape             | Close picker (if in popover) |

### Segmented Date/Time Field

- `role="group"` container with `aria-labelledby`
- Each segment: `role="spinbutton"` with `aria-valuenow`, `aria-valuetext`, `aria-valuemin`, `aria-valuemax`, `aria-label`
- Keyboard: Arrow Up/Down to increment/decrement, digit keys for direct entry, Tab between segments, Backspace to clear

### DatePicker Dialog/Combobox

Two W3C patterns exist:
- **Dialog pattern:** Button opens `role="dialog"` with `aria-modal="true"` containing calendar
- **Combobox pattern:** Input with `role="combobox"`, `aria-haspopup="dialog"`, `aria-expanded`

---

## 3. Selection Modes

All mature libraries support:
- **Single:** One date selected at a time
- **Multiple:** Multiple individual dates (not all libs)
- **Range:** Start + end date with visual range indication

Range selection adds: `data-range-start`, `data-range-middle`, `data-range-end` states on cells.

---

## 4. Rust/WASM Date Library Assessment

| Crate           | WASM?              | Calendar Math                                                | i18n                                | Status                     | Bundle Impact              |
| --------------- | ------------------ | ------------------------------------------------------------ | ----------------------------------- | -------------------------- | -------------------------- |
| **chrono**      | Yes (`wasmbind`)   | Complete                                                     | `unstable-locales` feature          | Stable, v0.4.43            | Moderate                   |
| **jiff**        | Yes (`js` feature) | Best API (`days_in_month()`, `first_of_month()`, `series()`) | Gregorian only; `jiff-icu` for more | Pre-1.0, targeting 2026    | Small                      |
| **time**        | Partial            | Basic                                                        | None                                | Stable, v0.3.47            | Small                      |
| **js-sys Date** | By definition      | Awkward API                                                  | Full via `Intl.DateTimeFormat`      | Stable                     | Zero                       |
| **icu4x**       | Designed for it    | 13+ calendar systems                                         | Gold standard                       | v2.0, production (Firefox) | 19KB–5MB depending on data |

**Recommendation:** `chrono` for calendar math (stable, WASM-proven, `leptos-use` uses it) + `js_sys::Intl::DateTimeFormat` for locale display (zero bundle cost, full locale support). Add `icu4x` later if non-Gregorian calendars or SSR locale support is needed.

### Minimal Calendar Grid Math

The core computation is ~20 lines:
1. First day of month → get weekday offset
2. Days in month → fill grid cells
3. Previous month trailing days → fill leading empty cells
4. Next month leading days → fill trailing empty cells
5. Month arithmetic for navigation

---

## 5. What Existing Leptos Ecosystem Offers

- **`leptos-use` `use_calendar`**: Minimal hook returning `Vec<CalendarDate>` with Previous/Current/Next markers. Uses chrono. Good reference for grid computation.
- **`cloud-shuttle/radix-leptos`**: Separate Radix port claiming 57+ components including Calendar/DatePicker.
  - DO NOT USE: THIS IS AI SLOP
- **`leptos-shadcn-ui`**: shadcn port with Calendar/DatePicker.
  - THIS IS UNMAINTAINED AND CALENDAR HAS NO IMPLEMENTATION; DO NOT USE
- **`leptos-shadcn-date-picker`**: Standalone date picker crate.
  - DO NOT USE: THIS IS AI SLOP

None of these are unstyled primitives following our compositional Radix-like pattern.

---

## 6. Design Decisions

### Calendar vs. DateField separation

Every library separates these concerns:
- **Calendar** = visual grid for date selection (mouse/touch-friendly)
- **DateField** = inline segmented input for keyboard-first date entry
- **DatePicker** = composition of DateField + Calendar + Popover

We should follow this pattern.

### Cell vs. CellTrigger split

Ark UI and Corvu separate `<td>` (Cell) from `<button>` (CellTrigger). This is the correct approach for a headless library — it gives consumers full control over cell layout vs. interaction styling.

### View switching (day → month → year)

Only Ark UI has this built-in. It's a nice UX feature but adds significant complexity. Good candidate for phase 2.

### Date library coupling

The component should accept generic date types or work with a trait, but the default/recommended usage should be with chrono. React Aria's `@internationalized/date` and base-ui's `TemporalAdapter` show that an adapter pattern enables library-agnostic date handling, but adds API complexity.

---

# Implementation Plans

## Plan A: Calendar-First (Minimal Viable)

**Scope:** Calendar primitive only. No DatePicker, no DateField, no TimePicker.

**Components:**
```
Calendar.Root         — Root container, state management, context provider
Calendar.Header       — Navigation bar container
Calendar.Heading      — Month/year display (aria-live="polite")
Calendar.PrevButton   — Navigate to previous month
Calendar.NextButton   — Navigate to next month
Calendar.Grid         — <table role="grid"> for the month
Calendar.GridHead     — <thead> weekday header row
Calendar.HeadCell     — <th> individual weekday label
Calendar.GridBody     — <tbody> weeks container
Calendar.GridRow      — <tr> single week row
Calendar.Cell         — <td role="gridcell"> day cell container
Calendar.CellTrigger  — <button> interactive day element
```

**Selection modes:** Single only (add Multiple and Range later).

**Props on Root:**
- `value` / `default_value` / `on_value_change` — controlled/uncontrolled date selection
- `month` / `default_month` / `on_month_change` — controlled/uncontrolled visible month
- `min_date` / `max_date` — navigation and selection bounds
- `is_date_disabled: Callback<NaiveDate, bool>` — predicate for disabled dates
- `is_date_unavailable: Callback<NaiveDate, bool>` — predicate for unavailable (focusable but not selectable) dates
- `disabled` / `read_only`
- `week_start: Weekday` — first day of week (default: Monday)
- `fixed_weeks: bool` — always show 6 rows

**Cell data attributes:**
- `data-selected`, `data-today`, `data-disabled`, `data-unavailable`, `data-outside-month`

**Keyboard:** Full grid navigation (arrows, Page Up/Down, Shift+Page, Home/End, Enter/Space).

**Date library:** `chrono::NaiveDate` directly. No adapter abstraction.

**Accessibility:** WAI-ARIA grid pattern, roving tabindex, aria-live heading.

**What this enables:** Consumers compose `Calendar` + our existing `Popover` primitive to build their own date picker. This is the base-ui philosophy — "we're not sure a dedicated DatePicker adds value versus composing DateField + Calendar + Popover."

**Estimated complexity:** Medium. The grid computation and keyboard navigation are well-understood. The main work is the roving tabindex 2D grid navigation and month transition logic.

**Deliverables:**
1. `calendar` component module
2. Story mirroring React Aria's Calendar examples
3. Research note

---

## Plan B: Calendar + DateField + DatePicker (Full Date Suite)

**Scope:** Everything in Plan A, plus DateField (segmented input), DatePicker (composed), and RangeCalendar/DateRangePicker.

**Additional components beyond Plan A:**

### DateField (segmented date input)
```
DateField.Root       — Root container, manages segment state
DateField.Label      — Accessible label
DateField.Input      — Container for segments (role="group")
DateField.Segment    — Individual editable segment (role="spinbutton")
```

Each segment renders as a `role="spinbutton"` with:
- `aria-valuenow`, `aria-valuetext`, `aria-valuemin`, `aria-valuemax`
- Keyboard: Arrow Up/Down for increment/decrement, digit entry, Tab between segments

**Segment types:** `year`, `month`, `day`, `literal` (separator like "/")

### DatePicker (composition of DateField + Popover + Calendar)
```
DatePicker.Root       — Manages combined state (field + calendar + popover)
DatePicker.Label      — Accessible label
DatePicker.Control    — Input area container
DatePicker.Input      — DateField segments container
DatePicker.Segment    — Individual date segment
DatePicker.Trigger    — Button to open/close calendar popover
DatePicker.Content    — Popover content (wraps Calendar)
DatePicker.Calendar   — Calendar embedded in the popover
```

**Props additions:**
- `format: &str` — date display format
- `placeholder_date: NaiveDate` — controls default segment values and initial month display
- `granularity: DateGranularity` — `Day` only for Plan B (Plan C adds `Hour`/`Minute`/`Second`)

### RangeCalendar
Same parts as Calendar but:
- `value: (NaiveDate, NaiveDate)` — start/end range
- Additional cell states: `data-range-start`, `data-range-end`, `data-range-middle`
- `anchor_date` internal state for in-progress range selection

### DateRangePicker
```
DateRangePicker.Root
DateRangePicker.Label
DateRangePicker.Control
DateRangePicker.StartInput / DateRangePicker.StartSegment
DateRangePicker.EndInput / DateRangePicker.EndSegment
DateRangePicker.Trigger
DateRangePicker.Content
DateRangePicker.Calendar   — RangeCalendar inside popover
```

**Implementation order:**
1. Calendar (Plan A)
2. RangeCalendar (extend Calendar with range selection)
3. DateField (segmented input — most complex new primitive)
4. DatePicker (compose DateField + Popover + Calendar)
5. DateRangePicker (compose two DateFields + Popover + RangeCalendar)

**Date library:** Still `chrono::NaiveDate`. The DateField segments are computed from the date format.

**Estimated complexity:** High. The DateField segment editing is the hardest part — handling digit accumulation, auto-advance, placeholder states, dynamic min/max (day depends on month), and the spinbutton ARIA contract. React Aria's `useDateSegment` is ~500 lines of heavily nuanced code.

**Deliverables:**
1. All Plan A deliverables
2. `date_field` component module + story
3. `date_picker` component module + story
4. `range_calendar` component module + story
5. `date_range_picker` component module + story
6. Research notes for each

---

## Plan C: Full Date/Time Suite with i18n Foundation (Comprehensive)

**Scope:** Everything in Plan B, plus TimeField, DateTimePicker, i18n adapter layer, and view switching.

**Additional components beyond Plan B:**

### TimeField
```
TimeField.Root       — Root container
TimeField.Label      — Accessible label
TimeField.Input      — Segment container
TimeField.Segment    — hour / minute / second / dayPeriod segments
```

**Segment types added:** `hour`, `minute`, `second`, `day_period` (AM/PM)

**Props:**
- `value: NaiveTime` / `on_value_change`
- `hour_cycle: HourCycle` — `H12` or `H24`
- `granularity: TimeGranularity` — `Hour`, `Minute`, `Second`
- `min_time` / `max_time`

### DateTimePicker
Extends DatePicker with time segments:
```
DateTimePicker.Root
DateTimePicker.Label
DateTimePicker.Control
DateTimePicker.DateInput + DateTimePicker.DateSegment
DateTimePicker.TimeInput + DateTimePicker.TimeSegment
DateTimePicker.Trigger
DateTimePicker.Content
DateTimePicker.Calendar
```

Uses `chrono::NaiveDateTime` as the value type.

### Calendar View Switching (à la Ark UI)
Adds day → month → year view hierarchy:
```
Calendar.ViewTrigger  — Click heading to switch from day → month view, month → year view
Calendar.MonthGrid    — Grid of 12 months for month selection
Calendar.YearGrid     — Grid of years for year selection
```

### i18n Adapter Layer

A trait-based abstraction allowing pluggable date libraries:

```rust
pub trait DateAdapter {
    type Date: Clone + PartialEq + PartialOrd;

    fn today() -> Self::Date;
    fn year(date: &Self::Date) -> i32;
    fn month(date: &Self::Date) -> u32;
    fn day(date: &Self::Date) -> u32;
    fn weekday(date: &Self::Date) -> Weekday;
    fn days_in_month(date: &Self::Date) -> u32;
    fn first_of_month(date: &Self::Date) -> Self::Date;
    fn add_months(date: &Self::Date, n: i32) -> Self::Date;
    fn add_days(date: &Self::Date, n: i32) -> Self::Date;
    fn format_month_year(date: &Self::Date, locale: &str) -> String;
    fn format_weekday_short(weekday: Weekday, locale: &str) -> String;
    fn format_day(date: &Self::Date) -> String;
    fn is_same_day(a: &Self::Date, b: &Self::Date) -> bool;
    fn is_same_month(a: &Self::Date, b: &Self::Date) -> bool;
}
```

**Default adapter:** `ChronoAdapter` (uses chrono + js_sys::Intl for formatting)
**Optional adapter:** `Icu4xAdapter` (uses icu4x for full multi-calendar, multi-locale support)

Provided via Leptos context: `<DateAdapterProvider adapter=ChronoAdapter::new("en-US")>`

### Multiple Calendar Support (via icu4x adapter)

The Icu4xAdapter would support 13+ calendar systems (Gregorian, Buddhist, Hijri, Hebrew, Persian, Japanese, etc.) by delegating to `icu_calendar`. This matches React Aria's `@internationalized/date` capability.

**Implementation order:**
1. Calendar (Plan A)
2. DateField + DatePicker (Plan B core)
3. TimeField
4. RangeCalendar + DateRangePicker
5. DateTimePicker
6. View switching
7. i18n adapter trait + ChronoAdapter
8. Icu4xAdapter (optional crate feature)

**Estimated complexity:** Very high. The i18n adapter adds a generics layer throughout. View switching requires managing three distinct grid layouts and animated transitions. The TimeField AM/PM segment has locale-dependent behavior. The DateTimePicker requires coordinating field state, calendar state, and time state simultaneously.

**Deliverables:**
1. All Plan B deliverables
2. `time_field` component module + story
3. `date_time_picker` component module + story
4. Calendar view switching
5. `pith-date-adapter` trait crate
6. `pith-date-adapter-chrono` default implementation
7. `pith-date-adapter-icu4x` optional implementation (feature-gated)
8. Research notes for each

---

## Plan Comparison

| Aspect               | Plan A                                       | Plan B                                                              | Plan C                                                    |
| -------------------- | -------------------------------------------- | ------------------------------------------------------------------- | --------------------------------------------------------- |
| **Components**       | 1 (Calendar)                                 | 5 (Calendar, RangeCalendar, DateField, DatePicker, DateRangePicker) | 8+ (all of B + TimeField, DateTimePicker, view switching) |
| **Selection modes**  | Single                                       | Single, Multiple, Range                                             | Single, Multiple, Range                                   |
| **Date input**       | Grid only                                    | Grid + segmented field                                              | Grid + segmented field + time                             |
| **Date library**     | chrono direct                                | chrono direct                                                       | Adapter trait (chrono default, icu4x optional)            |
| **i18n**             | Week start only                              | Week start + basic locale                                           | Full multi-calendar, multi-locale                         |
| **View switching**   | No                                           | No                                                                  | Yes (day/month/year)                                      |
| **Estimated effort** | Medium                                       | High                                                                | Very high                                                 |
| **Risk**             | Low                                          | Medium (DateField segments are complex)                             | High (adapter generics, icu4x integration, scope)         |
| **Value delivered**  | Calendar usable via composition with Popover | Complete date picking solution                                      | Enterprise-grade date/time suite                          |

---

## Recommendation

**Start with Plan A, iterate toward Plan B.**

Plan A is low-risk, delivers immediate value, and establishes the patterns (grid navigation, roving tabindex, cell state management) that everything else builds on. The Calendar primitive composes with our existing Popover to give users a functional date picker immediately.

Plan B's DateField is the highest-complexity single component (segment editing is ~500 lines of nuanced code in React Aria). Tackle it second once Calendar patterns are proven.

Plan C's i18n adapter layer should be deferred until there's demand. Starting with chrono directly keeps the API simple; extracting to a trait later is a straightforward refactor if needed.

## Test Cases

Test Cases for Manual Calendar Verification

Here are the test cases to put the Calendar through its paces, organized by category:

1. Basic Rendering (Styled story)

   - Calendar renders with current month heading ("Month Year")
   - 7 weekday headers display (Su, Mo, Tu, We, Th, Fr, Sa)
   - Correct number of week rows (4–6 depending on month)
   - Days from previous/next months fill leading/trailing cells with data-outside-month styling
   - Today's date has data-today styling (bold)
   - No date is selected by default

2. Date Selection

   - Clicking a date selects it (gets data-selected styling)
   - Clicking a different date moves selection
   - Clicking an outside-month date selects it AND navigates to that month
   - Clicking a disabled date does nothing
   - Clicking an unavailable date does nothing

3. Controlled State (Controlled story)

   - Selected date displays in the text above
   - Month displays in the text above
   - Clicking "Clear selection" removes the selection
   - Re-selecting a date after clearing works
   - Month navigation updates the displayed month text

4. Month Navigation

   - Prev button navigates to the previous month
   - Next button navigates to the next month
   - Heading updates with aria-live="polite" (screen reader announces)
   - Grid redraws with the new month's days
   - Rapid clicking prev/next works without glitches

5. Keyboard Navigation (focus a date cell first via Tab or click)

   - Arrow Right — moves focus to next day
   - Arrow Left — moves focus to previous day
   - Arrow Down — moves focus to same day next week
   - Arrow Up — moves focus to same day previous week
   - Page Down — moves to same day next month (auto-navigates month)
   - Page Up — moves to same day previous month (auto-navigates month)
   - Shift+Page Down — moves to same day next year
   - Shift+Page Up — moves to same day previous year
   - Home — moves to first day of current week
   - End — moves to last day of current week
   - Enter — selects the focused date
   - Space — selects the focused date
   - Arrow keys crossing month boundaries auto-navigate the grid

6. Roving Tabindex

   - Tab into the calendar lands on the correct cell (today or selected)
   - Only one cell has tabindex="0" at any time
   - Tab out of the calendar moves focus past the grid (not through every cell)
   - After navigating with arrows, Tab out and back in resumes at the last focused cell

7. Week Start (Chromatic: "Monday start")

   - First column header is "Mo" (not "Su")
   - Days align correctly under the Monday-start layout
   - Keyboard Home/End navigate to Monday/Sunday of the week

8. Fixed Weeks (Chromatic: "Fixed weeks")

   - Grid always shows exactly 6 rows (42 cells)
   - Months that normally fit in 4–5 rows still show 6 rows with trailing next-month days

9. Min/Max Date Constraints (Chromatic: "Min/Max constrained")

   - Dates before min have data-disabled styling
   - Dates after max have data-disabled styling
   - Prev button is disabled when entire previous month is before min
   - Next button is disabled when entire next month is after max
   - Keyboard navigation clamps at min/max (can't arrow past bounds)
   - Clicking disabled dates has no effect

10. Disabled Dates Predicate (Chromatic: "Disabled weekends")

    - Saturday and Sunday cells have data-disabled styling
    - Clicking a weekend date does not select it
    - Keyboard can still arrow through disabled dates (they participate in navigation)
    - Enter/Space on a disabled date does not select it

11. Unavailable Dates (Chromatic: "Unavailable dates")

    - 10th and 20th have data-unavailable styling (strikethrough + red)
    - Unavailable dates are focusable via keyboard
    - Clicking/Enter on unavailable dates does not select them
    - aria-disabled="true" is set on unavailable cells

12. Disabled Calendar (Chromatic: "Disabled")

    - All cells appear disabled
    - Prev/Next buttons are disabled
    - No cell is selectable via click or keyboard
    - data-disabled present on the root

13. Read-Only (Chromatic: "Read-only with selection")

    - Pre-selected date is visible (blue)
    - Month navigation (prev/next) still works
    - Clicking a date does NOT change the selection
    - Enter/Space does NOT change the selection
    - Arrow keys still navigate focus

14. Accessibility (all stories)

    - role="grid" on the table
    - aria-labelledby on the grid references the heading
    - aria-live="polite" on the heading
    - Each day button has an aria-label like "Friday, March 15, 2024"
    - aria-selected="true" on the selected cell
    - aria-disabled="true" on disabled and unavailable cells
    - aria-current="date" on today's cell
    - Weekday header row is aria-hidden="true"
    - Screen reader announces month changes when navigating