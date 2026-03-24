use cardo_ui::calendar::*;
use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Style definitions — shadcn/ui new-york calendar
//
// The primitive renders <table> > <thead>/<tbody> > <tr> > <td> > <button>
// internally. Since we can't pass classes to individual generated cells,
// we style them via descendant selectors on the root.
// ---------------------------------------------------------------------------

const ROOT_CLASS: &str = "inline-block rounded-md border bg-background p-3 shadow-xs";

const HEADER_CLASS: &str = "flex items-center justify-between mb-2";

const HEADING_CLASS: &str = "text-sm font-medium";

const NAV_BUTTON_CLASS: &str = "inline-flex items-center justify-center size-7 rounded-md border-none bg-transparent hover:bg-accent hover:text-accent-foreground disabled:disabled-base";

// Grid styles target generated table elements via CSS.
// See the @layer base block added to tailwind.css for calendar.
const GRID_CLASS: &str = "w-full border-collapse [&_th]:text-muted-foreground [&_th]:text-center [&_th]:text-xs [&_th]:font-normal [&_th]:p-0 [&_th]:size-8 [&_td]:p-0.5 [&_td]:text-center [&_button]:inline-flex [&_button]:items-center [&_button]:justify-center [&_button]:size-8 [&_button]:rounded-md [&_button]:text-sm [&_button]:font-normal [&_button]:bg-transparent [&_button]:border-none [&_button]:outline-none [&_button]:cursor-pointer [&_button:hover]:bg-accent [&_button:hover]:text-accent-foreground [&_button:focus-visible]:focus-ring [&_button[data-selected]]:bg-primary [&_button[data-selected]]:text-primary-foreground [&_button[data-selected]:hover]:bg-primary/80 [&_button[data-today]:not([data-selected])]:bg-accent [&_button[data-today]:not([data-selected])]:text-accent-foreground [&_button[data-outside-month]]:text-muted-foreground [&_button[data-outside-month]]:opacity-50 [&_button[data-disabled]]:text-muted-foreground [&_button[data-disabled]]:opacity-50 [&_button[data-disabled]]:cursor-not-allowed [&_button[data-disabled]:hover]:bg-transparent [&_button[data-unavailable]]:text-destructive [&_button[data-unavailable]]:line-through";

// ---------------------------------------------------------------------------
// Components
// ---------------------------------------------------------------------------

#[component]
pub fn ThemedCalendar(
    #[prop(into, optional)] value: MaybeProp<NaiveDate>,
    #[prop(into, optional)] default_value: MaybeProp<NaiveDate>,
    #[prop(into, optional)] on_value_change: Option<Callback<NaiveDate>>,
    #[prop(into, optional)] month: MaybeProp<NaiveDate>,
    #[prop(into, optional)] default_month: MaybeProp<NaiveDate>,
    #[prop(into, optional)] on_month_change: Option<Callback<NaiveDate>>,
    #[prop(into, optional)] min_date: MaybeProp<NaiveDate>,
    #[prop(into, optional)] max_date: MaybeProp<NaiveDate>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    // Calendar uses on_value_change.is_some() to decide controlled vs uncontrolled.
    // We must NOT pass the callback when the user doesn't provide one, otherwise
    // uncontrolled mode (basic calendar) won't update selection on click.
    // on_month_change doesn't affect controlledness so we always forward it.
    let children = StoredValue::new(children);
    let class = StoredValue::new(ROOT_CLASS);

    if let Some(value_cb) = on_value_change {
        view! {
            <Calendar
                attr:class=class.get_value()
                value=value
                default_value=default_value
                on_value_change=value_cb
                month=month
                default_month=default_month
                on_month_change=move |val: NaiveDate| {
                    if let Some(cb) = on_month_change { cb.run(val); }
                }
                min_date=min_date
                max_date=max_date
                disabled=disabled
            >
                {children.with_value(|c| c())}
            </Calendar>
        }
        .into_any()
    } else {
        view! {
            <Calendar
                attr:class=class.get_value()
                default_value=default_value
                month=month
                default_month=default_month
                on_month_change=move |val: NaiveDate| {
                    if let Some(cb) = on_month_change { cb.run(val); }
                }
                min_date=min_date
                max_date=max_date
                disabled=disabled
            >
                {children.with_value(|c| c())}
            </Calendar>
        }
        .into_any()
    }
}

#[component]
pub fn ThemedCalendarHeader(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(HEADER_CLASS);

    view! {
        <CalendarHeader attr:class=class.get_value()>
            {children()}
        </CalendarHeader>
    }
}

#[component]
pub fn ThemedCalendarHeading() -> impl IntoView {
    let class = StoredValue::new(HEADING_CLASS);

    view! {
        <CalendarHeading attr:class=class.get_value() />
    }
}

#[component]
pub fn ThemedCalendarPrevButton() -> impl IntoView {
    let class = StoredValue::new(NAV_BUTTON_CLASS);

    view! {
        <CalendarPrevButton attr:class=class.get_value()>
            <ChevronLeftIcon />
        </CalendarPrevButton>
    }
}

#[component]
pub fn ThemedCalendarNextButton() -> impl IntoView {
    let class = StoredValue::new(NAV_BUTTON_CLASS);

    view! {
        <CalendarNextButton attr:class=class.get_value()>
            <ChevronRightIcon />
        </CalendarNextButton>
    }
}

#[component]
pub fn ThemedCalendarGrid(children: ChildrenFn) -> impl IntoView {
    let class = StoredValue::new(GRID_CLASS);

    view! {
        <CalendarGrid attr:class=class.get_value()>
            {children()}
        </CalendarGrid>
    }
}

#[component]
pub fn ThemedCalendarGridHead() -> impl IntoView {
    view! {
        <CalendarGridHead />
    }
}

#[component]
pub fn ThemedCalendarGridBody() -> impl IntoView {
    view! {
        <CalendarGridBody />
    }
}

// ---------------------------------------------------------------------------
// Icons
// ---------------------------------------------------------------------------

#[component]
fn ChevronLeftIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m15 18-6-6 6-6" />
        </svg>
    }
}

#[component]
fn ChevronRightIcon() -> impl IntoView {
    view! {
        <svg
            class="size-4"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <path d="m9 18 6-6-6-6" />
        </svg>
    }
}
