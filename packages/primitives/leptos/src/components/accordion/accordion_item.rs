use super::*;

/* -------------------------------------------------------------------------------------------------
 * AccordionItem
 * -----------------------------------------------------------------------------------------------*/

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
    let item_value_str = item_value.get_value();
    let open = Signal::derive(move || value_context.value.get().contains(&item_value_str));
    let disabled =
        Signal::derive(move || accordion_context.disabled.get() || disabled.get().unwrap_or(false));

    let item_context = AccordionItemContextValue {
        open,
        disabled,
        trigger_id,
        item_value,
    };
    let item_context_stored = StoredValue::new(item_context);
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
                <Provider value=item_context_stored.get_value()>
                    {children.with_value(|children| children())}
                </Provider>
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
            attr:data-state=move || open_closed_state(item_context.open.get())
            attr:data-orientation=move || accordion_context.orientation.get().to_string()
            attr:data-disabled=data_attr(item_context.disabled)
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

    // Set accordion CSS var aliases via setProperty so they don't clobber
    // CollapsibleContent's own --radix-collapsible-content-height/width vars.
    // Only set if the user hasn't already provided their own values (overridable defaults).
    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            let style = el.style();
            if style.get_property_value("--radix-accordion-content-height").unwrap_or_default().is_empty() {
                style.set_property("--radix-accordion-content-height", "var(--radix-collapsible-content-height)").ok();
            }
            if style.get_property_value("--radix-accordion-content-width").unwrap_or_default().is_empty() {
                style.set_property("--radix-accordion-content-width", "var(--radix-collapsible-content-width)").ok();
            }
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <CollapsibleContent
                force_mount=force_mount
                as_child=as_child
                node_ref=node_ref
                attr:role="region"
                attr:aria-labelledby=move || item_context.trigger_id.get()
                attr:data-orientation=move || accordion_context.orientation.get().to_string()
                {..attrs}
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </CollapsibleContent>
        </AttributeInterceptor>
    }
}
