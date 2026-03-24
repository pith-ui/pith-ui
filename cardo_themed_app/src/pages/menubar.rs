use cardo_ui::menubar::CheckedState;
use leptos::prelude::*;

use crate::theme::menubar::*;

#[component]
pub fn MenubarPage() -> impl IntoView {
    let (show_bookmarks, set_show_bookmarks) = signal(true);
    let (show_full_urls, set_show_full_urls) = signal(false);
    let (person, set_person) = signal("pedro".to_string());

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Menubar"</h1>
                <p class="text-muted-foreground mb-6">
                    "A visually persistent menu common in desktop applications that provides quick access to a consistent set of commands."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Application Menubar"</h2>
                <ThemedMenubar>
                    // File menu
                    <ThemedMenubarMenu value="file">
                        <ThemedMenubarTrigger>"File"</ThemedMenubarTrigger>
                        <ThemedMenubarContent>
                            <ThemedMenubarItem>
                                "New Tab"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+T"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarItem>
                                "New Window"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+N"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarSub>
                                <ThemedMenubarSubTrigger>"Share"</ThemedMenubarSubTrigger>
                                <ThemedMenubarSubContent>
                                    <ThemedMenubarItem>"Email Link"</ThemedMenubarItem>
                                    <ThemedMenubarItem>"Messages"</ThemedMenubarItem>
                                    <ThemedMenubarItem>"Notes"</ThemedMenubarItem>
                                </ThemedMenubarSubContent>
                            </ThemedMenubarSub>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarItem>
                                "Print..."
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+P"</span>
                            </ThemedMenubarItem>
                        </ThemedMenubarContent>
                    </ThemedMenubarMenu>

                    // Edit menu
                    <ThemedMenubarMenu value="edit">
                        <ThemedMenubarTrigger>"Edit"</ThemedMenubarTrigger>
                        <ThemedMenubarContent>
                            <ThemedMenubarItem>
                                "Undo"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+Z"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarItem>
                                "Redo"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+Y"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarItem>
                                "Cut"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+X"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarItem>
                                "Copy"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+C"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarItem>
                                "Paste"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+V"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarItem>
                                "Find"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+F"</span>
                            </ThemedMenubarItem>
                        </ThemedMenubarContent>
                    </ThemedMenubarMenu>

                    // View menu
                    <ThemedMenubarMenu value="view">
                        <ThemedMenubarTrigger>"View"</ThemedMenubarTrigger>
                        <ThemedMenubarContent>
                            <ThemedMenubarCheckboxItem
                                checked=MaybeProp::derive(move || Some(if show_bookmarks.get() { CheckedState::True } else { CheckedState::False }))
                                on_checked_change=Callback::new(move |val: bool| set_show_bookmarks.set(val))
                            >
                                "Always Show Bookmarks Bar"
                            </ThemedMenubarCheckboxItem>
                            <ThemedMenubarCheckboxItem
                                checked=MaybeProp::derive(move || Some(if show_full_urls.get() { CheckedState::True } else { CheckedState::False }))
                                on_checked_change=Callback::new(move |val: bool| set_show_full_urls.set(val))
                            >
                                "Always Show Full URLs"
                            </ThemedMenubarCheckboxItem>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarItem>
                                "Reload"
                                <span class="ml-auto text-xs text-muted-foreground tracking-widest">"Ctrl+R"</span>
                            </ThemedMenubarItem>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarItem>"Toggle Fullscreen"</ThemedMenubarItem>
                        </ThemedMenubarContent>
                    </ThemedMenubarMenu>

                    // Profiles menu
                    <ThemedMenubarMenu value="profiles">
                        <ThemedMenubarTrigger>"Profiles"</ThemedMenubarTrigger>
                        <ThemedMenubarContent>
                            <ThemedMenubarLabel>"People"</ThemedMenubarLabel>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarRadioGroup
                                value=MaybeProp::derive(move || Some(person.get()))
                                on_value_change=Callback::new(move |val: String| set_person.set(val))
                            >
                                <ThemedMenubarRadioItem value="pedro">"Pedro Duarte"</ThemedMenubarRadioItem>
                                <ThemedMenubarRadioItem value="colm">"Colm Tuite"</ThemedMenubarRadioItem>
                            </ThemedMenubarRadioGroup>
                            <ThemedMenubarSeparator />
                            <ThemedMenubarItem>"Add Profile..."</ThemedMenubarItem>
                        </ThemedMenubarContent>
                    </ThemedMenubarMenu>
                </ThemedMenubar>
            </section>
        </div>
    }
}
