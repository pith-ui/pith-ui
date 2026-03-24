use leptos::prelude::*;

use crate::theme::one_time_password_field::*;

#[component]
pub fn OneTimePasswordFieldPage() -> impl IntoView {
    let (otp_value, set_otp_value) = signal(String::new());

    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"One-Time Password Field"</h1>
                <p class="text-muted-foreground mb-6">
                    "A segmented input for entering verification codes."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"6-Digit OTP"</h2>
                <div class="flex flex-col items-start gap-4">
                    <ThemedOneTimePasswordField
                        on_value_change=Callback::new(move |val: String| set_otp_value.set(val))
                    >
                        <ThemedOneTimePasswordFieldInput />
                        <ThemedOneTimePasswordFieldInput />
                        <ThemedOneTimePasswordFieldInput />
                        <span class="text-muted-foreground text-xl">"-"</span>
                        <ThemedOneTimePasswordFieldInput />
                        <ThemedOneTimePasswordFieldInput />
                        <ThemedOneTimePasswordFieldInput />
                    </ThemedOneTimePasswordField>
                    <p class="text-sm text-muted-foreground">
                        "Value: " {move || {
                            let v = otp_value.get();
                            if v.is_empty() { "(empty)".to_string() } else { v }
                        }}
                    </p>
                </div>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"4-Digit PIN"</h2>
                <ThemedOneTimePasswordField>
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                </ThemedOneTimePasswordField>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Disabled"</h2>
                <ThemedOneTimePasswordField disabled=true>
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                    <ThemedOneTimePasswordFieldInput />
                </ThemedOneTimePasswordField>
            </section>
        </div>
    }
}
