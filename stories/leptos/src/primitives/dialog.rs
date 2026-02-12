use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_dialog::*;

stylance::import_crate_style!(classes, "src/primitives/dialog.stories.module.css");

#[component]
pub fn Styled() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent attr:class=classes::contentDefault>
                    <DialogTitle>"Booking info"</DialogTitle>
                    <DialogDescription>"Please enter the info for your booking below."</DialogDescription>
                    <DialogClose attr:class=classes::close>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn NonModal() -> impl IntoView {
    view! {
        <Dialog modal=false>
            <DialogTrigger attr:class=classes::trigger>"open (non-modal)"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent
                    attr:class=format!("{} {}", classes::contentDefault, classes::contentSheet)
                    on_interact_outside=Callback::new(|event: web_sys::CustomEvent| {
                        event.prevent_default();
                    })
                >
                    <DialogTitle>"Booking info"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <DialogClose attr:class=classes::close>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        {(0..5).map(|_| view! {
            <div style="margin-top: 20px;">
                <textarea
                    style="width: 800px; height: 400px;"
                >"Lorem ipsum dolor sit amet consectetur adipisicing elit. Quaerat nobis at ipsa, nihil tempora debitis maxime dignissimos non amet, minima expedita alias et fugit voluptate laborum placeat odio dolore ab!"</textarea>
            </div>
        }).collect_view()}
    }
}

#[component]
pub fn Controlled() -> impl IntoView {
    let (open, set_open) = signal(false);

    view! {
        <Dialog
            open=open
            on_open_change=Callback::new(move |value: bool| set_open.set(value))
        >
            <DialogTrigger>
                {move || if open.get() { "close" } else { "open" }}
            </DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent attr:class=classes::contentDefault>
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <DialogClose>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn FocusTrap() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent attr:class=classes::contentDefault>
                    <DialogClose>"close"</DialogClose>
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <div>
                        <label r#for="firstName">"First Name"</label>
                        <input type="text" id="firstName" placeholder="John" />

                        <label r#for="lastName">"Last Name"</label>
                        <input type="text" id="lastName" placeholder="Doe" />

                        <button type="submit">"Send"</button>
                    </div>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        <p>"These elements can't be focused when the dialog is opened."</p>
        <button type="button">"A button"</button>
        <input type="text" placeholder="Another focusable element" />
    }
}

#[component]
pub fn CustomFocus() -> impl IntoView {
    let first_name_ref = AnyNodeRef::new();
    let search_field_ref = AnyNodeRef::new();

    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent
                    attr:class=classes::contentDefault
                    on_open_auto_focus=Callback::new(move |event: web_sys::Event| {
                        event.prevent_default();
                        if let Some(el) = first_name_ref.get_untracked() {
                            use web_sys::wasm_bindgen::JsCast;
                            let el: &web_sys::HtmlElement = el.unchecked_ref();
                            el.focus().ok();
                        }
                    })
                    on_close_auto_focus=Callback::new(move |event: web_sys::Event| {
                        event.prevent_default();
                        if let Some(el) = search_field_ref.get_untracked() {
                            use web_sys::wasm_bindgen::JsCast;
                            let el: &web_sys::HtmlElement = el.unchecked_ref();
                            el.focus().ok();
                        }
                    })
                >
                    <DialogClose>"close"</DialogClose>

                    <div>
                        <DialogTitle>"Title"</DialogTitle>
                        <DialogDescription>
                            "The first name input will receive the focus after opening the dialog."
                        </DialogDescription>
                        <label r#for="firstName">"First Name"</label>
                        <input type="text" id="firstName" placeholder="John" node_ref=first_name_ref />

                        <label r#for="lastName">"Last Name"</label>
                        <input type="text" id="lastName" placeholder="Doe" />

                        <button type="submit">"Send"</button>
                    </div>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        <div>
            <p>"The search input will receive the focus after closing the dialog."</p>
            <input type="text" placeholder="Search\u{2026}" node_ref=search_field_ref />
        </div>
    }
}

