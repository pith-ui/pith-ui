use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_dismissable_layer::DismissableLayer;
use radix_leptos_id::use_id;
use radix_leptos_presence::Presence;
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use radix_leptos_use_previous::use_previous;
use radix_leptos_visually_hidden::VisuallyHidden;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;

/* -------------------------------------------------------------------------------------------------
 * Types and constants
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::Horizontal => "horizontal",
                Orientation::Vertical => "vertical",
            }
        )
    }
}

const LINK_SELECT: &str = "navigationMenu.linkSelect";
const ROOT_CONTENT_DISMISS: &str = "navigationMenu.rootContentDismiss";

const ARROW_KEYS: &[&str] = &["ArrowRight", "ArrowLeft", "ArrowUp", "ArrowDown"];

/* -------------------------------------------------------------------------------------------------
 * Collection item data
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Debug)]
pub struct NavigationMenuItemData {
    pub value: String,
}

/// Empty item data for the FocusGroup collection.
#[derive(Clone, Debug)]
struct FocusGroupItemData;

/* -------------------------------------------------------------------------------------------------
 * Context types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct NavigationMenuContextValue {
    is_root_menu: bool,
    value: Signal<String>,
    previous_value: Memo<String>,
    base_id: ReadSignal<String>,
    dir: Signal<Direction>,
    orientation: Signal<Orientation>,
    root_navigation_menu: AnyNodeRef,
    indicator_track: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    viewport: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    /// Set synchronously during NavigationMenuViewport construction (not via Effect),
    /// so Content components know a viewport exists before any menu open interaction.
    has_viewport_component: RwSignal<bool>,
    on_trigger_enter: Callback<String>,
    on_trigger_leave: Callback<()>,
    on_content_enter: Callback<()>,
    on_content_leave: Callback<()>,
    on_item_select: Callback<String>,
    on_item_dismiss: Callback<()>,
    on_viewport_content_change: Callback<(String, ContentData)>,
    on_viewport_content_remove: Callback<String>,
}

#[derive(Clone)]
struct NavigationMenuItemContextValue {
    value: String,
    trigger_ref: AnyNodeRef,
    content_ref: AnyNodeRef,
    focus_proxy_ref: AnyNodeRef,
    was_escape_close_ref: RwSignal<bool>,
    on_entry_key_down: Callback<()>,
    on_focus_proxy_enter: Callback<&'static str>,
    on_root_content_close: Callback<()>,
    on_content_focus_outside: Callback<()>,
}

#[derive(Clone)]
struct ViewportContentContextValue {
    items: RwSignal<HashMap<String, ContentData>>,
}

#[derive(Clone)]
pub struct ContentData {
    pub value: String,
    pub trigger_ref: AnyNodeRef,
    pub focus_proxy_ref: AnyNodeRef,
    pub was_escape_close_ref: RwSignal<bool>,
    pub on_content_focus_outside: Callback<()>,
    pub on_root_content_close: Callback<()>,
    pub force_mount: bool,
    pub children: Option<ChildrenFn>,
    pub on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    pub on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    pub on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    pub on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    pub on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    pub on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    pub content_ref: AnyNodeRef,
}

/* -------------------------------------------------------------------------------------------------
 * Helper functions
 * -----------------------------------------------------------------------------------------------*/

fn get_open_state(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

fn make_trigger_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-trigger-{value}")
}

fn make_content_id(base_id: &str, value: &str) -> String {
    format!("{base_id}-content-{value}")
}

fn document() -> web_sys::Document {
    web_sys::window()
        .expect("Window should exist.")
        .document()
        .expect("Document should exist.")
}

fn set_timeout(f: impl FnOnce() + 'static, delay: i32) -> i32 {
    let closure = Closure::once_into_js(f);
    web_sys::window()
        .expect("Window should exist.")
        .set_timeout_with_callback_and_timeout_and_arguments_0(closure.unchecked_ref(), delay)
        .expect("setTimeout should succeed.")
}

fn clear_timeout(handle: StoredValue<Option<i32>>) {
    if let Some(id) = handle.get_value() {
        web_sys::window()
            .expect("Window should exist.")
            .clear_timeout_with_handle(id);
        handle.set_value(None);
    }
}

/// Returns a list of potential tabbable candidates.
fn get_tabbable_candidates(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes = Vec::new();

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_el) = node.dyn_ref::<web_sys::HtmlElement>() {
                // Check for hidden input
                if let Some(input) = node.dyn_ref::<web_sys::HtmlInputElement>()
                    && input.type_() == "hidden"
                {
                    return 3; // FILTER_SKIP
                }
                if html_el.hidden() {
                    return 3; // FILTER_SKIP
                }
                if html_el.tab_index() >= 0 { 1 } else { 3 } // FILTER_ACCEPT / FILTER_SKIP
            } else {
                3 // FILTER_SKIP
            }
        });

    let node_filter = web_sys::NodeFilter::new();
    node_filter.set_accept_node(accept_node_closure.as_ref().unchecked_ref());

    let walker = document()
        .create_tree_walker_with_what_to_show_and_filter(container, 0x1, Some(&node_filter))
        .expect("Tree walker should be created.");

    while let Some(node) = walker
        .next_node()
        .expect("Tree walker should return a next node.")
    {
        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
            nodes.push(element);
        }
    }

    nodes
}

fn focus_first(candidates: &[web_sys::HtmlElement]) -> bool {
    let previously_focused = document().active_element();
    candidates.iter().any(|candidate| {
        if previously_focused
            .as_ref()
            .is_some_and(|el| el == candidate.unchecked_ref::<web_sys::Element>())
        {
            return true;
        }
        candidate.focus().ok();
        document().active_element().as_ref() != previously_focused.as_ref()
    })
}

fn remove_from_tab_order(
    candidates: &[web_sys::HtmlElement],
) -> Vec<(web_sys::HtmlElement, String)> {
    candidates
        .iter()
        .map(|candidate| {
            let prev = candidate.get_attribute("tabindex").unwrap_or_default();
            candidate
                .set_attribute("tabindex", "-1")
                .expect("Attribute should be set.");
            (candidate.clone(), prev)
        })
        .collect()
}

fn restore_tab_order(saved: &[(web_sys::HtmlElement, String)]) {
    for (el, prev) in saved {
        el.set_attribute("tabindex", prev)
            .expect("Attribute should be set.");
    }
}

