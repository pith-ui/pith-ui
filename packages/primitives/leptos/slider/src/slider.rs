use std::marker::PhantomData;

use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_primitive::{Primitive, compose_callbacks};
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use radix_leptos_use_previous::use_previous;
use radix_leptos_use_size::use_size;
use radix_number::clamp;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

const PAGE_KEYS: [&str; 2] = ["PageUp", "PageDown"];
const ARROW_KEYS: [&str; 4] = ["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];

type SlideDirection = &'static str;
const FROM_LEFT: SlideDirection = "from-left";
const FROM_RIGHT: SlideDirection = "from-right";
const FROM_BOTTOM: SlideDirection = "from-bottom";
const FROM_TOP: SlideDirection = "from-top";

fn back_keys(direction: SlideDirection) -> &'static [&'static str] {
    match direction {
        FROM_LEFT => &["Home", "PageDown", "ArrowDown", "ArrowLeft"],
        FROM_RIGHT => &["Home", "PageDown", "ArrowDown", "ArrowRight"],
        FROM_BOTTOM => &["Home", "PageDown", "ArrowDown", "ArrowLeft"],
        FROM_TOP => &["Home", "PageDown", "ArrowUp", "ArrowLeft"],
        _ => &[],
    }
}

/* -------------------------------------------------------------------------------------------------
 * Slider
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Default, Debug, PartialEq)]
struct ItemData;

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[derive(Clone)]
struct SliderContextValue {
    name: Signal<Option<String>>,
    disabled: Signal<bool>,
    min: Signal<f64>,
    max: Signal<f64>,
    values: Signal<Vec<f64>>,
    value_index_to_change: RwSignal<usize>,
    thumbs: RwSignal<Vec<SendWrapper<web_sys::HtmlElement>>>,
    orientation: Signal<Orientation>,
    form: Signal<Option<String>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Horizontal => write!(f, "horizontal"),
            Orientation::Vertical => write!(f, "vertical"),
        }
    }
}

#[component]
pub fn Slider(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    #[prop(into, optional)] min_steps_between_thumbs: MaybeProp<f64>,
    #[prop(into, optional)] value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] on_value_commit: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] inverted: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let min_signal = Signal::derive(move || min.get().unwrap_or(0.0));
    let max_signal = Signal::derive(move || max.get().unwrap_or(100.0));
    let step_signal = Signal::derive(move || step.get().unwrap_or(1.0));
    let disabled_signal = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation_signal =
        Signal::derive(move || orientation.get().unwrap_or(Orientation::Horizontal));
    let min_steps_between_thumbs_signal =
        Signal::derive(move || min_steps_between_thumbs.get().unwrap_or(0.0));
    let inverted_signal = Signal::derive(move || inverted.get().unwrap_or(false));

    let thumbs: RwSignal<Vec<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(Vec::new());
    let value_index_to_change: RwSignal<usize> = RwSignal::new(0);

    let (current_values, set_values) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: MaybeProp::derive(move || {
            let default = default_value.get();
            Some(default.unwrap_or_else(|| vec![min_signal.get()]))
        }),
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<Vec<f64>>| {
                if let Some(value) = value {
                    // Focus the thumb that changed
                    let thumbs_snapshot = thumbs.get();
                    let idx = value_index_to_change.get_untracked();
                    if let Some(thumb) = thumbs_snapshot.get(idx) {
                        let _ = thumb.focus();
                    }
                    on_value_change.run(value);
                }
            })
        }),
    });

    let values = Signal::derive(move || current_values.get().unwrap_or_default());

    let values_before_slide_start: StoredValue<Vec<f64>> = StoredValue::new(vec![]);

    let update_values = StoredValue::new(SendWrapper::new({
        move |value: f64, at_index: usize, commit: bool| {
            let step_val = step_signal.get_untracked();
            let min_val = min_signal.get_untracked();
            let max_val = max_signal.get_untracked();
            let min_steps = min_steps_between_thumbs_signal.get_untracked();

            let decimal_count = get_decimal_count(step_val);
            let snap_to_step = round_value(
                ((value - min_val) / step_val).round() * step_val + min_val,
                decimal_count,
            );
            let next_value = clamp(snap_to_step, [min_val, max_val]);

            let prev_values = current_values.get_untracked().unwrap_or_default();
            let next_values = get_next_sorted_values(&prev_values, next_value, at_index);

            if has_min_steps_between_values(&next_values, min_steps * step_val) {
                let new_idx = next_values
                    .iter()
                    .position(|&v| v == next_value)
                    .unwrap_or(at_index);
                value_index_to_change.set(new_idx);
                let has_changed = next_values != prev_values;
                if has_changed {
                    if commit && let Some(on_value_commit) = on_value_commit {
                        on_value_commit.run(next_values.clone());
                    }
                    set_values.run(Some(next_values));
                }
            }
        }
    }));

    let handle_slide_start = move |value: f64| {
        let vals = values.get_untracked();
        let closest_index = get_closest_value_index(&vals, value);
        update_values.with_value(|f| f(value, closest_index, false));
    };

    let handle_slide_move = move |value: f64| {
        let idx = value_index_to_change.get_untracked();
        update_values.with_value(|f| f(value, idx, false));
    };

    let handle_slide_end = move || {
        let prev_values = values_before_slide_start.get_value();
        let idx = value_index_to_change.get_untracked();
        let prev_value = prev_values.get(idx).copied();
        let current = values.get_untracked();
        let next_value = current.get(idx).copied();
        if prev_value != next_value
            && let Some(on_value_commit) = on_value_commit
        {
            on_value_commit.run(current);
        }
    };

    let context_value = SliderContextValue {
        name: Signal::derive(move || name.get()),
        disabled: disabled_signal,
        min: min_signal,
        max: max_signal,
        values,
        value_index_to_change,
        thumbs,
        orientation: orientation_signal,
        form: Signal::derive(move || form.get()),
    };

    let is_horizontal = Signal::derive(move || orientation_signal.get() == Orientation::Horizontal);

    let on_pointer_down_for_start = move |_: ev::PointerEvent| {
        if !disabled_signal.get_untracked() {
            values_before_slide_start.set_value(values.get_untracked());
        }
    };

    let on_home_key_down = {
        move || {
            if !disabled_signal.get_untracked() {
                let min_val = min_signal.get_untracked();
                update_values.with_value(|f| f(min_val, 0, true));
            }
        }
    };

    let on_end_key_down = {
        move || {
            if !disabled_signal.get_untracked() {
                let max_val = max_signal.get_untracked();
                let len = values.get_untracked().len();
                if len > 0 {
                    update_values.with_value(|f| f(max_val, len - 1, true));
                }
            }
        }
    };

    let on_step_key_down = {
        move |event: ev::KeyboardEvent, direction: f64| {
            if !disabled_signal.get_untracked() {
                let is_page_key = PAGE_KEYS.contains(&event.key().as_str());
                let is_skip_key = is_page_key
                    || (event.shift_key() && ARROW_KEYS.contains(&event.key().as_str()));
                let multiplier: f64 = if is_skip_key { 10.0 } else { 1.0 };
                let at_index = value_index_to_change.get_untracked();
                let vals = values.get_untracked();
                if let Some(&current_value) = vals.get(at_index) {
                    let step_in_direction = step_signal.get_untracked() * multiplier * direction;
                    update_values
                        .with_value(|f| f(current_value + step_in_direction, at_index, true));
                }
            }
        }
    };

    view! {
        <Provider value=context_value>
            <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                <CollectionSlot item_data_type=ITEM_DATA_PHANTOM node_ref=node_ref>
                    {move || {
                        if is_horizontal.get() {
                            view! {
                                <SliderHorizontal
                                    min=min_signal
                                    max=max_signal
                                    inverted=inverted_signal
                                    dir=dir
                                    disabled=disabled_signal
                                    on_slide_start=Callback::new(handle_slide_start)
                                    on_slide_move=Callback::new(handle_slide_move)
                                    on_slide_end=Callback::new(move |_: ()| handle_slide_end())
                                    on_home_key_down=Callback::new(move |_: ()| on_home_key_down())
                                    on_end_key_down=Callback::new(move |_: ()| on_end_key_down())
                                    on_step_key_down=Callback::new(move |(event, direction): (ev::KeyboardEvent, f64)| on_step_key_down(event, direction))
                                    on_pointer_down=Callback::new(on_pointer_down_for_start)
                                    as_child=as_child
                                >
                                    {children.with_value(|children| children())}
                                </SliderHorizontal>
                            }.into_any()
                        } else {
                            view! {
                                <SliderVertical
                                    min=min_signal
                                    max=max_signal
                                    inverted=inverted_signal
                                    disabled=disabled_signal
                                    on_slide_start=Callback::new(handle_slide_start)
                                    on_slide_move=Callback::new(handle_slide_move)
                                    on_slide_end=Callback::new(move |_: ()| handle_slide_end())
                                    on_home_key_down=Callback::new(move |_: ()| on_home_key_down())
                                    on_end_key_down=Callback::new(move |_: ()| on_end_key_down())
                                    on_step_key_down=Callback::new(move |(event, direction): (ev::KeyboardEvent, f64)| on_step_key_down(event, direction))
                                    on_pointer_down=Callback::new(on_pointer_down_for_start)
                                    as_child=as_child
                                >
                                    {children.with_value(|children| children())}
                                </SliderVertical>
                            }.into_any()
                        }
                    }}
                </CollectionSlot>
            </CollectionProvider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderOrientationContext
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug)]
struct SliderOrientationContextValue {
    start_edge: &'static str,
    end_edge: &'static str,
    size: OrientationSize,
    direction: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum OrientationSize {
    Width,
    Height,
}

/* -------------------------------------------------------------------------------------------------
 * SliderHorizontal
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SliderHorizontal(
    #[prop(into)] min: Signal<f64>,
    #[prop(into)] max: Signal<f64>,
    #[prop(into)] inverted: Signal<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into)] disabled: Signal<bool>,
    on_slide_start: Callback<f64>,
    on_slide_move: Callback<f64>,
    on_slide_end: Callback<()>,
    on_home_key_down: Callback<()>,
    on_end_key_down: Callback<()>,
    on_step_key_down: Callback<(ev::KeyboardEvent, f64)>,
    on_pointer_down: Callback<ev::PointerEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let slider_ref = AnyNodeRef::new();
    let rect_ref: StoredValue<Option<SendWrapper<web_sys::DomRect>>> = StoredValue::new(None);
    let direction = use_direction(dir);

    let is_sliding_from_left = Signal::derive(move || {
        let is_ltr = direction.get() == Direction::Ltr;
        let inv = inverted.get();
        (is_ltr && !inv) || (!is_ltr && inv)
    });

    let get_value_from_pointer = move |pointer_position: f64| -> f64 {
        let cached_rect = rect_ref.get_value();
        let rect = cached_rect.unwrap_or_else(|| {
            let el = slider_ref.get().expect("Slider element should exist");
            let el: &web_sys::Element = (*el).unchecked_ref();
            SendWrapper::new(el.get_bounding_client_rect())
        });
        let input = [0.0, rect.width()];
        let output = if is_sliding_from_left.get_untracked() {
            [min.get_untracked(), max.get_untracked()]
        } else {
            [max.get_untracked(), min.get_untracked()]
        };
        let scale = linear_scale(input, output);
        rect_ref.set_value(Some(rect.clone()));
        scale(pointer_position - rect.left())
    };

    let orientation_context = Signal::derive(move || {
        let from_left = is_sliding_from_left.get();
        SliderOrientationContextValue {
            start_edge: if from_left { "left" } else { "right" },
            end_edge: if from_left { "right" } else { "left" },
            direction: if from_left { 1.0 } else { -1.0 },
            size: OrientationSize::Width,
        }
    });

    view! {
        <Provider value=orientation_context>
            <SliderImpl
                node_ref=slider_ref
                data_orientation="horizontal"
                dir=direction
                disabled=disabled
                thumb_transform="translateX(-50%)"
                on_slide_start=Callback::new(move |event: ev::PointerEvent| {
                    let value = get_value_from_pointer(event.client_x() as f64);
                    on_slide_start.run(value);
                })
                on_slide_move=Callback::new(move |event: ev::PointerEvent| {
                    let value = get_value_from_pointer(event.client_x() as f64);
                    on_slide_move.run(value);
                })
                on_slide_end=Callback::new(move |_: ev::PointerEvent| {
                    rect_ref.set_value(None);
                    on_slide_end.run(());
                })
                on_home_key_down=on_home_key_down
                on_end_key_down=on_end_key_down
                on_step_key_down=Callback::new(move |event: ev::KeyboardEvent| {
                    let slide_direction = if is_sliding_from_left.get_untracked() {
                        FROM_LEFT
                    } else {
                        FROM_RIGHT
                    };
                    let is_back_key = back_keys(slide_direction).contains(&event.key().as_str());
                    let direction = if is_back_key { -1.0 } else { 1.0 };
                    on_step_key_down.run((event, direction));
                })
                on_pointer_down=on_pointer_down
                as_child=as_child
            >
                {children.with_value(|children| children())}
            </SliderImpl>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderVertical
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SliderVertical(
    #[prop(into)] min: Signal<f64>,
    #[prop(into)] max: Signal<f64>,
    #[prop(into)] inverted: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    on_slide_start: Callback<f64>,
    on_slide_move: Callback<f64>,
    on_slide_end: Callback<()>,
    on_home_key_down: Callback<()>,
    on_end_key_down: Callback<()>,
    on_step_key_down: Callback<(ev::KeyboardEvent, f64)>,
    on_pointer_down: Callback<ev::PointerEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let slider_ref = AnyNodeRef::new();
    let rect_ref: StoredValue<Option<SendWrapper<web_sys::DomRect>>> = StoredValue::new(None);

    let is_sliding_from_bottom = Signal::derive(move || !inverted.get());

    let get_value_from_pointer = move |pointer_position: f64| -> f64 {
        let cached_rect = rect_ref.get_value();
        let rect = cached_rect.unwrap_or_else(|| {
            let el = slider_ref.get().expect("Slider element should exist");
            let el: &web_sys::Element = (*el).unchecked_ref();
            SendWrapper::new(el.get_bounding_client_rect())
        });
        let input = [0.0, rect.height()];
        let output = if is_sliding_from_bottom.get_untracked() {
            [max.get_untracked(), min.get_untracked()]
        } else {
            [min.get_untracked(), max.get_untracked()]
        };
        let scale = linear_scale(input, output);
        rect_ref.set_value(Some(rect.clone()));
        scale(pointer_position - rect.top())
    };

    let orientation_context = Signal::derive(move || {
        let from_bottom = is_sliding_from_bottom.get();
        SliderOrientationContextValue {
            start_edge: if from_bottom { "bottom" } else { "top" },
            end_edge: if from_bottom { "top" } else { "bottom" },
            direction: if from_bottom { 1.0 } else { -1.0 },
            size: OrientationSize::Height,
        }
    });

    view! {
        <Provider value=orientation_context>
            <SliderImpl
                node_ref=slider_ref
                data_orientation="vertical"
                disabled=disabled
                thumb_transform="translateY(50%)"
                on_slide_start=Callback::new(move |event: ev::PointerEvent| {
                    let value = get_value_from_pointer(event.client_y() as f64);
                    on_slide_start.run(value);
                })
                on_slide_move=Callback::new(move |event: ev::PointerEvent| {
                    let value = get_value_from_pointer(event.client_y() as f64);
                    on_slide_move.run(value);
                })
                on_slide_end=Callback::new(move |_: ev::PointerEvent| {
                    rect_ref.set_value(None);
                    on_slide_end.run(());
                })
                on_home_key_down=on_home_key_down
                on_end_key_down=on_end_key_down
                on_step_key_down=Callback::new(move |event: ev::KeyboardEvent| {
                    let slide_direction = if is_sliding_from_bottom.get_untracked() {
                        FROM_BOTTOM
                    } else {
                        FROM_TOP
                    };
                    let is_back_key = back_keys(slide_direction).contains(&event.key().as_str());
                    let direction = if is_back_key { -1.0 } else { 1.0 };
                    on_step_key_down.run((event, direction));
                })
                on_pointer_down=on_pointer_down
                as_child=as_child
            >
                {children.with_value(|children| children())}
            </SliderImpl>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SliderImpl(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    data_orientation: &'static str,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into)] disabled: Signal<bool>,
    thumb_transform: &'static str,
    on_slide_start: Callback<ev::PointerEvent>,
    on_slide_move: Callback<ev::PointerEvent>,
    on_slide_end: Callback<ev::PointerEvent>,
    on_home_key_down: Callback<()>,
    on_end_key_down: Callback<()>,
    on_step_key_down: Callback<ev::KeyboardEvent>,
    on_pointer_down: Callback<ev::PointerEvent>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SliderContextValue>();

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:aria-disabled=move || disabled.get().then_some("true")
            attr:data-disabled=move || disabled.get().then_some("")
            attr:data-orientation=data_orientation
            attr:dir=move || dir.get().map(|d| d.to_string())
            style:--radix-slider-thumb-transform=thumb_transform
            on:keydown=move |event: ev::KeyboardEvent| {
                let key = event.key();
                if key == "Home" {
                    on_home_key_down.run(());
                    event.prevent_default();
                } else if key == "End" {
                    on_end_key_down.run(());
                    event.prevent_default();
                } else if PAGE_KEYS.contains(&key.as_str()) || ARROW_KEYS.contains(&key.as_str()) {
                    on_step_key_down.run(event.clone());
                    event.prevent_default();
                }
            }
            on:pointerdown=compose_callbacks(
                None::<Callback<ev::PointerEvent>>,
                Some(Callback::new(move |event: ev::PointerEvent| {
                    on_pointer_down.run(event.clone());
                    let target = event.target().expect("Event should have target");
                    let target: web_sys::HtmlElement = target.unchecked_into();
                    target.set_pointer_capture(event.pointer_id()).ok();
                    // Prevent browser focus behaviour because we focus a thumb manually when values change.
                    event.prevent_default();
                    // Touch devices have a delay before focusing so won't focus if touch immediately moves
                    // away from target (sliding). We want thumb to focus regardless.
                    let thumbs = context.thumbs.get_untracked();
                    let target_is_thumb = thumbs.iter().any(|t| {
                        let t_node: &web_sys::Node = t.unchecked_ref();
                        let target_node: &web_sys::Node = target.unchecked_ref();
                        t_node.is_same_node(Some(target_node))
                    });
                    if target_is_thumb {
                        let _ = target.focus();
                    } else {
                        on_slide_start.run(event);
                    }
                })),
                None,
            )
            on:pointermove=move |event: ev::PointerEvent| {
                let target = event.target().expect("Event should have target");
                let target: web_sys::HtmlElement = target.unchecked_into();
                if target.has_pointer_capture(event.pointer_id()) {
                    on_slide_move.run(event);
                }
            }
            on:pointerup=move |event: ev::PointerEvent| {
                let target = event.target().expect("Event should have target");
                let target: web_sys::HtmlElement = target.unchecked_into();
                if target.has_pointer_capture(event.pointer_id()) {
                    target.release_pointer_capture(event.pointer_id()).ok();
                    on_slide_end.run(event);
                }
            }
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderTrack
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SliderTrack(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SliderContextValue>();

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-disabled=move || context.disabled.get().then_some("")
            attr:data-orientation=move || context.orientation.get().to_string()
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderRange
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SliderRange(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SliderContextValue>();
    let orientation = expect_context::<Signal<SliderOrientationContextValue>>();

    let percentages = Memo::new(move |_| {
        let vals = context.values.get();
        let min = context.min.get();
        let max = context.max.get();
        vals.iter()
            .map(|&v| convert_value_to_percentage(v, min, max))
            .collect::<Vec<_>>()
    });

    let offset_start = Memo::new(move |_| {
        let pcts = percentages.get();
        if pcts.len() > 1 {
            pcts.iter().cloned().fold(f64::INFINITY, f64::min)
        } else {
            0.0
        }
    });

    let offset_end = Memo::new(move |_| {
        let pcts = percentages.get();
        let max_pct = pcts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        100.0 - max_pct
    });

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=move || context.orientation.get().to_string()
            attr:data-disabled=move || context.disabled.get().then_some("")
            attr:style=move || {
                let orient = orientation.get();
                format!(
                    "{}: {}%; {}: {}%;",
                    orient.start_edge,
                    offset_start.get(),
                    orient.end_edge,
                    offset_end.get(),
                )
            }
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderThumb
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn SliderThumb(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let get_items = use_collection::<ItemData>();
    let thumb_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, thumb_ref]);

    let index = Memo::new(move |_| {
        let thumb_node = thumb_ref.get();
        if let Some(thumb_node) = thumb_node {
            let items = get_items();
            items
                .iter()
                .position(|item| {
                    item.r#ref
                        .get()
                        .map(|el| {
                            let item_node: &web_sys::Node = (*el).unchecked_ref();
                            let thumb: &web_sys::Node = (*thumb_node).unchecked_ref();
                            item_node.is_same_node(Some(thumb))
                        })
                        .unwrap_or(false)
                })
                .map(|i| i as i32)
                .unwrap_or(-1)
        } else {
            -1
        }
    });

    view! {
        <SliderThumbImpl
            index=index
            name=name
            as_child=as_child
            node_ref=composed_refs
        >
            {children.with_value(|children| children.as_ref().map(|children| children()))}
        </SliderThumbImpl>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderThumbImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SliderThumbImpl(
    #[prop(into)] index: Memo<i32>,
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SliderContextValue>();
    let orientation = expect_context::<Signal<SliderOrientationContextValue>>();

    let thumb_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, thumb_ref]);

    let size = use_size(thumb_ref);

    let value = Memo::new(move |_| {
        let idx = index.get();
        if idx >= 0 {
            context.values.get().get(idx as usize).copied()
        } else {
            None
        }
    });

    let percent = Memo::new(move |_| {
        value
            .get()
            .map(|v| convert_value_to_percentage(v, context.min.get(), context.max.get()))
            .unwrap_or(0.0)
    });

    let label = Memo::new(move |_| {
        let idx = index.get();
        let total = context.values.get().len();
        if idx >= 0 {
            get_label(idx as usize, total)
        } else {
            None
        }
    });

    let orientation_size = Memo::new(move |_| {
        size.get().map(|s| match orientation.get().size {
            OrientationSize::Width => s.width,
            OrientationSize::Height => s.height,
        })
    });

    let thumb_in_bounds_offset = Memo::new(move |_| {
        orientation_size
            .get()
            .map(|s| get_thumb_in_bounds_offset(s, percent.get(), orientation.get().direction))
            .unwrap_or(0.0)
    });

    // Register/unregister thumb in context
    Effect::new(move |_| {
        if let Some(thumb_el) = thumb_ref.get() {
            let html_el: web_sys::HtmlElement = thumb_el.unchecked_into();
            let wrapped = SendWrapper::new(html_el.clone());
            context.thumbs.update(|vec| {
                // Avoid duplicates
                let already_exists = vec.iter().any(|t| {
                    let t_node: &web_sys::Node = t.unchecked_ref();
                    let el_node: &web_sys::Node = html_el.unchecked_ref();
                    t_node.is_same_node(Some(el_node))
                });
                if !already_exists {
                    vec.push(wrapped);
                }
            });

            Owner::on_cleanup({
                let html_el = SendWrapper::new(html_el);
                move || {
                    context.thumbs.update(|vec| {
                        vec.retain(|t| {
                            let t_node: &web_sys::Node = t.unchecked_ref();
                            let el_node: &web_sys::Node = html_el.unchecked_ref();
                            !t_node.is_same_node(Some(el_node))
                        });
                    });
                }
            });
        }
    });

    // Determine if this thumb is inside a form for bubble input rendering
    let is_form_control = Memo::new(move |_| {
        let form_attr = context.form.get();
        if form_attr.is_some() {
            return true;
        }
        thumb_ref
            .get()
            .and_then(|el| {
                let el: &web_sys::Element = (*el).unchecked_ref();
                el.closest("form").ok().flatten()
            })
            .is_some()
    });

    let computed_name = Memo::new(move |_| {
        if let Some(thumb_name) = name.get() {
            Some(thumb_name)
        } else {
            context.name.get().map(|n| {
                if context.values.get().len() > 1 {
                    format!("{}[]", n)
                } else {
                    n
                }
            })
        }
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=Signal::derive(|| ItemData)>
            <Primitive
                element=html::span
                as_child=as_child
                node_ref=composed_ref
                attr:role="slider"
                attr:aria-label=move || label.get()
                attr:aria-valuemin=move || context.min.get()
                attr:aria-valuenow=move || value.get()
                attr:aria-valuemax=move || context.max.get()
                attr:aria-orientation=move || context.orientation.get().to_string()
                attr:data-orientation=move || context.orientation.get().to_string()
                attr:data-disabled=move || context.disabled.get().then_some("")
                attr:tabindex=move || if context.disabled.get() { None } else { Some("0") }
                attr:style=move || {
                    let orient = orientation.get();
                    let mut style = format!(
                        "transform: var(--radix-slider-thumb-transform); position: absolute; {}: calc({}% + {}px);",
                        orient.start_edge,
                        percent.get(),
                        thumb_in_bounds_offset.get(),
                    );
                    // Only hide when we have a valid index but no value at that position.
                    // Don't hide when index is -1 (initial render before mount/indexing).
                    if index.get() >= 0 && value.get().is_none() {
                        style.push_str(" display: none;");
                    }
                    style
                }
                on:focus=move |_: ev::FocusEvent| {
                    let idx = index.get();
                    if idx >= 0 {
                        context.value_index_to_change.set(idx as usize);
                    }
                }
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
                <Show when=move || is_form_control.get()>
                    <SliderBubbleInput
                        name=Signal::derive(move || computed_name.get())
                        form=Signal::derive(move || context.form.get())
                        value=Signal::derive(move || value.get())
                    />
                </Show>
            </Primitive>
        </CollectionItemSlot>
    }
}

/* -------------------------------------------------------------------------------------------------
 * SliderBubbleInput
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SliderBubbleInput(
    #[prop(into)] name: Signal<Option<String>>,
    #[prop(into)] form: Signal<Option<String>>,
    #[prop(into)] value: Signal<Option<f64>>,
) -> impl IntoView {
    let input_ref = AnyNodeRef::new();
    let prev_value = use_previous(value);

    // Bubble value change to parents (e.g. form change event)
    Effect::new(move |_| {
        if let Some(input_node) = input_ref.get() {
            let input: &web_sys::HtmlInputElement = input_node.unchecked_ref();
            let prev = prev_value.get();
            let current = value.get();

            if prev != current
                && let Some(val) = current
            {
                let val_str = val.to_string();
                input.set_value(&val_str);

                let init = web_sys::EventInit::new();
                init.set_bubbles(true);
                let event = web_sys::Event::new_with_event_init_dict("input", &init)
                    .expect("Input event should be instantiated.");
                input
                    .dispatch_event(&event)
                    .expect("Input event should be dispatched.");
            }
        }
    });

    view! {
        <input
            node_ref=input_ref
            type="hidden"
            name=move || name.get()
            form=move || form.get()
            value=move || value.get().map(|v| v.to_string()).unwrap_or_default()
            style:display="none"
        />
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utility functions
 * -----------------------------------------------------------------------------------------------*/

