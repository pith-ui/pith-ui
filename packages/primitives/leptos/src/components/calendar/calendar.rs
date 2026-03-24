use chrono::{Datelike, NaiveDate, Weekday};
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::support::id::use_id;
use crate::support::primitive::{Primitive, adapt_callback, data_attr, prop_or};
use crate::support::use_controllable_state::{UseControllableStateParams, use_controllable_state};

use super::{CalendarContextValue, add_months, last_day_of_month, month_name};

// ── Calendar (Root) ──────────────────────────────────────────────────

/// Root calendar component.
///
/// Manages selected value, visible month, focused date, and provides
/// context for all child components.
#[component]
pub fn Calendar(
    /// The controlled selected date.
    #[prop(into, optional)]
    value: MaybeProp<NaiveDate>,
    /// Default selected date (uncontrolled).
    #[prop(into, optional)]
    default_value: MaybeProp<NaiveDate>,
    /// Fires when the selected date changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<NaiveDate>>,
    /// The controlled visible month (any date in that month).
    #[prop(into, optional)]
    month: MaybeProp<NaiveDate>,
    /// Default visible month (uncontrolled).
    #[prop(into, optional)]
    default_month: MaybeProp<NaiveDate>,
    /// Fires when the visible month changes. Receives the first day of the new month.
    #[prop(into, optional)]
    on_month_change: Option<Callback<NaiveDate>>,
    /// Earliest selectable date.
    #[prop(into, optional)]
    min_date: MaybeProp<NaiveDate>,
    /// Latest selectable date.
    #[prop(into, optional)]
    max_date: MaybeProp<NaiveDate>,
    /// Predicate for dates that should be disabled (not selectable, not focusable).
    #[prop(into, optional)]
    is_date_disabled: Option<Callback<NaiveDate, bool>>,
    /// Predicate for dates that are unavailable (focusable but not selectable).
    #[prop(into, optional)]
    is_date_unavailable: Option<Callback<NaiveDate, bool>>,
    /// When `true`, the entire calendar is disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// When `true`, the selected value cannot be changed.
    #[prop(into, optional)]
    read_only: MaybeProp<bool>,
    /// First day of the week (default: Sunday).
    #[prop(into, optional)]
    week_start: MaybeProp<Weekday>,
    /// Always display 6 rows (42 cells).
    #[prop(into, optional)]
    fixed_weeks: MaybeProp<bool>,
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let today_date = chrono::Local::now().date_naive();
    let disabled = prop_or(disabled, false);
    let read_only = prop_or(read_only, false);
    let week_start = prop_or(week_start, Weekday::Sun);
    let fixed_weeks = prop_or(fixed_weeks, false);
    let min_date_signal = Signal::derive(move || min_date.get());
    let max_date_signal = Signal::derive(move || max_date.get());

    // ── Selected value (controlled / uncontrolled) ──

    // Note: we don't use `use_controllable_state` for value because
    // `MaybeProp<NaiveDate>` conflates "prop not provided" with "prop is None"
    // (no selection). Controlled calendars that clear their selection would
    // silently fall back to stale internal state.  Instead we determine
    // controlledness once at mount time: if on_value_change is provided the
    // consumer is driving the value.
    let is_value_controlled = on_value_change.is_some();
    let internal_value = RwSignal::new(default_value.get_untracked());

    let value_signal: Signal<Option<NaiveDate>> = Signal::derive(move || {
        if is_value_controlled {
            value.get()
        } else {
            internal_value.get()
        }
    });

    let set_value: Callback<Option<NaiveDate>> = Callback::new(move |new_val: Option<NaiveDate>| {
        if is_value_controlled {
            if let (Some(cb), Some(date)) = (on_value_change, new_val) {
                cb.run(date);
            }
        } else {
            internal_value.set(new_val);
        }
    });

    // ── Visible month (controlled / uncontrolled) ──

    // Compute a sensible default month at init time.
    let computed_default_month = default_month
        .get_untracked()
        .or_else(|| value.get_untracked())
        .or_else(|| default_value.get_untracked())
        .map(|d| NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap());

    let (month_signal, set_month_raw) = use_controllable_state(UseControllableStateParams {
        prop: month,
        on_change: adapt_callback(on_month_change),
        default_prop: MaybeProp::from(computed_default_month),
    });

    // Normalize to first-of-month, with fallback to today's month.
    let month_date = Signal::derive(move || {
        let date = month_signal.get().unwrap_or(today_date);
        NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
    });

    // Wrap setter to always normalize to first-of-month.
    let set_month = Callback::new(move |date: Option<NaiveDate>| {
        let normalized = date.map(|d| NaiveDate::from_ymd_opt(d.year(), d.month(), 1).unwrap());
        set_month_raw.run(normalized);
    });

    // ── Focused date (internal) ──

    let initial_focus = value
        .get_untracked()
        .or_else(|| default_value.get_untracked())
        .unwrap_or(today_date);
    let focused_date = RwSignal::new(initial_focus);

    // ── IDs ──

    let heading_id = use_id(None);

    // ── Grid ref for focus management ──

    let grid_ref = AnyNodeRef::new();

    // ── Context ──

    let context = CalendarContextValue {
        value: Signal::derive(move || value_signal.get()),
        set_value,
        month: month_date,
        set_month,
        focused_date,
        min_date: min_date_signal,
        max_date: max_date_signal,
        disabled,
        read_only,
        is_date_disabled,
        is_date_unavailable,
        week_start,
        fixed_weeks,
        today: today_date,
        heading_id,
        grid_ref,
    };

    view! {
        <Provider value=context>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-disabled=data_attr(disabled)
            >
                {children.with_value(|children| children())}
            </Primitive>
        </Provider>
    }
}

