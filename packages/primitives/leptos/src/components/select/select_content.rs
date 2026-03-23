use super::*;

/* -------------------------------------------------------------------------------------------------
 * SelectContent
 * -----------------------------------------------------------------------------------------------*/

/// When closed, renders children into a DocumentFragment to keep collection items registered.
/// When open, renders the full SelectContentImpl.
#[component]
pub fn SelectContent(
    #[prop(into, optional)] position: MaybeProp<String>,
    #[prop(into, optional)] on_close_auto_focus: Option<Callback<ev::Event>>,
    #[prop(into, optional)] on_escape_key_down: Option<Callback<ev::KeyboardEvent>>,
    #[prop(into, optional)] on_pointer_down_outside: Option<Callback<ev::CustomEvent>>,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(10.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();

    let on_close_auto_focus = StoredValue::new(on_close_auto_focus);
    let on_escape_key_down = StoredValue::new(on_escape_key_down);
    let on_pointer_down_outside = StoredValue::new(on_pointer_down_outside);

    // Track whether the select has ever been opened. Before the first open, we render
    // children in a hidden container so that SelectItemText can detect the selected item
    // and copy its text into SelectValue. This mirrors React's DocumentFragment approach.
    let has_been_opened = Memo::new(move |prev: Option<&bool>| {
        if prev == Some(&true) {
            true
        } else {
            context.open.get()
        }
    });

    // Minimal context for the hidden pre-mount container. Children (SelectViewport,
    // SelectItem, etc.) call expect_context for these, so we must provide them.
    let hidden_content_context = SelectContentContextValue {
        content_ref: AnyNodeRef::new(),
        viewport_ref: AnyNodeRef::new(),
        on_item_leave: Callback::new(|_| {}),
        position: StoredValue::new("popper".to_string()),
        is_positioned: signal(false).0,
        search_ref: StoredValue::new(String::new()),
    };
    let hidden_item_ref_callback: Callback<(
        Option<SendWrapper<web_sys::HtmlElement>>,
        String,
        bool,
    )> = Callback::new(|_| {});

    // React's SelectContent renders SelectContentImpl directly when open (no Presence
    // wrapper). When closed, it renders children into a DocumentFragment to keep
    // collection items registered. We mirror this with a simple conditional.
    view! {
        {move || {
            if context.open.get() {
                // When open, render SelectContentImpl directly (matches React).
                Some(view! {
                    <SelectContentImpl
                        position=position
                        on_close_auto_focus=on_close_auto_focus
                        on_escape_key_down=on_escape_key_down
                        on_pointer_down_outside=on_pointer_down_outside
                        side=side
                        side_offset=side_offset
                        align=align
                        align_offset=align_offset
                        arrow_padding=arrow_padding
                        avoid_collisions=avoid_collisions
                        collision_boundary=collision_boundary
                        collision_padding=collision_padding
                        sticky=sticky
                        hide_when_detached=hide_when_detached
                        update_position_strategy=update_position_strategy
                        as_child=as_child
                        node_ref=node_ref
                    >
                        {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                    </SelectContentImpl>
                }.into_any())
            } else if !has_been_opened.get() {
                // Before first open: render children in a hidden container so that
                // SelectItemText can portal the selected item's text into SelectValue.
                Some(view! {
                    <Provider value=hidden_content_context>
                        <Provider value=hidden_item_ref_callback>
                            <div style="display: none; position: absolute; overflow: hidden; pointer-events: none;">
                                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                            </div>
                        </Provider>
                    </Provider>
                }.into_any())
            } else {
                None
            }
        }}
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectContentImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SelectContentImpl(
    #[prop(into, optional)] position: MaybeProp<String>,
    on_close_auto_focus: StoredValue<Option<Callback<web_sys::Event>>>,
    #[allow(unused_variables)] on_escape_key_down: StoredValue<
        Option<Callback<web_sys::KeyboardEvent>>,
    >,
    #[allow(unused_variables)] on_pointer_down_outside: StoredValue<
        Option<Callback<web_sys::CustomEvent>>,
    >,
    // PopperContent forwarded props
    #[prop(into, optional, default = Side::Bottom.into())] side: Signal<Side>,
    #[prop(into, optional, default = 0.0.into())] side_offset: Signal<f64>,
    #[prop(into, optional, default = Align::Start.into())] align: Signal<Align>,
    #[prop(into, optional, default = 0.0.into())] align_offset: Signal<f64>,
    #[prop(into, optional, default = 0.0.into())] arrow_padding: Signal<f64>,
    #[prop(into, optional, default = true.into())] avoid_collisions: Signal<bool>,
    #[prop(into, optional, default = SendWrapper::new(vec![]).into())] collision_boundary: Signal<
        SendWrapper<Vec<web_sys::Element>>,
    >,
    #[prop(into, optional, default = Padding::All(10.0).into())] collision_padding: Signal<Padding>,
    #[prop(into, optional, default = Sticky::Partial.into())] sticky: Signal<Sticky>,
    #[prop(into, optional, default = false.into())] hide_when_detached: Signal<bool>,
    #[prop(into, optional, default = UpdatePositionStrategy::Optimized.into())]
    update_position_strategy: Signal<UpdatePositionStrategy>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let context = expect_context::<SelectContextValue>();
    let content_ref = AnyNodeRef::new();
    let viewport_ref = AnyNodeRef::new();
    let composed_refs = use_composed_refs(vec![node_ref, content_ref]);

    // Select content styles: applied as individual style: directives below.

    let _get_items = StoredValue::new(use_collection::<SelectItemData>());
    let (is_positioned, set_is_positioned) = signal(false);

    let position_mode = StoredValue::new(
        position
            .get_untracked()
            .unwrap_or_else(|| "item-aligned".to_string()),
    );

    let search_ref: StoredValue<String> = StoredValue::new(String::new());

    // Focus guards
    use_focus_guards();

    // aria-hide everything except the content (better supported equivalent to setting aria-modal)
    let hidden_elements: RwSignal<Vec<SendWrapper<web_sys::Element>>> = RwSignal::new(Vec::new());

    Effect::new(move |_| {
        if let Some(content) = content_ref.get() {
            let content: web_sys::HtmlElement = content.unchecked_into();
            hide_others(&content, hidden_elements);
        }
    });

    on_cleanup(move || {
        unhide_others(hidden_elements);
    });

    // selectedItem tracking (minimal)
    let selected_item_ref: StoredValue<Option<SendWrapper<web_sys::HtmlElement>>> =
        StoredValue::new(None);
    let first_valid_item_found_ref: StoredValue<bool> = StoredValue::new(false);

    let item_ref_callback = Callback::new(
        move |args: (Option<SendWrapper<web_sys::HtmlElement>>, String, bool)| {
            let (node, value, disabled) = args;
            let is_first_valid_item =
                !first_valid_item_found_ref.try_get_value().unwrap_or(true) && !disabled;
            let is_selected_item = context
                .value
                .get_untracked()
                .as_ref()
                .is_some_and(|v| v == &value);
            if is_selected_item || is_first_valid_item {
                let _ = selected_item_ref.try_set_value(node);
                if is_first_valid_item {
                    let _ = first_valid_item_found_ref.try_set_value(true);
                }
            }
        },
    );

    // Handle item leave: focus the content element
    let on_item_leave = Callback::new(move |_: ()| {
        if let Some(content_el) = content_ref.get_untracked() {
            let el: web_sys::HtmlElement = (*content_el).clone().unchecked_into();
            let _ = el.focus();
        }
    });

    let content_context = SelectContentContextValue {
        content_ref,
        viewport_ref,
        on_item_leave,
        position: position_mode,
        is_positioned,
        search_ref,
    };

    let content_wrapper_ref = AnyNodeRef::new();
    let viewport_context = SelectViewportContextValue {
        content_wrapper_ref,
    };

    let is_popper = position_mode.get_value() == "popper";

    // onCloseAutoFocus: restore focus to trigger
    let on_unmount_auto_focus = Callback::new(move |event: web_sys::Event| {
        let _ = on_close_auto_focus.try_with_value(|cb| {
            if let Some(cb) = cb {
                cb.run(event.clone());
            }
        });
        if let Some(trigger) = context.trigger_ref.get_untracked() {
            let el: &web_sys::HtmlElement = (*trigger).unchecked_ref();
            let opts = web_sys::FocusOptions::new();
            opts.set_prevent_scroll(true);
            let _ = el.focus_with_options(&opts);
        }
        event.prevent_default();
    });

    let on_dismiss = Callback::new(move |_: ()| {
        context.on_open_change.run(false);
    });

    // Reset first_valid_item tracking on each render
    let _ = first_valid_item_found_ref.try_set_value(false);

    // Typeahead search
    let get_items = use_collection::<SelectItemData>();
    let get_items = StoredValue::new(get_items);

    let (_typeahead_search_ref, handle_typeahead_search) =
        use_typeahead_search_no_reset(Callback::new(move |search: String| {
            let _ = content_context.search_ref.try_set_value(search.clone());

            // Find and focus the matching item (mirrors React behavior)
            let _ = get_items.try_with_value(|get_items| {
                let items = get_items();
                let enabled_items: Vec<_> =
                    items.iter().filter(|item| !item.data.disabled).collect();
                let current_item = enabled_items.iter().find(|item| {
                    item.r#ref.get_untracked().is_some_and(|el| {
                        let el: &web_sys::HtmlElement = (*el).unchecked_ref();
                        web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.active_element())
                            .is_some_and(|active| {
                                let active_node: &web_sys::Node = active.unchecked_ref();
                                let el_node: &web_sys::Node = el.unchecked_ref();
                                active_node.is_same_node(Some(el_node))
                            })
                    })
                });
                if let Some(next_item) =
                    find_next_item(&enabled_items, &search, current_item.copied())
                    && let Some(el) = next_item.r#ref.get_untracked()
                {
                    let el: web_sys::HtmlElement = (*el).clone().unchecked_into();
                    // Use setTimeout to avoid focus during keydown (matches React)
                    let closure = Closure::once_into_js(move || {
                        let opts = web_sys::FocusOptions::new();
                        opts.set_prevent_scroll(true);
                        let _ = el.focus_with_options(&opts);
                    });
                    let _ = web_sys::window()
                        .expect("Window should exist.")
                        .set_timeout_with_callback(closure.unchecked_ref());
                }
            });
        }));

    // Focus selected item after positioned
    Effect::new(move |_| {
        if is_positioned.get() {
            if let Some(selected_el) = selected_item_ref.try_get_value().flatten() {
                let opts = web_sys::FocusOptions::new();
                opts.set_prevent_scroll(true);
                let _ = selected_el.focus_with_options(&opts);
            } else if let Some(content_el) = content_ref.get() {
                let el: &web_sys::HtmlElement = (*content_el).unchecked_ref();
                let opts = web_sys::FocusOptions::new();
                opts.set_prevent_scroll(true);
                let _ = el.focus_with_options(&opts);
            }
        }
    });

    // Prevent selecting items on `pointerup` in some cases after opening from `pointerdown`
    // and close on `pointerup` outside. (matches React lines 614-651)
    {
        let pointer_move_closure: StoredValue<
            Option<SendWrapper<Closure<dyn Fn(web_sys::PointerEvent)>>>,
        > = StoredValue::new(None);
        let pointer_up_closure: StoredValue<
            Option<SendWrapper<Closure<dyn Fn(web_sys::PointerEvent)>>>,
        > = StoredValue::new(None);

        let cleanup_listeners = move || {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                let _ = pointer_move_closure.try_with_value(|c| {
                    if let Some(c) = c {
                        let _ = doc.remove_event_listener_with_callback(
                            "pointermove",
                            c.as_ref().unchecked_ref(),
                        );
                    }
                });
                let _ = pointer_up_closure.try_with_value(|c| {
                    if let Some(c) = c {
                        let opts = web_sys::EventListenerOptions::new();
                        opts.set_capture(true);
                        let _ = doc.remove_event_listener_with_callback_and_event_listener_options(
                            "pointerup",
                            c.as_ref().unchecked_ref(),
                            &opts,
                        );
                    }
                });
            }
        };

        Effect::new(move |_| {
            // Clean up any previous listeners before setting up new ones.
            cleanup_listeners();
            let _ = pointer_move_closure.try_set_value(None);
            let _ = pointer_up_closure.try_set_value(None);

            let Some(content_el) = content_ref.get() else {
                return;
            };
            let content_el: web_sys::HtmlElement = (*content_el).clone().unchecked_into();
            let trigger_pos = context
                .trigger_pointer_down_pos_ref
                .try_get_value()
                .flatten();

            if trigger_pos.is_none() {
                return;
            }

            let pointer_move_delta: Rc<Cell<(f64, f64)>> = Rc::new(Cell::new((0.0, 0.0)));

            let delta_for_move = pointer_move_delta.clone();
            let trigger_pos_for_move = trigger_pos;
            let move_closure = SendWrapper::new(Closure::<dyn Fn(web_sys::PointerEvent)>::new(
                move |event: web_sys::PointerEvent| {
                    if let Some((tx, ty)) = trigger_pos_for_move {
                        delta_for_move.set((
                            (event.page_x() as f64 - tx).abs(),
                            (event.page_y() as f64 - ty).abs(),
                        ));
                    }
                },
            ));

            let up_closure = SendWrapper::new(Closure::<dyn Fn(web_sys::PointerEvent)>::new({
                let content_el = content_el.clone();
                let cleanup = cleanup_listeners;
                move |event: web_sys::PointerEvent| {
                    let (dx, dy) = pointer_move_delta.get();
                    if dx <= 10.0 && dy <= 10.0 {
                        event.prevent_default();
                    } else {
                        // If the pointer moved but the event was outside the content, close.
                        if let Some(target) = event.target() {
                            let target_node: &web_sys::Node = target.unchecked_ref();
                            if !content_el.contains(Some(target_node)) {
                                context.on_open_change.run(false);
                            }
                        }
                    }
                    cleanup();
                    let _ = context.trigger_pointer_down_pos_ref.try_set_value(None);
                }
            }));

            let document = web_sys::window()
                .expect("Window should exist.")
                .document()
                .expect("Document should exist.");

            let _ = document.add_event_listener_with_callback(
                "pointermove",
                move_closure.as_ref().unchecked_ref(),
            );

            let options = web_sys::AddEventListenerOptions::new();
            options.set_capture(true);
            options.set_once(true);
            let _ = document.add_event_listener_with_callback_and_add_event_listener_options(
                "pointerup",
                up_closure.as_ref().unchecked_ref(),
                &options,
            );

            let _ = pointer_move_closure.try_set_value(Some(move_closure));
            let _ = pointer_up_closure.try_set_value(Some(up_closure));
        });

        on_cleanup(move || {
            cleanup_listeners();
        });
    }

    // Close on window blur and resize (matches React lines 653-661)
    {
        let close_blur = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            context.on_open_change.run(false);
        }));
        let close_resize = SendWrapper::new(Closure::<dyn Fn()>::new(move || {
            context.on_open_change.run(false);
        }));
        let window = web_sys::window().expect("Window should exist.");
        let _ =
            window.add_event_listener_with_callback("blur", close_blur.as_ref().unchecked_ref());
        let _ = window
            .add_event_listener_with_callback("resize", close_resize.as_ref().unchecked_ref());

        on_cleanup(move || {
            if let Some(win) = web_sys::window() {
                let _ = win.remove_event_listener_with_callback(
                    "blur",
                    close_blur.as_ref().unchecked_ref(),
                );
                let _ = win.remove_event_listener_with_callback(
                    "resize",
                    close_resize.as_ref().unchecked_ref(),
                );
            }
        });
    }

    // Keyboard handler
    let on_key_down = move |event: web_sys::KeyboardEvent| {
        let is_modifier_key = event.ctrl_key() || event.alt_key() || event.meta_key();

        // Prevent tab navigation
        if event.key() == "Tab" {
            event.prevent_default();
            return;
        }

        // Typeahead search for single printable characters
        if !is_modifier_key && event.key().len() == 1 {
            handle_typeahead_search.run(event.key());
        }

        let key = event.key();
        if ["ArrowUp", "ArrowDown", "Home", "End"].contains(&key.as_str()) {
            let _ = get_items.try_with_value(|get_items| {
                let items = get_items();
                let enabled_items: Vec<_> =
                    items.iter().filter(|item| !item.data.disabled).collect();
                let mut candidate_nodes: Vec<web_sys::HtmlElement> = enabled_items
                    .iter()
                    .filter_map(|item| {
                        item.r#ref
                            .get_untracked()
                            .map(|el| (*el).clone().unchecked_into::<web_sys::HtmlElement>())
                    })
                    .collect();

                if key == "ArrowUp" || key == "End" {
                    candidate_nodes.reverse();
                }
                if (key == "ArrowUp" || key == "ArrowDown")
                    && let Some(target) = event.target()
                {
                    let current_el: web_sys::HtmlElement = target.unchecked_into();
                    if let Some(current_index) =
                        candidate_nodes.iter().position(|n| *n == current_el)
                    {
                        candidate_nodes = candidate_nodes[current_index + 1..].to_vec();
                    }
                }

                // Focus first candidate
                if let Some(first) = candidate_nodes.first() {
                    let opts = web_sys::FocusOptions::new();
                    opts.set_prevent_scroll(true);
                    let _ = first.focus_with_options(&opts);
                }
            });
            event.prevent_default();
        }
    };

    // Item-aligned positioning Effect
    if !is_popper {
        // Track the pending rAF handle so we can cancel it if the content
        // unmounts before the callback fires (prevents WASM panic from
        // accessing disposed signals).
        let raf_id: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));
        {
            let raf_id = raf_id.clone();
            on_cleanup(move || {
                let id = raf_id.load(Ordering::Relaxed);
                if id != 0 {
                    if let Some(win) = web_sys::window() {
                        let _ = win.cancel_animation_frame(id);
                    }
                }
            });
        }

        Effect::new(move |_| {
            // Track value changes so this Effect re-runs when the selection changes.
            // This ensures item-aligned positioning recalculates after a new item is
            // selected (e.g., in always-open Chromatic stories).
            let _value = context.value.get();

            // Cancel any pending rAF from a previous Effect run.
            let prev = raf_id.load(Ordering::Relaxed);
            if prev != 0 {
                if let Some(win) = web_sys::window() {
                    let _ = win.cancel_animation_frame(prev);
                }
                raf_id.store(0, Ordering::Relaxed);
            }

            // Reset so the focus Effect re-runs after repositioning completes.
            set_is_positioned.set(false);

            let raf_id_inner = raf_id.clone();
            let cb = Closure::once_into_js(move || {
                raf_id_inner.store(0, Ordering::Relaxed);

                let Some(wrapper_el) = content_wrapper_ref.get_untracked() else {
                    return;
                };
                let Some(content_el) = content_ref.get_untracked() else {
                    return;
                };
                let Some(trigger_el) = context.trigger_ref.get_untracked() else {
                    return;
                };
                let Some(value_node_el) = context.value_node_ref.get_untracked() else {
                    return;
                };
                let Some(viewport_el) = viewport_ref.get_untracked() else {
                    return;
                };

                let wrapper: &web_sys::HtmlElement = (*wrapper_el).unchecked_ref();
                let content: &web_sys::HtmlElement = (*content_el).unchecked_ref();
                let trigger: &web_sys::HtmlElement = (*trigger_el).unchecked_ref();
                let value_node: &web_sys::HtmlElement = (*value_node_el).unchecked_ref();
                let viewport: &web_sys::HtmlElement = (*viewport_el).unchecked_ref();

                // Clear old positioning properties before recalculating.
                // position_item_aligned conditionally sets top OR bottom, so stale
                // values from a previous layout must be removed.
                let ws = wrapper.style();
                let _ = ws.remove_property("top");
                let _ = ws.remove_property("bottom");
                let _ = ws.remove_property("left");
                let _ = ws.remove_property("right");
                let _ = ws.remove_property("height");
                let _ = ws.remove_property("min-width");
                let _ = ws.remove_property("min-height");
                let _ = ws.remove_property("max-height");
                let _ = ws.remove_property("margin");

                // Look up the selected item from the collection based on the
                // current value. This is fresher than selected_item_ref which is
                // only populated during the initial render pass.
                let selected_item = get_items
                    .try_with_value(|get_items| {
                        let items = get_items();
                        let current_value = context.value.get_untracked();
                        items.iter().find_map(|item| {
                            if current_value
                                .as_ref()
                                .is_some_and(|v| v == &item.data.value)
                            {
                                item.r#ref.get_untracked().map(|el| {
                                    SendWrapper::new(
                                        (*el).clone().unchecked_into::<web_sys::HtmlElement>(),
                                    )
                                })
                            } else {
                                None
                            }
                        })
                    })
                    .flatten()
                    // Fall back to selected_item_ref (first-valid-item from initial render)
                    .or_else(|| selected_item_ref.try_get_value().flatten());

                // Update selected_item_ref so the focus Effect uses the right element.
                let _ = selected_item_ref.try_set_value(selected_item.clone());

                // Determine is_first/is_last from collection items
                let (is_first, is_last) = get_items
                    .try_with_value(|get_items| {
                        let items = get_items();
                        let is_first = items.first().is_some_and(|first| {
                            first.r#ref.get_untracked().is_some_and(|el| {
                                selected_item.as_ref().is_some_and(|si| {
                                    let el: &web_sys::Node = (*el).unchecked_ref();
                                    let si: &web_sys::Node = (**si).unchecked_ref();
                                    el.is_same_node(Some(si))
                                })
                            })
                        });
                        let is_last = items.last().is_some_and(|last| {
                            last.r#ref.get_untracked().is_some_and(|el| {
                                selected_item.as_ref().is_some_and(|si| {
                                    let el: &web_sys::Node = (*el).unchecked_ref();
                                    let si: &web_sys::Node = (**si).unchecked_ref();
                                    el.is_same_node(Some(si))
                                })
                            })
                        });
                        (is_first, is_last)
                    })
                    .unwrap_or((false, false));

                if let Some(selected_item) = selected_item.as_deref() {
                    let dir = context.dir.get_untracked();
                    position_item_aligned(
                        wrapper,
                        content,
                        trigger,
                        value_node,
                        viewport,
                        selected_item,
                        dir,
                        is_first,
                        is_last,
                    );
                } else {
                    // No selected item -- position near the trigger as a fallback
                    let trigger_rect = trigger.get_bounding_client_rect();
                    let _ = wrapper
                        .style()
                        .set_property("left", &format!("{}px", trigger_rect.left()));
                    let _ = wrapper
                        .style()
                        .set_property("top", &format!("{}px", trigger_rect.bottom()));
                    let _ = wrapper
                        .style()
                        .set_property("min-width", &format!("{}px", trigger_rect.width()));
                }

                // Copy z-index from content to wrapper
                if let Ok(Some(styles)) = web_sys::window()
                    .expect("Window should exist.")
                    .get_computed_style(content)
                {
                    let z_index = styles.get_property_value("z-index").unwrap_or_default();
                    if !z_index.is_empty() && z_index != "auto" {
                        let _ = wrapper.style().set_property("z-index", &z_index);
                    }
                }

                set_is_positioned.set(true);
            });
            if let Ok(id) = web_sys::window()
                .expect("Window should exist.")
                .request_animation_frame(cb.unchecked_ref())
            {
                raf_id.store(id, Ordering::Relaxed);
            }
        });
    }

    if is_popper {
        view! {
            <Provider value=content_context>
                <Provider value=viewport_context>
                    <Provider value=item_ref_callback>
                        <FocusScope
                            as_child=true
                            trapped=Signal::derive(move || context.open.get())
                            on_mount_auto_focus=Callback::new(move |event: web_sys::Event| {
                                event.prevent_default();
                            })
                            on_unmount_auto_focus=on_unmount_auto_focus
                        >
                            <DismissableLayer
                                as_child=true
                                disable_outside_pointer_events=true
                                on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                                    let _ = on_escape_key_down.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    let _ = on_pointer_down_outside.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    event.prevent_default();
                                })
                                on_dismiss=Callback::new(move |_: ()| {
                                    on_dismiss.run(());
                                })
                            >
                                <PopperContent
                                    side=side
                                    side_offset=side_offset
                                    align=align
                                    align_offset=align_offset
                                    arrow_padding=arrow_padding
                                    avoid_collisions=avoid_collisions
                                    collision_boundary=collision_boundary
                                    collision_padding=collision_padding
                                    sticky=sticky
                                    hide_when_detached=hide_when_detached
                                    update_position_strategy=update_position_strategy
                                    as_child=as_child
                                    node_ref=composed_refs
                                    style:display="flex"
                                    style:flex-direction="column"
                                    style:outline="none"
                                    style:box-sizing="border-box"
                                    style:--radix-select-content-transform-origin="var(--radix-popper-transform-origin)"
                                    style:--radix-select-content-available-width="var(--radix-popper-available-width)"
                                    style:--radix-select-content-available-height="var(--radix-popper-available-height)"
                                    style:--radix-select-trigger-width="var(--radix-popper-anchor-width)"
                                    style:--radix-select-trigger-height="var(--radix-popper-anchor-height)"
                                    on_placed=Some(Callback::new(move |_: ()| {
                                        set_is_positioned.set(true);
                                    }))
                                    attr:role="listbox"
                                    attr:id=move || context.content_id.get()
                                    attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                                    attr:dir=move || context.dir.get().to_string()
                                    on:keydown=on_key_down
                                    on:contextmenu=move |event: ev::MouseEvent| {
                                        event.prevent_default();
                                    }
                                >
                                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                                </PopperContent>
                            </DismissableLayer>
                        </FocusScope>
                    </Provider>
                </Provider>
            </Provider>
        }
        .into_any()
    } else {
        view! {
            <Provider value=content_context>
                <Provider value=viewport_context>
                    <Provider value=item_ref_callback>
                        <FocusScope
                            as_child=true
                            trapped=Signal::derive(move || context.open.get())
                            on_mount_auto_focus=Callback::new(move |event: web_sys::Event| {
                                event.prevent_default();
                            })
                            on_unmount_auto_focus=on_unmount_auto_focus
                        >
                            <DismissableLayer
                                as_child=true
                                disable_outside_pointer_events=true
                                on_escape_key_down=Callback::new(move |event: web_sys::KeyboardEvent| {
                                    let _ = on_escape_key_down.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_pointer_down_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    let _ = on_pointer_down_outside.try_with_value(|cb| {
                                        if let Some(cb) = cb {
                                            cb.run(event);
                                        }
                                    });
                                })
                                on_focus_outside=Callback::new(move |event: web_sys::CustomEvent| {
                                    event.prevent_default();
                                })
                                on_dismiss=Callback::new(move |_: ()| {
                                    on_dismiss.run(());
                                })
                            >
                                <SelectItemAlignedPosition
                                    content_wrapper_ref=content_wrapper_ref
                                    as_child=as_child
                                    node_ref=composed_refs
                                    on_key_down=Callback::new(on_key_down)
                                >
                                    {children.try_with_value(|children| children.as_ref().map(|c| c()))}
                                </SelectItemAlignedPosition>
                            </DismissableLayer>
                        </FocusScope>
                    </Provider>
                </Provider>
            </Provider>
        }
        .into_any()
    }
}

