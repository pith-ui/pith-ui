use std::marker::PhantomData;

use leptos::prelude::*;
use radix_leptos_collection::*;

#[derive(Clone, Debug, PartialEq)]
struct ItemData {
    disabled: bool,
}

const ITEM_DATA_PHANTOM: PhantomData<ItemData> = PhantomData;

#[component]
fn List(children: TypedChildrenFn<impl IntoView + 'static>) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());

    view! {
        <CollectionProvider item_data_type=ITEM_DATA_PHANTOM>
            <CollectionSlot item_data_type=ITEM_DATA_PHANTOM>
                <ul style="width: 200px;">
                    {children.with_value(|children| children())}
                </ul>
            </CollectionSlot>
        </CollectionProvider>
    }
}

#[component]
fn Item(
    #[prop(into, optional)] disabled: MaybeProp<bool>,
    #[prop(into, optional)] style: Option<String>,
    children: TypedChildrenFn<impl IntoView + 'static>,
) -> impl IntoView {
    let children = StoredValue::new(children.into_inner());
    let style = StoredValue::new(style);
    let item_data = Signal::derive(move || ItemData {
        disabled: disabled.get().unwrap_or(false),
    });

    view! {
        <CollectionItemSlot item_data_type=ITEM_DATA_PHANTOM item_data=item_data>
            <li style=move || {
                let opacity = if disabled.get().unwrap_or(false) { "opacity: 0.3" } else { "" };
                let custom = style.with_value(|s| s.clone().unwrap_or_default());
                let combined = [opacity, custom.as_str()]
                    .iter()
                    .filter(|s| !s.is_empty())
                    .copied()
                    .collect::<Vec<_>>()
                    .join("; ");
                if combined.is_empty() { None } else { Some(combined) }
            }>
                {children.with_value(|children| children())}
            </li>
        </CollectionItemSlot>
    }
}

#[component]
fn LogItems(#[prop(default = "items".to_string())] name: String) -> impl IntoView {
    let get_items = use_collection::<ItemData>();
    let items_text = Memo::new(move |_| {
        let items = get_items();
        let descriptions: Vec<String> = items
            .iter()
            .map(|item| format!("disabled={}", item.data.disabled))
            .collect();
        format!("[{}]", descriptions.join(", "))
    });

    view! {
        <li style="list-style: none; padding: 5px; margin-top: 10px; font-size: 0.85em; font-family: monospace; background: #f0f0f0; border-radius: 4px;">
            {move || format!("{name}: {}", items_text.get())}
        </li>
    }
}

#[component]
pub fn Basic() -> impl IntoView {
    view! {
        <List>
            <Item>"Red"</Item>
            <Item disabled=true>"Green"</Item>
            <Item>"Blue"</Item>
            <LogItems />
        </List>
    }
}

#[component]
pub fn WithElementsInBetween() -> impl IntoView {
    view! {
        <List>
            <div style="font-variant: small-caps;">"Colors"</div>
            <Item>"Red"</Item>
            <Item disabled=true>"Green"</Item>
            <Item>"Blue"</Item>
            <div style="font-variant: small-caps;">"Words"</div>
            <Item>"Hello"</Item>
            <Item>"World"</Item>
            <LogItems />
        </List>
    }
}

#[component]
fn Tomato() -> impl IntoView {
    view! {
        <Item style="color: tomato;">"Tomato"</Item>
    }
}

#[component]
pub fn WithWrappedItem() -> impl IntoView {
    view! {
        <List>
            <Item>"Red"</Item>
            <Item disabled=true>"Green"</Item>
            <Item>"Blue"</Item>
            <Tomato />
            <LogItems />
        </List>
    }
}

#[component]
pub fn WithFragment() -> impl IntoView {
    view! {
        <List>
            <Item>"France"</Item>
            <Item disabled=true>"UK"</Item>
            <Item>"Spain"</Item>
            <LogItems />
        </List>
    }
}

#[component]
pub fn DynamicInsertion() -> impl IntoView {
    let (has_tomato, set_has_tomato) = signal(false);

    view! {
        <button on:click=move |_| set_has_tomato.update(|v| *v = !*v)>
            {move || if has_tomato.get() { "Remove Tomato" } else { "Add Tomato" }}
        </button>

        <List>
            <Item>"Red"</Item>
            {move || has_tomato.get().then(|| view! { <Tomato /> })}
            <Item disabled=true>"Green"</Item>
            <Item>"Blue"</Item>
            <LogItems />
        </List>
    }
}

#[component]
pub fn WithChangingItem() -> impl IntoView {
    let (is_disabled, set_is_disabled) = signal(false);

    view! {
        <button on:click=move |_| set_is_disabled.update(|v| *v = !*v)>
            {move || if is_disabled.get() { "Enable Green" } else { "Disable Green" }}
        </button>

        <List>
            <Item>"Red"</Item>
            <Item disabled=is_disabled>"Green"</Item>
            <Item>"Blue"</Item>
            <LogItems />
        </List>
    }
}

#[component]
pub fn Nested() -> impl IntoView {
    view! {
        <List>
            <Item>"1"</Item>
            <Item>
                "2"
                <List>
                    <Item>"2.1"</Item>
                    <Item>"2.2"</Item>
                    <Item>"2.3"</Item>
                    <LogItems name="items inside 2".to_string() />
                </List>
            </Item>
            <Item>"3"</Item>
            <LogItems name="top-level items".to_string() />
        </List>
    }
}
