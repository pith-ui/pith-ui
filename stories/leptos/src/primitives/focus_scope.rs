use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_focus_scope::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn Basic() -> impl IntoView {
    let (trapped, set_trapped) = signal(false);
    let (has_destroy_button, set_has_destroy_button) = signal(true);

    view! {
        <div>
            <button type="button" on:click=move |_| set_trapped.set(true)>
                "Trap"
            </button>
            " "
            <input /> " " <input />
        </div>
        <Show when=move || trapped.get()>
            <FocusScope as_child=true r#loop=trapped trapped=trapped>
                <form
                    style:display="inline-flex"
                    style:flex-direction="column"
                    style:gap="20px"
                    style:padding="20px"
                    style:margin="50px"
                    style:max-width="500px"
                    style:border="2px solid"
                >
                    <input type="text" placeholder="First name" />
                    <input type="text" placeholder="Last name" />
                    <input type="number" placeholder="Age" />
                    <Show when=move || has_destroy_button.get()>
                        <div>
                            <button type="button" on:click=move |_| set_has_destroy_button.set(false)>
                                "Destroy me"
                            </button>
                        </div>
                    </Show>
                    <button type="button" on:click=move |_| set_trapped.set(false)>
                        "Close"
                    </button>
                </form>
            </FocusScope>
        </Show>
        <div>
            <input /> " " <input />
        </div>
    }
}

#[component]
pub fn Multiple() -> impl IntoView {
    let (trapped1, set_trapped1) = signal(false);
    let (trapped2, set_trapped2) = signal(false);

    view! {
        <div
            style:display="inline-flex"
            style:flex-direction="column"
            style:gap="10px"
        >
            <div>
                <button type="button" on:click=move |_| set_trapped1.set(true)>
                    "Trap 1"
                </button>
            </div>
            <Show when=move || trapped1.get()>
                <FocusScope as_child=true r#loop=trapped1 trapped=trapped1>
                    <form
                        style:display="inline-flex"
                        style:flex-direction="column"
                        style:gap="20px"
                        style:padding="20px"
                        style:max-width="500px"
                        style:border="2px solid"
                    >
                        <h1>"One"</h1>
                        <input type="text" placeholder="First name" />
                        <input type="text" placeholder="Last name" />
                        <input type="number" placeholder="Age" />
                        <button type="button" on:click=move |_| set_trapped1.set(false)>
                            "Close"
                        </button>
                    </form>
                </FocusScope>
            </Show>
            <div>
                <button type="button" on:click=move |_| set_trapped2.set(true)>
                    "Trap 2"
                </button>
            </div>
            <Show when=move || trapped2.get()>
                <FocusScope as_child=true r#loop=trapped2 trapped=trapped2>
                    <form
                        style:display="inline-flex"
                        style:flex-direction="column"
                        style:gap="20px"
                        style:padding="20px"
                        style:max-width="500px"
                        style:border="2px solid"
                    >
                        <h1>"Two"</h1>
                        <input type="text" placeholder="First name" />
                        <input type="text" placeholder="Last name" />
                        <input type="number" placeholder="Age" />
                        <button type="button" on:click=move |_| set_trapped2.set(false)>
                            "Close"
                        </button>
                    </form>
                </FocusScope>
            </Show>
            <div>
                <input />
            </div>
        </div>
    }
}

/// true => default focus, false => no focus, AnyNodeRef => focus specific element
#[derive(Clone, Copy)]
enum FocusParam {
    Default,
    None,
    Ref(AnyNodeRef),
}

impl PartialEq for FocusParam {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (FocusParam::Default, FocusParam::Default)
                | (FocusParam::None, FocusParam::None)
                | (FocusParam::Ref(_), FocusParam::Ref(_))
        )
    }
}

