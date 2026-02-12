use leptos::prelude::*;
use radix_leptos_alert_dialog::*;

stylance::import_crate_style!(classes, "src/primitives/alert_dialog.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
            <AlertDialogPortal>
                <AlertDialogOverlay attr:class=classes::overlay />
                <AlertDialogContent attr:class=classes::content>
                    <AlertDialogTitle attr:class=classes::title>"Are you sure?"</AlertDialogTitle>
                    <AlertDialogDescription attr:class=classes::description>
                        "This will do a very dangerous thing. Thar be dragons!"
                    </AlertDialogDescription>
                    <AlertDialogAction attr:class=classes::action>"yolo, do it"</AlertDialogAction>
                    <AlertDialogCancel attr:class=classes::cancel>"maybe not"</AlertDialogCancel>
                </AlertDialogContent>
            </AlertDialogPortal>
        </AlertDialog>
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (house_purchased, set_house_purchased) = signal(false);

    view! {
        <div>
            <div>
                <img src="https://i.ibb.co/K54hsKt/house.jpg" alt="a large white house with a red roof" />
            </div>
            <AlertDialog
                open=open
                on_open_change=Callback::new(move |value: bool| set_open.set(value))
            >
                <AlertDialogTrigger
                    on_click=Callback::new(move |e: web_sys::MouseEvent| {
                        if house_purchased.get_untracked() {
                            e.prevent_default();
                            set_house_purchased.set(false);
                        }
                    })
                >
                    {move || if house_purchased.get() {
                        "You bought the house! Sell it!"
                    } else {
                        "Buy this house"
                    }}
                </AlertDialogTrigger>
                <AlertDialogPortal>
                    <AlertDialogOverlay attr:class=classes::overlay />
                    <AlertDialogContent attr:class=classes::content>
                        <AlertDialogTitle>"Are you sure?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Houses are very expensive and it looks like you only have \u{20AC}20 in the bank. Maybe consult with a financial advisor?"
                        </AlertDialogDescription>
                        <AlertDialogAction
                            attr:class=classes::action
                            on_click=Callback::new(move |_: web_sys::MouseEvent| {
                                set_house_purchased.set(true);
                            })
                        >
                            "buy it anyway"
                        </AlertDialogAction>
                        <AlertDialogCancel attr:class=classes::cancel>
                            "good point, I'll reconsider"
                        </AlertDialogCancel>
                    </AlertDialogContent>
                </AlertDialogPortal>
            </AlertDialog>
        </div>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(4, 1fr); grid-template-rows: repeat(2, 1fr); height: 100vh;">
            <div>
                <h1>"Uncontrolled"</h1>
                <h2>"Closed"</h2>
                <AlertDialog>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay attr:class=classes::overlay />
                        <AlertDialogContent attr:class=classes::chromaticContent>
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>

                <h2>"Open"</h2>
                <AlertDialog default_open=true>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 0; bottom: 50%; width: 25%;"
                        />
                        <AlertDialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 12%;"
                        >
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>
            </div>

            <div>
                <h1>"Uncontrolled with reordered parts"</h1>
                <h2>"Closed"</h2>
                <AlertDialog>
                    <AlertDialogPortal>
                        <AlertDialogOverlay attr:class=classes::overlay />
                        <AlertDialogContent attr:class=classes::chromaticContent>
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                </AlertDialog>

                <h2>"Open"</h2>
                <AlertDialog default_open=true>
                    <AlertDialogPortal>
                        <AlertDialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 25%; bottom: 50%; width: 25%;"
                        />
                        <AlertDialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 37%;"
                        >
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                </AlertDialog>
            </div>

            <div>
                <h1>"Controlled"</h1>
                <h2>"Closed"</h2>
                <AlertDialog open=false>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay attr:class=classes::overlay />
                        <AlertDialogContent attr:class=classes::chromaticContent>
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>

                <h2>"Open"</h2>
                <AlertDialog open=true>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 50%; bottom: 50%; width: 25%;"
                        />
                        <AlertDialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 62%;"
                        >
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>
            </div>

            <div>
                <h1>"Controlled with reordered parts"</h1>
                <h2>"Closed"</h2>
                <AlertDialog open=false>
                    <AlertDialogPortal>
                        <AlertDialogOverlay attr:class=classes::overlay />
                        <AlertDialogContent attr:class=classes::chromaticContent>
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                </AlertDialog>

                <h2>"Open"</h2>
                <AlertDialog open=true>
                    <AlertDialogPortal>
                        <AlertDialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 75%; bottom: 50%; width: 25%;"
                        />
                        <AlertDialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 88%;"
                        >
                            <AlertDialogTitle attr:class=classes::title>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::description>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::action>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancel>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                    <AlertDialogTrigger attr:class=classes::trigger>"delete everything"</AlertDialogTrigger>
                </AlertDialog>
            </div>

            <div>
                <h1>"State attributes"</h1>
                <h2>"Closed"</h2>
                <AlertDialog>
                    <AlertDialogTrigger attr:class=classes::triggerAttr>"delete everything"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay attr:class=classes::overlayAttr />
                        <AlertDialogContent attr:class=classes::contentAttr>
                            <AlertDialogTitle attr:class=classes::titleAttr>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::descriptionAttr>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::actionAttr>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancelAttr>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>

                <h2>"Open"</h2>
                <AlertDialog default_open=true>
                    <AlertDialogTrigger attr:class=classes::triggerAttr>"delete everything"</AlertDialogTrigger>
                    <AlertDialogPortal>
                        <AlertDialogOverlay
                            attr:class=classes::overlayAttr
                            attr:style="top: 50%;"
                        />
                        <AlertDialogContent
                            attr:class=classes::contentAttr
                            attr:style="top: 75%;"
                        >
                            <AlertDialogTitle attr:class=classes::titleAttr>"Title"</AlertDialogTitle>
                            <AlertDialogDescription attr:class=classes::descriptionAttr>
                                "Description"
                            </AlertDialogDescription>
                            <AlertDialogAction attr:class=classes::actionAttr>"Confirm"</AlertDialogAction>
                            <AlertDialogCancel attr:class=classes::cancelAttr>"Cancel"</AlertDialogCancel>
                        </AlertDialogContent>
                    </AlertDialogPortal>
                </AlertDialog>
            </div>
        </div>
    }
}
