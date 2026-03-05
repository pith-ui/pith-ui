use std::marker::PhantomData;
use std::{collections::HashMap, fmt::Debug};

use crate::compose_refs::use_composed_refs;
use leptos::{html, prelude::*};
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
    let children = StoredValue::new(children);

    let context_value = CollectionContextValue::<ItemData> {
        collection_ref: AnyNodeRef::new(),
        item_map: RwSignal::new(HashMap::new()),
    };

    provide_context(context_value);

    children.with_value(|children| children())
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

    Owner::on_cleanup(move || {
        context.item_map.update(|item_map| {
            item_map.remove(&id_for_cleanup);
        });
    });

    children
        .with_value(|children| children())
        .add_any_attr(leptos::attr::custom::custom_attribute(
            "data-radix-collection-item",
            "",
        ))
        .add_any_attr(any_node_ref::<html::Div, _>(composed_ref))
}

fn node_list_to_vec(node_list: web_sys::NodeList) -> Vec<web_sys::Node> {
    let mut nodes = vec![];
    for n in 0..node_list.length() {
        if let Some(node) = node_list.item(n) {
            nodes.push(node);
        }
    }
    nodes
}

pub fn use_collection<ItemData: Clone + Send + Sync + 'static>()
-> SendWrapper<Box<dyn Fn() -> Vec<CollectionItemValue<ItemData>>>> {
    let context = expect_context::<CollectionContextValue<ItemData>>();

    let get_items = move || {
        let collection_node = context.collection_ref.get_untracked();
        if let Some(collection_node) = collection_node {
            let element: &web_sys::Element = (*collection_node).unchecked_ref();
            let ordered_nodes = node_list_to_vec(
                element
                    .query_selector_all(format!("[{ITEM_DATA_ATTR}]").as_str())
                    .expect("Node should be queried."),
            );

            let mut ordered_items = context
                .item_map
                .get_untracked()
                .into_values()
                .collect::<Vec<_>>();
            ordered_items.sort_by(|a, b| {
                let index_a = ordered_nodes.iter().position(|node| {
                    a.r#ref
                        .get_untracked()
                        .map(|el| {
                            let n: &web_sys::Node = (*el).unchecked_ref();
                            node == n
                        })
                        .unwrap_or(false)
                });
                let index_b = ordered_nodes.iter().position(|node| {
                    b.r#ref
                        .get_untracked()
                        .map(|el| {
                            let n: &web_sys::Node = (*el).unchecked_ref();
                            node == n
                        })
                        .unwrap_or(false)
                });

                index_a.cmp(&index_b)
            });

            ordered_items
        } else {
            vec![]
        }
    };

    SendWrapper::new(Box::new(get_items))
}