fn use_resize_observer(
    element: Signal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_resize: Callback<()>,
) {
    #[allow(clippy::type_complexity)]
    let observer: StoredValue<Option<SendWrapper<web_sys::ResizeObserver>>> =
        StoredValue::new(None);
    let raf_id: StoredValue<Option<i32>> = StoredValue::new(None);

    Effect::new(move |_| {
        // Clean up previous observer
        observer.with_value(|obs| {
            if let Some(obs) = obs {
                obs.disconnect();
            }
        });
        if let Some(raf) = raf_id.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .cancel_animation_frame(raf)
                .ok();
        }

        if let Some(el) = element.get() {
            let closure = Closure::<dyn Fn(web_sys::js_sys::Array)>::new(
                move |_entries: web_sys::js_sys::Array| {
                    if let Some(raf) = raf_id.get_value() {
                        web_sys::window()
                            .expect("Window should exist.")
                            .cancel_animation_frame(raf)
                            .ok();
                    }
                    let id = web_sys::window()
                        .expect("Window should exist.")
                        .request_animation_frame(
                            Closure::once_into_js(move || {
                                on_resize.run(());
                            })
                            .unchecked_ref(),
                        )
                        .expect("rAF should succeed.");
                    raf_id.set_value(Some(id));
                },
            );
            let ro = web_sys::ResizeObserver::new(closure.as_ref().unchecked_ref())
                .expect("ResizeObserver should be created.");
            ro.observe(&el);
            // Leak the closure so it lives as long as the observer
            closure.forget();
            observer.set_value(Some(SendWrapper::new(ro)));
        }
    });

    Owner::on_cleanup(move || {
        observer.with_value(|obs| {
            if let Some(obs) = obs {
                obs.disconnect();
            }
        });
        if let Some(raf) = raf_id.get_value() {
            web_sys::window()
                .expect("Window should exist.")
                .cancel_animation_frame(raf)
                .ok();
        }
    });
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenu
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenu(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))]
    orientation: MaybeProp<Orientation>,
    #[prop(into, optional, default = MaybeProp::from(200.0))] delay_duration: MaybeProp<f64>,
    #[prop(into, optional, default = MaybeProp::from(300.0))] skip_delay_duration: MaybeProp<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let direction = use_direction(dir);
    let orientation_signal =
        Signal::derive(move || orientation.get().unwrap_or(Orientation::Horizontal));

    let open_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let close_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let skip_delay_timer_ref: StoredValue<Option<i32>> = StoredValue::new(None);
    let is_open_delayed = RwSignal::new(true);

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: MaybeProp::derive(move || Some(default_value.get().unwrap_or_default())),
        on_change: Some(Callback::new(move |val: Option<String>| {
            let val = val.unwrap_or_default();
            let is_open = !val.is_empty();
            let has_skip_delay = skip_delay_duration.get().unwrap_or(300.0) > 0.0;
            let skip_dur = skip_delay_duration.get().unwrap_or(300.0) as i32;

            if is_open {
                clear_timeout(skip_delay_timer_ref);
                if has_skip_delay {
                    is_open_delayed.set(false);
                }
            } else {
                clear_timeout(skip_delay_timer_ref);
                let timeout_id = set_timeout(
                    move || {
                        is_open_delayed.set(true);
                    },
                    skip_dur,
                );
                skip_delay_timer_ref.set_value(Some(timeout_id));
            }

            if let Some(cb) = on_value_change {
                cb.run(val);
            }
        })),
    });

    let current_value = Signal::derive(move || value_signal.get().unwrap_or_default());

    let nav_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, nav_ref]);

    let handle_open = Callback::new(move |item_value: String| {
        clear_timeout(close_timer_ref);
        set_value.run(Some(item_value));
    });

    let start_close_timer = Callback::new(move |_: ()| {
        clear_timeout(close_timer_ref);
        let timeout_id = set_timeout(
            move || {
                set_value.run(Some(String::new()));
            },
            150,
        );
        close_timer_ref.set_value(Some(timeout_id));
    });

    let handle_delayed_open = Callback::new(move |item_value: String| {
        let is_open_item = current_value.get_untracked() == item_value;
        if is_open_item {
            clear_timeout(close_timer_ref);
        } else {
            let delay = delay_duration.get().unwrap_or(200.0) as i32;
            let timeout_id = set_timeout(
                move || {
                    clear_timeout(close_timer_ref);
                    set_value.run(Some(item_value));
                },
                delay,
            );
            open_timer_ref.set_value(Some(timeout_id));
        }
    });

    on_cleanup(move || {
        clear_timeout(open_timer_ref);
        clear_timeout(close_timer_ref);
        clear_timeout(skip_delay_timer_ref);
    });

    let on_trigger_enter = Callback::new(move |item_value: String| {
        clear_timeout(open_timer_ref);
        if is_open_delayed.get_untracked() {
            handle_delayed_open.run(item_value);
        } else {
            handle_open.run(item_value);
        }
    });

    let on_trigger_leave = Callback::new(move |_: ()| {
        clear_timeout(open_timer_ref);
        start_close_timer.run(());
    });

    let on_content_enter = Callback::new(move |_: ()| {
        clear_timeout(close_timer_ref);
    });

    let on_content_leave = start_close_timer;

    let on_item_select = Callback::new(move |item_value: String| {
        let prev = current_value.get_untracked();
        if prev == item_value {
            set_value.run(Some(String::new()));
        } else {
            set_value.run(Some(item_value));
        }
    });

    let on_item_dismiss = Callback::new(move |_: ()| {
        set_value.run(Some(String::new()));
    });

    view! {
        <NavigationMenuProvider
            is_root_menu=true
            value=current_value
            dir=direction
            orientation=orientation_signal
            root_navigation_menu=nav_ref
            on_trigger_enter=on_trigger_enter
            on_trigger_leave=on_trigger_leave
            on_content_enter=on_content_enter
            on_content_leave=on_content_leave
            on_item_select=on_item_select
            on_item_dismiss=on_item_dismiss
        >
            <Primitive
                element=html::nav
                as_child=as_child
                node_ref=composed_refs
                attr:aria-label="Main"
                attr:data-orientation=move || orientation_signal.get().to_string()
                attr:dir=move || direction.get().to_string()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </NavigationMenuProvider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuSub
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuSub(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional, default = MaybeProp::from(Orientation::Horizontal))]
    orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let orientation_signal =
        Signal::derive(move || orientation.get().unwrap_or(Orientation::Horizontal));

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: MaybeProp::derive(move || Some(default_value.get().unwrap_or_default())),
        on_change: on_value_change.map(|cb| {
            Callback::new(move |val: Option<String>| {
                if let Some(val) = val {
                    cb.run(val);
                }
            })
        }),
    });

    let current_value = Signal::derive(move || value_signal.get().unwrap_or_default());

    let on_trigger_enter = Callback::new(move |item_value: String| {
        set_value.run(Some(item_value));
    });

    let on_item_select = Callback::new(move |item_value: String| {
        set_value.run(Some(item_value));
    });

    let on_item_dismiss = Callback::new(move |_: ()| {
        set_value.run(Some(String::new()));
    });

    view! {
        <NavigationMenuProvider
            is_root_menu=false
            value=current_value
            dir=context.dir
            orientation=orientation_signal
            root_navigation_menu=context.root_navigation_menu
            on_trigger_enter=on_trigger_enter
            on_trigger_leave=Callback::new(|_: ()| {})
            on_content_enter=Callback::new(|_: ()| {})
            on_content_leave=Callback::new(|_: ()| {})
            on_item_select=on_item_select
            on_item_dismiss=on_item_dismiss
        >
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-orientation=move || orientation_signal.get().to_string()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </NavigationMenuProvider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuProvider (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn NavigationMenuProvider(
    is_root_menu: bool,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] dir: Signal<Direction>,
    #[prop(into)] orientation: Signal<Orientation>,
    root_navigation_menu: AnyNodeRef,
    on_trigger_enter: Callback<String>,
    on_trigger_leave: Callback<()>,
    on_content_enter: Callback<()>,
    on_content_leave: Callback<()>,
    on_item_select: Callback<String>,
    on_item_dismiss: Callback<()>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let viewport: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let viewport_content: RwSignal<HashMap<String, ContentData>> = RwSignal::new(HashMap::new());
    let indicator_track: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);

    let previous_value = use_previous(value);
    let base_id = use_id(None);

    let on_viewport_content_change = Callback::new(
        move |(content_value, content_data): (String, ContentData)| {
            viewport_content.update(|map| {
                map.insert(content_value, content_data);
            });
        },
    );

    let on_viewport_content_remove = Callback::new(move |content_value: String| {
        viewport_content.update(|map| {
            map.remove(&content_value);
        });
    });

    let context = NavigationMenuContextValue {
        is_root_menu,
        value,
        previous_value,
        base_id,
        dir,
        orientation,
        root_navigation_menu,
        indicator_track,
        viewport,
        has_viewport_component: RwSignal::new(false),
        on_trigger_enter,
        on_trigger_leave,
        on_content_enter,
        on_content_leave,
        on_item_select,
        on_item_dismiss,
        on_viewport_content_change,
        on_viewport_content_remove,
    };

    provide_context(context);
    provide_context(ViewportContentContextValue {
        items: viewport_content,
    });

    view! {
        <CollectionProvider<NavigationMenuItemData> item_data_type=PhantomData>
            {children.with_value(|children| children())}
        </CollectionProvider<NavigationMenuItemData>>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuList
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();

    // Use the <ul> itself as the indicator track (position: relative container).
    // React wraps in a separate <Primitive.div>, but in Leptos that outer div would
    // intercept user spread attributes (e.g. attr:class). By merging the track role
    // into the <ul>, user attributes correctly land on the list element.
    let indicator_track_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, indicator_track_ref]);

    // Store indicator track element when mounted
    Effect::new(move |_| {
        if let Some(el) = indicator_track_ref.get() {
            let html_el: web_sys::HtmlElement = el.unchecked_into();
            context.indicator_track.set(Some(SendWrapper::new(html_el)));
        }
    });

    let list = StoredValue::new(move || {
        view! {
            <Primitive
                element=html::ul
                as_child=as_child
                node_ref=composed_refs
                style:position="relative"
                attr:data-orientation=move || context.orientation.get().to_string()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        }
    });

    view! {
        <CollectionSlot<NavigationMenuItemData> item_data_type=PhantomData>
            {move || {
                if context.is_root_menu {
                    view! {
                        <FocusGroup>
                            {list.with_value(|l| l())}
                        </FocusGroup>
                    }.into_any()
                } else {
                    list.with_value(|l| l()).into_any()
                }
            }}
        </CollectionSlot<NavigationMenuItemData>>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuItem(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let auto_value = use_id(None);
    let item_value = Memo::new(move |_| value.get().unwrap_or_else(|| auto_value.get()));

    let content_ref = AnyNodeRef::new();
    let trigger_ref = AnyNodeRef::new();
    let focus_proxy_ref = AnyNodeRef::new();
    let was_escape_close_ref = RwSignal::new(false);

    type TabOrderBackup = Option<SendWrapper<Vec<(web_sys::HtmlElement, String)>>>;
    let restore_content_tab_order: StoredValue<TabOrderBackup> = StoredValue::new(None);

    let handle_content_entry = Callback::new(move |side: &'static str| {
        if let Some(el) = content_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            // Restore tab order first
            restore_content_tab_order.with_value(|saved| {
                if let Some(saved) = saved {
                    restore_tab_order(saved);
                }
            });
            restore_content_tab_order.set_value(None);

            let candidates = get_tabbable_candidates(&el);
            if !candidates.is_empty() {
                if side == "start" {
                    focus_first(&candidates);
                } else {
                    let mut reversed = candidates;
                    reversed.reverse();
                    focus_first(&reversed);
                }
            }
        }
    });

    let handle_content_exit = Callback::new(move |_: ()| {
        if let Some(el) = content_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            let candidates = get_tabbable_candidates(&el);
            if !candidates.is_empty() {
                let saved = remove_from_tab_order(&candidates);
                restore_content_tab_order.set_value(Some(SendWrapper::new(saved)));
            }
        }
    });

    let item_value_for_ctx = item_value;
    let item_context = NavigationMenuItemContextValue {
        value: item_value_for_ctx.get_untracked(),
        trigger_ref,
        content_ref,
        focus_proxy_ref,
        was_escape_close_ref,
        on_entry_key_down: Callback::new(move |_: ()| {
            handle_content_entry.run("start");
        }),
        on_focus_proxy_enter: handle_content_entry,
        on_root_content_close: handle_content_exit,
        on_content_focus_outside: handle_content_exit,
    };

    // Update context value reactively
    let item_context_signal = RwSignal::new(item_context);
    Effect::new(move |_| {
        let val = item_value.get();
        item_context_signal.update(|ctx| {
            ctx.value = val;
        });
    });

    view! {
        <Provider value=item_context_signal>
            <Primitive
                element=html::li
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuTrigger(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_move: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] on_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let item_context_signal = expect_context::<RwSignal<NavigationMenuItemContextValue>>();

    let trigger_node_ref = AnyNodeRef::new();
    let item_trigger_ref = item_context_signal.get_untracked().trigger_ref;
    let composed_refs = use_composed_refs(vec![node_ref, trigger_node_ref, item_trigger_ref]);

    let has_pointer_move_opened_ref: StoredValue<bool> = StoredValue::new(false);
    let was_click_close_ref: StoredValue<bool> = StoredValue::new(false);

    let open = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        item_ctx.value == context.value.get()
    });

    let trigger_id = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        make_trigger_id(&context.base_id.get(), &item_ctx.value)
    });
    let content_id = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        make_content_id(&context.base_id.get(), &item_ctx.value)
    });

    let item_value = Memo::new(move |_| item_context_signal.get().value.clone());

    view! {
        <CollectionItemSlot<NavigationMenuItemData>
            item_data_type=PhantomData
            item_data=Signal::derive(move || NavigationMenuItemData { value: item_value.get() })
            node_ref=composed_refs
        >
            <FocusGroupItem>
                <Primitive
                    element=html::button
                    as_child=as_child
                    node_ref=composed_refs
                    attr:id=move || trigger_id.get()
                    attr:disabled=move || disabled.get().unwrap_or(false).then_some("")
                    attr:data-disabled=move || disabled.get().unwrap_or(false).then_some("")
                    attr:data-state=move || get_open_state(open.get())
                    attr:aria-expanded=move || open.get().to_string()
                    attr:aria-controls=move || content_id.get()
                    on:pointerenter=compose_callbacks(
                        on_pointer_enter,
                        Some(Callback::new(move |_: ev::PointerEvent| {
                            was_click_close_ref.set_value(false);
                            item_context_signal.get_untracked().was_escape_close_ref.set(false);
                        })),
                        None,
                    )
                    on:pointermove=compose_callbacks(
                        on_pointer_move,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() != "mouse" { return; }
                            if disabled.get().unwrap_or(false)
                                || was_click_close_ref.get_value()
                                || item_context_signal.get_untracked().was_escape_close_ref.get_untracked()
                                || has_pointer_move_opened_ref.get_value()
                            {
                                return;
                            }
                            let item_ctx = item_context_signal.get_untracked();
                            context.on_trigger_enter.run(item_ctx.value.clone());
                            has_pointer_move_opened_ref.set_value(true);
                        })),
                        None,
                    )
                    on:pointerleave=compose_callbacks(
                        on_pointer_leave,
                        Some(Callback::new(move |event: ev::PointerEvent| {
                            if event.pointer_type() != "mouse" { return; }
                            if disabled.get().unwrap_or(false) { return; }
                            context.on_trigger_leave.run(());
                            has_pointer_move_opened_ref.set_value(false);
                        })),
                        None,
                    )
                    on:click=compose_callbacks(
                        on_click,
                        Some(Callback::new(move |_: ev::MouseEvent| {
                            let item_ctx = item_context_signal.get_untracked();
                            context.on_item_select.run(item_ctx.value.clone());
                            was_click_close_ref.set_value(open.get_untracked());
                        })),
                        None,
                    )
                    on:keydown=compose_callbacks(
                        on_key_down,
                        Some(Callback::new(move |event: ev::KeyboardEvent| {
                            let vertical_entry_key = if context.dir.get_untracked() == Direction::Rtl {
                                "ArrowLeft"
                            } else {
                                "ArrowRight"
                            };
                            let entry_key = match context.orientation.get_untracked() {
                                Orientation::Horizontal => "ArrowDown",
                                Orientation::Vertical => vertical_entry_key,
                            };
                            if open.get_untracked() && event.key() == entry_key {
                                item_context_signal.get_untracked().on_entry_key_down.run(());
                                event.prevent_default();
                            }
                        })),
                        None,
                    )
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </FocusGroupItem>
        </CollectionItemSlot<NavigationMenuItemData>>

        // Focus proxy and aria-owns when open
        {move || {
            let is_open = open.get();
            let item_ctx = item_context_signal.get();
            if is_open {
                let has_viewport = context.viewport.get().is_some();
                let cid = content_id.get();
                Some(view! {
                    <VisuallyHidden
                        attr:aria-hidden="true"
                        attr:tabindex="0"
                        node_ref=item_ctx.focus_proxy_ref
                        on:focus=move |event: ev::FocusEvent| {
                            let item_ctx = item_context_signal.get_untracked();
                            let prev = event.related_target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());
                            let was_trigger_focused = prev.as_ref().map(|p| {
                                trigger_node_ref.get().map(|t| {
                                    let t: web_sys::HtmlElement = t.unchecked_into();
                                    *p == t
                                }).unwrap_or(false)
                            }).unwrap_or(false);

                            let was_focus_from_content = prev.as_ref().map(|p| {
                                item_ctx.content_ref.get().map(|content| {
                                    let content: web_sys::Node = content.unchecked_into();
                                    content.contains(Some(p.unchecked_ref()))
                                }).unwrap_or(false)
                            }).unwrap_or(false);

                            if was_trigger_focused || !was_focus_from_content {
                                let side = if was_trigger_focused { "start" } else { "end" };
                                item_ctx.on_focus_proxy_enter.run(side);
                            }
                        }
                    >
                        {""}
                    </VisuallyHidden>

                    // Restructure a11y tree
                    {if has_viewport {
                        Some(view! { <span aria-owns=cid></span> })
                    } else {
                        None
                    }}
                })
            } else {
                None
            }
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuLink
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuLink(
    #[prop(into, optional)] active: MaybeProp<bool>,
    #[prop(into, optional)] on_select: Option<Callback<web_sys::Event>>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <FocusGroupItem>
            <Primitive
                element=html::a
                as_child=as_child
                node_ref=node_ref
                attr:data-active=move || active.get().unwrap_or(false).then_some("")
                attr:aria-current=move || if active.get().unwrap_or(false) { Some("page") } else { None }
                on:click=compose_callbacks(
                    on_click,
                    Some(Callback::new(move |event: ev::MouseEvent| {
                        let target = event.target()
                            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok());
                        if let Some(target) = target {
                            let mut init = web_sys::CustomEventInit::new();
                            init.set_bubbles(true);
                            init.set_cancelable(true);
                            let link_select_event = web_sys::CustomEvent::new_with_event_init_dict(
                                LINK_SELECT,
                                &init,
                            ).expect("CustomEvent should be created.");

                            if let Some(on_select) = on_select {
                                // Add one-time listener
                                let listener = Closure::once_into_js(move |event: web_sys::Event| {
                                    on_select.run(event);
                                });
                                let mut opts = web_sys::AddEventListenerOptions::new();
                                opts.set_once(true);
                                target
                                    .add_event_listener_with_callback_and_add_event_listener_options(
                                        LINK_SELECT,
                                        listener.unchecked_ref(),
                                        &opts,
                                    )
                                    .ok();
                            }

                            target.dispatch_event(&link_select_event).ok();

                            if !link_select_event.default_prevented() && !event.meta_key() {
                                let mut dismiss_init = web_sys::CustomEventInit::new();
                                dismiss_init.set_bubbles(true);
                                dismiss_init.set_cancelable(true);
                                let dismiss_event = web_sys::CustomEvent::new_with_event_init_dict(
                                    ROOT_CONTENT_DISMISS,
                                    &dismiss_init,
                                ).expect("CustomEvent should be created.");
                                target.dispatch_event(&dismiss_event).ok();
                            }
                        }
                    })),
                    Some(false),
                )
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </FocusGroupItem>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let is_visible = Signal::derive(move || !context.value.get().is_empty());

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || is_visible.get());

    let presence_ref = AnyNodeRef::new();

    // Render into indicator track via Portal pattern (mount_to)
    view! {
        {move || {
            let track = context.indicator_track.get();
            if track.is_some() {
                Some(view! {
                    <Presence present=present node_ref=presence_ref>
                        <NavigationMenuIndicatorImpl
                            as_child=as_child
                            node_ref=node_ref
                            presence_ref=presence_ref
                        >
                            {children.with_value(|children| children.as_ref().map(|children| children()))}
                        </NavigationMenuIndicatorImpl>
                    </Presence>
                })
            } else {
                None
            }
        }}
    }
}

