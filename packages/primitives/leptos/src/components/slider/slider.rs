use super::*;

/// Root slider component.
///
/// Renders as a `<span>`. Manages value state, step snapping, and
/// provides context for [`SliderTrack`], [`SliderRange`], and [`SliderThumb`].
#[component]
pub fn Slider(
    #[prop(into, optional)] name: MaybeProp<String>,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] min: MaybeProp<f64>,
    #[prop(into, optional)] max: MaybeProp<f64>,
    #[prop(into, optional)] step: MaybeProp<f64>,
    /// Minimum number of steps between multiple thumbs. Prevents thumbs
    /// from overlapping.
    #[prop(into, optional)]
    min_steps_between_thumbs: MaybeProp<f64>,
    #[prop(into, optional)] value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] default_value: MaybeProp<Vec<f64>>,
    #[prop(into, optional)] on_value_change: Option<Callback<Vec<f64>>>,
    /// Called when the user finishes dragging (pointer up). Receives the
    /// final committed values.
    #[prop(into, optional)]
    on_value_commit: Option<Callback<Vec<f64>>>,
    #[prop(into, optional)] inverted: MaybeProp<bool>,
    #[prop(into, optional)] form: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let min_signal = prop_or(min, 0.0);
    let max_signal = prop_or(max, 100.0);
    let step_signal = prop_or(step, 1.0);
    let disabled_signal = prop_or_default(disabled);
    let orientation_signal = prop_or_default(orientation);
    let min_steps_between_thumbs_signal = prop_or(min_steps_between_thumbs, 0.0);
    let inverted_signal = prop_or_default(inverted);

    let thumbs: RwSignal<Vec<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(Vec::new());
    let value_index_to_change: RwSignal<usize> = RwSignal::new(0);

    let (current_values, set_values) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: MaybeProp::derive(move || {
            let default = default_value.get();
            Some(default.unwrap_or_else(|| vec![min_signal.get()]))
        }),
        on_change: Some(Callback::new(move |value: Option<Vec<f64>>| {
            if let Some(value) = value {
                let thumbs_snapshot = thumbs.get();
                let idx = value_index_to_change.get_untracked();
                if let Some(thumb) = thumbs_snapshot.get(idx) {
                    let _ = thumb.focus();
                }
                if let Some(on_value_change) = on_value_change {
                    on_value_change.run(value);
                }
            }
        })),
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
            attr:data-disabled=data_attr(disabled)
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
