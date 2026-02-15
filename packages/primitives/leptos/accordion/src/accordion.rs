use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use leptos::{
    attribute_interceptor::AttributeInterceptor, context::Provider, ev, html, prelude::*,
};
use leptos_node_ref::AnyNodeRef;
use radix_leptos_collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
use radix_leptos_collection::{
    CollectionItemSlot, CollectionProvider, CollectionSlot, use_collection,
};
use radix_leptos_compose_refs::use_composed_refs;
use radix_leptos_direction::{Direction, use_direction};
use radix_leptos_id::use_id;
use radix_leptos_primitive::Primitive;
use radix_leptos_use_controllable_state::{UseControllableStateParams, use_controllable_state};
use web_sys::wasm_bindgen::JsCast;

/* -------------------------------------------------------------------------------------------------
 * Accordion
 * -----------------------------------------------------------------------------------------------*/

const ACCORDION_KEYS: &[&str] = &[
    "Home",
    "End",
    "ArrowDown",
    "ArrowUp",
    "ArrowLeft",
    "ArrowRight",
];

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AccordionType {
    Single,
    Multiple,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum Orientation {
    Horizontal,
    #[default]
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

#[derive(Clone, Debug, Default, PartialEq)]
struct ItemData;

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[component]
pub fn Accordion(
    /// The type of accordion: single or multiple open items.
    r#type: AccordionType,

    // -- Single mode props --
    /// The controlled value of the open item (single mode).
    #[prop(into, optional)]
    value: MaybeProp<String>,
    /// The default open item value (single mode).
    #[prop(into, optional)]
    default_value: MaybeProp<String>,
    /// Callback when value changes (single mode).
    #[prop(into, optional)]
    on_value_change: Option<Callback<String>>,
    /// Whether an item can be collapsed after opening (single mode). Default false.
    #[prop(into, optional)]
    collapsible: MaybeProp<bool>,

    // -- Multiple mode props --
    /// The controlled values of open items (multiple mode).
    #[prop(into, optional)]
    values: MaybeProp<Vec<String>>,
    /// The default open item values (multiple mode).
    #[prop(into, optional)]
    default_values: MaybeProp<Vec<String>>,
    /// Callback when values change (multiple mode).
    #[prop(into, optional)]
    on_values_change: Option<Callback<Vec<String>>>,

    // -- Shared props --
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] orientation: MaybeProp<Orientation>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    // Create context values based on type. Each is wrapped in a <Provider> to create
    // a child Owner, ensuring sibling Accordion instances get isolated context scopes.
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
 * Context Creation Helpers
 * -----------------------------------------------------------------------------------------------*/

fn create_single_contexts(
    value: MaybeProp<String>,
    default_value: MaybeProp<String>,
    on_value_change: Option<Callback<String>>,
    collapsible: MaybeProp<bool>,
) -> (AccordionValueContextValue, AccordionCollapsibleContextValue) {
    let collapsible_flag = Signal::derive(move || collapsible.get().unwrap_or(false));

    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<String>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });

    // Wrap single value in a Vec for the shared AccordionValue context.
    let value_as_vec = Signal::derive(move || {
        value_signal
            .get()
            .map(|v| if v.is_empty() { vec![] } else { vec![v] })
            .unwrap_or_default()
    });

    let on_item_open = Callback::new(move |item_value: String| {
        set_value.run(Some(item_value));
    });

    let on_item_close = Callback::new(move |_: String| {
        if collapsible_flag.get() {
            set_value.run(Some(String::new()));
        }
    });

    (
        AccordionValueContextValue {
            value: value_as_vec,
            on_item_open,
            on_item_close,
        },
        AccordionCollapsibleContextValue {
            collapsible: collapsible_flag,
        },
    )
}

fn create_multiple_contexts(
    value: MaybeProp<Vec<String>>,
    default_value: MaybeProp<Vec<String>>,
    on_value_change: Option<Callback<Vec<String>>>,
) -> (AccordionValueContextValue, AccordionCollapsibleContextValue) {
    let (value_signal, set_value) = use_controllable_state(UseControllableStateParams {
        prop: value,
        default_prop: default_value,
        on_change: on_value_change.map(|on_value_change| {
            Callback::new(move |value: Option<Vec<String>>| {
                if let Some(value) = value {
                    on_value_change.run(value);
                }
            })
        }),
    });

    let value_vec = Signal::derive(move || value_signal.get().unwrap_or_default());

    let on_item_open = Callback::new(move |item_value: String| {
        let mut current = value_signal.get().unwrap_or_default();
        current.push(item_value);
        set_value.run(Some(current));
    });

    let on_item_close = Callback::new(move |item_value: String| {
        let current = value_signal.get().unwrap_or_default();
        let filtered: Vec<String> = current.into_iter().filter(|v| *v != item_value).collect();
        set_value.run(Some(filtered));
    });

    (
        AccordionValueContextValue {
            value: value_vec,
            on_item_open,
            on_item_close,
        },
        AccordionCollapsibleContextValue {
            // Multiple mode is always collapsible.
            collapsible: Signal::derive(|| true),
        },
    )
}

