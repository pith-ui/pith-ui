//! Escape key listener hook.
//!
//! Registers a document-level `keydown` listener that fires a callback
//! when the Escape key is pressed. Used by dismissable layers, dialogs,
//! and other overlay components.

use std::cell::RefCell;
use std::rc::Rc;

use leptos::{ev::KeyboardEvent, prelude::*};
use send_wrapper::SendWrapper;
use web_sys::{
    AddEventListenerOptions, Document, EventListenerOptions,
    wasm_bindgen::{JsCast, closure::Closure},
};

/// Listens for when the escape key is down.
pub fn use_escape_keydown(
    on_escape_key_down: Option<Callback<KeyboardEvent>>,
    owner_document: Option<Document>,
) {
    // Store the document in an Rc<RefCell> instead of StoredValue so it
    // survives scope disposal. The on_cleanup callback MUST be able to
    // access the document to remove the listener; StoredValue may be
    // dropped before on_cleanup runs.
    let owner_document = SendWrapper::new(Rc::new(owner_document.unwrap_or(document())));

    type HandleKeyDown = dyn Fn(KeyboardEvent);
    // Use Rc<RefCell<Option<Closure>>> so both the Effect and on_cleanup
    // hold references. The Closure stays alive until on_cleanup removes
    // the listener and drops its Rc clone.
    let handle_key_down: SendWrapper<Rc<RefCell<Option<Closure<HandleKeyDown>>>>> =
        SendWrapper::new(Rc::new(RefCell::new(Some(Closure::new(
            move |event: KeyboardEvent| {
                if event.key() == "Escape"
                    && let Some(on_escape_key_down) = on_escape_key_down
                {
                    on_escape_key_down.run(event);
                }
            },
        )))));

    Effect::new({
        let handle_key_down = handle_key_down.clone();
        let owner_document = owner_document.clone();

        move |_| {
            if let Some(closure) = handle_key_down.borrow().as_ref() {
                let options = AddEventListenerOptions::new();
                options.set_capture(true);

                owner_document
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                        &options,
                    )
                    .expect("Key down event listener should be added.");
            }
        }
    });

    on_cleanup(move || {
        if let Some(closure) = handle_key_down.borrow().as_ref() {
            let options = EventListenerOptions::new();
            options.set_capture(true);

            owner_document
                .remove_event_listener_with_callback_and_event_listener_options(
                    "keydown",
                    closure.as_ref().unchecked_ref(),
                    &options,
                )
                .expect("Key down event listener should be removed.");
        }
    });
}