fn get_next_sorted_values(prev_values: &[f64], next_value: f64, at_index: usize) -> Vec<f64> {
    let mut next_values = prev_values.to_vec();
    if at_index < next_values.len() {
        next_values[at_index] = next_value;
    }
    next_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    next_values
}

fn convert_value_to_percentage(value: f64, min: f64, max: f64) -> f64 {
    let max_steps = max - min;
    let percent_per_step = 100.0 / max_steps;
    let percentage = percent_per_step * (value - min);
    clamp(percentage, [0.0, 100.0])
}

fn get_label(index: usize, total_values: usize) -> Option<String> {
    if total_values > 2 {
        Some(format!("Value {} of {}", index + 1, total_values))
    } else if total_values == 2 {
        ["Minimum", "Maximum"].get(index).map(|s| s.to_string())
    } else {
        None
    }
}

fn get_closest_value_index(values: &[f64], next_value: f64) -> usize {
    if values.len() == 1 {
        return 0;
    }
    let distances: Vec<f64> = values.iter().map(|v| (v - next_value).abs()).collect();
    let closest_distance = distances.iter().cloned().fold(f64::INFINITY, f64::min);
    distances
        .iter()
        .position(|&d| d == closest_distance)
        .unwrap_or(0)
}

fn get_thumb_in_bounds_offset(width: f64, left: f64, direction: f64) -> f64 {
    let half_width = width / 2.0;
    let half_percent = 50.0;
    let offset = linear_scale([0.0, half_percent], [0.0, half_width]);
    (half_width - offset(left) * direction) * direction
}

fn get_steps_between_values(values: &[f64]) -> Vec<f64> {
    values.windows(2).map(|w| w[1] - w[0]).collect()
}

fn has_min_steps_between_values(values: &[f64], min_steps_between_values: f64) -> bool {
    if min_steps_between_values > 0.0 {
        let steps_between = get_steps_between_values(values);
        let actual_min = steps_between.iter().cloned().fold(f64::INFINITY, f64::min);
        actual_min >= min_steps_between_values
    } else {
        true
    }
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

fn get_decimal_count(value: f64) -> u32 {
    let s = value.to_string();
    if let Some(dot_pos) = s.find('.') {
        (s.len() - dot_pos - 1) as u32
    } else {
        0
    }
}

fn round_value(value: f64, decimal_count: u32) -> f64 {
    let rounder = 10f64.powi(decimal_count as i32);
    (value * rounder).round() / rounder
}
