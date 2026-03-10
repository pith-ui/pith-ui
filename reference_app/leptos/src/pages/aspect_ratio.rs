use leptos::prelude::*;
use radix_leptos_primitives::aspect_ratio::*;

#[component]
pub fn AspectRatioPage() -> impl IntoView {
    view! {
        <div style:max-width="400px" style:margin="20px">
            <h2>"Default (1:1)"</h2>
            <AspectRatio attr:data-testid="default-ratio">
                <span>"1:1 content"</span>
            </AspectRatio>

            <h2>"Custom Ratio (16:9)"</h2>
            <AspectRatio ratio={16.0 / 9.0} attr:data-testid="custom-ratio">
                <span>"16:9 content"</span>
            </AspectRatio>

            <h2>"With Custom Style (background)"</h2>
            <AspectRatio
                ratio={16.0 / 9.0}
                attr:data-testid="with-custom-style"
                attr:style="background: tomato"
            >
                <span>"Custom background"</span>
            </AspectRatio>

            <h2>"With asChild"</h2>
            <AspectRatio ratio={16.0 / 9.0} as_child=true attr:data-testid="with-as-child">
                <img
                    src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='160' height='90'%3E%3Crect fill='%23ccc' width='160' height='90'/%3E%3C/svg%3E"
                    alt="placeholder"
                    style="object-fit: cover; width: 100%; height: 100%"
                />
            </AspectRatio>

            <h2>"With Conflicting Style (position + top)"</h2>
            <AspectRatio
                ratio={16.0 / 9.0}
                attr:data-testid="with-conflicting-style"
                attr:style="position: relative; top: 10px; background: cornflowerblue"
            >
                <span>"Conflicting styles"</span>
            </AspectRatio>
        </div>
    }
}
