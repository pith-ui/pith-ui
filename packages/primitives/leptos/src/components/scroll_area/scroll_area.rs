use super::*;

/* -------------------------------------------------------------------------------------------------
 * ScrollArea
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollArea(
    #[prop(into, optional)] r#type: Option<ScrollAreaType>,
    #[prop(into, optional)] dir: MaybeProp<Direction>,
    #[prop(into, optional)] scroll_hide_delay: Option<u32>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let r#type = r#type.unwrap_or_default();
    let scroll_hide_delay = scroll_hide_delay.unwrap_or(600);
    let direction = use_direction(dir);

    let scroll_area_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, scroll_area_ref]);

    let corner_width: RwSignal<f64> = RwSignal::new(0.0);
    let corner_height: RwSignal<f64> = RwSignal::new(0.0);

    let context = ScrollAreaContextValue {
        r#type,
        dir: direction,
        scroll_hide_delay,
        scroll_area: scroll_area_ref,
        viewport: RwSignal::new(None),
        content: RwSignal::new(None),
        scrollbar_x: RwSignal::new(None),
        scrollbar_x_enabled: RwSignal::new(false),
        scrollbar_y: RwSignal::new(None),
        scrollbar_y_enabled: RwSignal::new(false),
        corner_width,
        corner_height,
    };

    view! {
        <Provider value=context>
            <AttributeInterceptor let:attrs>
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=composed_ref
                    attr:dir=move || direction.get().to_string()
                    style:position="relative"
                    style:--scroll-area-corner-width=move || format!("{}px", corner_width.get())
                    style:--scroll-area-corner-height=move || format!("{}px", corner_height.get())
                    {..attrs}
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </AttributeInterceptor>
        </Provider>
    }
}

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaViewport
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaViewport(
    #[prop(into, optional)] nonce: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let nonce = StoredValue::new(nonce);

    let context = expect_context::<ScrollAreaContextValue>();

    let viewport_ref = AnyNodeRef::new();
    let composed_ref = use_composed_refs(vec![node_ref, viewport_ref]);

    // Set viewport element in context when ref is available
    Effect::new(move |_| {
        if let Some(node) = viewport_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            context.viewport.set(Some(SendWrapper::new(el)));
        }
    });

    let scrollbar_x_enabled = context.scrollbar_x_enabled;
    let scrollbar_y_enabled = context.scrollbar_y_enabled;
    let content = context.content;

    let content_ref = AnyNodeRef::new();
    Effect::new(move |_| {
        if let Some(node) = content_ref.get() {
            let el: web_sys::HtmlElement = node.unchecked_into();
            content.set(Some(SendWrapper::new(el)));
        }
    });

    view! {
        <AttributeInterceptor let:attrs>
            <style
                nonce=nonce.get_value()
                inner_html="[data-radix-scroll-area-viewport]{scrollbar-width:none;-ms-overflow-style:none;-webkit-overflow-scrolling:touch;}[data-radix-scroll-area-viewport]::-webkit-scrollbar{display:none}"
            />
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=composed_ref
                attr:data-radix-scroll-area-viewport=""
                style:overflow-x=move || if scrollbar_x_enabled.get() { "scroll" } else { "hidden" }
                style:overflow-y=move || if scrollbar_y_enabled.get() { "scroll" } else { "hidden" }
                {..attrs}
            >
                <div
                    style="min-width: 100%; display: table;"
                    node_ref=content_ref
                >
                    {children.with_value(|children| children())}
                </div>
            </Primitive>
        </AttributeInterceptor>
    }
}