#[component]
pub fn WithOptions() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (is_empty_form, set_is_empty_form) = signal(false);

    let (trap_focus, set_trap_focus) = signal(false);
    let (focus_on_mount, set_focus_on_mount) = signal(FocusParam::None);
    let (focus_on_unmount, set_focus_on_unmount) = signal(FocusParam::None);

    let age_field_ref = AnyNodeRef::new();
    let next_button_ref = AnyNodeRef::new();

    let on_mount_auto_focus = Callback::new(move |event: web_sys::Event| {
        let focus_on_mount = focus_on_mount.get_untracked();
        if focus_on_mount != FocusParam::Default {
            event.prevent_default();
            if let FocusParam::Ref(node_ref) = focus_on_mount
                && let Some(el) = node_ref.get_untracked()
            {
                let el: &web_sys::HtmlElement = el.unchecked_ref();
                let _ = el.focus();
            }
        }
    });

    let on_unmount_auto_focus = Callback::new(move |event: web_sys::Event| {
        let focus_on_unmount = focus_on_unmount.get_untracked();
        if focus_on_unmount != FocusParam::Default {
            event.prevent_default();
            if let FocusParam::Ref(node_ref) = focus_on_unmount
                && let Some(el) = node_ref.get_untracked()
            {
                let el: &web_sys::HtmlElement = el.unchecked_ref();
                let _ = el.focus();
            }
        }
    });

    view! {
        <div style:font-family="sans-serif" style:text-align="center">
            <h1>"FocusScope"</h1>

            <div style:display="inline-block" style:text-align="left" style:margin-bottom="20px">
                <label style:display="block">
                    <input
                        type="checkbox"
                        prop:checked=move || trap_focus.get()
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            set_trap_focus.set(checked);
                        }
                    />
                    " Trap focus?"
                </label>
                <label style:display="block">
                    <input
                        type="checkbox"
                        prop:checked=move || focus_on_mount.get() != FocusParam::None
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            if checked {
                                set_focus_on_mount.set(FocusParam::Default);
                            } else {
                                set_focus_on_mount.set(FocusParam::None);
                                set_is_empty_form.set(false);
                            }
                        }
                    />
                    " Focus on mount?"
                </label>
                <Show when=move || focus_on_mount.get() != FocusParam::None && !is_empty_form.get()>
                    <label style:display="block" style:margin-left="20px">
                        <input
                            type="checkbox"
                            prop:checked=move || matches!(focus_on_mount.get(), FocusParam::Ref(_))
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                if checked {
                                    set_focus_on_mount.set(FocusParam::Ref(age_field_ref));
                                } else {
                                    set_focus_on_mount.set(FocusParam::Default);
                                }
                            }
                        />
                        " on \"age\" field?"
                    </label>
                </Show>
                <Show when=move || focus_on_mount.get() != FocusParam::None>
                    <label style:display="block" style:margin-left="20px">
                        <input
                            type="checkbox"
                            prop:checked=move || is_empty_form.get()
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                set_is_empty_form.set(checked);
                                set_focus_on_mount.set(FocusParam::Default);
                            }
                        />
                        " empty form?"
                    </label>
                </Show>
                <label style:display="block">
                    <input
                        type="checkbox"
                        prop:checked=move || focus_on_unmount.get() != FocusParam::None
                        on:change=move |ev| {
                            let checked = event_target_checked(&ev);
                            if checked {
                                set_focus_on_unmount.set(FocusParam::Default);
                            } else {
                                set_focus_on_unmount.set(FocusParam::None);
                            }
                        }
                    />
                    " Focus on unmount?"
                </label>
                <Show when=move || focus_on_unmount.get() != FocusParam::None>
                    <label style:display="block" style:margin-left="20px">
                        <input
                            type="checkbox"
                            prop:checked=move || matches!(focus_on_unmount.get(), FocusParam::Ref(_))
                            on:change=move |ev| {
                                let checked = event_target_checked(&ev);
                                if checked {
                                    set_focus_on_unmount.set(FocusParam::Ref(next_button_ref));
                                } else {
                                    set_focus_on_unmount.set(FocusParam::Default);
                                }
                            }
                        />
                        " on \"next\" button?"
                    </label>
                </Show>
            </div>

            <div style:margin-bottom="20px">
                <button type="button" on:click=move |_| set_open.update(|open| *open = !*open)>
                    {move || if open.get() { "Close" } else { "Open" }}
                    " form in between buttons"
                </button>
            </div>

            <button type="button" style:margin-right="10px">
                "previous"
            </button>

            <Show when=move || open.get()>
                <FocusScope
                    as_child=true
                    r#loop=trap_focus
                    trapped=trap_focus
                    on_mount_auto_focus=on_mount_auto_focus
                    on_unmount_auto_focus=on_unmount_auto_focus
                >
                    <form
                        style:display="inline-flex"
                        style:flex-direction="column"
                        style:gap="20px"
                        style:padding="20px"
                        style:margin="50px"
                        style:max-width="500px"
                        style:border="2px solid"
                    >
                        <Show when=move || !is_empty_form.get()>
                            <input type="text" placeholder="First name" />
                            <input type="text" placeholder="Last name" />
                            <input type="number" placeholder="Age" node_ref=age_field_ref />
                            <button type="button" on:click=move |_| set_open.set(false)>
                                "Close"
                            </button>
                        </Show>
                    </form>
                </FocusScope>
            </Show>

            <button type="button" style:margin-left="10px" node_ref=next_button_ref>
                "next"
            </button>
        </div>
    }
}