/* -------------------------------------------------------------------------------------------------
 * Contexts
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct AccordionValueContextValue {
    value: Signal<Vec<String>>,
    on_item_open: Callback<String>,
    on_item_close: Callback<String>,
}

#[derive(Clone)]
struct AccordionCollapsibleContextValue {
    collapsible: Signal<bool>,
}

#[derive(Clone)]
struct AccordionImplContextValue {
    disabled: Signal<bool>,
    orientation: Signal<Orientation>,
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

    let disabled = Signal::derive(move || disabled.get().unwrap_or(false));
    let orientation = Signal::derive(move || orientation.get().unwrap_or_default());
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

/* -------------------------------------------------------------------------------------------------
 * AccordionItem
 * -----------------------------------------------------------------------------------------------*/

#[derive(Clone)]
struct AccordionItemContextValue {
    open: Signal<bool>,
    disabled: Signal<bool>,
    trigger_id: ReadSignal<String>,
}

#[component]
pub fn AccordionItem(
    /// A unique string value for this item.
    value: String,
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let accordion_context = expect_context::<AccordionImplContextValue>();
    let value_context = expect_context::<AccordionValueContextValue>();
    let trigger_id = use_id(None);

    let item_value = StoredValue::new(value);
    let open: Signal<bool> = Memo::new(move |_| {
        let val = item_value.get_value();
        value_context.value.get().contains(&val)
    })
    .into();
    let disabled =
        Signal::derive(move || accordion_context.disabled.get() || disabled.get().unwrap_or(false));

    let item_context = AccordionItemContextValue {
        open,
        disabled,
        trigger_id,
    };
    provide_context(item_context);

    let on_open_change = Callback::new(move |open_val: bool| {
        let val = item_value.get_value();
        if open_val {
            value_context.on_item_open.run(val);
        } else {
            value_context.on_item_close.run(val);
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Collapsible
                open=open
                disabled=disabled
                on_open_change=on_open_change
                as_child=as_child
                node_ref=node_ref
                attr:data-orientation=move || accordion_context.orientation.get().to_string()
                {..attrs}
            >
                {children.with_value(|children| children())}
            </Collapsible>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionHeader
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AccordionHeader(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let accordion_context = expect_context::<AccordionImplContextValue>();
    let item_context = expect_context::<AccordionItemContextValue>();

    view! {
        <Primitive
            element=html::h3
            as_child=as_child
            node_ref=node_ref
            attr:data-orientation=move || accordion_context.orientation.get().to_string()
            attr:data-state=move || get_state(item_context.open.get())
            attr:data-disabled=move || item_context.disabled.get().then_some("")
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionTrigger
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AccordionTrigger(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let accordion_context = expect_context::<AccordionImplContextValue>();
    let item_context = expect_context::<AccordionItemContextValue>();
    let collapsible_context = expect_context::<AccordionCollapsibleContextValue>();

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=Signal::derive(|| ItemData)>
            <CollapsibleTrigger
                as_child=as_child
                node_ref=node_ref
                attr:aria-disabled=move || {
                    (item_context.open.get() && !collapsible_context.collapsible.get())
                        .then_some("true")
                }
                attr:data-orientation=move || accordion_context.orientation.get().to_string()
                attr:id=move || item_context.trigger_id.get()
            >
                {children.with_value(|children| children())}
            </CollapsibleTrigger>
        </CollectionItemSlot>
    }
}

/* -------------------------------------------------------------------------------------------------
 * AccordionContent
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn AccordionContent(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let accordion_context = expect_context::<AccordionImplContextValue>();
    let item_context = expect_context::<AccordionItemContextValue>();

    view! {
        <AttributeInterceptor let:attrs>
            <CollapsibleContent
                force_mount=force_mount
                as_child=as_child
                node_ref=node_ref
                attr:role="region"
                attr:aria-labelledby=move || item_context.trigger_id.get()
                attr:data-orientation=move || accordion_context.orientation.get().to_string()
                attr:style="--radix-accordion-content-height: var(--radix-collapsible-content-height); --radix-accordion-content-width: var(--radix-collapsible-content-width);"
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </CollapsibleContent>
        </AttributeInterceptor>
    }
}

/* -------------------------------------------------------------------------------------------------
 * Utils
 * -----------------------------------------------------------------------------------------------*/

fn get_state(open: bool) -> &'static str {
    match open {
        true => "open",
        false => "closed",
    }
}
