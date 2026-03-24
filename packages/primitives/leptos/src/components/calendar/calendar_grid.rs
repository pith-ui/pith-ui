use chrono::{Datelike, NaiveDate, TimeDelta};
use leptos::{ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::support::compose_refs::use_composed_refs;
use crate::support::primitive::Primitive;

use super::{
    CalendarContextValue, add_months, cell_is_disabled, cell_is_unavailable, clamp_to_range,
    compute_calendar_weeks, end_of_week, format_date_label, is_tab_target, ordered_weekdays,
    queue_focus_date, start_of_week, weekday_long, weekday_short,
};

// ── CalendarGrid ─────────────────────────────────────────────────────

/// The calendar grid container.
///
/// Renders as a `<table role="grid">` with `aria-labelledby` pointing
/// to the [`CalendarHeading`](super::CalendarHeading).
#[component]
pub fn CalendarGrid(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<CalendarContextValue>();
    let composed_ref = use_composed_refs(vec![node_ref, context.grid_ref]);

    view! {
        <Primitive
            element=html::table
            as_child=as_child
            node_ref=composed_ref
            attr:role="grid"
            attr:aria-labelledby=move || context.heading_id.get()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

// ── CalendarGridHead ─────────────────────────────────────────────────

/// Auto-renders a weekday header row.
///
/// Renders `<thead aria-hidden="true">` with a single `<tr>` of `<th>`
/// elements for each day of the week. The header is aria-hidden because
/// each day button carries its own `aria-label` with the full date.
#[component]
pub fn CalendarGridHead(
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<CalendarContextValue>();

    view! {
        <Primitive
            element=html::thead
            node_ref=node_ref
            attr:aria-hidden="true"
        >
            <tr>
                {move || {
                    ordered_weekdays(context.week_start.get())
                        .into_iter()
                        .map(|wd| {
                            view! {
                                <th scope="col" abbr=weekday_long(wd)>
                                    {weekday_short(wd)}
                                </th>
                            }
                        })
                        .collect_view()
                }}
            </tr>
        </Primitive>
    }
}

// ── CalendarGridBody ─────────────────────────────────────────────────

/// Auto-renders week rows with day cells.
///
/// Renders `<tbody>` containing one `<tr>` per week. Each row has 7
/// `<td role="gridcell">` cells, each containing an interactive
/// `<button>` with roving tabindex and full ARIA / data attributes.
#[component]
pub fn CalendarGridBody(
    #[prop(into, optional)] node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<CalendarContextValue>();

    let weeks = Memo::new(move |_| {
        let m = context.month.get();
        compute_calendar_weeks(m.year(), m.month(), context.week_start.get(), context.fixed_weeks.get())
    });

    view! {
        <Primitive element=html::tbody node_ref=node_ref>
            <For
                each=move || weeks.get()
                key=|week| week[0]
                children=move |week: [NaiveDate; 7]| {
                    view! {
                        <tr>
                            {week.into_iter().map(|date| {
                                view! { <CalendarDayCell date=date /> }
                            }).collect_view()}
                        </tr>
                    }
                }
            />
        </Primitive>
    }
}

// ── CalendarDayCell (internal) ───────────────────────────────────────

/// A single day cell rendered inside `CalendarGridBody`.
///
/// Not part of the public API — consumers style cells via data attributes
/// on the rendered `<td>` and `<button>` elements.
#[component]
fn CalendarDayCell(date: NaiveDate) -> impl IntoView {
    let context = expect_context::<CalendarContextValue>();

    // ── Reactive cell state ──

    let is_selected = Signal::derive(move || context.value.get() == Some(date));

    let is_today = context.today == date;

    let is_outside_month = Signal::derive(move || {
        let m = context.month.get();
        date.month() != m.month() || date.year() != m.year()
    });

    let is_disabled = Signal::derive(move || {
        if context.disabled.get() {
            return true;
        }
        if let Some(min) = context.min_date.get() {
            if date < min {
                return true;
            }
        }
        if let Some(max) = context.max_date.get() {
            if date > max {
                return true;
            }
        }
        context
            .is_date_disabled
            .map(|f| f.run(date))
            .unwrap_or(false)
    });

    let is_unavailable = Signal::derive(move || {
        context
            .is_date_unavailable
            .map(|f| f.run(date))
            .unwrap_or(false)
    });

    let is_tab_stop = Signal::derive(move || is_tab_target(date, context));

    // ── Pre-compute static values ──

    let date_str = date.to_string();
    let aria_label = format_date_label(date);
    let day_text = date.day().to_string();

    // ── Data-attribute helpers ──

    let opt_attr = |sig: Signal<bool>| move || sig.get().then_some("");
    let data_selected = opt_attr(is_selected);
    let data_disabled = opt_attr(is_disabled);
    let data_unavailable = opt_attr(is_unavailable);
    let data_outside = opt_attr(is_outside_month);

    view! {
        <td
            role="gridcell"
            aria-selected=move || is_selected.get().then_some("true")
            aria-disabled=move || (is_disabled.get() || is_unavailable.get()).then_some("true")
            data-selected=data_selected
            data-today=is_today.then_some("")
            data-disabled=data_disabled
            data-unavailable=data_unavailable
            data-outside-month=data_outside
        >
            <button
                r#type="button"
                tabindex=move || if is_tab_stop.get() { "0" } else { "-1" }
                aria-label=aria_label
                aria-selected=move || is_selected.get().then_some("true")
                aria-disabled=move || (is_disabled.get() || is_unavailable.get()).then_some("true")
                aria-current=is_today.then_some("date")
                data-date=date_str
                data-selected=data_selected
                data-today=is_today.then_some("")
                data-disabled=data_disabled
                data-unavailable=data_unavailable
                data-outside-month=data_outside
                on:focus=move |_| {
                    context.focused_date.set(date);
                }
                on:click=move |_| {
                    handle_cell_click(date, context);
                }
                on:keydown=move |event: ev::KeyboardEvent| {
                    handle_cell_keydown(event, context);
                }
            >
                {day_text}
            </button>
        </td>
    }
}

// ── Click handler ────────────────────────────────────────────────────

fn handle_cell_click(date: NaiveDate, ctx: CalendarContextValue) {
    if ctx.read_only.get_untracked() || ctx.disabled.get_untracked() {
        return;
    }
    if cell_is_disabled(date, ctx) || cell_is_unavailable(date, ctx) {
        return;
    }

    ctx.set_value.run(Some(date));
    ctx.focused_date.set(date);

    // Navigate to the clicked date's month if it is outside the current view.
    let m = ctx.month.get_untracked();
    if date.month() != m.month() || date.year() != m.year() {
        ctx.set_month
            .run(Some(NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()));
    }
}

// ── Keyboard handler ─────────────────────────────────────────────────

fn handle_cell_keydown(event: ev::KeyboardEvent, ctx: CalendarContextValue) {
    let current = ctx.focused_date.get_untracked();
    let ws = ctx.week_start.get_untracked();

    let new_date = match event.key().as_str() {
        "ArrowRight" => Some(current + TimeDelta::days(1)),
        "ArrowLeft" => Some(current - TimeDelta::days(1)),
        "ArrowDown" => Some(current + TimeDelta::days(7)),
        "ArrowUp" => Some(current - TimeDelta::days(7)),
        "PageDown" if event.shift_key() => Some(add_months(current, 12)),
        "PageDown" => Some(add_months(current, 1)),
        "PageUp" if event.shift_key() => Some(add_months(current, -12)),
        "PageUp" => Some(add_months(current, -1)),
        "Home" => Some(start_of_week(current, ws)),
        "End" => Some(end_of_week(current, ws)),
        " " | "Enter" => {
            event.prevent_default();
            if !ctx.disabled.get_untracked()
                && !ctx.read_only.get_untracked()
                && !cell_is_disabled(current, ctx)
                && !cell_is_unavailable(current, ctx)
            {
                ctx.set_value.run(Some(current));
            }
            return;
        }
        _ => None,
    };

    if let Some(new_date) = new_date {
        event.prevent_default();

        let new_date = clamp_to_range(
            new_date,
            ctx.min_date.get_untracked(),
            ctx.max_date.get_untracked(),
        );

        ctx.focused_date.set(new_date);

        // Advance month if the focused date left the visible range.
        let current_month = ctx.month.get_untracked();
        if new_date.month() != current_month.month() || new_date.year() != current_month.year() {
            let new_month =
                NaiveDate::from_ymd_opt(new_date.year(), new_date.month(), 1).unwrap();
            ctx.set_month.run(Some(new_month));
        }

        queue_focus_date(ctx.grid_ref, new_date);
    }
}
