use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use radix_rect::{Rect, observe_element_rect};
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

type CleanupFn = Arc<Mutex<Option<SendWrapper<Box<dyn FnOnce()>>>>>;

/// Use this custom hook to get access to an element's rect (getBoundingClientRect)
/// and observe it along time.
pub fn use_rect(element_ref: AnyNodeRef) -> ReadSignal<Option<Rect>> {
    let (rect, set_rect) = signal::<Option<Rect>>(None);

    let cleanup: CleanupFn = Arc::new(Mutex::new(None));
    let effect_cleanup = cleanup.clone();

    Effect::new(move |_| {
        // Clean up previous observation.
        if let Some(prev) = effect_cleanup
            .lock()
            .expect("Lock should be acquired.")
            .take()
        {
            set_rect.set(None);
            prev.take()();
        }

        if let Some(element) = element_ref
            .get()
            .and_then(|element| element.dyn_into::<web_sys::Element>().ok())
        {
            let unobserve = observe_element_rect(&element, move |r| {
                set_rect.set(Some(r));
            });

            *effect_cleanup.lock().expect("Lock should be acquired.") =
                Some(SendWrapper::new(Box::new(unobserve) as Box<dyn FnOnce()>));
        } else {
            set_rect.set(None);
        }
    });

    on_cleanup(move || {
        set_rect.set(None);
        if let Some(prev) = cleanup.lock().expect("Lock should be acquired.").take() {
            prev.take()();
        }
    });

    rect
}
