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
