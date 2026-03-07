use super::*;

/* -------------------------------------------------------------------------------------------------
 * NavigationMenuItem
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn NavigationMenuItem(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let auto_value = use_id(None);
    let item_value = Memo::new(move |_| value.get().unwrap_or_else(|| auto_value.get()));

    let content_ref = AnyNodeRef::new();
    let trigger_ref = AnyNodeRef::new();
    let focus_proxy_ref = AnyNodeRef::new();
    let was_escape_close_ref = RwSignal::new(false);

    type TabOrderBackup = Option<SendWrapper<Vec<(web_sys::HtmlElement, String)>>>;
    let restore_content_tab_order: StoredValue<TabOrderBackup> = StoredValue::new(None);

    let handle_content_entry = Callback::new(move |side: &'static str| {
        if let Some(el) = content_ref.get_untracked() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            // Restore tab order first
            restore_content_tab_order.with_value(|saved| {
                if let Some(saved) = saved {
                    restore_tab_order(saved);
                }
            });
            restore_content_tab_order.set_value(None);

            let candidates = get_tabbable_candidates(&el);
            if !candidates.is_empty() {
                if side == "start" {
                    focus_first(&candidates);
                } else {
                    let mut reversed = candidates;
                    reversed.reverse();
                    focus_first(&reversed);
                }
            }
        }
    });

    let handle_content_exit = Callback::new(move |_: ()| {
        if let Some(el) = content_ref.get_untracked() {
            let el: web_sys::HtmlElement = el.unchecked_into();
            let candidates = get_tabbable_candidates(&el);
            if !candidates.is_empty() {
                let saved = remove_from_tab_order(&candidates);
                restore_content_tab_order.set_value(Some(SendWrapper::new(saved)));
            }
        }
    });

    let item_value_for_ctx = item_value;
    let item_context = NavigationMenuItemContextValue {
        value: item_value_for_ctx.get_untracked(),
        trigger_ref,
        content_ref,
        focus_proxy_ref,
        was_escape_close_ref,
        on_entry_key_down: Callback::new(move |_: ()| {
            handle_content_entry.run("start");
        }),
        on_focus_proxy_enter: handle_content_entry,
        on_root_content_close: handle_content_exit,
        on_content_focus_outside: handle_content_exit,
    };

    // Update context value reactively
    let item_context_signal = RwSignal::new(item_context);
    Effect::new(move |_| {
        let val = item_value.get();
        item_context_signal.update(|ctx| {
            ctx.value = val;
        });
    });

    view! {
        <Provider value=item_context_signal>
            <Primitive
                element=html::li
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|children| children.as_ref().map(|children| children()))}
            </Primitive>
        </Provider>
    }
}
