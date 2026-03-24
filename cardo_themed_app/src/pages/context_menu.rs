use cardo_ui::context_menu::CheckedState;
use leptos::prelude::*;

use crate::theme::context_menu::*;

#[component]
pub fn ContextMenuPage() -> impl IntoView {
    let (show_bookmarks, set_show_bookmarks) = signal(true);
    let (show_urls, set_show_urls) = signal(false);
    let (person, set_person) = signal("pedro".to_string());

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Context Menu"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays a menu at the pointer position via right-click or long-press."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Right-Click Area"</h2>
                <ThemedContextMenu>
                    <ThemedContextMenuTrigger>
                        <div class="flex h-[200px] w-[350px] items-center justify-center rounded-md border border-dashed border-border text-sm text-muted-foreground">
                            "Right click here"
                        </div>
                    </ThemedContextMenuTrigger>
                    <ThemedContextMenuContent>
                        <ThemedContextMenuItem>"Back"</ThemedContextMenuItem>
                        <ThemedContextMenuItem>"Forward"</ThemedContextMenuItem>
                        <ThemedContextMenuItem>"Reload"</ThemedContextMenuItem>
                        <ThemedContextMenuSeparator />
                        <ThemedContextMenuSub>
                            <ThemedContextMenuSubTrigger>"More Tools"</ThemedContextMenuSubTrigger>
                            <ThemedContextMenuSubContent>
                                <ThemedContextMenuItem>"Save Page As..."</ThemedContextMenuItem>
                                <ThemedContextMenuItem>"Create Shortcut..."</ThemedContextMenuItem>
                                <ThemedContextMenuItem>"Name Window..."</ThemedContextMenuItem>
                                <ThemedContextMenuSeparator />
                                <ThemedContextMenuItem>"Developer Tools"</ThemedContextMenuItem>
                            </ThemedContextMenuSubContent>
                        </ThemedContextMenuSub>
                        <ThemedContextMenuSeparator />
                        <ThemedContextMenuCheckboxItem
                            checked=MaybeProp::derive(move || Some(if show_bookmarks.get() { CheckedState::True } else { CheckedState::False }))
                            on_checked_change=Callback::new(move |val: bool| set_show_bookmarks.set(val))
                        >
                            "Show Bookmarks Bar"
                        </ThemedContextMenuCheckboxItem>
                        <ThemedContextMenuCheckboxItem
                            checked=MaybeProp::derive(move || Some(if show_urls.get() { CheckedState::True } else { CheckedState::False }))
                            on_checked_change=Callback::new(move |val: bool| set_show_urls.set(val))
                        >
                            "Show Full URLs"
                        </ThemedContextMenuCheckboxItem>
                        <ThemedContextMenuSeparator />
                        <ThemedContextMenuLabel>"People"</ThemedContextMenuLabel>
                        <ThemedContextMenuRadioGroup
                            value=MaybeProp::derive(move || Some(person.get()))
                            on_value_change=Callback::new(move |val: String| set_person.set(val))
                        >
                            <ThemedContextMenuRadioItem value="pedro">"Pedro Duarte"</ThemedContextMenuRadioItem>
                            <ThemedContextMenuRadioItem value="colm">"Colm Tuite"</ThemedContextMenuRadioItem>
                        </ThemedContextMenuRadioGroup>
                    </ThemedContextMenuContent>
                </ThemedContextMenu>
            </section>
        </div>
    }
}
