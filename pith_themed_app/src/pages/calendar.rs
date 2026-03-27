use pith_ui::calendar::NaiveDate;
use leptos::prelude::*;

use crate::theme::calendar::*;

#[component]
pub fn CalendarPage() -> impl IntoView {
    let (selected, set_selected) = signal::<Option<NaiveDate>>(None);

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Calendar"</h1>
                <p class="text-muted-foreground mb-6">
                    "A date picker calendar grid with keyboard navigation and accessible date selection."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic Calendar"</h2>
                <ThemedCalendar>
                    <ThemedCalendarHeader>
                        <ThemedCalendarPrevButton />
                        <ThemedCalendarHeading />
                        <ThemedCalendarNextButton />
                    </ThemedCalendarHeader>
                    <ThemedCalendarGrid>
                        <ThemedCalendarGridHead />
                        <ThemedCalendarGridBody />
                    </ThemedCalendarGrid>
                </ThemedCalendar>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Controlled Calendar"</h2>
                <p class="text-sm text-muted-foreground">
                    "Selected date: "
                    {move || match selected.get() {
                        Some(date) => date.to_string(),
                        None => "None".to_string(),
                    }}
                </p>
                <ThemedCalendar
                    value=MaybeProp::derive(move || selected.get())
                    on_value_change=Callback::new(move |date: NaiveDate| {
                        set_selected.set(Some(date));
                    })
                >
                    <ThemedCalendarHeader>
                        <ThemedCalendarPrevButton />
                        <ThemedCalendarHeading />
                        <ThemedCalendarNextButton />
                    </ThemedCalendarHeader>
                    <ThemedCalendarGrid>
                        <ThemedCalendarGridHead />
                        <ThemedCalendarGridBody />
                    </ThemedCalendarGrid>
                </ThemedCalendar>
            </section>
        </div>
    }
}
