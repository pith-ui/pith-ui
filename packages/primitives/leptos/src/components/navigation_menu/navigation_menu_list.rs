use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuProvider (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub(super) fn NavigationMenuProvider(
    is_root_menu: bool,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] dir: Signal<Direction>,
    #[prop(into)] orientation: Signal<Orientation>,
    root_navigation_menu: AnyNodeRef,
    on_trigger_enter: Callback<String>,
    on_trigger_leave: Callback<()>,
    on_content_enter: Callback<()>,
    on_content_leave: Callback<()>,
    on_item_select: Callback<String>,
    on_item_dismiss: Callback<()>,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let viewport: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let viewport_content: RwSignal<HashMap<String, ContentData>> = RwSignal::new(HashMap::new());
    let indicator_track: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);

    let previous_value = use_previous(value);
    let base_id = use_id(None);

    let on_viewport_content_change = Callback::new(
        move |(content_value, content_data): (String, ContentData)| {
            viewport_content.update(|map| {
                map.insert(content_value, content_data);
            });
        },
    );

    let on_viewport_content_remove = Callback::new(move |content_value: String| {
        viewport_content.update(|map| {
            map.remove(&content_value);
        });
    });

    let context = NavigationMenuContextValue {
        is_root_menu,
        value,
        previous_value,
        base_id,
        dir,
        orientation,
        root_navigation_menu,
        indicator_track,
        viewport,
        has_viewport_component: RwSignal::new(false),
        on_trigger_enter,
        on_trigger_leave,
        on_content_enter,
        on_content_leave,
        on_item_select,
        on_item_dismiss,
        on_viewport_content_change,
        on_viewport_content_remove,
    };

    provide_context(context);
    provide_context(ViewportContentContextValue {
        items: viewport_content,
    });

    view! {
        <CollectionProvider<NavigationMenuItemData> item_data_type=PhantomData>
            {children.with_value(|children| children())}
        </CollectionProvider<NavigationMenuItemData>>
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuList
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuList(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();

    // React wraps the <ul> in a <div style="position: relative"> that serves as the
    // indicator track. The indicator is portaled into this wrapper div (as a sibling
    // of the <ul>), keeping it out of the <ul>'s flow. We mirror that structure here.
    //
    // User attrs (e.g. attr:class) land on the hidden capture span (the first DOM element),
    // then are forwarded to the <ul> via an Effect — the same pattern used by
    // NavigationMenuContent and NavigationMenuIndicator.
    let indicator_track_ref = AnyNodeRef::new();
    let ul_ref = AnyNodeRef::new();
    let composed_ul_refs = use_composed_refs(vec![node_ref, ul_ref]);

    // Hidden span for capturing user attributes (e.g., attr:class="mainList")
    let attr_capture_ref = AnyNodeRef::new();
    let captured_attrs: StoredValue<Vec<(String, String)>> = StoredValue::new(vec![]);

    // Capture user attrs from the hidden span
    Effect::new(move |_| {
        if let Some(el) = attr_capture_ref.get() {
            let el: web_sys::Element = el.unchecked_into();
            let attrs = el.attributes();
            let mut user_attrs = vec![];
            for i in 0..attrs.length() {
                if let Some(attr) = attrs.item(i) {
                    let name = attr.name();
                    if matches!(name.as_str(), "style" | "hidden" | "aria-hidden") {
                        continue;
                    }
                    user_attrs.push((name, attr.value()));
                }
            }
            for (name, _) in &user_attrs {
                el.remove_attribute(name).ok();
            }
            captured_attrs.set_value(user_attrs);
        }
    });

    // Forward captured attrs to the <ul>
    Effect::new(move |_| {
        if let Some(el) = ul_ref.get() {
            let el: web_sys::Element = el.unchecked_into();
            captured_attrs.with_value(|attrs| {
                for (name, value) in attrs {
                    el.set_attribute(name, value).ok();
                }
            });
        }
    });

    // Store indicator track element when mounted
    Effect::new(move |_| {
        if let Some(el) = indicator_track_ref.get() {
            let html_el: web_sys::HtmlElement = el.unchecked_into();
            context.indicator_track.set(Some(SendWrapper::new(html_el)));
        }
    });

    let list = StoredValue::new(move || {
        view! {
            <Primitive
                element=html::ul
                as_child=as_child
                node_ref=composed_ul_refs
                attr:data-orientation=move || context.orientation.get().to_string()
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        }
    });

    view! {
        <span node_ref=attr_capture_ref hidden=true aria-hidden="true" style="display:none" />
        <div style="position: relative" node_ref=indicator_track_ref>
            <CollectionSlot<NavigationMenuItemData> item_data_type=PhantomData>
                {move || {
                    if context.is_root_menu {
                        view! {
                            <FocusGroup>
                                {list.with_value(|l| l())}
                            </FocusGroup>
                        }.into_any()
                    } else {
                        list.with_value(|l| l()).into_any()
                    }
                }}
            </CollectionSlot<NavigationMenuItemData>>
        </div>
    }
}