#[component]
fn NavigationMenuIndicatorImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let get_items = use_collection::<NavigationMenuItemData>();

    let active_trigger: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let position: RwSignal<Option<(f64, f64)>> = RwSignal::new(None);
    let is_horizontal =
        Signal::derive(move || context.orientation.get() == Orientation::Horizontal);
    let is_visible = Signal::derive(move || !context.value.get().is_empty());

    // Update active trigger when value changes
    Effect::new(move |_| {
        let value = context.value.get();
        let items = get_items();
        let trigger_node = items.iter().find_map(|item| {
            if item.data.value == value {
                item.r#ref.get().map(|el| {
                    let html_el: web_sys::HtmlElement = el.unchecked_into();
                    SendWrapper::new(html_el)
                })
            } else {
                None
            }
        });
        if trigger_node.is_some() {
            active_trigger.set(trigger_node);
        }
    });

    let handle_position_change = Callback::new(move |_: ()| {
        if let Some(trigger) = active_trigger.get_untracked() {
            if is_horizontal.get_untracked() {
                position.set(Some((
                    trigger.offset_width() as f64,
                    trigger.offset_left() as f64,
                )));
            } else {
                position.set(Some((
                    trigger.offset_height() as f64,
                    trigger.offset_top() as f64,
                )));
            }
        }
    });

    use_resize_observer(
        Signal::derive(move || active_trigger.get()),
        handle_position_change,
    );
    use_resize_observer(
        Signal::derive(move || context.indicator_track.get()),
        handle_position_change,
    );

    let indicator_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, indicator_ref, presence_ref]);

    view! {
        {move || {
            position.get().map(|(size, offset)| {
                let horiz = is_horizontal.get();
                view! {
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_refs
                        attr:aria-hidden="true"
                        attr:data-state=move || if is_visible.get() { "visible" } else { "hidden" }
                        attr:data-orientation=move || context.orientation.get().to_string()
                        style:position="absolute"
                        style:left=move || if horiz { Some("0".to_string()) } else { None }
                        style:top=move || if !horiz { Some("0".to_string()) } else { None }
                        style:width=move || if horiz { Some(format!("{size}px")) } else { None }
                        style:height=move || if !horiz { Some(format!("{size}px")) } else { None }
                        style:transform=move || {
                            if horiz {
                                format!("translateX({offset}px)")
                            } else {
                                format!("translateY({offset}px)")
                            }
                        }
                    >
                        {children.with_value(|children| children.as_ref().map(|children| children()))}
                    </Primitive>
                }
            })
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] on_pointer_enter: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_pointer_leave: Option<Callback<ev::PointerEvent>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let item_context_signal = expect_context::<RwSignal<NavigationMenuItemContextValue>>();
    let item_context = item_context_signal.get_untracked();

    let composed_refs = use_composed_refs(vec![node_ref, item_context.content_ref]);

    let open = Memo::new(move |_| {
        let item_ctx = item_context_signal.get();
        item_ctx.value == context.value.get()
    });

    let item_value = StoredValue::new(item_context.value.clone());
    let trigger_ref = item_context.trigger_ref;
    let focus_proxy_ref = item_context.focus_proxy_ref;
    let was_escape_close_ref = item_context.was_escape_close_ref;
    let on_content_focus_outside = item_context.on_content_focus_outside;
    let on_root_content_close = item_context.on_root_content_close;

    let force_mount_val = force_mount.get_untracked().unwrap_or(false);

    // Viewport registration: when a viewport component exists, register content for viewport
    // rendering. Uses has_viewport_component (set synchronously during NavigationMenuViewport
    // construction) instead of the DOM element ref, avoiding a flash where content renders
    // inline before the viewport Effect fires.
    let has_viewport = Signal::derive(move || context.has_viewport_component.get());

    Effect::new(move |_| {
        if has_viewport.get() {
            let content_data = ContentData {
                value: item_value.get_value(),
                trigger_ref,
                focus_proxy_ref,
                was_escape_close_ref,
                on_content_focus_outside,
                on_root_content_close,
                force_mount: force_mount_val,
                children: Some(std::sync::Arc::new(move || {
                    children.with_value(|c| c.as_ref().map(|c| c())).into_any()
                }) as ChildrenFn),
                on_pointer_enter,
                on_pointer_leave,
                on_escape_key_down,
                on_focus_outside,
                on_pointer_down_outside,
                on_interact_outside,
                content_ref: node_ref,
            };
            context
                .on_viewport_content_change
                .run((item_value.get_value(), content_data));
        }
    });

    on_cleanup(move || {
        context
            .on_viewport_content_remove
            .run(item_value.get_value());
    });

    // Inline rendering: render via Presence when there is no viewport.
    // By placing <Presence> directly as the root (rather than inside a reactive closure),
    // caller spread attributes (e.g. attr:class) propagate through the transparent component
    // chain: Presence → Show → ContentImpl → FocusGroup → DismissableLayer → Primitive div.
    let present = Signal::derive(move || {
        !has_viewport.get() && (force_mount.get().unwrap_or(false) || open.get())
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <Presence present=present node_ref=presence_ref>
            <NavigationMenuContentImpl
                value=item_value.get_value()
                trigger_ref=trigger_ref
                focus_proxy_ref=focus_proxy_ref
                was_escape_close_ref=was_escape_close_ref
                on_content_focus_outside=on_content_focus_outside
                on_root_content_close=on_root_content_close
                as_child=as_child
                node_ref=composed_refs
                presence_ref=presence_ref
                on_pointer_enter=Callback::new(compose_callbacks(
                    on_pointer_enter,
                    Some(Callback::new(move |_: ev::PointerEvent| {
                        context.on_content_enter.run(());
                    })),
                    None,
                ))
                on_pointer_leave=Callback::new(compose_callbacks(
                    on_pointer_leave,
                    Some(Callback::new(move |event: ev::PointerEvent| {
                        if event.pointer_type() == "mouse" {
                            context.on_content_leave.run(());
                        }
                    })),
                    None,
                ))
                on_escape_key_down=on_escape_key_down.unwrap_or(Callback::new(|_| {}))
                on_focus_outside=on_focus_outside.unwrap_or(Callback::new(|_| {}))
                on_pointer_down_outside=on_pointer_down_outside.unwrap_or(Callback::new(|_| {}))
                on_interact_outside=on_interact_outside.unwrap_or(Callback::new(|_| {}))
                attr:data-state=move || get_open_state(open.get())
                style:pointer-events=move || {
                    if !open.get() && context.is_root_menu { Some("none") } else { None }
                }
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </NavigationMenuContentImpl>
        </Presence>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuContentImpl (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn NavigationMenuContentImpl(
    #[prop(into)] value: String,
    trigger_ref: AnyNodeRef,
    focus_proxy_ref: AnyNodeRef,
    was_escape_close_ref: RwSignal<bool>,
    on_content_focus_outside: Callback<()>,
    on_root_content_close: Callback<()>,
    #[prop(into)] on_pointer_enter: Callback<ev::PointerEvent>,
    #[prop(into)] on_pointer_leave: Callback<ev::PointerEvent>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<web_sys::KeyboardEvent>>,
    #[prop(into, optional)] on_focus_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] on_interact_outside: Option<Callback<web_sys::CustomEvent>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref, presence_ref]);

    let value_clone = value.clone();
    let trigger_id = Memo::new(move |_| make_trigger_id(&context.base_id.get(), &value_clone));
    let value_clone2 = value.clone();
    let content_id = Memo::new(move |_| make_content_id(&context.base_id.get(), &value_clone2));

    let get_items = use_collection::<NavigationMenuItemData>();

    let prev_motion_attribute: StoredValue<Option<&'static str>> = StoredValue::new(None);

    // Bubble dismiss to root content node
    Effect::new(move |_| {
        if context.is_root_menu
            && let Some(content) = content_ref.get()
        {
            let content: web_sys::HtmlElement = content.unchecked_into();
            let content_el = content.clone();
            let trigger_ref_clone = trigger_ref;
            let on_item_dismiss = context.on_item_dismiss;

            let handler = Closure::<dyn Fn(web_sys::Event)>::new(move |_: web_sys::Event| {
                on_item_dismiss.run(());
                on_root_content_close.run(());
                if let Some(active) = document().active_element()
                    && content_el.contains(Some(active.unchecked_ref()))
                    && let Some(trigger) = trigger_ref_clone.get()
                {
                    let trigger: web_sys::HtmlElement = trigger.unchecked_into();
                    trigger.focus().ok();
                }
            });

            content
                .add_event_listener_with_callback(
                    ROOT_CONTENT_DISMISS,
                    handler.as_ref().unchecked_ref(),
                )
                .ok();

            let content_for_cleanup = SendWrapper::new(content.clone());
            let handler_for_cleanup = SendWrapper::new(handler);

            Owner::on_cleanup(move || {
                content_for_cleanup
                    .remove_event_listener_with_callback(
                        ROOT_CONTENT_DISMISS,
                        handler_for_cleanup.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let value_for_motion = value.clone();
    let motion_attribute = Memo::new(move |_| {
        let items = get_items();
        let mut values: Vec<String> = items.iter().map(|item| item.data.value.clone()).collect();
        if context.dir.get() == Direction::Rtl {
            values.reverse();
        }
        let current_value = context.value.get();
        let previous_value = context.previous_value.get();
        let index = values.iter().position(|v| *v == current_value);
        let prev_index = values.iter().position(|v| *v == previous_value);
        let is_selected = value_for_motion == current_value;
        let was_selected = prev_index == values.iter().position(|v| *v == value_for_motion);

        if !is_selected && !was_selected {
            return prev_motion_attribute.get_value();
        }

        let attribute: Option<&'static str> =
            if let (Some(idx), Some(prev_idx)) = (index, prev_index) {
                if idx != prev_idx {
                    if is_selected && prev_idx != usize::MAX {
                        Some(if idx > prev_idx {
                            "from-end"
                        } else {
                            "from-start"
                        })
                    } else if was_selected {
                        Some(if idx > prev_idx { "to-start" } else { "to-end" })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

        prev_motion_attribute.set_value(attribute);
        attribute
    });

    let composed_on_escape = Callback::new(move |event: web_sys::KeyboardEvent| {
        if let Some(cb) = on_escape_key_down {
            cb.run(event);
        }
        was_escape_close_ref.set(true);
    });

    let composed_on_focus_outside = Callback::new(move |event: web_sys::CustomEvent| {
        on_content_focus_outside.run(());
        if let Some(cb) = on_focus_outside {
            cb.run(event.clone());
        }
        // Only dismiss content when focus moves outside of the menu
        if let Some(target) = event
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            && let Some(root) = context.root_navigation_menu.get()
        {
            let root: web_sys::Node = root.unchecked_into();
            if root.contains(Some(target.unchecked_ref())) {
                event.prevent_default();
            }
        }
    });

    let get_items_for_pdo = use_collection::<NavigationMenuItemData>();
    let composed_on_pointer_down_outside = Callback::new(move |event: web_sys::CustomEvent| {
        if let Some(cb) = on_pointer_down_outside {
            cb.run(event.clone());
        }
        // untrack: this runs in an event handler (non-reactive context), we just need
        // the current values without creating reactive subscriptions.
        untrack(|| {
            if let Some(target) = event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok())
            {
                let items = get_items_for_pdo();
                let is_trigger = items.iter().any(|item| {
                    item.r#ref
                        .get()
                        .map(|el| {
                            let el: web_sys::Node = el.unchecked_into();
                            el.contains(Some(target.unchecked_ref()))
                        })
                        .unwrap_or(false)
                });
                let is_root_viewport = context.is_root_menu
                    && context
                        .viewport
                        .get()
                        .map(|vp| {
                            let vp: &web_sys::Node = vp.unchecked_ref();
                            vp.contains(Some(target.unchecked_ref()))
                        })
                        .unwrap_or(false);

                if is_trigger || is_root_viewport || !context.is_root_menu {
                    event.prevent_default();
                }
            }
        });
    });

    view! {
        <FocusGroup>
            <DismissableLayer
                as_child=true
                disable_outside_pointer_events=false
                on_dismiss=Callback::new(move |_| {
                    if let Some(el) = content_ref.get() {
                        let el: web_sys::HtmlElement = el.unchecked_into();
                        let mut init = web_sys::EventInit::new();
                        init.set_bubbles(true);
                        init.set_cancelable(true);
                        let event = web_sys::Event::new_with_event_init_dict(ROOT_CONTENT_DISMISS, &init)
                            .expect("Event should be created.");
                        el.dispatch_event(&event).ok();
                    }
                })
                on_escape_key_down=composed_on_escape
                on_focus_outside=composed_on_focus_outside
                on_pointer_down_outside=composed_on_pointer_down_outside
                on_interact_outside=on_interact_outside.unwrap_or(Callback::new(|_| {}))
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=composed_refs
                    attr:id=move || content_id.get()
                    attr:aria-labelledby=move || trigger_id.get()
                    attr:data-motion=move || motion_attribute.get()
                    attr:data-orientation=move || context.orientation.get().to_string()
                    on:pointerenter=move |event: ev::PointerEvent| {
                        on_pointer_enter.run(event);
                    }
                    on:pointerleave=move |event: ev::PointerEvent| {
                        on_pointer_leave.run(event);
                    }
                    on:keydown=move |event: ev::KeyboardEvent| {
                        let is_meta_key = event.alt_key() || event.ctrl_key() || event.meta_key();
                        let is_tab_key = event.key() == "Tab" && !is_meta_key;
                        if is_tab_key
                            && let Some(current_target) = event.current_target()
                        {
                            let current_target: web_sys::HtmlElement = current_target.unchecked_into();
                            let candidates = get_tabbable_candidates(&current_target);
                            let focused_element = document().active_element();
                            let index = candidates.iter().position(|c| {
                                focused_element.as_ref().map(|f| {
                                    c == f.unchecked_ref::<web_sys::HtmlElement>()
                                }).unwrap_or(false)
                            }).unwrap_or(0);
                            let is_moving_backwards = event.shift_key();
                            let next_candidates = if is_moving_backwards {
                                let mut slice = candidates[..index].to_vec();
                                slice.reverse();
                                slice
                            } else {
                                candidates[index + 1..].to_vec()
                            };

                            if focus_first(&next_candidates) {
                                event.prevent_default();
                            } else {
                                // Focus the proxy and let browser handle Tab
                                if let Some(proxy) = focus_proxy_ref.get() {
                                    let proxy: web_sys::HtmlElement = proxy.unchecked_into();
                                    proxy.focus().ok();
                                }
                            }
                        }
                    }
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </Primitive>
            </DismissableLayer>
        </FocusGroup>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuViewport(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();

    // Signal viewport existence synchronously during construction so Content components
    // know to use viewport rendering before any menu open interaction occurs.
    context.has_viewport_component.set(true);
    on_cleanup(move || {
        context.has_viewport_component.set(false);
    });

    let open = Signal::derive(move || !context.value.get().is_empty());

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || open.get());
    let presence_ref = AnyNodeRef::new();

    view! {
        <Presence present=present node_ref=presence_ref>
            <NavigationMenuViewportImpl
                as_child=as_child
                node_ref=node_ref
                presence_ref=presence_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </NavigationMenuViewportImpl>
        </Presence>
    }
}

#[component]
fn NavigationMenuViewportImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let _children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let viewport_content_context = expect_context::<ViewportContentContextValue>();

    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, viewport_ref, presence_ref]);

    // Register viewport element
    Effect::new(move |_| {
        if let Some(el) = viewport_ref.get() {
            let html_el: web_sys::HtmlElement = el.unchecked_into();
            context.viewport.set(Some(SendWrapper::new(html_el)));
        }
    });

    on_cleanup(move || {
        context.viewport.set(None);
    });

    let size: RwSignal<Option<(f64, f64)>> = RwSignal::new(None);
    let content_el: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);

    let open = Signal::derive(move || !context.value.get().is_empty());
    let active_content_value = Memo::new(move |_| {
        if open.get() {
            context.value.get()
        } else {
            context.previous_value.get()
        }
    });

    let handle_size_change = Callback::new(move |_: ()| {
        if let Some(el) = content_el.get_untracked() {
            size.set(Some((el.offset_width() as f64, el.offset_height() as f64)));
        }
    });

    use_resize_observer(Signal::derive(move || content_el.get()), handle_size_change);

    // Set CSS custom properties via Effect
    Effect::new(move |_| {
        if let Some(vp) = viewport_ref.get() {
            let vp: web_sys::HtmlElement = vp.unchecked_into();
            let style = vp.style();
            if let Some((w, h)) = size.get() {
                let _ =
                    style.set_property("--radix-navigation-menu-viewport-width", &format!("{w}px"));
                let _ = style
                    .set_property("--radix-navigation-menu-viewport-height", &format!("{h}px"));
            }
            if !open.get() && context.is_root_menu {
                let _ = style.set_property("pointer-events", "none");
            } else {
                let _ = style.remove_property("pointer-events");
            }
        }
    });

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_refs
            attr:data-state=move || get_open_state(open.get())
            attr:data-orientation=move || context.orientation.get().to_string()
            on:pointerenter=move |_: ev::PointerEvent| {
                context.on_content_enter.run(());
            }
            on:pointerleave=move |event: ev::PointerEvent| {
                if event.pointer_type() == "mouse" {
                    context.on_content_leave.run(());
                }
            }
        >
            {move || {
                let items = viewport_content_context.items.get();
                let active_val = active_content_value.get();
                items.iter().map(|(value, data)| {
                    let is_active = active_val == *value;
                    let present = Signal::derive({
                        let force_mount = data.force_mount;
                        move || force_mount || is_active
                    });
                    let presence_ref = AnyNodeRef::new();

                    let data_value = StoredValue::new(data.value.clone());
                    let data_trigger_ref = data.trigger_ref;
                    let data_focus_proxy_ref = data.focus_proxy_ref;
                    let data_was_escape_close_ref = data.was_escape_close_ref;
                    let data_on_content_focus_outside = data.on_content_focus_outside;
                    let data_on_root_content_close = data.on_root_content_close;
                    let data_children = StoredValue::new(data.children.clone());
                    let data_on_pointer_enter = data.on_pointer_enter;
                    let data_on_pointer_leave = data.on_pointer_leave;
                    let data_on_escape_key_down = StoredValue::new(data.on_escape_key_down);
                    let data_on_focus_outside = StoredValue::new(data.on_focus_outside);
                    let data_on_pointer_down_outside = StoredValue::new(data.on_pointer_down_outside);
                    let data_on_interact_outside = StoredValue::new(data.on_interact_outside);
                    let data_content_ref = data.content_ref;

                    // Capture content element ref for viewport sizing
                    let inner_ref = AnyNodeRef::new();
                    let combined_ref = use_composed_refs(vec![data_content_ref, inner_ref]);

                    // When active, set content_el for resize observation
                    Effect::new(move |_| {
                        if is_active
                            && let Some(el) = inner_ref.get()
                        {
                            let html_el: web_sys::HtmlElement = el.unchecked_into();
                            content_el.set(Some(SendWrapper::new(html_el)));
                        }
                    });

                    view! {
                        <Presence present=present node_ref=presence_ref>
                            <NavigationMenuContentImpl
                                value=data_value.get_value()
                                trigger_ref=data_trigger_ref
                                focus_proxy_ref=data_focus_proxy_ref
                                was_escape_close_ref=data_was_escape_close_ref
                                on_content_focus_outside=data_on_content_focus_outside
                                on_root_content_close=data_on_root_content_close
                                node_ref=combined_ref
                                presence_ref=presence_ref
                                on_pointer_enter=Callback::new(compose_callbacks(
                                    data_on_pointer_enter,
                                    Some(Callback::new(move |_: ev::PointerEvent| {
                                        context.on_content_enter.run(());
                                    })),
                                    None,
                                ))
                                on_pointer_leave=Callback::new(compose_callbacks(
                                    data_on_pointer_leave,
                                    Some(Callback::new(move |event: ev::PointerEvent| {
                                        if event.pointer_type() == "mouse" {
                                            context.on_content_leave.run(());
                                        }
                                    })),
                                    None,
                                ))
                                on_escape_key_down=data_on_escape_key_down.get_value().unwrap_or(Callback::new(|_| {}))
                                on_focus_outside=data_on_focus_outside.get_value().unwrap_or(Callback::new(|_| {}))
                                on_pointer_down_outside=data_on_pointer_down_outside.get_value().unwrap_or(Callback::new(|_| {}))
                                on_interact_outside=data_on_interact_outside.get_value().unwrap_or(Callback::new(|_| {}))
                            >
                                {data_children.with_value(|c| c.as_ref().map(|c| c()))}
                            </NavigationMenuContentImpl>
                        </Presence>
                    }
                }).collect_view()
            }}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FocusGroup (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FocusGroup(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();

    view! {
        <CollectionProvider<FocusGroupItemData> item_data_type=PhantomData>
            <CollectionSlot<FocusGroupItemData> item_data_type=PhantomData>
                <Primitive
                    element=html::div
                    as_child=MaybeProp::from(true)
                    attr:dir=move || context.dir.get().to_string()
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </CollectionSlot<FocusGroupItemData>>
        </CollectionProvider<FocusGroupItemData>>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FocusGroupItem (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FocusGroupItem(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let get_items = StoredValue::new(use_collection::<FocusGroupItemData>());

    view! {
        <CollectionItemSlot<FocusGroupItemData>
            item_data_type=PhantomData
            item_data=FocusGroupItemData
        >
            <Primitive
                element=html::button
                as_child=MaybeProp::from(true)
                on:keydown=move |event: ev::KeyboardEvent| {
                    let is_focus_navigation_key = event.key() == "Home"
                        || event.key() == "End"
                        || ARROW_KEYS.contains(&event.key().as_str());
                    if is_focus_navigation_key {
                        // untrack: event handler reads signals without needing subscriptions.
                        untrack(|| {
                            let items = get_items.with_value(|gi| gi());
                            let mut candidate_nodes: Vec<web_sys::HtmlElement> = items
                                .iter()
                                .filter_map(|item| {
                                    item.r#ref.get().map(|el| el.unchecked_into())
                                })
                                .collect();

                            let prev_item_key = if context.dir.get() == Direction::Rtl {
                                "ArrowRight"
                            } else {
                                "ArrowLeft"
                            };
                            let prev_keys = [prev_item_key, "ArrowUp", "End"];
                            if prev_keys.contains(&event.key().as_str()) {
                                candidate_nodes.reverse();
                            }

                            if ARROW_KEYS.contains(&event.key().as_str())
                                && let Some(current_target) = event.current_target()
                            {
                                let current_target: web_sys::HtmlElement = current_target.unchecked_into();
                                if let Some(current_index) = candidate_nodes.iter().position(|n| *n == current_target) {
                                    candidate_nodes = candidate_nodes[current_index + 1..].to_vec();
                                }
                            }

                            // Use setTimeout to defer focus (avoid batching issues)
                            let candidates = SendWrapper::new(candidate_nodes);
                            set_timeout(move || {
                                focus_first(&candidates);
                            }, 0);
                        });

                        event.prevent_default();
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </CollectionItemSlot<FocusGroupItemData>>
    }
}