// ── CalendarHeader ───────────────────────────────────────────────────

/// Container for calendar navigation controls (prev/next buttons, heading).
///
/// Renders as a `<div>`. Purely structural — provides no context or state.
#[component]
pub fn CalendarHeader(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

// ── CalendarHeading ──────────────────────────────────────────────────

/// Displays the month and year for the visible calendar month.
///
/// Renders as a `<div>` with `aria-live="polite"` so screen readers
/// announce month changes. Auto-renders "Month Year" text when no
/// children are provided.
#[component]
pub fn CalendarHeading(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let context = expect_context::<CalendarContextValue>();
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
            attr:id=move || context.heading_id.get()
            attr:aria-live="polite"
        >
            {move || {
                let m = context.month.get();
                children.with_value(|c| match c {
                    Some(f) => f(),
                    None => format!("{} {}", month_name(m.month()), m.year()).into_any(),
                })
            }}
        </Primitive>
    }
}

// ── CalendarPrevButton ───────────────────────────────────────────────

/// Navigates to the previous month.
///
/// Renders as a `<button>`. Automatically disabled when navigating
/// backwards would go before `min_date`.
#[component]
pub fn CalendarPrevButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<CalendarContextValue>();

    let is_disabled = Signal::derive(move || {
        if context.disabled.get() {
            return true;
        }
        if let Some(min) = context.min_date.get() {
            // Disable if the entire previous month is before min_date.
            let current = context.month.get();
            current.pred_opt().is_some_and(|last_of_prev| last_of_prev < min)
        } else {
            false
        }
    });

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:aria-label="Previous month"
            attr:disabled=move || is_disabled.get().then_some("")
            attr:data-disabled=move || is_disabled.get().then_some("")
            on:click=move |event: ev::MouseEvent| {
                if let Some(on_click) = on_click {
                    on_click.run(event);
                }
                if is_disabled.get_untracked() { return; }

                let current = context.month.get_untracked();
                let new_month = add_months(current, -1);
                context.set_month.run(Some(new_month));

                // Adjust focused date to stay in the new month.
                let focused = context.focused_date.get_untracked();
                let max_day = last_day_of_month(new_month.year(), new_month.month()).day();
                let clamped_day = focused.day().min(max_day);
                let new_focused = NaiveDate::from_ymd_opt(new_month.year(), new_month.month(), clamped_day).unwrap();
                context.focused_date.set(new_focused);
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

// ── CalendarNextButton ───────────────────────────────────────────────

/// Navigates to the next month.
///
/// Renders as a `<button>`. Automatically disabled when navigating
/// forward would go past `max_date`.
#[component]
pub fn CalendarNextButton(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<CalendarContextValue>();

    let is_disabled = Signal::derive(move || {
        if context.disabled.get() {
            return true;
        }
        if let Some(max) = context.max_date.get() {
            let current = context.month.get();
            let next_first = add_months(current, 1);
            next_first > max
        } else {
            false
        }
    });

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="button"
            attr:aria-label="Next month"
            attr:disabled=move || is_disabled.get().then_some("")
            attr:data-disabled=move || is_disabled.get().then_some("")
            on:click=move |event: ev::MouseEvent| {
                if let Some(on_click) = on_click {
                    on_click.run(event);
                }
                if is_disabled.get_untracked() { return; }

                let current = context.month.get_untracked();
                let new_month = add_months(current, 1);
                context.set_month.run(Some(new_month));

                let focused = context.focused_date.get_untracked();
                let max_day = last_day_of_month(new_month.year(), new_month.month()).day();
                let clamped_day = focused.day().min(max_day);
                let new_focused = NaiveDate::from_ymd_opt(new_month.year(), new_month.month(), clamped_day).unwrap();
                context.focused_date.set(new_focused);
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
