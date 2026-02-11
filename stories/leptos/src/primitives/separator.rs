use leptos::prelude::*;
use radix_leptos_separator::*;

stylance::import_crate_style!(classes, "src/primitives/separator.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <h1>"Horizontal"</h1>
        <p>"The following separator is horizontal and has semantic meaning."</p>
        <Separator attr:class=classes::root orientation=Orientation::Horizontal />
        <p>
            "The following separator is horizontal and is purely decorative. Assistive technology will ignore this element."
        </p>
        <Separator attr:class=classes::root orientation=Orientation::Horizontal decorative=true />

        <h1>"Vertical"</h1>
        <div style:display="flex" style:align-items="center">
            <p>"The following separator is vertical and has semantic meaning."</p>
            <Separator attr:class=classes::root orientation=Orientation::Vertical />
            <p>
                "The following separator is vertical and is purely decorative. Assistive technology will ignore this element."
            </p>
            <Separator attr:class=classes::root orientation=Orientation::Vertical decorative=true />
        </div>
    }
}
