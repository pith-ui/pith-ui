use std::sync::Arc;

use super::*;

/* -------------------------------------------------------------------------------------------------
 * ComboboxVirtualItems
 * -----------------------------------------------------------------------------------------------*/

/// Renders combobox items with virtual scrolling. Place inside `ComboboxViewport`.
///
/// Instead of rendering all items as children, provide `count` and `render_item`
/// and only visible items will be mounted in the DOM.
///
/// # Example
///
/// ```rust,ignore
/// <ComboboxVirtualItems
///     count=Signal::derive(move || items.get().len())
///     estimate_size=35.0
///     render_item=Arc::new(move |vi: VirtualItem| {
///         let items = items.get();
///         let item = &items[vi.index];
///         view! {
///             <ComboboxItem value=item.value.clone()>
///                 <ComboboxItemText>{item.label.clone()}</ComboboxItemText>
///             </ComboboxItem>
///         }.into_any()
///     })
/// />
/// ```
#[component]
pub fn ComboboxVirtualItems(
    /// Total number of items.
    #[prop(into)] count: Signal<usize>,
    /// Estimated height in pixels for each item. Used for initial layout
    /// before measurement. Defaults to 35.0.
    #[prop(optional, default = 35.0)] estimate_size: f64,
    /// Number of items to render beyond the visible area. Defaults to 5.
    #[prop(optional, default = 5)] overscan: usize,
    /// Render callback for each virtual item.
    render_item: Arc<dyn Fn(pith_virtual_leptos::VirtualItem) -> AnyView + Send + Sync>,
) -> impl IntoView {
    let context = expect_context::<ComboboxContextValue>();
    let content_context = expect_context::<ComboboxContentContextValue>();

    // Use the viewport ref as the scroll container.
    let scroll_ref = content_context.viewport_ref;

    let virtualizer = pith_virtual_leptos::use_virtualizer(
        pith_virtual_leptos::UseVirtualizerOptions::new(count, scroll_ref, move |_| estimate_size)
            .overscan(overscan),
    );

    // Store virtualizer handle in combobox context for navigation.
    let _ = context.virtualizer.try_set_value(Some(virtualizer.clone()));

    // Keep item count in sync.
    Effect::new(move |_| {
        context.virtual_item_count.set(count.get());
    });

    // Clean up on unmount.
    on_cleanup(move || {
        let _ = context.virtualizer.try_set_value(None);
        context.virtual_item_count.set(0);
        context.highlighted_virtual_index.set(None);
    });

    let v_total = virtualizer.clone();
    let v_items = virtualizer.clone();
    let render_item = StoredValue::new(render_item);

    view! {
        <div
            role="presentation"
            style:height=move || format!("{}px", v_total.get_total_size())
            style:width="100%"
            style:position="relative"
        >
            <div
                node_ref=virtualizer.container_ref()
                role="presentation"
                style:position="absolute"
                style:top="0"
                style:left="0"
                style:width="100%"
            >
                {move || {
                    v_items.get_virtual_items().into_iter().map(|vi| {
                        let index = vi.index;
                        let start = vi.start;
                        view! {
                            <div
                                data-index=index
                                role="presentation"
                                style:position="absolute"
                                style:top="0"
                                style:left="0"
                                style:width="100%"
                                style:transform=format!("translateY({}px)", start)
                            >
                                <VirtualItemRenderer index=index vi=vi render_item=render_item />
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

/// Internal wrapper that provides `ComboboxVirtualItemIndex` context
/// *before* invoking the render callback, so `ComboboxItem` can discover
/// its virtual index during initialization via `use_context`.
#[component]
fn VirtualItemRenderer(
    index: usize,
    vi: pith_virtual_leptos::VirtualItem,
    render_item: StoredValue<Arc<dyn Fn(pith_virtual_leptos::VirtualItem) -> AnyView + Send + Sync>>,
) -> impl IntoView {
    provide_context(ComboboxVirtualItemIndex(index));
    render_item
        .try_with_value(|f| f(vi))
        .unwrap_or_else(|| ().into_any())
}