/* -------------------------------------------------------------------------------------------------
 * SelectItemAlignedPosition
 * Separate component so that user attrs (class, data-testid, etc.) forwarded via
 * FocusScope/DismissableLayer land on the inner content div, not the outer
 * positioning wrapper. This mirrors React's SelectItemAlignedPosition which
 * explicitly spreads contentProps on the inner Primitive.
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn SelectItemAlignedPosition(
    content_wrapper_ref: AnyNodeRef,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into)] on_key_down: Callback<web_sys::KeyboardEvent>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<SelectContextValue>();
    let content_ref = AnyNodeRef::new();
    let composed_content_ref = use_composed_refs(vec![node_ref, content_ref]);

    // Transfer caller-added attributes from the wrapper div to the inner content div.
    //
    // In React, SelectItemAlignedPosition explicitly spreads contentProps (className,
    // data-testid, etc.) onto the inner Primitive.div, while the wrapper div only has
    // hardcoded positioning styles. In Leptos, attrs from FocusScope/DismissableLayer's
    // as_child chain bypass the component boundary and land on the first DOM element
    // (the wrapper). This Effect moves class, non-positioning styles, and user attrs
    // from the wrapper to the inner content div after mount.
    Effect::new(move |_| {
        let (Some(wrapper), Some(inner)) = (content_wrapper_ref.get(), content_ref.get()) else {
            return;
        };
        let wrapper: web_sys::HtmlElement = (*wrapper).clone().unchecked_into();
        let inner: web_sys::HtmlElement = (*inner).clone().unchecked_into();

        // Transfer class
        let wrapper_class = wrapper.get_attribute("class").unwrap_or_default();
        if !wrapper_class.is_empty() {
            let inner_class = inner.get_attribute("class").unwrap_or_default();
            let combined = if inner_class.is_empty() {
                wrapper_class
            } else {
                format!("{inner_class} {wrapper_class}")
            };
            inner.set_attribute("class", &combined).ok();
            wrapper.remove_attribute("class").ok();
        }

        // Transfer non-positioning style properties
        let wrapper_style = wrapper.style();
        let inner_style = inner.style();
        let mut caller_props: Vec<(String, String)> = Vec::new();
        let len = wrapper_style.length();
        for i in 0..len {
            let prop = wrapper_style.item(i);
            if prop.is_empty() {
                continue;
            }
            // Skip properties managed by SelectItemAlignedPosition's positioning
            let is_positioning = matches!(
                prop.as_str(),
                "display"
                    | "flex-direction"
                    | "position"
                    | "top"
                    | "bottom"
                    | "left"
                    | "right"
                    | "min-width"
                    | "height"
                    | "min-height"
                    | "max-height"
                    | "margin"
                    | "z-index"
                    | "pointer-events"
            );
            if !is_positioning && let Ok(value) = wrapper_style.get_property_value(&prop) {
                caller_props.push((prop, value));
            }
        }
        for (prop, value) in &caller_props {
            let _ = inner_style.set_property(prop, value);
            let _ = wrapper_style.remove_property(prop);
        }

        // Restore critical wrapper styles that may have been overwritten by
        // attr:style forwarding through the as_child chain. When a caller sets
        // attr:style on SelectContent (e.g., `attr:style="opacity: 0.7;"`), Leptos
        // forwards it via FocusScope/DismissableLayer's as_child=true, which
        // replaces the wrapper's entire style attribute -- destroying the hardcoded
        // `display: flex; flex-direction: column; position: fixed;`. We restore
        // these here after transferring caller styles to the inner div.
        let _ = wrapper_style.set_property("display", "flex");
        let _ = wrapper_style.set_property("flex-direction", "column");
        let _ = wrapper_style.set_property("position", "fixed");

        // Transfer non-internal attributes from wrapper to inner
        let attrs = wrapper.attributes();
        let mut attrs_to_transfer: Vec<(String, String)> = Vec::new();
        for i in 0..attrs.length() {
            if let Some(attr) = attrs.item(i) {
                let name = attr.name();
                // Skip wrapper's own attributes
                if matches!(name.as_str(), "style" | "class") {
                    continue;
                }
                attrs_to_transfer.push((name, attr.value()));
            }
        }
        for (name, value) in &attrs_to_transfer {
            inner.set_attribute(name, value).ok();
            wrapper.remove_attribute(name).ok();
        }
    });

    view! {
        <div
            node_ref=content_wrapper_ref
            style="display: flex; flex-direction: column; position: fixed;"
        >
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_content_ref
                style:box-sizing="border-box"
                style:max-height="100%"
                style:outline="none"
                attr:role="listbox"
                attr:id=move || context.content_id.get()
                attr:data-state=move || if context.open.get() { "open" } else { "closed" }
                attr:dir=move || context.dir.get().to_string()
                on:keydown=move |event: web_sys::KeyboardEvent| {
                    on_key_down.run(event);
                }
                on:contextmenu=move |event: ev::MouseEvent| {
                    event.prevent_default();
                }
            >
                {children.try_with_value(|children| children.as_ref().map(|c| c()))}
            </Primitive>
        </div>
    }
}
