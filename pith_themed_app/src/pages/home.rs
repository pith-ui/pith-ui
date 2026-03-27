use leptos::prelude::*;

/// Renders a single color swatch.
#[component]
fn Swatch(label: &'static str, class: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-1.5">
            <div class={format!("h-16 w-full rounded-md border border-border {class}")} />
            <span class="text-xs text-muted-foreground">{label}</span>
        </div>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="space-y-10">
            <div>
                <h1 class="text-2xl font-bold text-foreground mb-1">"Design System"</h1>
                <p class="text-muted-foreground mb-6">
                    "shadcn/ui new-york baseline. All colors use OKLCH via CSS custom properties. "
                    "Toggle dark mode with the button in the sidebar."
                </p>
            </div>

            // Semantic color palette
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Semantic Colors"</h2>
                <div class="grid grid-cols-4 gap-3">
                    <Swatch label="background" class="bg-background" />
                    <Swatch label="foreground" class="bg-foreground" />
                    <Swatch label="card" class="bg-card" />
                    <Swatch label="popover" class="bg-popover" />
                </div>
                <div class="grid grid-cols-4 gap-3">
                    <Swatch label="primary" class="bg-primary" />
                    <Swatch label="primary-fg" class="bg-primary-foreground" />
                    <Swatch label="secondary" class="bg-secondary" />
                    <Swatch label="secondary-fg" class="bg-secondary-foreground" />
                </div>
                <div class="grid grid-cols-4 gap-3">
                    <Swatch label="muted" class="bg-muted" />
                    <Swatch label="muted-fg" class="bg-muted-foreground" />
                    <Swatch label="accent" class="bg-accent" />
                    <Swatch label="accent-fg" class="bg-accent-foreground" />
                </div>
                <div class="grid grid-cols-4 gap-3">
                    <Swatch label="destructive" class="bg-destructive" />
                    <Swatch label="destructive-fg" class="bg-destructive-foreground" />
                    <Swatch label="border" class="bg-border" />
                    <Swatch label="ring" class="bg-ring" />
                </div>
            </section>

            // Token mapping explanation
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Token Architecture"</h2>
                <div class="bg-card rounded-lg border border-border p-4 text-sm text-muted-foreground space-y-2">
                    <p>
                        <span class="font-medium text-foreground">"tokens.css"</span>
                        " \u{2192} OKLCH channels for each semantic role (primary, secondary, etc.)"
                    </p>
                    <p>
                        <span class="font-medium text-foreground">"tailwind.config.js"</span>
                        " \u{2192} wraps channels as "
                        <code class="text-foreground bg-muted px-1 rounded-sm text-xs">"oklch(var(--primary) / <alpha-value>)"</code>
                    </p>
                    <p>
                        <span class="font-medium text-foreground">"Components"</span>
                        " \u{2192} use Tailwind utilities like "
                        <code class="text-foreground bg-muted px-1 rounded-sm text-xs">"bg-primary"</code>
                        ", "
                        <code class="text-foreground bg-muted px-1 rounded-sm text-xs">"text-muted-foreground"</code>
                        ", "
                        <code class="text-foreground bg-muted px-1 rounded-sm text-xs">"hover:bg-primary/90"</code>
                    </p>
                    <p>
                        <span class="font-medium text-foreground">"Opacity modifiers"</span>
                        " \u{2192} "
                        <code class="text-foreground bg-muted px-1 rounded-sm text-xs">"/90"</code>
                        " works because channels are stored without the oklch() wrapper"
                    </p>
                </div>
            </section>

            // Border radius
            <section class="space-y-4">
                <h2 class="text-lg font-semibold text-foreground">"Border Radius"</h2>
                <p class="text-sm text-muted-foreground">
                    "Based on " <code class="bg-muted px-1 rounded-sm text-xs">"--radius: 0.625rem"</code>
                    " with sm/md/lg computed from it."
                </p>
                <div class="flex items-end gap-4">
                    <div class="flex flex-col items-center gap-1.5">
                        <div class="w-16 h-16 bg-primary/10 border border-border rounded-sm" />
                        <span class="text-xs text-muted-foreground">"sm"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1.5">
                        <div class="w-16 h-16 bg-primary/10 border border-border rounded-md" />
                        <span class="text-xs text-muted-foreground">"md"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1.5">
                        <div class="w-16 h-16 bg-primary/10 border border-border rounded-lg" />
                        <span class="text-xs text-muted-foreground">"lg"</span>
                    </div>
                    <div class="flex flex-col items-center gap-1.5">
                        <div class="w-16 h-16 bg-primary/10 border border-border rounded-full" />
                        <span class="text-xs text-muted-foreground">"full"</span>
                    </div>
                </div>
            </section>
        </div>
    }
}
