use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuIndicator
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuIndicator(
    #[prop(into, optional)] force_mount: MaybeProp<bool>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let is_visible = Signal::derive(move || !context.value.get().is_empty());

    let present = Signal::derive(move || force_mount.get().unwrap_or(false) || is_visible.get());

    let presence_ref = AnyNodeRef::new();

    // Capture user attributes (e.g., data-testid, class) from a hidden span for forwarding.
    let indicator_attr_capture_ref = AnyNodeRef::new();
    let indicator_captured_attrs: StoredValue<Vec<(String, String)>> = StoredValue::new(vec![]);

    Effect::new(move |_| {
        if let Some(el) = indicator_attr_capture_ref.get() {
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
            indicator_captured_attrs.set_value(user_attrs);
        }
    });

    // Apply captured attrs to the indicator element after mount
    Effect::new(move |_| {
        if let Some(el) = node_ref.get() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            indicator_captured_attrs.with_value(|attrs| {
                for (name, value) in attrs {
                    el.set_attribute(name, value).ok();
                }
            });
        }
    });

    // Capture contexts before portal boundary so they can be re-provided inside mount_to.
    let context_for_portal = context.clone();
    let collection_scope = use_collection_scope::<NavigationMenuItemData>();

    // Build the portal children as an Arc closure (same pattern as LeptosPortal).
    let portal_children: Arc<dyn Fn() -> AnyView + Send + Sync> = Arc::new(move || {
        view! {
            <Presence present=present node_ref=presence_ref>
                <NavigationMenuIndicatorImpl
                    as_child=as_child
                    node_ref=node_ref
                    presence_ref=presence_ref
                >
                    {children.with_value(|children| children.as_ref().map(|children| children()))}
                </NavigationMenuIndicatorImpl>
            </Presence>
        }
        .into_any()
    });

    // Track the current mount target to handle changes.
    let current_mount: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);

    Effect::new(move |_| {
        let track = context.indicator_track.get();
        if current_mount.get_untracked().as_deref() != track.as_deref() {
            current_mount.set(track);
        }
    });

    // Portal the indicator into the indicator track (wrapper div) via mount_to,
    // matching React's ReactDOM.createPortal(…, context.indicatorTrack) pattern.
    Effect::new(move |_| {
        if let Some(mount) = current_mount.get() {
            let ctx = context_for_portal.clone();
            let scope = collection_scope;
            let children_fn = Arc::clone(&portal_children);
            let handle = SendWrapper::new(mount_to((*mount).clone().unchecked_into(), move || {
                // Re-provide contexts across the portal boundary
                provide_context(ctx);
                if let Some(scope) = scope {
                    provide_collection_scope(scope);
                }
                untrack(|| children_fn())
            }));

            Owner::on_cleanup(move || {
                let handle = handle.take();
                drop(handle);
            });
        }
    });

    // Only the hidden attr-capture span renders in the component tree.
    // The actual indicator content is portaled into the indicator track wrapper div.
    view! {
        <span node_ref=indicator_attr_capture_ref hidden=true aria-hidden="true" style="display:none" />
    }
}

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuIndicatorImpl (internal)
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn NavigationMenuIndicatorImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    presence_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<NavigationMenuContextValue>();
    let get_items = use_collection::<NavigationMenuItemData>();

    let active_trigger: RwSignal<Option<SendWrapper<web_sys::HtmlElement>>> = RwSignal::new(None);
    let position: RwSignal<Option<(f64, f64)>> = RwSignal::new(None);
    let is_horizontal =
        Signal::derive(move || context.orientation.get() == Orientation::Horizontal);
    let is_visible = Signal::derive(move || !context.value.get().is_empty());

    // Update active trigger when value changes
    Effect::new(move |_| {
        let value = context.value.get();
        let items = get_items();
        let trigger_node = items.iter().find_map(|item| {
            if item.data.value == value {
                item.r#ref.get().map(|el| {
                    let html_el: web_sys::HtmlElement = el.unchecked_into();
                    SendWrapper::new(html_el)
                })
            } else {
                None
            }
        });
        if trigger_node.is_some() {
            active_trigger.set(trigger_node);
        }
    });

    let handle_position_change = Callback::new(move |_: ()| {
        if let Some(trigger) = active_trigger.get_untracked() {
            if is_horizontal.get_untracked() {
                position.set(Some((
                    trigger.offset_width() as f64,
                    trigger.offset_left() as f64,
                )));
            } else {
                position.set(Some((
                    trigger.offset_height() as f64,
                    trigger.offset_top() as f64,
                )));
            }
        }
    });

    use_resize_observer(
        Signal::derive(move || active_trigger.get()),
        handle_position_change,
    );
    use_resize_observer(
        Signal::derive(move || context.indicator_track.get()),
        handle_position_change,
    );

    let indicator_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, indicator_ref, presence_ref]);

    view! {
        {move || {
            position.get().map(|(size, offset)| {
                let horiz = is_horizontal.get();
                view! {
                    <Primitive
                        element=html::div
                        as_child=as_child
                        node_ref=composed_refs
                        attr:aria-hidden="true"
                        attr:data-state=move || if is_visible.get() { "visible" } else { "hidden" }
                        attr:data-orientation=move || context.orientation.get().to_string()
                        style:position="absolute"
                        style:left=move || if horiz { Some("0".to_string()) } else { None }
                        style:top=move || if !horiz { Some("0".to_string()) } else { None }
                        style:width=move || if horiz { Some(format!("{size}px")) } else { None }
                        style:height=move || if !horiz { Some(format!("{size}px")) } else { None }
                        style:transform=move || {
                            if horiz {
                                format!("translateX({offset}px)")
                            } else {
                                format!("translateY({offset}px)")
                            }
                        }
                    >
                        {children.with_value(|children| children.as_ref().map(|children| children()))}
                    </Primitive>
                }
            })
        }}
    }
}