#[component]
pub fn NoEscapeDismiss() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent
                    attr:class=classes::contentDefault
                    on_escape_key_down=Callback::new(|event: web_sys::KeyboardEvent| {
                        event.prevent_default();
                    })
                >
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>
                        "The first name input will receive the focus after opening the dialog."
                    </DialogDescription>
                    <DialogClose>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn NoPointerDownOutsideDismiss() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent
                    attr:class=classes::contentDefault
                    on_pointer_down_outside=Callback::new(|event: web_sys::CustomEvent| {
                        event.prevent_default();
                    })
                >
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <DialogClose>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn WithPortalContainer() -> impl IntoView {
    let container_ref = AnyNodeRef::new();

    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal container_ref=container_ref>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent attr:class=classes::contentDefault>
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <DialogClose>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
        <div data-portal-container="" node_ref=container_ref />
    }
}

#[component]
pub fn Animated() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=format!("{} {}", classes::overlay, classes::animatedOverlay) />
                <DialogContent attr:class=format!("{} {}", classes::contentDefault, classes::animatedContent)>
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <DialogClose>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn ForcedMount() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger>"open"</DialogTrigger>
            <DialogPortal force_mount=true>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent attr:class=classes::contentDefault>
                    <DialogTitle>"Title"</DialogTitle>
                    <DialogDescription>"Description"</DialogDescription>
                    <DialogClose>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn InnerScrollable() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogOverlay attr:class=classes::overlay />
                <DialogContent attr:class=format!("{} {}", classes::contentDefault, classes::contentScrollable)>
                    <DialogTitle>"Booking info"</DialogTitle>
                    <DialogDescription>"Please enter the info for your booking below."</DialogDescription>
                    <div style="background-color: #eee; height: 500px;" />
                    <DialogClose attr:class=classes::close>"close"</DialogClose>
                </DialogContent>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn OuterScrollable() -> impl IntoView {
    view! {
        <Dialog>
            <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
            <div style="background-color: #eee; width: 300px; height: 1000px;" />
            <DialogPortal>
                <DialogOverlay attr:class=format!("{} {}", classes::overlay, classes::scrollableOverlay)>
                    <DialogContent
                        attr:class=format!("{} {}", classes::contentDefault, classes::contentInScrollableOverlay)
                    >
                        <DialogTitle>"Booking info"</DialogTitle>
                        <DialogDescription>"Please enter the info for your booking below."</DialogDescription>
                        <div style="background-color: #eee; height: 500px;" />
                        <DialogClose attr:class=classes::close>"close"</DialogClose>
                    </DialogContent>
                </DialogOverlay>
            </DialogPortal>
        </Dialog>
    }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: repeat(4, 1fr); height: 50vh;">
            <div>
                <h1>"Uncontrolled"</h1>
                <h2>"Closed"</h2>
                <Dialog>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay attr:class=classes::overlay />
                        <DialogContent attr:class=format!("{} {}", classes::contentDefault, classes::chromaticContent)>
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>

                <h2>"Open"</h2>
                <Dialog default_open=true>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 0; bottom: 50%; width: 25%;"
                        />
                        <DialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 12%;"
                        >
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>

            <div>
                <h1>"Uncontrolled with reordered parts"</h1>
                <h2>"Closed"</h2>
                <Dialog>
                    <DialogPortal>
                        <DialogOverlay attr:class=classes::overlay />
                        <DialogContent attr:class=classes::chromaticContent>
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                </Dialog>

                <h2>"Open"</h2>
                <Dialog default_open=true>
                    <DialogPortal>
                        <DialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 25%; bottom: 50%; width: 25%;"
                        />
                        <DialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 37%;"
                        >
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                </Dialog>
            </div>

            <div>
                <h1>"Controlled"</h1>
                <h2>"Closed"</h2>
                <Dialog open=false>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay attr:class=classes::overlay />
                        <DialogContent attr:class=classes::chromaticContent>
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>

                <h2>"Open"</h2>
                <Dialog open=true>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 50%; bottom: 50%; width: 25%;"
                        />
                        <DialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 62%;"
                        >
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>

            <div>
                <h1>"Controlled with reordered parts"</h1>
                <h2>"Closed"</h2>
                <Dialog open=false>
                    <DialogPortal>
                        <DialogOverlay attr:class=classes::overlay />
                        <DialogContent attr:class=classes::chromaticContent>
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                </Dialog>

                <h2>"Open"</h2>
                <Dialog open=true>
                    <DialogPortal>
                        <DialogOverlay
                            attr:class=classes::overlay
                            attr:style="left: 75%; bottom: 50%; width: 25%;"
                        />
                        <DialogContent
                            attr:class=classes::chromaticContent
                            attr:style="top: 25%; left: 88%;"
                        >
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                </Dialog>
            </div>
        </div>

        <div style="display: grid; grid-template-columns: repeat(2, 1fr); height: 50vh;">
            <div>
                <h1>"Forced mount"</h1>
                <Dialog>
                    <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
                    <DialogPortal force_mount=true>
                        <DialogOverlay
                            attr:class=classes::overlay
                            attr:style="top: 50%; background-color: rgba(0, 0, 0, 0.3);"
                        />
                        <DialogContent
                            attr:class=classes::chromaticContent
                            attr:style="left: 25%; top: 75%;"
                        >
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::close>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>

            <div>
                <h1>"State attributes"</h1>
                <h2>"Closed"</h2>
                <Dialog>
                    <DialogTrigger attr:class=classes::triggerAttr>"open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay attr:class=classes::overlayAttr />
                        <DialogContent attr:class=format!("{} {}", classes::chromaticContent, classes::contentAttr)>
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::closeAttr>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>

                <h2>"Open"</h2>
                <Dialog default_open=true>
                    <DialogTrigger attr:class=classes::triggerAttr>"open"</DialogTrigger>
                    <DialogPortal>
                        <DialogOverlay
                            attr:class=format!("{} {}", classes::overlay, classes::overlayAttr)
                            attr:style="left: 50%; top: 50%;"
                        />
                        <DialogContent
                            attr:class=format!("{} {}", classes::chromaticContent, classes::contentAttr)
                            attr:style="left: 75%; top: 75%;"
                        >
                            <DialogTitle>"Title"</DialogTitle>
                            <DialogDescription>"Description"</DialogDescription>
                            <DialogClose attr:class=classes::closeAttr>"close"</DialogClose>
                        </DialogContent>
                    </DialogPortal>
                </Dialog>
            </div>
        </div>
    }
}

