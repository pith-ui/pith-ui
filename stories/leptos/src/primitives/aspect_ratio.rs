use leptos::prelude::*;
use radix_leptos_aspect_ratio::*;

stylance::import_crate_style!(
    aspect_ratio_classes,
    "src/primitives/aspect_ratio.stories.module.css"
);

#[component]
fn Image() -> impl IntoView {
    view! {
        <img
            src="https://images.unsplash.com/photo-1605030753481-bb38b08c384a?&auto=format&fit=crop&w=400&q=80"
            alt="A house in a forest"
            style:object-fit="cover"
            style:width="100%"
            style:height="100%"
        />
    }
}

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <div style:width="500px">
            <AspectRatio attr:class=aspect_ratio_classes::root>
                <h1>Default ratio (1/1)</h1>
            </AspectRatio>
        </div>
    }
}

#[component]
pub fn CustomRatios() -> impl IntoView {
    view! {
        <div style:display="flex" style:gap="20px">
            <div style:width="200px">
                <AspectRatio ratio=1.0 / 2.0 style:bottom="100px"><Image /></AspectRatio>
            </div>
            <div style:width="200px">
                <AspectRatio><Image /></AspectRatio>
            </div>
            <div style:width="200px">
                <AspectRatio ratio=16.0 / 9.0><Image /></AspectRatio>
            </div>
            <div style:width="200px">
                <AspectRatio ratio=2.0 / 1.0><Image /></AspectRatio>
            </div>
        </div>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <h1>Default ratio</h1>
        <div style:width="300px">
            <AspectRatio attr:class=aspect_ratio_classes::root>
                <p>Default ratio (1/1)</p>
            </AspectRatio>
        </div>

        <h1>Custom ratios</h1>
        <div style:display="flex" style:gap="20px">
            <div style:width="200px">
                <AspectRatio ratio=1.0 / 2.0><Image /></AspectRatio>
            </div>
            <div style:width="200px">
                <AspectRatio><Image /></AspectRatio>
            </div>
            <div style:width="200px">
                <AspectRatio ratio=16.0 / 9.0><Image /></AspectRatio>
            </div>
            <div style:width="200px">
                <AspectRatio ratio=2.0 / 1.0><Image /></AspectRatio>
            </div>
        </div>
    }
}
