use cardo_ui::dropdown_menu::CheckedState;
use leptos::prelude::*;

use crate::theme::button::*;
use crate::theme::dropdown_menu::*;

#[component]
pub fn DropdownMenuPage() -> impl IntoView {
    let (show_status_bar, set_show_status_bar) = signal(true);
    let (show_activity_bar, set_show_activity_bar) = signal(false);
    let (show_panel, set_show_panel) = signal(false);
    let (person, set_person) = signal("pedro".to_string());

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Dropdown Menu"</h1>
                <p class="text-muted-foreground mb-6">
                    "Displays a menu to the user — such as a set of actions or functions — triggered by a button."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Basic Menu"</h2>
                <ThemedDropdownMenu>
                    <ThemedDropdownMenuTrigger>
                        <Button variant=ButtonVariant::Outline>"Open Menu"</Button>
                    </ThemedDropdownMenuTrigger>
                    <ThemedDropdownMenuContent>
                        <ThemedDropdownMenuLabel>"My Account"</ThemedDropdownMenuLabel>
                        <ThemedDropdownMenuSeparator />
                        <ThemedDropdownMenuItem>"Profile"</ThemedDropdownMenuItem>
                        <ThemedDropdownMenuItem>"Billing"</ThemedDropdownMenuItem>
                        <ThemedDropdownMenuItem>"Settings"</ThemedDropdownMenuItem>
                        <ThemedDropdownMenuItem>"Keyboard shortcuts"</ThemedDropdownMenuItem>
                        <ThemedDropdownMenuSeparator />
                        <ThemedDropdownMenuSub>
                            <ThemedDropdownMenuSubTrigger>"More Tools"</ThemedDropdownMenuSubTrigger>
                            <ThemedDropdownMenuSubContent>
                                <ThemedDropdownMenuItem>"Save Page As..."</ThemedDropdownMenuItem>
                                <ThemedDropdownMenuItem>"Create Shortcut..."</ThemedDropdownMenuItem>
                                <ThemedDropdownMenuItem>"Name Window..."</ThemedDropdownMenuItem>
                                <ThemedDropdownMenuSeparator />
                                <ThemedDropdownMenuItem>"Developer Tools"</ThemedDropdownMenuItem>
                            </ThemedDropdownMenuSubContent>
                        </ThemedDropdownMenuSub>
                        <ThemedDropdownMenuSeparator />
                        <ThemedDropdownMenuItem>"Log out"</ThemedDropdownMenuItem>
                    </ThemedDropdownMenuContent>
                </ThemedDropdownMenu>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Checkbox Items"</h2>
                <ThemedDropdownMenu>
                    <ThemedDropdownMenuTrigger>
                        <Button variant=ButtonVariant::Outline>"View Options"</Button>
                    </ThemedDropdownMenuTrigger>
                    <ThemedDropdownMenuContent>
                        <ThemedDropdownMenuLabel>"Appearance"</ThemedDropdownMenuLabel>
                        <ThemedDropdownMenuSeparator />
                        <ThemedDropdownMenuCheckboxItem
                            checked=MaybeProp::derive(move || Some(if show_status_bar.get() { CheckedState::True } else { CheckedState::False }))
                            on_checked_change=Callback::new(move |val: bool| set_show_status_bar.set(val))
                        >
                            "Status Bar"
                        </ThemedDropdownMenuCheckboxItem>
                        <ThemedDropdownMenuCheckboxItem
                            checked=MaybeProp::derive(move || Some(if show_activity_bar.get() { CheckedState::True } else { CheckedState::False }))
                            on_checked_change=Callback::new(move |val: bool| set_show_activity_bar.set(val))
                        >
                            "Activity Bar"
                        </ThemedDropdownMenuCheckboxItem>
                        <ThemedDropdownMenuCheckboxItem
                            checked=MaybeProp::derive(move || Some(if show_panel.get() { CheckedState::True } else { CheckedState::False }))
                            on_checked_change=Callback::new(move |val: bool| set_show_panel.set(val))
                        >
                            "Panel"
                        </ThemedDropdownMenuCheckboxItem>
                    </ThemedDropdownMenuContent>
                </ThemedDropdownMenu>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Radio Items"</h2>
                <ThemedDropdownMenu>
                    <ThemedDropdownMenuTrigger>
                        <Button variant=ButtonVariant::Outline>"Select Person"</Button>
                    </ThemedDropdownMenuTrigger>
                    <ThemedDropdownMenuContent>
                        <ThemedDropdownMenuLabel>"People"</ThemedDropdownMenuLabel>
                        <ThemedDropdownMenuSeparator />
                        <ThemedDropdownMenuRadioGroup
                            value=MaybeProp::derive(move || Some(person.get()))
                            on_value_change=Callback::new(move |val: String| set_person.set(val))
                        >
                            <ThemedDropdownMenuRadioItem value="pedro">"Pedro Duarte"</ThemedDropdownMenuRadioItem>
                            <ThemedDropdownMenuRadioItem value="colm">"Colm Tuite"</ThemedDropdownMenuRadioItem>
                        </ThemedDropdownMenuRadioGroup>
                    </ThemedDropdownMenuContent>
                </ThemedDropdownMenu>
            </section>
        </div>
    }
}
