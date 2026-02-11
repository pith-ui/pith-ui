use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use leptos::{attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_presence::Presence;
use radix_leptos_primitive::Primitive;
use radix_number::clamp;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::{JsCast, closure::Closure};

/* -------------------------------------------------------------------------------------------------
 * Types
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum ScrollAreaType {
    Auto,
    Always,
    Scroll,
    #[default]
    Hover,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[derive(Clone, Debug)]
struct Sizes {
    content: f64,
    viewport: f64,
    scrollbar: ScrollbarSizes,
}

#[derive(Clone, Debug)]
struct ScrollbarSizes {
    size: f64,
    padding_start: f64,
    padding_end: f64,
}

impl Default for Sizes {
    fn default() -> Self {
        Self {
            content: 0.0,
            viewport: 0.0,
            scrollbar: ScrollbarSizes {
                size: 0.0,
                padding_start: 0.0,
                padding_end: 0.0,
            },
        }
    }
}

/* -------------------------------------------------------------------------------------------------
 * Context
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct ScrollAreaContextValue {
    r#type: ScrollAreaType,
    dir: Signal<Direction>,
    scroll_hide_delay: u32,
    scroll_area: AnyNodeRef,
    viewport: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    content: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    scrollbar_x: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    scrollbar_x_enabled: RwSignal<bool>,
    scrollbar_y: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    scrollbar_y_enabled: RwSignal<bool>,
    corner_width: RwSignal<f64>,
    corner_height: RwSignal<f64>,
}

#[derive(Clone)]
struct ScrollbarContextValue {
    has_thumb: Signal<bool>,
    #[allow(dead_code)]
    scrollbar: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
}

/// Forwarded CSS class from public component to inner Impl, bypassing Presence/Show boundaries.
#[derive(Clone)]
struct ForwardedScrollbarClass(Option<String>);

/// Forwarded CSS class from public ScrollAreaThumb to ScrollAreaThumbImpl.
#[derive(Clone)]
struct ForwardedThumbClass(Option<String>);

/// Forwarded CSS class from public ScrollAreaCorner to ScrollAreaCornerImpl.
#[derive(Clone)]
struct ForwardedCornerClass(Option<String>);

/* -------------------------------------------------------------------------------------------------
 * State machine (used by ScrollAreaScrollbarScroll)
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScrollbarMachineState {
    Hidden,
    Scrolling,
    Interacting,
    Idle,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ScrollbarMachineEvent {
    Scroll,
    ScrollEnd,
    PointerEnter,
    PointerLeave,
    Hide,
}

fn use_scrollbar_state_machine() -> (
    ReadSignal<ScrollbarMachineState>,
    Callback<ScrollbarMachineEvent>,
) {
    use ScrollbarMachineEvent::*;
    use ScrollbarMachineState::*;

    let (state, set_state) = signal(Hidden);

    let machine: HashMap<
        ScrollbarMachineState,
        HashMap<ScrollbarMachineEvent, ScrollbarMachineState>,
    > = HashMap::from([
        (Hidden, HashMap::from([(Scroll, Scrolling)])),
        (
            Scrolling,
            HashMap::from([(ScrollEnd, Idle), (PointerEnter, Interacting)]),
        ),
        (
            Interacting,
            HashMap::from([(Scroll, Interacting), (PointerLeave, Idle)]),
        ),
        (
            Idle,
            HashMap::from([
                (Hide, Hidden),
                (Scroll, Scrolling),
                (PointerEnter, Interacting),
            ]),
        ),
    ]);

    (
        state,
        Callback::new(move |event| {
            let current_state = state.get_untracked();
            let next_state = machine
                .get(&current_state)
                .and_then(|events| events.get(&event));
            if let Some(next_state) = next_state {
                set_state.set(*next_state);
            }
        }),
    )
}

/* -------------------------------------------------------------------------------------------------
 * Helpers
 * -----------------------------------------------------------------------------------------------*/

fn to_int(value: &str) -> f64 {
    // Mimic JS parseInt(value, 10): skip whitespace, read optional sign + digits, ignore rest (e.g. "8px" → 8).
    let trimmed = value.trim_start();
    let numeric: String = trimmed
        .chars()
        .enumerate()
        .take_while(|(i, c)| c.is_ascii_digit() || (*i == 0 && (*c == '-' || *c == '+')))
        .map(|(_, c)| c)
        .collect();
    numeric.parse::<f64>().unwrap_or(0.0)
}

fn get_thumb_ratio(viewport_size: f64, content_size: f64) -> f64 {
    let ratio = viewport_size / content_size;
    if ratio.is_nan() { 0.0 } else { ratio }
}

fn get_thumb_size(sizes: &Sizes) -> f64 {
    let ratio = get_thumb_ratio(sizes.viewport, sizes.content);
    let scrollbar_padding = sizes.scrollbar.padding_start + sizes.scrollbar.padding_end;
    let thumb_size = (sizes.scrollbar.size - scrollbar_padding) * ratio;
    // minimum of 18 matches macOS minimum
    thumb_size.max(18.0)
}

fn get_scroll_position_from_pointer(
    pointer_pos: f64,
    pointer_offset: f64,
    sizes: &Sizes,
    dir: Direction,
) -> f64 {
    let thumb_size_px = get_thumb_size(sizes);
    let thumb_center = thumb_size_px / 2.0;
    let offset = if pointer_offset != 0.0 {
        pointer_offset
    } else {
        thumb_center
    };
    let thumb_offset_from_end = thumb_size_px - offset;
    let min_pointer_pos = sizes.scrollbar.padding_start + offset;
    let max_pointer_pos =
        sizes.scrollbar.size - sizes.scrollbar.padding_end - thumb_offset_from_end;
    let max_scroll_pos = sizes.content - sizes.viewport;
    let scroll_range = match dir {
        Direction::Ltr => [0.0, max_scroll_pos],
        Direction::Rtl => [-max_scroll_pos, 0.0],
    };
    let interpolate = linear_scale([min_pointer_pos, max_pointer_pos], scroll_range);
    interpolate(pointer_pos)
}

fn get_thumb_offset_from_scroll(scroll_pos: f64, sizes: &Sizes, dir: Direction) -> f64 {
    let thumb_size_px = get_thumb_size(sizes);
    let scrollbar_padding = sizes.scrollbar.padding_start + sizes.scrollbar.padding_end;
    let scrollbar = sizes.scrollbar.size - scrollbar_padding;
    let max_scroll_pos = sizes.content - sizes.viewport;
    let max_thumb_pos = scrollbar - thumb_size_px;
    let scroll_clamp_range = match dir {
        Direction::Ltr => [0.0, max_scroll_pos],
        Direction::Rtl => [-max_scroll_pos, 0.0],
    };
    let scroll_without_momentum = clamp(scroll_pos, scroll_clamp_range);
    let interpolate = linear_scale([0.0, max_scroll_pos], [0.0, max_thumb_pos]);
    interpolate(scroll_without_momentum)
}

fn linear_scale(input: [f64; 2], output: [f64; 2]) -> impl Fn(f64) -> f64 {
    move |value: f64| {
        if input[0] == input[1] || output[0] == output[1] {
            return output[0];
        }
        let ratio = (output[1] - output[0]) / (input[1] - input[0]);
        output[0] + ratio * (value - input[0])
    }
}

fn is_scrolling_within_scrollbar_bounds(scroll_pos: f64, max_scroll_pos: f64) -> bool {
    scroll_pos > 0.0 && scroll_pos < max_scroll_pos
}

type RafClosureHolder = std::rc::Rc<std::cell::RefCell<Option<Closure<dyn Fn()>>>>;

/// Custom scroll handler to avoid scroll-linked effects.
/// Returns a cleanup function that cancels the rAF loop.
#[allow(dead_code)]
fn add_unlinked_scroll_listener(
    node: &web_sys::HtmlElement,
    handler: impl Fn() + 'static,
) -> impl FnOnce() {
    let node = node.clone();
    let prev_left = std::cell::Cell::new(node.scroll_left() as f64);
    let prev_top = std::cell::Cell::new(node.scroll_top() as f64);
    let raf_id = std::rc::Rc::new(std::cell::Cell::new(0i32));

    let window = web_sys::window().expect("Window should exist.");

    let closure: RafClosureHolder = std::rc::Rc::new(std::cell::RefCell::new(None));
    let closure_clone = closure.clone();

    let cb = Closure::new({
        let node = node.clone();
        let window = window.clone();
        let closure = closure.clone();
        let raf_id = raf_id.clone();
        move || {
            let left = node.scroll_left() as f64;
            let top = node.scroll_top() as f64;
            let is_horizontal_scroll = prev_left.get() != left;
            let is_vertical_scroll = prev_top.get() != top;
            if is_horizontal_scroll || is_vertical_scroll {
                handler();
            }
            prev_left.set(left);
            prev_top.set(top);
            if let Some(c) = closure.borrow().as_ref() {
                raf_id.set(
                    window
                        .request_animation_frame(c.as_ref().unchecked_ref())
                        .unwrap_or(0),
                );
            }
        }
    });

    raf_id.set(
        window
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .unwrap_or(0),
    );
    *closure_clone.borrow_mut() = Some(cb);

    let window_clone = window.clone();
    move || {
        window_clone.cancel_animation_frame(raf_id.get()).ok();
        // Drop the closure to break the circular reference
        closure_clone.borrow_mut().take();
    }
}

fn use_debounce_callback(
    callback: impl Fn() + Send + Sync + 'static,
    delay_ms: i32,
) -> Callback<()> {
    let timer_id: StoredValue<i32> = StoredValue::new(0);

    Owner::on_cleanup(move || {
        let window = web_sys::window().expect("Window should exist.");
        window.clear_timeout_with_handle(timer_id.get_value());
    });

    let callback = SendWrapper::new(callback);
    let callback = StoredValue::new(callback);

    Callback::new(move |()| {
        let window = web_sys::window().expect("Window should exist.");
        window.clear_timeout_with_handle(timer_id.get_value());
        let cb = callback;
        let handle = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &Closure::once_into_js(move || {
                    cb.with_value(|cb| cb());
                })
                .unchecked_into(),
                delay_ms,
            )
            .unwrap_or(0);
        timer_id.set_value(handle);
    })
}

