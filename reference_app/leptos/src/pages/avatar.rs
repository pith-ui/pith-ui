use leptos::prelude::*;
use radix_leptos_primitives::avatar::*;

const WORKING_SRC: &str = "https://picsum.photos/id/1005/200/200";
const BROKEN_SRC: &str = "https://broken.example.com/no-image.png";

#[component]
pub fn AvatarPage() -> impl IntoView {
    let (working_status, set_working_status) = signal("idle".to_string());
    let (broken_status, set_broken_status) = signal("idle".to_string());
    let (dynamic_src, set_dynamic_src) = signal::<Option<String>>(None);

    let status_to_string = |status: ImageLoadingStatus| -> String {
        match status {
            ImageLoadingStatus::Idle => "idle".to_string(),
            ImageLoadingStatus::Loading => "loading".to_string(),
            ImageLoadingStatus::Loaded => "loaded".to_string(),
            ImageLoadingStatus::Error => "error".to_string(),
        }
    };

    view! {
        <h2>"Avatar"</h2>

        // No image — immediate fallback
        <div class="avatar-container">
            <Avatar attr:class="avatar-root" attr:data-testid="avatar-no-image">
                <AvatarImage attr:class="avatar-image" />
                <AvatarFallback attr:class="avatar-fallback">"NI"</AvatarFallback>
            </Avatar>
            <span>"No image"</span>
        </div>

        // Working image
        <div class="avatar-container">
            <Avatar attr:class="avatar-root" attr:data-testid="avatar-working">
                <AvatarImage
                    attr:class="avatar-image"
                    src=WORKING_SRC.to_string()
                    attr:alt="Working avatar"
                    on_loading_status_change=Callback::new(move |status: ImageLoadingStatus| {
                        set_working_status.set(status_to_string(status));
                    })
                />
                <AvatarFallback attr:class="avatar-fallback">"WI"</AvatarFallback>
            </Avatar>
            <span>
                "Working image (status: "
                <span data-testid="status-working">{move || working_status.get()}</span>
                ")"
            </span>
        </div>

        // Broken image
        <div class="avatar-container">
            <Avatar attr:class="avatar-root" attr:data-testid="avatar-broken">
                <AvatarImage
                    attr:class="avatar-image"
                    src=BROKEN_SRC.to_string()
                    attr:alt="Broken avatar"
                    on_loading_status_change=Callback::new(move |status: ImageLoadingStatus| {
                        set_broken_status.set(status_to_string(status));
                    })
                />
                <AvatarFallback attr:class="avatar-fallback">"BI"</AvatarFallback>
            </Avatar>
            <span>
                "Broken image (status: "
                <span data-testid="status-broken">{move || broken_status.get()}</span>
                ")"
            </span>
        </div>

        // Delayed fallback with broken image
        <div class="avatar-container">
            <Avatar attr:class="avatar-root" attr:data-testid="avatar-delayed">
                <AvatarImage
                    attr:class="avatar-image"
                    src=BROKEN_SRC.to_string()
                    attr:alt="Delayed avatar"
                />
                <AvatarFallback attr:class="avatar-fallback" delay_ms=300>
                    "DI"
                </AvatarFallback>
            </Avatar>
            <span>"Delayed fallback (300ms)"</span>
        </div>

        // Dynamic source
        <div class="avatar-container">
            <Avatar attr:class="avatar-root" attr:data-testid="avatar-dynamic">
                <AvatarImage
                    attr:class="avatar-image"
                    src=Signal::derive(move || dynamic_src.get())
                    attr:alt="Dynamic avatar"
                />
                <AvatarFallback attr:class="avatar-fallback">"DY"</AvatarFallback>
            </Avatar>
            <span>"Dynamic source"</span>
        </div>

        <div>
            <button data-testid="set-working-src" on:click=move |_| {
                set_dynamic_src.set(Some(WORKING_SRC.to_string()));
            }>
                "Set working src"
            </button>
            " "
            <button data-testid="set-broken-src" on:click=move |_| {
                set_dynamic_src.set(Some(BROKEN_SRC.to_string()));
            }>
                "Set broken src"
            </button>
            " "
            <button data-testid="clear-src" on:click=move |_| {
                set_dynamic_src.set(None);
            }>
                "Clear src"
            </button>
        </div>
    }
}
