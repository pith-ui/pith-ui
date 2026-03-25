use leptos::prelude::*;

use crate::theme::avatar::ThemedAvatar;

#[component]
pub fn AvatarPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Avatar"</h1>
                <p class="text-muted-foreground mb-6">
                    "Image element with fallback for representing a user."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"With Image"</h2>
                <div class="flex items-center gap-4">
                    <ThemedAvatar
                        src="https://github.com/shadcn.png"
                        alt="@shadcn"
                        fallback="CN".to_string()
                    />
                    <ThemedAvatar
                        src="https://github.com/leerob.png"
                        alt="@leerob"
                        fallback="LR".to_string()
                    />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Fallback Only"</h2>
                <div class="flex items-center gap-4">
                    <ThemedAvatar fallback="AB".to_string() />
                    <ThemedAvatar fallback="CD".to_string() />
                    <ThemedAvatar fallback="EF".to_string() />
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Broken Image (Fallback)"</h2>
                <div class="flex items-center gap-4">
                    <ThemedAvatar
                        src="https://broken-link.example/avatar.png"
                        fallback="??".to_string()
                    />
                </div>
                <p class="text-xs text-muted-foreground">
                    "The image URL is intentionally broken to show the fallback."
                </p>
            </section>
        </div>
    }
}
