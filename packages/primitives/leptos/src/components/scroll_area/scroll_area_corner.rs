use super::*;

/* -------------------------------------------------------------------------------------------------
 * ScrollAreaCorner
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn ScrollAreaCorner(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();

    let has_both_scrollbars = Memo::new(move |_| {
        context.scrollbar_x.get().is_some() && context.scrollbar_y.get().is_some()
    });
    let has_corner = Memo::new(move |_| {
        !matches!(context.r#type, ScrollAreaType::Scroll) && has_both_scrollbars.get()
    });

    view! {
        <AttributeInterceptor let:attrs>
            <Show
                when=move || has_corner.get()
                {..attrs}
            >
                <ScrollAreaCornerImpl
                    as_child=as_child
                    node_ref=node_ref
                >
                    {children.with_value(|children| children())}
                </ScrollAreaCornerImpl>
            </Show>
        </AttributeInterceptor>
    }
}

#[component]
fn ScrollAreaCornerImpl(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);
    let context = expect_context::<ScrollAreaContextValue>();

    let width = RwSignal::new(0.0_f64);
    let height = RwSignal::new(0.0_f64);
    let has_size = Memo::new(move |_| width.get() != 0.0 && height.get() != 0.0);

    use_resize_observer(
        Signal::derive(move || context.scrollbar_x.get()),
        move || {
            if let Some(scrollbar_x) = context.scrollbar_x.get_untracked() {
                let h = scrollbar_x.offset_height() as f64;
                context.corner_height.set(h);
                height.set(h);
            }
        },
    );

    use_resize_observer(
        Signal::derive(move || context.scrollbar_y.get()),
        move || {
            if let Some(scrollbar_y) = context.scrollbar_y.get_untracked() {
                let w = scrollbar_y.offset_width() as f64;
                context.corner_width.set(w);
                width.set(w);
            }
        },
    );

    let dir = context.dir;

    view! {
        <AttributeInterceptor let:attrs>
            <Show
                when=move || has_size.get()
                {..attrs}
            >
                <Primitive
                    element=html::div
                    as_child=as_child
                    node_ref=node_ref
                    style:width=move || format!("{}px", width.get())
                    style:height=move || format!("{}px", height.get())
                    style:position="absolute"
                    style:bottom="0"
                    style:right=move || (dir.get() == Direction::Ltr).then_some("0")
                    style:left=move || (dir.get() == Direction::Rtl).then_some("0")
                >
                    {children.with_value(|children| children())}
                </Primitive>
            </Show>
        </AttributeInterceptor>
    }
}
