use leptos::prelude::*;

use crate::theme::tabs::*;

#[component]
pub fn TabsPage() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Tabs"</h1>
                <p class="text-muted-foreground mb-6">
                    "A set of layered sections of content displayed one at a time."
                </p>
            </div>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Default"</h2>
                <ThemedTabs default_value="account">
                    <ThemedTabsList>
                        <ThemedTabsTrigger value="account">"Account"</ThemedTabsTrigger>
                        <ThemedTabsTrigger value="password">"Password"</ThemedTabsTrigger>
                        <ThemedTabsTrigger value="settings">"Settings"</ThemedTabsTrigger>
                    </ThemedTabsList>
                    <ThemedTabsContent value="account">
                        <div class="mt-4 rounded-lg border border-border bg-card p-4">
                            <p class="text-sm text-foreground">"Make changes to your account here."</p>
                        </div>
                    </ThemedTabsContent>
                    <ThemedTabsContent value="password">
                        <div class="mt-4 rounded-lg border border-border bg-card p-4">
                            <p class="text-sm text-foreground">"Change your password here."</p>
                        </div>
                    </ThemedTabsContent>
                    <ThemedTabsContent value="settings">
                        <div class="mt-4 rounded-lg border border-border bg-card p-4">
                            <p class="text-sm text-foreground">"Adjust your settings here."</p>
                        </div>
                    </ThemedTabsContent>
                </ThemedTabs>
            </section>

            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"With Disabled Tab"</h2>
                <ThemedTabs default_value="active">
                    <ThemedTabsList>
                        <ThemedTabsTrigger value="active">"Active"</ThemedTabsTrigger>
                        <ThemedTabsTrigger value="disabled" disabled=true>"Disabled"</ThemedTabsTrigger>
                        <ThemedTabsTrigger value="other">"Other"</ThemedTabsTrigger>
                    </ThemedTabsList>
                    <ThemedTabsContent value="active">
                        <div class="mt-4 rounded-lg border border-border bg-card p-4">
                            <p class="text-sm text-foreground">"This tab is active."</p>
                        </div>
                    </ThemedTabsContent>
                    <ThemedTabsContent value="other">
                        <div class="mt-4 rounded-lg border border-border bg-card p-4">
                            <p class="text-sm text-foreground">"Other content."</p>
                        </div>
                    </ThemedTabsContent>
                </ThemedTabs>
            </section>
        </div>
    }
}
