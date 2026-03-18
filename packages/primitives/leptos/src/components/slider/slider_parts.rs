use super::*;

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
            attr:data-disabled=data_attr(context.disabled)
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
            attr:data-disabled=data_attr(context.disabled)
            style:left=move || (orientation.get().start_edge == "left").then(|| format!("{}%", offset_start.get()))
            style:right=move || (orientation.get().end_edge == "right").then(|| format!("{}%", offset_end.get()))
            style:bottom=move || (orientation.get().start_edge == "bottom").then(|| format!("{}%", offset_start.get()))
            style:top=move || (orientation.get().end_edge == "top").then(|| format!("{}%", offset_end.get()))
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

    // Default to true when ref is unavailable (SSR) so the hidden input renders
    // and form events bubble without JS. Matches React's approach.
    let is_form_control = Memo::new(move |_| {
        let form_attr = context.form.get();
        if form_attr.is_some() {
            return true;
        }
        match thumb_ref.get() {
            Some(el) => {
                let el: &web_sys::Element = (*el).unchecked_ref();
                el.closest("form").ok().flatten().is_some()
            }
            None => true,
        }
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
                attr:data-disabled=data_attr(context.disabled)
                attr:tabindex=move || if context.disabled.get() { None } else { Some("0") }
                style:transform="var(--radix-slider-thumb-transform)"
                style:position="absolute"
                style:left=move || (orientation.get().start_edge == "left").then(|| format!("calc({}% + {}px)", percent.get(), thumb_in_bounds_offset.get()))
                style:right=move || (orientation.get().start_edge == "right").then(|| format!("calc({}% + {}px)", percent.get(), thumb_in_bounds_offset.get()))
                style:top=move || (orientation.get().start_edge == "top").then(|| format!("calc({}% + {}px)", percent.get(), thumb_in_bounds_offset.get()))
                style:bottom=move || (orientation.get().start_edge == "bottom").then(|| format!("calc({}% + {}px)", percent.get(), thumb_in_bounds_offset.get()))
                // Hide when there is no value for this thumb. This covers both SSR
                // (index=-1, value=None) and runtime (thumb exists but no corresponding
                // value). Matches React's `value === undefined ? { display: 'none' }`.
                style:display=move || value.get().is_none().then_some("none")
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
        // NOTE: We intentionally avoid type="hidden" here — hidden inputs are not
        // included in the form's elements list and cannot be accessed via FormData.
        // We use display:none instead to hide the input visually while preserving
        // form participation (matching the React reference approach).
        <input
            node_ref=input_ref
            name=move || name.get()
            form=move || form.get()
            value=move || value.get().map(|v| v.to_string()).unwrap_or_default()
            style:display="none"
        />
    }
}
