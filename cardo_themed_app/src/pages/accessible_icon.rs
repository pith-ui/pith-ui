use leptos::prelude::*;

use cardo_ui::accessible_icon::AccessibleIcon;

#[component]
pub fn AccessibleIconPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Accessible Icon"</h1>
                <p class="text-muted-foreground mb-6">
                    "Makes icons accessible by adding a visually hidden label for screen readers."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Icons with Labels"</h2>
                <p class="text-sm text-muted-foreground">
                    "Each icon below has a visually hidden label that screen readers will announce. "
                    "The icon itself is hidden from assistive technology with aria-hidden."
                </p>
                <div class="flex items-center gap-6">
                    <div class="flex flex-col items-center gap-2">
                        <AccessibleIcon label=Signal::stored("Close".to_string())>
                            <svg
                                class="size-6 text-foreground"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M18 6 6 18" />
                                <path d="m6 6 12 12" />
                            </svg>
                        </AccessibleIcon>
                        <span class="text-xs text-muted-foreground">"Close"</span>
                    </div>

                    <div class="flex flex-col items-center gap-2">
                        <AccessibleIcon label=Signal::stored("Search".to_string())>
                            <svg
                                class="size-6 text-foreground"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <circle cx="11" cy="11" r="8" />
                                <path d="m21 21-4.3-4.3" />
                            </svg>
                        </AccessibleIcon>
                        <span class="text-xs text-muted-foreground">"Search"</span>
                    </div>

                    <div class="flex flex-col items-center gap-2">
                        <AccessibleIcon label=Signal::stored("Settings".to_string())>
                            <svg
                                class="size-6 text-foreground"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
                                <circle cx="12" cy="12" r="3" />
                            </svg>
                        </AccessibleIcon>
                        <span class="text-xs text-muted-foreground">"Settings"</span>
                    </div>

                    <div class="flex flex-col items-center gap-2">
                        <AccessibleIcon label=Signal::stored("Notifications".to_string())>
                            <svg
                                class="size-6 text-foreground"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            >
                                <path d="M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" />
                                <path d="M10.3 21a1.94 1.94 0 0 0 3.4 0" />
                            </svg>
                        </AccessibleIcon>
                        <span class="text-xs text-muted-foreground">"Notifications"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
