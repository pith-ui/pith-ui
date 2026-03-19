//! DOM-order-aware collection of items within a component.
//!
//! Tracks child items registered via [`CollectionItemSlot`] and returns them
//! in DOM order (not insertion order). Used by roving focus, select, menu,
//! and other components that need ordered access to their children.

use std::marker::PhantomData;
use std::{collections::HashMap, fmt::Debug};

use crate::support::compose_refs::use_composed_refs;
use leptos::{context::Provider, html, prelude::*, tachys::html::node_ref::NodeRefContainer};
use leptos_node_ref::{AnyNodeRef, any_node_ref};
use nanoid::nanoid;
use send_wrapper::SendWrapper;
use web_sys::wasm_bindgen::JsCast;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct CollectionItemId(String);

impl CollectionItemId {
    fn new() -> Self {
        Self(nanoid!())
    }
}

#[derive(Clone)]
pub struct CollectionItemValue<ItemData> {
    pub r#ref: AnyNodeRef,
    pub data: ItemData,
}

impl<ItemData: Debug> Debug for CollectionItemValue<ItemData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionItemValue")
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Clone)]
struct CollectionContextValue<ItemData: Clone + Send + Sync + 'static> {
    collection_ref: AnyNodeRef,
    item_map: RwSignal<HashMap<CollectionItemId, CollectionItemValue<ItemData>>>,
}

// Manual Copy impl: AnyNodeRef and RwSignal are always Copy regardless of ItemData.
impl<ItemData: Clone + Send + Sync + 'static> Copy for CollectionContextValue<ItemData> {}

/// Opaque handle capturing the current Collection scope from the reactive owner chain.
///
/// Call [`use_collection_scope`] before a portal boundary, then call
/// [`provide_collection_scope`] inside the portal's children to re-establish the context.
#[derive(Clone)]
pub struct CollectionScope<ItemData: Clone + Send + Sync + 'static>(
    CollectionContextValue<ItemData>,
);

impl<ItemData: Clone + Send + Sync + 'static> Copy for CollectionScope<ItemData> {}

/// Captures the current Collection scope, if one exists.
pub fn use_collection_scope<ItemData: Clone + Send + Sync + 'static>()
-> Option<CollectionScope<ItemData>> {
    use_context::<CollectionContextValue<ItemData>>().map(CollectionScope)
}

/// Re-provides a previously captured Collection scope into the current reactive owner.
pub fn provide_collection_scope<ItemData: Clone + Send + Sync + 'static>(
    scope: CollectionScope<ItemData>,
) {
    provide_context(scope.0);
}

#[component]
pub fn CollectionProvider<ItemData: Clone + Send + Sync + 'static>(
    #[expect(unused_variables)]
    #[prop(into, optional)]
    item_data_type: Option<PhantomData<ItemData>>,
    children: ChildrenFn,
) -> impl IntoView {
    let context_value = CollectionContextValue::<ItemData> {
        collection_ref: AnyNodeRef::new(),
        item_map: RwSignal::new(HashMap::new()),
    };

    view! {
        <Provider value=context_value>
            {children()}
        </Provider>
    }
}

const ITEM_DATA_ATTR: &str = "data-radix-collection-item";

#[component]
pub fn CollectionSlot<ItemData: Clone + Send + Sync + 'static>(
    #[expect(unused_variables)]
    #[prop(into, optional)]
    item_data_type: Option<PhantomData<ItemData>>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<CollectionContextValue<ItemData>>();
    let composed_ref = use_composed_refs(vec![node_ref, context.collection_ref]);

    // Fallback: if node_ref is set externally (e.g. by an inner component's own
    // node_ref mechanism through deep as_child chains), propagate to collection_ref.
    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            NodeRefContainer::<html::Div>::load(context.collection_ref, &el);
        }
    });

    children
        .with_value(|children| children())
        .add_any_attr(any_node_ref::<html::Div, _>(composed_ref))
}

#[component]
pub fn CollectionItemSlot<ItemData: Clone + Debug + Send + Sync + 'static>(
    #[expect(unused_variables)]
    #[prop(into, optional)]
    item_data_type: Option<PhantomData<ItemData>>,
    #[prop(into, optional)] item_data: MaybeProp<ItemData>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let id = CollectionItemId::new();
    let id_for_effect = id.clone();
    let id_for_cleanup = id.clone();
    let item_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, item_ref]);
    let context = expect_context::<CollectionContextValue<ItemData>>();

    // Register eagerly so items are available during SSR.
    // The NodeRef will be empty during SSR, but item metadata and count are correct.
    // On the client, the effect below re-registers when item_data changes reactively.
    if let Some(data) = item_data.get_untracked() {
        context.item_map.update(|item_map| {
            item_map.insert(
                id,
                CollectionItemValue {
                    r#ref: item_ref,
                    data,
                },
            );
        });
    }

    Effect::new(move |_| {
        if let Some(item_data) = item_data.get() {
            context.item_map.update(|item_map| {
                item_map.insert(
                    id_for_effect.clone(),
                    CollectionItemValue {
                        r#ref: item_ref,
                        data: item_data,
                    },
                );
            });
        }
    });

    // Fallback: if node_ref is set externally (e.g. by an inner component's own
    // node_ref mechanism through deep as_child chains), propagate to item_ref
    // so the item_map entry has the actual DOM element.
    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            NodeRefContainer::<html::Div>::load(item_ref, &el);
        }
    });

    Owner::on_cleanup(move || {
        context.item_map.update(|item_map| {
            item_map.remove(&id_for_cleanup);
        });
    });

    children
        .with_value(|children| children())
        .add_any_attr(leptos::attr::custom::custom_attribute(ITEM_DATA_ATTR, ""))
        .add_any_attr(any_node_ref::<html::Div, _>(composed_ref))
}

pub fn use_collection<ItemData: Clone + Send + Sync + 'static>()
-> SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>> {
    let context = expect_context::<CollectionContextValue<ItemData>>();

    let get_items = move || {
        let mut ordered_items = context
            .item_map
            .get_untracked()
            .into_values()
            .collect::<Vec<_>>();

        // Sort by DOM position. During SSR all refs are None, so the sort is
        // a no-op and items retain their HashMap iteration order.
        ordered_items.sort_by(
            |a, b| match (a.r#ref.get_untracked(), b.r#ref.get_untracked()) {
                (Some(el_a), Some(el_b)) => {
                    let node_a: &web_sys::Node = (*el_a).unchecked_ref();
                    let node_b: &web_sys::Node = (*el_b).unchecked_ref();
                    let position = node_a.compare_document_position(node_b);
                    if position & web_sys::Node::DOCUMENT_POSITION_FOLLOWING != 0 {
                        std::cmp::Ordering::Less
                    } else if position & web_sys::Node::DOCUMENT_POSITION_PRECEDING != 0 {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            },
        );

        ordered_items
    };

    SendWrapper::new(Box::new(get_items))
}
