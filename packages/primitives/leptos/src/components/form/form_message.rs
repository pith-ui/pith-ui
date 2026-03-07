use super::*;

/* -------------------------------------------------------------------------------------------------
 * FormMessage
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormMessage(
    #[prop(into, optional)] r#match: Option<Match>,
    #[prop(into, optional)] force_match: MaybeProp<bool>,
    #[prop(into, optional)] name: Option<String>,
    #[prop(into, optional)] id: Option<String>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView {
    let field_context = expect_context::<FormFieldContextValue>();
    let name = name.unwrap_or_else(|| field_context.name.clone());
    let generated_id = use_id(None);
    let id = id.unwrap_or_else(|| generated_id.get_untracked());
    let children = StoredValue::new(children);

    match r#match {
        None => view! {
            <FormMessageImpl name=name.clone() id=id.clone() as_child=as_child node_ref=node_ref>
                {children.with_value(|c| match c {
                    Some(c) => c().into_any(),
                    None => view! { {DEFAULT_INVALID_MESSAGE} }.into_any(),
                })}
            </FormMessageImpl>
        }
        .into_any(),
        Some(Match::BuiltIn(matcher)) => {
            let default_msg = matcher.default_message();
            view! {
                <FormBuiltInMessage
                    r#match=matcher
                    force_match=force_match
                    name=name.clone()
                    id=id.clone()
                    as_child=as_child
                    node_ref=node_ref
                >
                    {children.with_value(|c| match c {
                        Some(c) => c().into_any(),
                        None => view! { {default_msg} }.into_any(),
                    })}
                </FormBuiltInMessage>
            }
            .into_any()
        }
        Some(Match::Custom(matcher)) => view! {
            <FormCustomMessage
                matcher=CustomMatcher::Sync(matcher)
                force_match=force_match
                name=name.clone()
                id=id.clone()
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|c| match c {
                    Some(c) => c().into_any(),
                    None => view! { {DEFAULT_INVALID_MESSAGE} }.into_any(),
                })}
            </FormCustomMessage>
        }
        .into_any(),
        Some(Match::CustomAsync(matcher)) => view! {
            <FormCustomMessage
                matcher=CustomMatcher::Async(matcher)
                force_match=force_match
                name=name.clone()
                id=id.clone()
                as_child=as_child
                node_ref=node_ref
            >
                {children.with_value(|c| match c {
                    Some(c) => c().into_any(),
                    None => view! { {DEFAULT_INVALID_MESSAGE} }.into_any(),
                })}
            </FormCustomMessage>
        }
        .into_any(),
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormBuiltInMessage
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FormBuiltInMessage(
    r#match: ValidityMatcher,
    #[prop(into, optional)] force_match: MaybeProp<bool>,
    #[prop(into)] name: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let matcher = r#match;
    let field_name = name.clone();

    let validity = Memo::new(move |_| validation_context.get_field_validity(&field_name));

    let matches = Memo::new(move |_| {
        if force_match.get().unwrap_or(false) {
            return true;
        }
        if let Some(v) = validity.get() {
            matcher.matches(&v)
        } else {
            false
        }
    });

    let children = StoredValue::new(children);

    view! {
        <Show when=move || matches.get()>
            <FormMessageImpl name=name.clone() id=id.clone() as_child=as_child node_ref=node_ref>
                {children.with_value(|children| children())}
            </FormMessageImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormCustomMessage
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FormCustomMessage(
    matcher: CustomMatcher,
    #[prop(into, optional)] force_match: MaybeProp<bool>,
    #[prop(into)] name: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();

    let entry = SendWrapper::new(CustomMatcherEntry {
        id: id.clone(),
        matcher,
    });

    let entry_name = name.clone();
    validation_context.add_field_custom_matcher_entry(&entry_name, entry);

    let cleanup_name = name.clone();
    let cleanup_id = id.clone();
    let cleanup_ctx = validation_context.clone();
    Owner::on_cleanup(move || {
        cleanup_ctx.remove_field_custom_matcher_entry(&cleanup_name, &cleanup_id);
    });

    let matches_name = name.clone();
    let matches_id = id.clone();
    let matches = Memo::new(move |_| {
        if force_match.get().unwrap_or(false) {
            return true;
        }
        let validity = validation_context.get_field_validity(&matches_name);
        let custom_errors = validation_context.get_field_custom_errors(&matches_name);
        let has_matching_error = custom_errors.get(&matches_id).copied().unwrap_or(false);
        validity.is_some() && !has_built_in_error(validity.as_ref().unwrap()) && has_matching_error
    });

    let children = StoredValue::new(children);

    view! {
        <Show when=move || matches.get()>
            <FormMessageImpl name=name.clone() id=id.clone() as_child=as_child node_ref=node_ref>
                {children.with_value(|children| children())}
            </FormMessageImpl>
        </Show>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormMessageImpl
 * -----------------------------------------------------------------------------------------------*/

#[component]
fn FormMessageImpl(
    #[prop(into)] name: String,
    #[prop(into)] id: String,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let aria_description_context = expect_context::<AriaDescriptionContextValue>();

    aria_description_context.add_field_message_id(&name, &id);

    let cleanup_name = name;
    let cleanup_id = id.clone();
    Owner::on_cleanup(move || {
        aria_description_context.remove_field_message_id(&cleanup_name, &cleanup_id);
    });

    view! {
        <Primitive
            element=html::span
            as_child=as_child
            node_ref=node_ref
            attr:id=id
        >
            {children()}
        </Primitive>
    }
}

/* -------------------------------------------------------------------------------------------------
 * FormValidityState
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormValidityState(
    #[prop(into, optional)] name: Option<String>,
    children: Callback<Option<Validity>, AnyView>,
) -> impl IntoView {
    let validation_context = expect_context::<ValidationContextValue>();
    let field_context = expect_context::<FormFieldContextValue>();
    let name = name.unwrap_or_else(|| field_context.name.clone());

    let validity = Memo::new(move |_| validation_context.get_field_validity(&name));

    move || children.run(validity.get())
}

/* -------------------------------------------------------------------------------------------------
 * FormSubmit
 * -----------------------------------------------------------------------------------------------*/

#[component]
pub fn FormSubmit(
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    view! {
        <Primitive
            element=html::button
            as_child=as_child
            node_ref=node_ref
            attr:r#type="submit"
        >
            {children.with_value(|children| children())}
        </Primitive>
    }
}