#[component]
pub fn Cypress() -> impl IntoView {
    let (modal, set_modal) = signal(true);
    let (animated, set_animated) = signal(false);
    let (count, set_count) = signal(0);
    let (has_destroy_button, set_has_destroy_button) = signal(true);

    view! {
        <Dialog modal=modal>
            <DialogTrigger attr:class=classes::trigger>"open"</DialogTrigger>
            <DialogPortal>
                <DialogContent
                    attr:class=move || {
                        let mut class_names = vec![classes::contentDefault.to_string()];
                        if animated.get() {
                            class_names.push(classes::animatedContent.to_string());
                            class_names.push(classes::duration50.to_string());
                        }
                        class_names.join(" ")
                    }
                >
                    <DialogTitle>"title"</DialogTitle>
                    <DialogDescription>"description"</DialogDescription>
                    <DialogClose attr:class=classes::close>"close"</DialogClose>
                    <Show when=move || has_destroy_button.get()>
                        <div>
                            <button type="button" on:click=move |_| set_has_destroy_button.set(false)>
                                "destroy me"
                            </button>
                        </div>
                    </Show>
                </DialogContent>
            </DialogPortal>
        </Dialog>

        <br />
        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || modal.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_modal.set(target.checked());
                }
            />
            " modal"
        </label>

        <br />

        <label>
            <input
                type="checkbox"
                prop:checked=move || animated.get()
                on:change=move |ev| {
                    use web_sys::wasm_bindgen::JsCast;
                    let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                    set_animated.set(target.checked());
                }
            />
            " animated"
        </label>

        <br />

        <label>
            "count up "
            <button type="button" on:click=move |_| set_count.update(|c| *c += 1)>
                {move || count.get().to_string()}
            </button>
        </label>

        <br />

        <label>
            "name: "
            <input type="text" placeholder="name" />
        </label>
    }
}
