use super::*;

/* -------------------------------------------------------------------------------------------------
 * AccordionSingle
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AccordionSingle(
    /// The controlled value of the open item.
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The default open item value.
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Callback when value changes.
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Whether an item can be collapsed after opening. Default false.
    #[prop(into, optional)]
    collapsible: MaybeProp<bool>,

    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (value_context, collapsible_context) =
        create_single_contexts(value, default_value, on_value_change, collapsible);

    view! {
        <Provider value=value_context>
            <Provider value=collapsible_context>
                <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                    <AccordionImpl
                        disabled=disabled
                        dir=dir
                        orientation=orientation
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </AccordionImpl>
                </CollectionProvider>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionMultiple
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AccordionMultiple(
    /// The controlled values of open items.
    #[prop(into, optional)]
    values: MaybeProp<Vec<String>>,
    /// The default open item values.
    #[prop(into, optional)]
    default_values: MaybeProp<Vec<String>>,
    /// Callback when values change.
    #[prop(into, optional)]
    on_values_change: Option<Callback<Vec<String>>>,

    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (value_context, collapsible_context) =
        create_multiple_contexts(values, default_values, on_values_change);

    view! {
        <Provider value=value_context>
            <Provider value=collapsible_context>
                <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                    <AccordionImpl
                        disabled=disabled
                        dir=dir
                        orientation=orientation
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </AccordionImpl>
                </CollectionProvider>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Accordion (convenience wrapper — delegates to AccordionSingle or AccordionMultiple)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn Accordion(
    /// The type of accordion: single or multiple open items.
    r#type: AccordionType,

    // -- Single mode props --
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] default_value: MaybeProp<String>,
    #[prop(into, optional)] on_value_change: Option<Callback<String>>,
    #[prop(into, optional)] collapsible: MaybeProp<bool>,

    // -- Multiple mode props --
    #[prop(into, optional)] values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] default_values: MaybeProp<Vec<String>>,
    #[prop(into, optional)] on_values_change: Option<Callback<Vec<String>>>,

    // -- Shared props --
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let (value_context, collapsible_context) = match r#type {
        AccordionType::Single => {
            create_single_contexts(value, default_value, on_value_change, collapsible)
        }
        AccordionType::Multiple => {
            create_multiple_contexts(values, default_values, on_values_change)
        }
    };

    view! {
        <Provider value=value_context>
            <Provider value=collapsible_context>
                <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
                    <AccordionImpl
                        disabled=disabled
                        dir=dir
                        orientation=orientation
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.with_value(|children| children())}
                    </AccordionImpl>
                </CollectionProvider>
            </Provider>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn AccordionImpl(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let disabled = prop_or_default(disabled);
    let orientation = prop_or_default(orientation);
    let direction = use_direction(dir);

    let accordion_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, accordion_ref]);

    let get_items = StoredValue::new(use_collection::<ItemData>());

    let context = AccordionImplContextValue {
        disabled,
        orientation,
    };

    view! {
        <Provider value=context>
        <CollectionSlot item_data_type=ITEM_DATA_PHANTOM node_ref=composed_ref>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:data-orientation=move || orientation.get().to_string()
                on:keydown=move |event: ev::KeyboardEvent| {
                    if disabled.get() {
                        return;
                    }

                    let key = event.key();
                    if !ACCORDION_KEYS.contains(&key.as_str()) {
                        return;
                    }

                    let target: web_sys::HtmlElement = event
                        .target()
                        .expect("Event should have target.")
                        .unchecked_into();

                    let items = get_items.with_value(|get_items| get_items());
                    let trigger_collection: Vec<_> = items
                        .iter()
                        .filter(|item| {
                            item.r#ref
                                .get()
                                .map(|el| {
                                    let btn: &web_sys::HtmlButtonElement = (*el).unchecked_ref();
                                    !btn.disabled()
                                })
                                .unwrap_or(false)
                        })
                        .collect();

                    let trigger_index = trigger_collection.iter().position(|item| {
                        item.r#ref
                            .get()
                            .map(|el| {
                                let node: &web_sys::Node = (*el).unchecked_ref();
                                let target_node: &web_sys::Node = target.unchecked_ref();
                                node == target_node
                            })
                            .unwrap_or(false)
                    });

                    let Some(trigger_index) = trigger_index else {
                        return;
                    };

                    // Prevents page scroll while user is navigating
                    event.prevent_default();

                    let trigger_count = trigger_collection.len();
                    let mut next_index = trigger_index;
                    let home_index = 0;
                    let end_index = trigger_count - 1;

                    let orientation_val = orientation.get();
                    let is_direction_ltr = direction.get() == Direction::Ltr;

                    let move_next = |idx: usize| -> usize {
                        if idx + 1 > end_index { home_index } else { idx + 1 }
                    };

                    let move_prev = |idx: usize| -> usize {
                        if idx == 0 { end_index } else { idx - 1 }
                    };

                    match key.as_str() {
                        "Home" => {
                            next_index = home_index;
                        }
                        "End" => {
                            next_index = end_index;
                        }
                        "ArrowRight" => {
                            if orientation_val == Orientation::Horizontal {
                                if is_direction_ltr {
                                    next_index = move_next(trigger_index);
                                } else {
                                    next_index = move_prev(trigger_index);
                                }
                            }
                        }
                        "ArrowDown" => {
                            if orientation_val == Orientation::Vertical {
                                next_index = move_next(trigger_index);
                            }
                        }
                        "ArrowLeft" => {
                            if orientation_val == Orientation::Horizontal {
                                if is_direction_ltr {
                                    next_index = move_prev(trigger_index);
                                } else {
                                    next_index = move_next(trigger_index);
                                }
                            }
                        }
                        "ArrowUp" => {
                            if orientation_val == Orientation::Vertical {
                                next_index = move_prev(trigger_index);
                            }
                        }
                        _ => {}
                    }

                    let clamped_index = next_index % trigger_count;
                    if let Some(item) = trigger_collection.get(clamped_index)
                        && let Some(el) = item.r#ref.get()
                    {
                        let html_el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        html_el.focus().ok();
                    }
                }
            >
                {children.with_value(|children| children())}
            </Primitive>
        </CollectionSlot>
        </Provider>
    }
}