fn use_resize_observer(
    element: Signal<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_resize: impl Fn() + 'static,
) {
    let on_resize = SendWrapper::new(on_resize);
    let on_resize = StoredValue::new(on_resize);

    Effect::new(move |_| {
        if let Some(el) = element.get() {
            let window = web_sys::window().expect("Window should exist.");
            let raf_id = std::cell::Cell::new(0i32);
            let window_clone = window.clone();

            let resize_callback = Closure::<dyn Fn(web_sys::js_sys::Array)>::new(
                move |_entries: web_sys::js_sys::Array| {
                    window_clone.cancel_animation_frame(raf_id.get()).ok();
                    let on_resize = on_resize;
                    let w = web_sys::window().expect("Window should exist.");
                    let handle = w
                        .request_animation_frame(
                            &Closure::once_into_js(move || {
                                on_resize.with_value(|cb| cb());
                            })
                            .unchecked_into(),
                        )
                        .unwrap_or(0);
                    raf_id.set(handle);
                },
            );

            let observer = web_sys::ResizeObserver::new(resize_callback.as_ref().unchecked_ref())
                .expect("ResizeObserver should be created.");
            observer.observe(el.unchecked_ref::<web_sys::Element>());

            // Prevent the closure from being dropped while the observer is active
            let resize_callback = SendWrapper::new(resize_callback);
            let observer = SendWrapper::new(observer);
            Owner::on_cleanup(move || {
                observer.disconnect();
                drop(resize_callback);
            });
        }
    });
}

