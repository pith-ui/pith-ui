use std::cell::RefCell;
use std::sync::Arc;

use leptos::{html, prelude::*};
use leptos_dom::helpers::document;
use leptos_node_ref::AnyNodeRef;
use radix_leptos_primitive::Primitive;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RegionType {
    #[default]
    Polite,
    Assertive,
    Off,
}

impl RegionType {
    fn as_str(self) -> &'static str {
        match self {
            RegionType::Polite => "polite",
            RegionType::Assertive => "assertive",
            RegionType::Off => "off",
        }
    }

    fn default_role(self) -> RegionRole {
        match self {
            RegionType::Polite => RegionRole::Status,
            RegionType::Assertive => RegionRole::Alert,
            RegionType::Off => RegionRole::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegionRole {
    Status,
    Alert,
    Log,
    None,
}

impl RegionRole {
    fn as_str(self) -> &'static str {
        match self {
            RegionRole::Status => "status",
            RegionRole::Alert => "alert",
            RegionRole::Log => "log",
            RegionRole::None => "none",
        }
    }
}

struct LiveRegionOptions {
    r#type: &'static str,
    role: &'static str,
    relevant: Option<String>,
    atomic: bool,
    id: Option<String>,
}

// Tracks how many Announce instances share each live region element,
// so we only attach one `visibilitychange` listener per region.
thread_local! {
    static LISTENER_MAP: RefCell<Vec<(web_sys::Element, u32)>> = const { RefCell::new(Vec::new()) };
}

fn listener_count(element: &web_sys::Element) -> u32 {
    LISTENER_MAP.with(|map| {
        map.borrow()
            .iter()
            .find(|(el, _)| el == element)
            .map(|(_, count)| *count)
            .unwrap_or(0)
    })
}

fn increment_listener(element: &web_sys::Element) {
    LISTENER_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(entry) = map.iter_mut().find(|(el, _)| el == element) {
            entry.1 += 1;
        } else {
            map.push((element.clone(), 1));
        }
    });
}

fn decrement_listener(element: &web_sys::Element) -> u32 {
    LISTENER_MAP.with(|map| {
        let mut map = map.borrow_mut();
        if let Some(entry) = map.iter_mut().find(|(el, _)| el == element) {
            entry.1 -= 1;
            let count = entry.1;
            if count == 0 {
                map.retain(|(el, _)| el != element);
            }
            count
        } else {
            0
        }
    })
}

#[component]
pub fn Announce(
    #[prop(into, optional)] r#type: Option<RegionType>,
    #[prop(into, optional)] role: Option<RegionRole>,
    #[prop(into, optional)] aria_atomic: Option<bool>,
    #[prop(into, optional)] aria_relevant: Option<String>,
    #[prop(into, optional)] region_identifier: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let region_type = r#type.unwrap_or_default();
    let region_role = role.unwrap_or_else(|| region_type.default_role());
    let atomic = aria_atomic.unwrap_or(false);

    let opts = LiveRegionOptions {
        r#type: region_type.as_str(),
        role: region_role.as_str(),
        relevant: aria_relevant,
        atomic,
        id: region_identifier,
    };

    let children = StoredValue::new(children);

    if cfg!(target_arch = "wasm32")
        && Owner::current_shared_context()
            .map(|sc| sc.is_browser())
            .unwrap_or(true)
    {
        let owner_document = document();

        // Find or create the hidden live region element.
        let region_element = {
            let selector = build_selector(&opts);
            owner_document
                .query_selector(&selector)
                .ok()
                .flatten()
                .unwrap_or_else(|| build_live_region_element(&owner_document, &opts))
        };

        // Set up visibilitychange listener (ref-counted per region).
        // In some screen reader/browser combinations, alerts from inactive tabs may be announced.
        // We suppress this by toggling role/aria-live when visibility changes.
        let count = listener_count(&region_element);
        if count == 0 {
            let region_el = region_element.clone();
            let doc = owner_document.clone();
            let type_str = region_type.as_str().to_owned();
            let role_str = region_role.as_str().to_owned();

            let closure = Closure::<dyn Fn()>::new(move || {
                let hidden = doc.hidden();
                region_el
                    .set_attribute("role", if hidden { "none" } else { &role_str })
                    .ok();
                region_el
                    .set_attribute("aria-live", if hidden { "off" } else { &type_str })
                    .ok();
            });

            owner_document
                .add_event_listener_with_callback(
                    "visibilitychange",
                    closure.as_ref().unchecked_ref(),
                )
                .ok();

            // Leak the closure so it lives as long as the listener.
            closure.forget();
        }
        increment_listener(&region_element);

        // Portal children into the live region via mount_to.
        let mount_element: web_sys::HtmlElement = region_element.clone().unchecked_into();
        let portal_children: Arc<dyn Fn() -> AnyView> =
            Arc::new(move || children.with_value(|children| children()));

        let handle = SendWrapper::new(mount_to(mount_element, {
            let portal_children = Arc::clone(&portal_children);
            move || untrack(|| view! { <div>{portal_children()}</div> }.into_any())
        }));

        // Wrap non-Send types in SendWrapper for the cleanup closure.
        let cleanup_region = SendWrapper::new(region_element);
        let type_str_cleanup = region_type.as_str();
        let role_str_cleanup = region_role.as_str();

        Owner::on_cleanup(move || {
            // Drop the portal mount handle.
            drop(handle);

            let remaining = decrement_listener(&cleanup_region);
            if remaining == 0 {
                cleanup_region.set_attribute("role", role_str_cleanup).ok();
                cleanup_region
                    .set_attribute("aria-live", type_str_cleanup)
                    .ok();
            }
        });
    }

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=node_ref
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

pub use Announce as Root;

fn build_live_region_element(
    owner_document: &web_sys::Document,
    opts: &LiveRegionOptions,
) -> web_sys::Element {
    let element = owner_document
        .create_element("div")
        .expect("Failed to create div element");

    element
        .set_attribute(&get_live_region_part_data_attr(opts.id.as_deref()), "")
        .ok();
    element
        .set_attribute(
            "style",
            "position: absolute; top: -1px; width: 1px; height: 1px; overflow: hidden;",
        )
        .ok();

    if let Some(body) = owner_document.body() {
        body.append_child(&element).ok();
    }

    element.set_attribute("aria-live", opts.r#type).ok();
    element
        .set_attribute("aria-atomic", &opts.atomic.to_string())
        .ok();
    element.set_attribute("role", opts.role).ok();

    if let Some(ref relevant) = opts.relevant {
        element.set_attribute("aria-relevant", relevant).ok();
    }

    element
}

fn build_selector(opts: &LiveRegionOptions) -> String {
    let mut selector = format!("[{}]", get_live_region_part_data_attr(opts.id.as_deref()));

    let pairs: &[(&str, Option<String>)] = &[
        ("aria-live", Some(opts.r#type.to_owned())),
        ("aria-atomic", Some(opts.atomic.to_string())),
        ("aria-relevant", opts.relevant.clone()),
        ("role", Some(opts.role.to_owned())),
    ];

    for (attr, val) in pairs {
        if let Some(val) = val {
            selector.push_str(&format!("[{attr}={val}]"));
        }
    }

    selector
}

fn get_live_region_part_data_attr(id: Option<&str>) -> String {
    match id {
        Some(id) => format!("data-radix-announce-region-{id}"),
        None => "data-radix-announce-region".to_owned(),
    }
}
