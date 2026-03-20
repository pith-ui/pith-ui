use leptos::prelude::*;
use cardo_ui::popper::*;

#[component]
pub fn PopperPage() -> impl IntoView {
    view! {
        <div style="padding: 100px;">
            <h2>"Logical \"start\" alignment (LTR)"</h2>
            <div
                style="display: flex; align-items: center; gap: 150px; border: 1px solid black; padding: 20px;"
            >
                <Popper>
                    <PopperAnchor class:popper-anchor=true attr:data-testid="ltr-anchor">
                        "LTR"
                    </PopperAnchor>
                    <PopperContent
                        class:popper-content=true
                        align=Align::Start
                        side_offset=5.0
                        attr:data-testid="ltr-content"
                    >
                        "LTR content"
                    </PopperContent>
                </Popper>
            </div>

            <h2>"Logical \"start\" alignment (RTL)"</h2>
            <div
                style="display: flex; align-items: center; gap: 150px; border: 1px solid black; padding: 20px;"
            >
                <Popper>
                    <PopperAnchor class:popper-anchor=true attr:data-testid="rtl-anchor">
                        "RTL"
                    </PopperAnchor>
                    <PopperContent
                        class:popper-content=true
                        align=Align::Start
                        side_offset=5.0
                        dir="rtl".to_string()
                        attr:data-testid="rtl-content"
                    >
                        "RTL content"
                    </PopperContent>
                </Popper>
            </div>
        </div>
    }
}