/* -------------------------------------------------------------------------------------------------
 * ScrollArea
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollArea(
    #[prop(into, optional)] r#type: Option<ScrollAreaType>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] scroll_hide_delay: Option<u32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    let r#type = r#type.unwrap_or_default();
    let scroll_hide_delay = scroll_hide_delay.unwrap_or(600);
    let direction = use_direction(dir);

    let scroll_area_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scroll_area_ref]);

    let corner_width: RwSignal<f64> = RwSignal::new(0.0);
    let corner_height: RwSignal<f64> = RwSignal::new(0.0);

    let context = ScrollAreaContextValue {
        r#type,
        dir: direction,
        scroll_hide_delay,
        scroll_area: scroll_area_ref,
        viewport: RwSignal::new(None),
        content: RwSignal::new(None),
        scrollbar_x: RwSignal::new(None),
        scrollbar_x_enabled: RwSignal::new(false),
        scrollbar_y: RwSignal::new(None),
        scrollbar_y_enabled: RwSignal::new(false),
        corner_width,
        corner_height,
    };

    view! {
        <Provider value=context>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=composed_ref
                    attr:dir=move || direction.get().to_string()
                    attr:style=move || {
                        format!(
                            "position: relative; --radix-scroll-area-corner-width: {}px; --radix-scroll-area-corner-height: {}px;",
                            corner_width.get(),
                            corner_height.get()
                        )
                    }
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaViewport(
    #[prop(into, optional)] nonce: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let nonce = StoredValue::new(nonce);

    let context = expect_context::<ScrollAreaContextValue>();

    let viewport_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, viewport_ref]);

    // Set viewport element in context when ref is available
    Effect::new(move |_| {
        if let Some(node) = viewport_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            context.viewport.set(Some(SendWrapper::new(el)));
        }
    });

    let scrollbar_x_enabled = context.scrollbar_x_enabled;
    let scrollbar_y_enabled = context.scrollbar_y_enabled;
    let content = context.content;

    let content_ref = AnyNodeRef::new();
    Effect::new(move |_| {
        if let Some(node) = content_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            content.set(Some(SendWrapper::new(el)));
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <style
                nonce=nonce.get_value()
                inner_html="[data-radix-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-scroll-area-viewport]::-webkit-scrollbar{display:none}"
            />
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:data-radix-scroll-area-viewport=""
                attr:style=move || {
                    format!(
                        "overflow-x: {}; overflow-y: {};",
                        if scrollbar_x_enabled.get() { "scroll" } else { "hidden" },
                        if scrollbar_y_enabled.get() { "scroll" } else { "hidden" },
                    )
                }
                {..attrs}
            >
                <div
                    style="min-width: 100%; display: table;"
                    node_ref=content_ref
                >
                    {children.with_value(|children| children())}
                </div>
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbar
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaScrollbar(
    #[prop(into, optional)] orientation: Option<Orientation>,
    #[prop(into, optional)] force_mount: Option<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let orientation = orientation.unwrap_or_default();
    let force_mount = force_mount.unwrap_or(false);
    let context = expect_context::<ScrollAreaContextValue>();

    // Forward class via context to bypass Presence/Show boundary
    provide_context(ForwardedScrollbarClass(class.get_untracked()));

    let is_horizontal = orientation == Orientation::Horizontal;

    // Register scrollbar axis with context
    Effect::new(move |_| {
        if is_horizontal {
            context.scrollbar_x_enabled.set(true);
        } else {
            context.scrollbar_y_enabled.set(true);
        }
    });
    Owner::on_cleanup(move || {
        if is_horizontal {
            context.scrollbar_x_enabled.set(false);
        } else {
            context.scrollbar_y_enabled.set(false);
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            {match context.r#type {
                ScrollAreaType::Hover => view! {
                    <ScrollAreaScrollbarHover
                        orientation=orientation
                        force_mount=force_mount
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarHover>
                }
                .add_any_attr(attrs)
                .into_any(),
                ScrollAreaType::Scroll => view! {
                    <ScrollAreaScrollbarScroll
                        orientation=orientation
                        force_mount=force_mount
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarScroll>
                }
                .add_any_attr(attrs)
                .into_any(),
                ScrollAreaType::Auto => view! {
                    <ScrollAreaScrollbarAuto
                        orientation=orientation
                        force_mount=force_mount
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarAuto>
                }
                .add_any_attr(attrs)
                .into_any(),
                ScrollAreaType::Always => view! {
                    <ScrollAreaScrollbarVisible
                        orientation=orientation
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarVisible>
                }
                .add_any_attr(attrs)
                .into_any(),
            }}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarHover
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarHover(
    #[prop(into)] orientation: Orientation,
    #[prop(into)] force_mount: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        if let Some(scroll_area_node) = context.scroll_area.get() {
            let scroll_area: &web_sys::HtmlElement = scroll_area_node.unchecked_ref();

            let scroll_hide_delay = context.scroll_hide_delay;
            let hide_timer: StoredValue<i32> = StoredValue::new(0);

            let visible_enter = visible;
            let enter_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                let window = web_sys::window().expect("Window should exist.");
                window.clear_timeout_with_handle(hide_timer.get_value());
                visible_enter.set(true);
            }));

            let visible_leave = visible;
            let leave_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                let window = web_sys::window().expect("Window should exist.");
                let handle = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        &Closure::once_into_js(move || {
                            visible_leave.set(false);
                        })
                        .unchecked_into(),
                        scroll_hide_delay as i32,
                    )
                    .unwrap_or(0);
                hide_timer.set_value(handle);
            }));

            scroll_area
                .add_event_listener_with_callback(
                    "pointerenter",
                    enter_closure.as_ref().unchecked_ref(),
                )
                .ok();
            scroll_area
                .add_event_listener_with_callback(
                    "pointerleave",
                    leave_closure.as_ref().unchecked_ref(),
                )
                .ok();

            let scroll_area = SendWrapper::new(scroll_area.clone());
            Owner::on_cleanup(move || {
                let window = web_sys::window().expect("Window should exist.");
                window.clear_timeout_with_handle(hide_timer.get_value());
                scroll_area
                    .remove_event_listener_with_callback(
                        "pointerenter",
                        enter_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
                scroll_area
                    .remove_event_listener_with_callback(
                        "pointerleave",
                        leave_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || visible.get()) node_ref=presence_ref>
                    <ScrollAreaScrollbarAuto
                        orientation=orientation
                        force_mount=false
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                        attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarAuto>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarScroll
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarScroll(
    #[prop(into)] orientation: Orientation,
    #[prop(into)] force_mount: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();
    let is_horizontal = orientation == Orientation::Horizontal;

    let (state, send) = use_scrollbar_state_machine();
    let debounce_scroll_end =
        use_debounce_callback(move || send.run(ScrollbarMachineEvent::ScrollEnd), 100);

    // Hide timer for idle state
    Effect::new(move |_| {
        if state.get() == ScrollbarMachineState::Idle {
            let scroll_hide_delay = context.scroll_hide_delay;
            let window = web_sys::window().expect("Window should exist.");
            let handle = window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    &Closure::once_into_js(move || {
                        send.run(ScrollbarMachineEvent::Hide);
                    })
                    .unchecked_into(),
                    scroll_hide_delay as i32,
                )
                .unwrap_or(0);

            Owner::on_cleanup(move || {
                let window = web_sys::window().expect("Window should exist.");
                window.clear_timeout_with_handle(handle);
            });
        }
    });

    // Scroll listener on viewport
    Effect::new(move |_| {
        if let Some(viewport) = context.viewport.get() {
            let send = send;
            let debounce = debounce_scroll_end;
            let prev_scroll_pos = std::cell::Cell::new(if is_horizontal {
                viewport.scroll_left() as f64
            } else {
                viewport.scroll_top() as f64
            });

            let viewport_for_closure = viewport.clone();
            let scroll_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                let scroll_pos = if is_horizontal {
                    viewport_for_closure.scroll_left() as f64
                } else {
                    viewport_for_closure.scroll_top() as f64
                };
                if prev_scroll_pos.get() != scroll_pos {
                    send.run(ScrollbarMachineEvent::Scroll);
                    debounce.run(());
                }
                prev_scroll_pos.set(scroll_pos);
            }));

            let viewport_target: web_sys::EventTarget =
                <web_sys::HtmlElement as Clone>::clone(&viewport).unchecked_into();
            viewport_target
                .add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref())
                .ok();

            let viewport_target = SendWrapper::new(viewport_target);
            Owner::on_cleanup(move || {
                viewport_target
                    .remove_event_listener_with_callback(
                        "scroll",
                        scroll_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
            });
        }
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || state.get() != ScrollbarMachineState::Hidden) node_ref=presence_ref>
                    <ScrollAreaScrollbarVisible
                        orientation=orientation
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                        attr:data-state=move || if state.get() == ScrollbarMachineState::Hidden { "hidden" } else { "visible" }
                        on:pointerenter=move |_| send.run(ScrollbarMachineEvent::PointerEnter)
                        on:pointerleave=move |_| send.run(ScrollbarMachineEvent::PointerLeave)
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarVisible>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarAuto
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarAuto(
    #[prop(into)] orientation: Orientation,
    #[prop(into)] force_mount: bool,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();
    let visible = RwSignal::new(false);
    let is_horizontal = orientation == Orientation::Horizontal;

    let handle_resize = use_debounce_callback(
        move || {
            if let Some(viewport) = context.viewport.get_untracked() {
                let is_overflow = if is_horizontal {
                    viewport.offset_width() < viewport.scroll_width()
                } else {
                    viewport.offset_height() < viewport.scroll_height()
                };
                visible.set(is_overflow);
            }
        },
        10,
    );

    use_resize_observer(Signal::derive(move || context.viewport.get()), move || {
        handle_resize.run(())
    });
    use_resize_observer(Signal::derive(move || context.content.get()), move || {
        handle_resize.run(())
    });

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || visible.get()) node_ref=presence_ref>
                    <ScrollAreaScrollbarVisible
                        orientation=orientation
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                        attr:data-state=move || if visible.get() { "visible" } else { "hidden" }
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaScrollbarVisible>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarVisible
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarVisible(
    #[prop(into, optional)] orientation: Option<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let orientation = orientation.unwrap_or_default();
    let context = expect_context::<ScrollAreaContextValue>();

    let thumb_ref: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let pointer_offset = RwSignal::new(0.0_f64);
    let sizes = RwSignal::new(Sizes::default());

    let has_thumb = Signal::derive(move || {
        let s = sizes.get();
        let r = get_thumb_ratio(s.viewport, s.content);
        r > 0.0 && r < 1.0
    });

    let on_thumb_change = Callback::new(move |thumb: Option<SendWrapper<web_sys::HtmlElement>>| {
        thumb_ref.set(thumb);
    });
    let on_thumb_pointer_up = Callback::new(move |()| {
        pointer_offset.set(0.0);
    });

    view! {
        <AttributeInterceptor let:attrs>
            {match orientation {
                Orientation::Horizontal => {
                    let on_thumb_pointer_down = Callback::new(move |(x, _y): (f64, f64)| {
                        pointer_offset.set(x);
                    });

                let on_thumb_position_change = Callback::new(move |()| {
                    if let Some(viewport) = context.viewport.get()
                        && let Some(thumb) = thumb_ref.get()
                    {
                        let scroll_pos = viewport.scroll_left() as f64;
                        let offset =
                            get_thumb_offset_from_scroll(scroll_pos, &sizes.get(), context.dir.get());
                        thumb
                            .style()
                            .set_property("transform", &format!("translate3d({}px, 0, 0)", offset))
                            .ok();
                    }
                });

                let on_wheel_scroll = Callback::new(move |scroll_pos: f64| {
                    if let Some(viewport) = context.viewport.get() {
                        viewport.set_scroll_left(scroll_pos as i32);
                    }
                });

                let on_drag_scroll = Callback::new(move |pointer_pos: f64| {
                    if let Some(viewport) = context.viewport.get() {
                        let scroll_pos = get_scroll_position_from_pointer(
                            pointer_pos,
                            pointer_offset.get_untracked(),
                            &sizes.get_untracked(),
                            context.dir.get_untracked(),
                        );
                        viewport.set_scroll_left(scroll_pos as i32);
                    }
                });

                    view! {
                        <ScrollAreaScrollbarX
                            sizes=sizes
                            has_thumb=has_thumb
                            on_thumb_change=on_thumb_change
                            on_thumb_pointer_up=on_thumb_pointer_up
                            on_thumb_pointer_down=on_thumb_pointer_down
                            on_thumb_position_change=on_thumb_position_change
                            on_wheel_scroll=on_wheel_scroll
                            on_drag_scroll=on_drag_scroll
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </ScrollAreaScrollbarX>
                    }
                    .add_any_attr(attrs)
                    .into_any()
                }
                Orientation::Vertical => {
                let on_thumb_pointer_down = Callback::new(move |(_x, y): (f64, f64)| {
                    pointer_offset.set(y);
                });

                let on_thumb_position_change = Callback::new(move |()| {
                    if let Some(viewport) = context.viewport.get()
                        && let Some(thumb) = thumb_ref.get()
                    {
                        let scroll_pos = viewport.scroll_top() as f64;
                        let offset =
                            get_thumb_offset_from_scroll(scroll_pos, &sizes.get(), Direction::Ltr);
                        thumb
                            .style()
                            .set_property("transform", &format!("translate3d(0, {}px, 0)", offset))
                            .ok();
                    }
                });

                let on_wheel_scroll = Callback::new(move |scroll_pos: f64| {
                    if let Some(viewport) = context.viewport.get() {
                        viewport.set_scroll_top(scroll_pos as i32);
                    }
                });

                let on_drag_scroll = Callback::new(move |pointer_pos: f64| {
                    if let Some(viewport) = context.viewport.get() {
                        let scroll_pos = get_scroll_position_from_pointer(
                            pointer_pos,
                            pointer_offset.get_untracked(),
                            &sizes.get_untracked(),
                            Direction::Ltr,
                        );
                        viewport.set_scroll_top(scroll_pos as i32);
                    }
                });

                    view! {
                        <ScrollAreaScrollbarY
                            sizes=sizes
                            has_thumb=has_thumb
                            on_thumb_change=on_thumb_change
                            on_thumb_pointer_up=on_thumb_pointer_up
                            on_thumb_pointer_down=on_thumb_pointer_down
                            on_thumb_position_change=on_thumb_position_change
                            on_wheel_scroll=on_wheel_scroll
                            on_drag_scroll=on_drag_scroll
                            as_child=as_child
                            node_ref=node_ref
                        >
                            {children.with_value(|children| children())}
                        </ScrollAreaScrollbarY>
                    }
                    .add_any_attr(attrs)
                    .into_any()
                }
            }}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarX / ScrollAreaScrollbarY
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarX(
    sizes: RwSignal<Sizes>,
    has_thumb: Signal<bool>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
    on_wheel_scroll: Callback<f64>,
    on_drag_scroll: Callback<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();
    let computed_style: RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>> =
        RwSignal::new(None);

    let scrollbar_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scrollbar_ref]);

    // Set computed style when ref is available
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();
            let window = web_sys::window().expect("Window should exist.");
            if let Ok(style) = window.get_computed_style(node) {
                computed_style.set(style.map(SendWrapper::new));
            }
        }
    });

    // Set scrollbar X element in context
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            context.scrollbar_x.set(Some(SendWrapper::new(el)));
        }
    });
    Owner::on_cleanup(move || {
        context.scrollbar_x.set(None);
    });

    let on_resize = Callback::new(move |()| {
        if let Some(node) = scrollbar_ref.get()
            && let Some(viewport) = context.viewport.get()
            && let Some(cs) = computed_style.get()
        {
            let scrollbar_el: &web_sys::HtmlElement = node.unchecked_ref();
            sizes.set(Sizes {
                content: viewport.scroll_width() as f64,
                viewport: viewport.offset_width() as f64,
                scrollbar: ScrollbarSizes {
                    size: scrollbar_el.client_width() as f64,
                    padding_start: to_int(
                        &cs.get_property_value("padding-left").unwrap_or_default(),
                    ),
                    padding_end: to_int(
                        &cs.get_property_value("padding-right").unwrap_or_default(),
                    ),
                },
            });
        }
    });

    let on_wheel_scroll_impl = Callback::new(
        move |(event, max_scroll_pos): (SendWrapper<web_sys::WheelEvent>, f64)| {
            if let Some(viewport) = context.viewport.get() {
                let scroll_pos = viewport.scroll_left() as f64 + event.delta_x();
                on_wheel_scroll.run(scroll_pos);
                if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
                    event.prevent_default();
                }
            }
        },
    );

    let on_drag_scroll_impl = Callback::new(move |(x, _y): (f64, f64)| {
        on_drag_scroll.run(x);
    });

    let on_thumb_pointer_down_impl = Callback::new(move |(x, y): (f64, f64)| {
        on_thumb_pointer_down.run((x, y));
    });

    let dir = context.dir;

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <ScrollAreaScrollbarImpl
                    data_orientation="horizontal"
                    sizes=sizes
                    has_thumb=has_thumb
                    on_thumb_change=on_thumb_change
                    on_thumb_pointer_up=on_thumb_pointer_up
                    on_thumb_pointer_down=on_thumb_pointer_down_impl
                    on_thumb_position_change=on_thumb_position_change
                    on_wheel_scroll=on_wheel_scroll_impl
                    on_drag_scroll=on_drag_scroll_impl
                    on_resize=on_resize
                    style=Signal::derive(move || {
                        let d = dir.get();
                        let thumb_width = get_thumb_size(&sizes.get());
                        format!(
                            "bottom: 0; {}: var(--radix-scroll-area-corner-width); {}: 0; --radix-scroll-area-thumb-width: {}px;",
                            if d == Direction::Rtl { "left" } else { "right" },
                            if d == Direction::Rtl { "right" } else { "left" },
                            thumb_width
                        )
                    })
                    as_child=as_child
                    node_ref=composed_ref
                >
                    {children.with_value(|children| children())}
                </ScrollAreaScrollbarImpl>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

#[component]
fn ScrollAreaScrollbarY(
    sizes: RwSignal<Sizes>,
    has_thumb: Signal<bool>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
    on_wheel_scroll: Callback<f64>,
    on_drag_scroll: Callback<f64>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();
    let computed_style: RwSignal<Option<SendWrapper<web_sys::CssStyleDeclaration>>> =
        RwSignal::new(None);

    let scrollbar_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scrollbar_ref]);

    // Set computed style when ref is available
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let node: &web_sys::HtmlElement = node.unchecked_ref();
            let window = web_sys::window().expect("Window should exist.");
            if let Ok(style) = window.get_computed_style(node) {
                computed_style.set(style.map(SendWrapper::new));
            }
        }
    });

    // Set scrollbar Y element in context
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            context.scrollbar_y.set(Some(SendWrapper::new(el)));
        }
    });
    Owner::on_cleanup(move || {
        context.scrollbar_y.set(None);
    });

    let on_resize = Callback::new(move |()| {
        if let Some(node) = scrollbar_ref.get()
            && let Some(viewport) = context.viewport.get()
            && let Some(cs) = computed_style.get()
        {
            let scrollbar_el: &web_sys::HtmlElement = node.unchecked_ref();
            sizes.set(Sizes {
                content: viewport.scroll_height() as f64,
                viewport: viewport.offset_height() as f64,
                scrollbar: ScrollbarSizes {
                    size: scrollbar_el.client_height() as f64,
                    padding_start: to_int(
                        &cs.get_property_value("padding-top").unwrap_or_default(),
                    ),
                    padding_end: to_int(
                        &cs.get_property_value("padding-bottom").unwrap_or_default(),
                    ),
                },
            });
        }
    });

    let on_wheel_scroll_impl = Callback::new(
        move |(event, max_scroll_pos): (SendWrapper<web_sys::WheelEvent>, f64)| {
            if let Some(viewport) = context.viewport.get() {
                let scroll_pos = viewport.scroll_top() as f64 + event.delta_y();
                on_wheel_scroll.run(scroll_pos);
                if is_scrolling_within_scrollbar_bounds(scroll_pos, max_scroll_pos) {
                    event.prevent_default();
                }
            }
        },
    );

    let on_drag_scroll_impl = Callback::new(move |(_x, y): (f64, f64)| {
        on_drag_scroll.run(y);
    });

    let on_thumb_pointer_down_impl = Callback::new(move |(x, y): (f64, f64)| {
        on_thumb_pointer_down.run((x, y));
    });

    let dir = context.dir;

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <ScrollAreaScrollbarImpl
                    data_orientation="vertical"
                    sizes=sizes
                    has_thumb=has_thumb
                    on_thumb_change=on_thumb_change
                    on_thumb_pointer_up=on_thumb_pointer_up
                    on_thumb_pointer_down=on_thumb_pointer_down_impl
                    on_thumb_position_change=on_thumb_position_change
                    on_wheel_scroll=on_wheel_scroll_impl
                    on_drag_scroll=on_drag_scroll_impl
                    on_resize=on_resize
                    style=Signal::derive(move || {
                        let d = dir.get();
                        let thumb_height = get_thumb_size(&sizes.get());
                        format!(
                            "top: 0; {}: 0; bottom: var(--radix-scroll-area-corner-height); --radix-scroll-area-thumb-height: {}px;",
                            if d == Direction::Ltr { "right" } else { "left" },
                            thumb_height
                        )
                    })
                    as_child=as_child
                    node_ref=composed_ref
                >
                    {children.with_value(|children| children())}
                </ScrollAreaScrollbarImpl>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaScrollbarImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn ScrollAreaScrollbarImpl(
    #[prop(into)] data_orientation: &'static str,
    sizes: RwSignal<Sizes>,
    has_thumb: Signal<bool>,
    on_thumb_change: Callback<Option<SendWrapper<web_sys::HtmlElement>>>,
    on_thumb_pointer_up: Callback<()>,
    on_thumb_pointer_down: Callback<(f64, f64)>,
    on_thumb_position_change: Callback<()>,
    on_wheel_scroll: Callback<(SendWrapper<web_sys::WheelEvent>, f64)>,
    on_drag_scroll: Callback<(f64, f64)>,
    on_resize: Callback<()>,
    #[prop(into)] style: Signal<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();

    // Read forwarded class from public ScrollAreaScrollbar component (StoredValue is Copy,
    // allowing capture by Fn closures inside the view! macro / AttributeInterceptor).
    let forwarded_class = StoredValue::new(
        use_context::<ForwardedScrollbarClass>()
            .and_then(|c| c.0)
            .unwrap_or_default(),
    );

    let scrollbar: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let scrollbar_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scrollbar_ref]);

    // Track scrollbar element
    Effect::new(move |_| {
        if let Some(node) = scrollbar_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            scrollbar.set(Some(SendWrapper::new(el)));
        } else {
            scrollbar.set(None);
        }
    });

    let rect: StoredValue<Option<SendWrapper<web_sys::DomRect>>> = StoredValue::new(None);
    let prev_webkit_user_select: StoredValue<String> = StoredValue::new(String::new());

    let max_scroll_pos = Memo::new(move |_| {
        let s = sizes.get();
        s.content - s.viewport
    });

    // Document-level wheel listener (passive: false to allow preventDefault)
    Effect::new(move |_| {
        let _scrollbar = scrollbar.get();
        let _max = max_scroll_pos.get();

        let on_wheel_scroll = on_wheel_scroll;
        let scrollbar = scrollbar;
        let wheel_closure = SendWrapper::new(Closure::<dyn Fn(web_sys::WheelEvent)>::new(
            move |event: web_sys::WheelEvent| {
                if let Some(target) = event.target()
                    && let Some(sb) = scrollbar.get_untracked()
                {
                    let target_el: &web_sys::Element = target.unchecked_ref();
                    if sb.contains(Some(target_el)) {
                        on_wheel_scroll
                            .run((SendWrapper::new(event), max_scroll_pos.get_untracked()));
                    }
                }
            },
        ));

        let document = web_sys::window()
            .expect("Window should exist.")
            .document()
            .expect("Document should exist.");

        let options = web_sys::AddEventListenerOptions::new();
        options.set_passive(false);
        document
            .add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                wheel_closure.as_ref().unchecked_ref(),
                &options,
            )
            .ok();

        let document = SendWrapper::new(document);
        Owner::on_cleanup(move || {
            document
                .remove_event_listener_with_callback(
                    "wheel",
                    wheel_closure.as_ref().unchecked_ref(),
                )
                .ok();
        });
    });

    // Update thumb position when sizes change
    Effect::new(move |_| {
        let _ = sizes.get();
        on_thumb_position_change.run(());
    });

    // Resize observers
    let handle_resize = use_debounce_callback(move || on_resize.run(()), 10);
    use_resize_observer(Signal::derive(move || scrollbar.get()), move || {
        handle_resize.run(())
    });
    use_resize_observer(Signal::derive(move || context.content.get()), move || {
        handle_resize.run(())
    });

    // Provide scrollbar context
    let scrollbar_context = ScrollbarContextValue {
        has_thumb,
        scrollbar,
        on_thumb_change,
        on_thumb_pointer_up,
        on_thumb_pointer_down,
        on_thumb_position_change,
    };

    view! {
        <Provider value=scrollbar_context>
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:class=forwarded_class.get_value()
                attr:data-orientation=data_orientation
                attr:style=move || format!("position: absolute; {}", style.get())
                on:pointerdown=move |event: ev::PointerEvent| {
                    let main_pointer = 0;
                    if event.button() == main_pointer {
                        let element: web_sys::HtmlElement = event.target().unwrap().unchecked_into();
                        element.set_pointer_capture(event.pointer_id()).ok();
                        if let Some(sb) = scrollbar.get_untracked() {
                            rect.set_value(Some(SendWrapper::new(sb.get_bounding_client_rect())));
                        }
                        // Pointer capture doesn't prevent text selection in Safari
                        let document = web_sys::window().unwrap().document().unwrap();
                        let body = document.body().unwrap();
                        prev_webkit_user_select.set_value(
                            body.style().get_property_value("webkitUserSelect").unwrap_or_default()
                        );
                        body.style().set_property("webkitUserSelect", "none").ok();
                        if let Some(viewport) = context.viewport.get_untracked() {
                            viewport.style().set_property("scroll-behavior", "auto").ok();
                        }
                        // Handle drag scroll
                        if let Some(r) = rect.get_value() {
                            let x = event.client_x() as f64 - r.left();
                            let y = event.client_y() as f64 - r.top();
                            on_drag_scroll.run((x, y));
                        }
                    }
                }
                on:pointermove=move |event: ev::PointerEvent| {
                    if let Some(r) = rect.get_value() {
                        let x = event.client_x() as f64 - r.left();
                        let y = event.client_y() as f64 - r.top();
                        on_drag_scroll.run((x, y));
                    }
                }
                on:pointerup=move |event: ev::PointerEvent| {
                    let element: web_sys::HtmlElement = event.target().unwrap().unchecked_into();
                    if element.has_pointer_capture(event.pointer_id()) {
                        element.release_pointer_capture(event.pointer_id()).ok();
                    }
                    let document = web_sys::window().unwrap().document().unwrap();
                    let body = document.body().unwrap();
                    body.style().set_property("webkitUserSelect", &prev_webkit_user_select.get_value()).ok();
                    if let Some(viewport) = context.viewport.get_untracked() {
                        viewport.style().set_property("scroll-behavior", "").ok();
                    }
                    rect.set_value(None);
                }
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaThumb
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaThumb(
    #[prop(into, optional)] force_mount: Option<bool>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let force_mount = force_mount.unwrap_or(false);
    let scrollbar_context = expect_context::<ScrollbarContextValue>();

    // Forward class via context to bypass Presence/Show boundary
    provide_context(ForwardedThumbClass(class.get_untracked()));

    let presence_ref = AnyNodeRef::new();

    view! {
        <AttributeInterceptor let:attrs>
            {view! {
                <Presence present=Signal::derive(move || force_mount || scrollbar_context.has_thumb.get()) node_ref=presence_ref>
                    <ScrollAreaThumbImpl
                        as_child=as_child
                        node_ref=use_composed_refs(vec![node_ref, presence_ref])
                    >
                        {children.with_value(|children| children())}
                    </ScrollAreaThumbImpl>
                </Presence>
            }.add_any_attr(attrs)}
        </AttributeInterceptor>
    }
}

#[component]
fn ScrollAreaThumbImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let scroll_area_context = expect_context::<ScrollAreaContextValue>();
    let scrollbar_context = expect_context::<ScrollbarContextValue>();
    let on_thumb_position_change = scrollbar_context.on_thumb_position_change;

    // Read forwarded class from public ScrollAreaThumb component (StoredValue is Copy,
    // allowing capture by Fn closures inside the view! macro / AttributeInterceptor).
    let forwarded_class = StoredValue::new(
        use_context::<ForwardedThumbClass>()
            .and_then(|c| c.0)
            .unwrap_or_default(),
    );

    let thumb_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, thumb_ref]);

    // Set thumb element in scrollbar context
    Effect::new(move |_| {
        if let Some(node) = thumb_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            scrollbar_context
                .on_thumb_change
                .run(Some(SendWrapper::new(el)));
        }
    });
    Owner::on_cleanup(move || {
        scrollbar_context.on_thumb_change.run(None);
    });

    // Track whether an unlinked scroll listener (rAF loop) is active.
    // We use a StoredValue<bool> flag + a StoredValue for the cancel handle.
    let raf_active = StoredValue::new(false);
    let raf_cancel_id = StoredValue::new(0i32);

    let debounce_scroll_end = use_debounce_callback(
        move || {
            // Cancel the rAF loop when scrolling ends
            if raf_active.get_value() {
                let window = web_sys::window().expect("Window should exist.");
                window
                    .cancel_animation_frame(raf_cancel_id.get_value())
                    .ok();
                raf_active.set_value(false);
            }
        },
        100,
    );

    // Scroll listener on viewport
    Effect::new(move |_| {
        if let Some(viewport) = scroll_area_context.viewport.get() {
            on_thumb_position_change.run(());

            let debounce = debounce_scroll_end;
            let viewport_for_raf = viewport.clone();
            let on_thumb_pos = on_thumb_position_change;

            let scroll_closure = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
                debounce.run(());
                if !raf_active.get_value() {
                    // Start an unlinked scroll listener (rAF loop) that calls
                    // onThumbPositionChange on each frame until debounce stops it.
                    raf_active.set_value(true);
                    on_thumb_pos.run(());

                    let viewport_raf = viewport_for_raf.clone();
                    let prev_left =
                        std::rc::Rc::new(std::cell::Cell::new(viewport_raf.scroll_left() as f64));
                    let prev_top =
                        std::rc::Rc::new(std::cell::Cell::new(viewport_raf.scroll_top() as f64));

                    let closure_holder: RafClosureHolder =
                        std::rc::Rc::new(std::cell::RefCell::new(None));
                    let closure_holder_inner = closure_holder.clone();
                    let window = web_sys::window().expect("Window should exist.");
                    let window_inner = window.clone();

                    let raf_cb = Closure::new(move || {
                        if !raf_active.get_value() {
                            return;
                        }
                        let left = viewport_raf.scroll_left() as f64;
                        let top = viewport_raf.scroll_top() as f64;
                        if prev_left.get() != left || prev_top.get() != top {
                            on_thumb_pos.run(());
                        }
                        prev_left.set(left);
                        prev_top.set(top);
                        if let Some(c) = closure_holder_inner.borrow().as_ref() {
                            let id = window_inner
                                .request_animation_frame(c.as_ref().unchecked_ref())
                                .unwrap_or(0);
                            raf_cancel_id.set_value(id);
                        }
                    });

                    let id = window
                        .request_animation_frame(raf_cb.as_ref().unchecked_ref())
                        .unwrap_or(0);
                    raf_cancel_id.set_value(id);
                    *closure_holder.borrow_mut() = Some(raf_cb);
                }
            }));

            let viewport_target: web_sys::EventTarget =
                <web_sys::HtmlElement as Clone>::clone(&viewport).unchecked_into();
            viewport_target
                .add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref())
                .ok();

            let viewport_target = SendWrapper::new(viewport_target);
            Owner::on_cleanup(move || {
                viewport_target
                    .remove_event_listener_with_callback(
                        "scroll",
                        scroll_closure.as_ref().unchecked_ref(),
                    )
                    .ok();
                // Cancel any active rAF loop
                if raf_active.get_value() {
                    let window = web_sys::window().expect("Window should exist.");
                    window
                        .cancel_animation_frame(raf_cancel_id.get_value())
                        .ok();
                    raf_active.set_value(false);
                }
            });
        }
    });

    let has_thumb = scrollbar_context.has_thumb;
    let on_thumb_pointer_down = scrollbar_context.on_thumb_pointer_down;
    let on_thumb_pointer_up = scrollbar_context.on_thumb_pointer_up;

    view! {
        <AttributeInterceptor let:attrs>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:class=forwarded_class.get_value()
                attr:data-state=move || if has_thumb.get() { "visible" } else { "hidden" }
                attr:style="width: var(--radix-scroll-area-thumb-width); height: var(--radix-scroll-area-thumb-height);"
                on:pointerdown=move |event: ev::PointerEvent| {
                    let thumb: web_sys::HtmlElement = event.target().unwrap().unchecked_into();
                    let thumb_rect = thumb.get_bounding_client_rect();
                    let x = event.client_x() as f64 - thumb_rect.left();
                    let y = event.client_y() as f64 - thumb_rect.top();
                    on_thumb_pointer_down.run((x, y));
                }
                on:pointerup=move |_: ev::PointerEvent| {
                    on_thumb_pointer_up.run(());
                }
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Primitive>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaCorner
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaCorner(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();

    // Forward class via context to bypass Show boundary
    provide_context(ForwardedCornerClass(class.get_untracked()));

    let has_both_scrollbars = Memo::new(move |_| {
        context.scrollbar_x.get().is_some() && context.scrollbar_y.get().is_some()
    });
    let has_corner = Memo::new(move |_| {
        !matches!(context.r#type, ScrollAreaType::Scroll) && has_both_scrollbars.get()
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Show
                when=move || has_corner.get()
                {..attrs}
            >
                <ScrollAreaCornerImpl
                    as_child=as_child
                    node_ref=node_ref
                >
                    {children.with_value(|children| children())}
                </ScrollAreaCornerImpl>
            </Show>
        </AttributeInterceptor>
    }
}

#[component]
fn ScrollAreaCornerImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let context = expect_context::<ScrollAreaContextValue>();

    // Read forwarded class from public ScrollAreaCorner component (StoredValue is Copy,
    // allowing capture by Fn closures inside the view! macro / AttributeInterceptor).
    let forwarded_class = StoredValue::new(
        use_context::<ForwardedCornerClass>()
            .and_then(|c| c.0)
            .unwrap_or_default(),
    );

    let width = RwSignal::new(0.0_f64);
    let height = RwSignal::new(0.0_f64);
    let has_size = Memo::new(move |_| width.get() != 0.0 && height.get() != 0.0);

    use_resize_observer(
        Signal::derive(move || context.scrollbar_x.get()),
        move || {
            if let Some(scrollbar_x) = context.scrollbar_x.get() {
                let h = scrollbar_x.offset_height() as f64;
                context.corner_height.set(h);
                height.set(h);
            }
        },
    );

    use_resize_observer(
        Signal::derive(move || context.scrollbar_y.get()),
        move || {
            if let Some(scrollbar_y) = context.scrollbar_y.get() {
                let w = scrollbar_y.offset_width() as f64;
                context.corner_width.set(w);
                width.set(w);
            }
        },
    );

    let dir = context.dir;

    view! {
        <AttributeInterceptor let:attrs>
            <Show
                when=move || has_size.get()
                {..attrs}
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    attr:class=forwarded_class.get_value()
                    attr:style=move || {
                        let d = dir.get();
                        format!(
                            "width: {}px; height: {}px; position: absolute; {}: 0; bottom: 0;",
                            width.get(),
                            height.get(),
                            if d == Direction::Ltr { "right" } else { "left" }
                        )
                    }
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </Show>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Re-exports
 * -----------------------------------------------------------------------------------------------*/

pub use ScrollArea as Root;
pub use ScrollAreaCorner as Corner;
pub use ScrollAreaScrollbar as Scrollbar;
pub use ScrollAreaThumb as Thumb;
pub use ScrollAreaViewport as Viewport;
