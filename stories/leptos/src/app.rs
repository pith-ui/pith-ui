use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes, ToHref},
    hooks::use_location,
    path,
};

use crate::primitives::{
    accessible_icon, accordion, alert_dialog, arrow, aspect_ratio, collapsible, collection, dialog,
    dismissable_layer, focus_scope, form, hover_card, label, one_time_password_field,
    password_toggle_field, popover, popper, portal, presence, progress, radio_group, roving_focus,
    scroll_area, separator, slider, tabs, toggle, toggle_group, toolbar, visually_hidden,
};

#[component]
fn NavLink<H>(href: H, children: Children) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    // TODO: add class when active
    view! {
        <A href=href attr:class="text-inherit decoration-inherit no-underline">
            {children()}
        </A>
    }
}

/// A collapsible top-level tier (e.g. "Components", "Utilities").
/// Starts expanded. Force-opens when filter is active.
#[component]
fn NavTier(title: &'static str, children: Children) -> impl IntoView {
    let filter = expect_context::<RwSignal<String>>();
    let open = RwSignal::new(true);

    let is_open = move || {
        if !filter.get().is_empty() {
            true
        } else {
            open.get()
        }
    };

    view! {
        <li>
            <span
                style="cursor: pointer; user-select: none; display: flex; align-items: center; gap: 0.25em; font-weight: 600;"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span style="display: inline-block; width: 1em; text-align: center; flex-shrink: 0;">
                    {move || if is_open() { "\u{25be}" } else { "\u{25b8}" }}
                </span>
                {title}
            </span>
            <ul
                class="list-none m-0 ms-2 p-0"
                style:display=move || if is_open() { "" } else { "none" }
            >
                {children()}
            </ul>
        </li>
    }
}

/// A collapsible component section with data-driven stories.
/// Filtering matches against both the section title and individual story titles.
/// When filtering, only matching stories are shown and the section is force-opened.
/// Set `tested=false` to color the section to indicate it needs story testing.
#[component]
fn NavSection(
    title: &'static str,
    stories: Vec<(&'static str, &'static str)>,
    #[prop(default = true)] tested: bool,
) -> impl IntoView {
    let filter = expect_context::<RwSignal<String>>();
    let open = RwSignal::new(false);
    let color = if tested { "" } else { "#b45309" };

    let stories_for_visible = stories.clone();
    let visible = move || {
        let f = filter.get().to_lowercase();
        if f.is_empty() {
            return true;
        }
        title.to_lowercase().contains(&f)
            || stories_for_visible
                .iter()
                .any(|(_, name)| name.to_lowercase().contains(&f))
    };

    let is_open = move || {
        if !filter.get().is_empty() {
            true
        } else {
            open.get()
        }
    };

    view! {
        <li style:display=move || if visible() { "" } else { "none" } style:color=color>
            <span
                style="cursor: pointer; user-select: none; display: flex; align-items: center; gap: 0.25em;"
                on:click=move |_| open.update(|v| *v = !*v)
            >
                <span style="display: inline-block; width: 1em; text-align: center; flex-shrink: 0;">
                    {move || if is_open() { "\u{25be}" } else { "\u{25b8}" }}
                </span>
                {title}
            </span>
            <ul
                class="list-none m-0 ms-4 p-0"
                style:display=move || if is_open() { "" } else { "none" }
            >
                {stories
                    .into_iter()
                    .map(|(href, name)| {
                        let story_visible = move || {
                            let f = filter.get().to_lowercase();
                            if f.is_empty() {
                                return true;
                            }
                            title.to_lowercase().contains(&f)
                                || name.to_lowercase().contains(&f)
                        };
                        view! {
                            <li style:display=move || if story_visible() { "" } else { "none" }>
                                <NavLink href=href>{name}</NavLink>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </li>
    }
}

#[component]
fn Index() -> impl IntoView {
    view! {
        <h1>Radix Leptos Stories</h1>
    }
}

/// Renders the current story path inside an iframe with `?embed` query param.
/// The iframe src updates reactively when the parent Router's location changes.
#[component]
fn StoryIframe() -> impl IntoView {
    let location = use_location();
    let src = Memo::new(move |_| {
        let path = location.pathname.get();
        format!("{}?embed", path)
    });

    view! {
        <iframe src=move || src.get() style="width: 100%; height: 100%; border: none;" />
    }
}

/// Embed mode: renders just the Router + Routes without any nav shell.
/// This is loaded inside the iframe.
#[component]
fn EmbedApp() -> impl IntoView {
    view! {
        <Router>
            <main class="p-4">
                <Routes fallback=|| "Not found.".into_view()>
                    <Route path=path!("/") view=Index />

                    <Route path=path!("/accordion/single") view=accordion::Single />
                    <Route path=path!("/accordion/multiple") view=accordion::Multiple />
                    <Route path=path!("/accordion/animated") view=accordion::Animated />
                    <Route path=path!("/accordion/animated-2d") view=accordion::Animated2D />
                    <Route path=path!("/accordion/animated-controlled") view=accordion::AnimatedControlled />
                    <Route path=path!("/accordion/outside-viewport") view=accordion::OutsideViewport />
                    <Route path=path!("/accordion/horizontal") view=accordion::Horizontal />
                    <Route path=path!("/accordion/chromatic") view=accordion::Chromatic />

                    <Route path=path!("/accessible-icon/styled") view=accessible_icon::Styled />
                    <Route path=path!("/accessible-icon/chromatic") view=accessible_icon::Chromatic />

                    <Route path=path!("/alert-dialog/styled") view=alert_dialog::Styled />
                    <Route path=path!("/alert-dialog/controlled") view=alert_dialog::Controlled />
                    <Route path=path!("/alert-dialog/chromatic") view=alert_dialog::Chromatic />

                    <Route path=path!("/arrow/styled") view=arrow::Styled />
                    <Route path=path!("/arrow/custom-sizes") view=arrow::CustomSizes />
                    <Route path=path!("/arrow/custom-arrow") view=arrow::CustomArrow />

                    <Route path=path!("/aspect-ratio/styled") view=aspect_ratio::Styled />
                    <Route path=path!("/aspect-ratio/custom-ratios") view=aspect_ratio::CustomRatios />
                    <Route path=path!("/aspect-ratio/chromatic") view=aspect_ratio::Chromatic />

                    <Route path=path!("/collapsible/styled") view=collapsible::Styled />
                    <Route path=path!("/collapsible/controlled") view=collapsible::Controlled />
                    <Route path=path!("/collapsible/animated") view=collapsible::Animated />
                    <Route path=path!("/collapsible/animated-horizontal") view=collapsible::AnimatedHorizontal />
                    <Route path=path!("/collapsible/chromatic") view=collapsible::Chromatic />

                    <Route path=path!("/dialog/styled") view=dialog::Styled />
                    <Route path=path!("/dialog/non-modal") view=dialog::NonModal />
                    <Route path=path!("/dialog/controlled") view=dialog::Controlled />
                    <Route path=path!("/dialog/focus-trap") view=dialog::FocusTrap />
                    <Route path=path!("/dialog/custom-focus") view=dialog::CustomFocus />
                    <Route path=path!("/dialog/no-escape-dismiss") view=dialog::NoEscapeDismiss />
                    <Route path=path!("/dialog/no-pointer-down-outside-dismiss") view=dialog::NoPointerDownOutsideDismiss />
                    <Route path=path!("/dialog/with-portal-container") view=dialog::WithPortalContainer />
                    <Route path=path!("/dialog/animated") view=dialog::Animated />
                    <Route path=path!("/dialog/forced-mount") view=dialog::ForcedMount />
                    <Route path=path!("/dialog/inner-scrollable") view=dialog::InnerScrollable />
                    <Route path=path!("/dialog/outer-scrollable") view=dialog::OuterScrollable />
                    <Route path=path!("/dialog/chromatic") view=dialog::Chromatic />
                    <Route path=path!("/dialog/cypress") view=dialog::Cypress />

                    // <Route path="/avatar/styled" view=avatar::Styled />
                    // <Route path="/avatar/chromatic" view=avatar::Chromatic />

                    // <Route path="/checkbox/styled" view=checkbox::Styled />
                    // <Route path="/checkbox/controlled" view=checkbox::Controlled />
                    // <Route path="/checkbox/indeterminate" view=checkbox::Indeterminate />
                    // <Route path="/checkbox/within-form" view=checkbox::WithinForm />
                    // <Route path="/checkbox/animated" view=checkbox::Animated />
                    // <Route path="/checkbox/chromatic" view=checkbox::Chromatic />

                    <Route path=path!("/dismissable-layer/basic") view=dismissable_layer::Basic />
                    <Route path=path!("/dismissable-layer/nested") view=dismissable_layer::Nested />
                    <Route path=path!("/dismissable-layer/with-focus-scope") view=dismissable_layer::WithFocusScope />
                    <Route path=path!("/dismissable-layer/dialog-example") view=dismissable_layer::DialogExample />
                    <Route path=path!("/dismissable-layer/popover-fully-modal") view=dismissable_layer::PopoverFullyModal />
                    <Route path=path!("/dismissable-layer/popover-semi-modal") view=dismissable_layer::PopoverSemiModal />
                    <Route path=path!("/dismissable-layer/popover-non-modal") view=dismissable_layer::PopoverNonModal />
                    <Route path=path!("/dismissable-layer/popover-in-dialog") view=dismissable_layer::PopoverInDialog />
                    <Route path=path!("/dismissable-layer/popover-nested") view=dismissable_layer::PopoverNested />

                    <Route path=path!("/collection/basic") view=collection::Basic />
                    <Route path=path!("/collection/with-element-in-between") view=collection::WithElementsInBetween />
                    <Route path=path!("/collection/with-wrapped-item") view=collection::WithWrappedItem />
                    <Route path=path!("/collection/with-fragment") view=collection::WithFragment />
                    <Route path=path!("/collection/dynamic-insertion") view=collection::DynamicInsertion />
                    <Route path=path!("/collection/with-changing-item") view=collection::WithChangingItem />
                    <Route path=path!("/collection/nested") view=collection::Nested />

                    <Route path=path!("/focus-scope/basic") view=focus_scope::Basic />
                    <Route path=path!("/focus-scope/multiple") view=focus_scope::Multiple />
                    <Route path=path!("/focus-scope/with-options") view=focus_scope::WithOptions />

                    <Route path=path!("/form/basic") view=form::Basic />
                    <Route path=path!("/form/cypress") view=form::Cypress />

                    <Route path=path!("/hover-card/basic") view=hover_card::Basic />
                    <Route path=path!("/hover-card/contain-text-selection") view=hover_card::ContainTextSelection />
                    <Route path=path!("/hover-card/async-update") view=hover_card::AsyncUpdate />
                    <Route path=path!("/hover-card/custom-durations") view=hover_card::CustomDurations />
                    <Route path=path!("/hover-card/controlled") view=hover_card::Controlled />
                    <Route path=path!("/hover-card/layerable") view=hover_card::Layerable />
                    <Route path=path!("/hover-card/animated") view=hover_card::Animated />
                    <Route path=path!("/hover-card/forced-mount") view=hover_card::ForcedMount />
                    <Route path=path!("/hover-card/nested") view=hover_card::Nested />
                    <Route path=path!("/hover-card/non-portal") view=hover_card::NonPortal />
                    <Route path=path!("/hover-card/with-slotted-trigger") view=hover_card::WithSlottedTrigger />
                    <Route path=path!("/hover-card/with-slotted-content") view=hover_card::WithSlottedContent />
                    <Route path=path!("/hover-card/chromatic") view=hover_card::Chromatic />

                    <Route path=path!("/label/styled") view=label::Styled />
                    <Route path=path!("/label/with-control") view=label::WithControl />
                    <Route path=path!("/label/with-input-number") view=label::WithInputNumber />

                    // <Route path=path!("/menu/styled") view=menu::Styled />
                    // <Route path=path!("/menu/submenus") view=menu::Submenus />
                    // <Route path=path!("/menu/with-labels") view=menu::WithLabels />
                    // <Route path=path!("/menu/typeahead") view=menu::Typeahead />
                    // <Route path=path!("/menu/checkbox-items") view=menu::CheckboxItems />
                    // <Route path=path!("/menu/radio-items") view=menu::RadioItems />
                    // <Route path=path!("/menu/animated") view=menu::Animated />

                    <Route path=path!("/one-time-password-field/uncontrolled") view=one_time_password_field::Uncontrolled />
                    <Route path=path!("/one-time-password-field/controlled") view=one_time_password_field::Controlled />

                    <Route path=path!("/password-toggle-field/uncontrolled") view=password_toggle_field::Uncontrolled />
                    <Route path=path!("/password-toggle-field/controlled") view=password_toggle_field::Controlled />
                    <Route path=path!("/password-toggle-field/inside-form") view=password_toggle_field::InsideForm />

                    <Route path=path!("/popover/styled") view=popover::Styled />
                    <Route path=path!("/popover/boundary") view=popover::Boundary />
                    <Route path=path!("/popover/modality") view=popover::Modality />
                    <Route path=path!("/popover/controlled") view=popover::Controlled />
                    <Route path=path!("/popover/animated") view=popover::Animated />
                    <Route path=path!("/popover/forced-mount") view=popover::ForcedMount />
                    <Route path=path!("/popover/nested") view=popover::Nested />
                    <Route path=path!("/popover/custom-anchor") view=popover::CustomAnchor />
                    <Route path=path!("/popover/with-slotted-trigger") view=popover::WithSlottedTrigger />
                    <Route path=path!("/popover/chromatic") view=popover::Chromatic />

                    <Route path=path!("/popper/styled") view=popper::Styled />
                    <Route path=path!("/popper/with-custom-arrow") view=popper::WithCustomArrow />
                    <Route path=path!("/popper/animated") view=popper::Animated />
                    <Route path=path!("/popper/with-portal") view=popper::WithPortal />
                    <Route path=path!("/popper/with-update-position-strategy-always") view=popper::WithUpdatePositionStrategyAlways />
                    <Route path=path!("/popper/chromatic") view=popper::Chromatic />

                    <Route path=path!("/portal/base") view=portal::Base />
                    <Route path=path!("/portal/custom-container") view=portal::CustomContainer />
                    <Route path=path!("/portal/chromatic") view=portal::Chromatic />

                    <Route path=path!("/presence/basic") view=presence::Basic />
                    <Route path=path!("/presence/with-mount-animation") view=presence::WithMountAnimation />
                    <Route path=path!("/presence/with-unmount-animation") view=presence::WithUnmountAnimation />
                    <Route path=path!("/presence/with-multiple-mount-animations") view=presence::WithMultipleMountAnimations />
                    <Route path=path!("/presence/with-open-and-close-animation") view=presence::WithOpenAndCloseAnimation />
                    <Route path=path!("/presence/with-multiple-open-and-close-animations") view=presence::WithMultipleOpenAndCloseAnimations />
                    <Route path=path!("/presence/with-deferred-mount-animation") view=presence::WithDeferredMountAnimation />

                    <Route path=path!("/progress/styled") view=progress::Styled />
                    <Route path=path!("/progress/chromatic") view=progress::Chromatic />

                    <Route path=path!("/radio-group/styled") view=radio_group::LegacyStyled />
                    <Route path=path!("/radio-group/controlled") view=radio_group::LegacyControlled />
                    <Route path=path!("/radio-group/unset") view=radio_group::LegacyUnset />
                    <Route path=path!("/radio-group/within-form") view=radio_group::LegacyWithinForm />
                    <Route path=path!("/radio-group/animated") view=radio_group::LegacyAnimated />
                    <Route path=path!("/radio-group/chromatic") view=radio_group::LegacyChromatic />

                    <Route path=path!("/roving-focus/basic") view=roving_focus::Basic />
                    <Route path=path!("/roving-focus/nested") view=roving_focus::Nested />
                    <Route path=path!("/roving-focus/edge-cases") view=roving_focus::EdgeCases />

                    <Route path=path!("/scroll-area/basic") view=scroll_area::Basic />
                    <Route path=path!("/scroll-area/resizable") view=scroll_area::Resizable />
                    <Route path=path!("/scroll-area/content-change") view=scroll_area::ContentChange />
                    <Route path=path!("/scroll-area/animated") view=scroll_area::Animated />
                    <Route path=path!("/scroll-area/chromatic") view=scroll_area::Chromatic />
                    <Route path=path!("/scroll-area/chromatic-dynamic-content-before-loaded") view=scroll_area::ChromaticDynamicContentBeforeLoaded />
                    <Route path=path!("/scroll-area/chromatic-dynamic-content-after-loaded") view=scroll_area::ChromaticDynamicContentAfterLoaded />

                    <Route path=path!("/separator/styled") view=separator::Styled />

                    <Route path=path!("/slider/styled") view=slider::Styled />
                    <Route path=path!("/slider/with-on-value-commit") view=slider::WithOnValueCommit />
                    <Route path=path!("/slider/right-to-left") view=slider::RightToLeft />
                    <Route path=path!("/slider/horizontal") view=slider::Horizontal />
                    <Route path=path!("/slider/vertical") view=slider::Vertical />
                    <Route path=path!("/slider/inversions") view=slider::Inversions />
                    <Route path=path!("/slider/with-minimum-steps-between-thumbs") view=slider::WithMinimumStepsBetweenThumbs />
                    <Route path=path!("/slider/with-multiple-ranges") view=slider::WithMultipleRanges />
                    <Route path=path!("/slider/small-steps") view=slider::SmallSteps />
                    <Route path=path!("/slider/within-form") view=slider::WithinForm />
                    <Route path=path!("/slider/strict") view=slider::Strict />
                    <Route path=path!("/slider/chromatic") view=slider::Chromatic />

                    <Route path=path!("/tabs/styled") view=tabs::Styled />
                    <Route path=path!("/tabs/animated") view=tabs::Animated />
                    <Route path=path!("/tabs/chromatic") view=tabs::Chromatic />

                    // <Route path="/slot/without-slottable" view=slot::WithoutSlottable />
                    // <Route path="/slot/with-slottable" view=slot::WithSlottable />

                    // <Route path="/switch/styled" view=switch::Styled />
                    // <Route path="/switch/controlled" view=switch::Controlled />
                    // <Route path="/switch/within-form" view=switch::WithinForm />
                    // <Route path="/switch/chromatic" view=switch::Chromatic />

                    <Route path=path!("/toggle/styled") view=toggle::Styled />
                    <Route path=path!("/toggle/controlled") view=toggle::Controlled />
                    <Route path=path!("/toggle/chromatic") view=toggle::Chromatic />

                    <Route path=path!("/toggle-group/single") view=toggle_group::Single />
                    <Route path=path!("/toggle-group/vertical") view=toggle_group::Vertical />
                    <Route path=path!("/toggle-group/multiple") view=toggle_group::Multiple />
                    <Route path=path!("/toggle-group/chromatic") view=toggle_group::Chromatic />

                    <Route path=path!("/toolbar/styled") view=toolbar::Styled />
                    <Route path=path!("/toolbar/chromatic") view=toolbar::Chromatic />

                    <Route path=path!("/visually-hidden/basic") view=visually_hidden::Basic />
                </Routes>
            </main>
        </Router>
    }
}

/// Shell mode: renders the nav sidebar and an iframe that loads the story content.
/// The iframe provides full CSS/positioning isolation between the nav and stories.
#[component]
fn ShellApp() -> impl IntoView {
    let filter = RwSignal::new(String::new());
    provide_context(filter);

    view! {
        <Router>
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 box-border overflow-y-auto leading-normal">
                <input
                    type="text"
                    placeholder="Filter\u{2026}"
                    class="w-full p-1.5 mb-3 border border-slate-300 rounded text-sm box-border"
                    prop:value=move || filter.get()
                    on:input=move |ev| filter.set(event_target_value(&ev))
                />
                <ul class="list-none m-0 p-0">
                    <li>
                        <NavLink href="/">Index</NavLink>
                    </li>

                    // -- Components --
                    <NavTier title="Components">
                        <NavSection title="Accordion" stories=vec![
                            ("/accordion/single", "Single"),
                            ("/accordion/multiple", "Multiple"),
                            ("/accordion/animated", "Animated"),
                            ("/accordion/animated-2d", "Animated 2D"),
                            ("/accordion/animated-controlled", "Animated Controlled"),
                            ("/accordion/outside-viewport", "Outside Viewport"),
                            ("/accordion/horizontal", "Horizontal"),
                            ("/accordion/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Alert Dialog" stories=vec![
                            ("/alert-dialog/styled", "Styled"),
                            ("/alert-dialog/controlled", "Controlled"),
                            ("/alert-dialog/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Aspect Ratio" stories=vec![
                            ("/aspect-ratio/styled", "Styled"),
                            ("/aspect-ratio/custom-ratios", "Custom Ratios"),
                            ("/aspect-ratio/chromatic", "Chromatic"),
                        ] />
                        // <NavSection title="Avatar" stories=vec![
                        //     ("/avatar/styled", "Styled"),
                        //     ("/avatar/chromatic", "Chromatic"),
                        // ] />
                        // <NavSection title="Checkbox" stories=vec![
                        //     ("/checkbox/styled", "Styled"),
                        //     ("/checkbox/controlled", "Controlled"),
                        //     ("/checkbox/indeterminate", "Indeterminate"),
                        //     ("/checkbox/within-form", "Within Form"),
                        //     ("/checkbox/animated", "Animated"),
                        //     ("/checkbox/chromatic", "Chromatic"),
                        // ] />
                        <NavSection title="Collapsible" stories=vec![
                            ("/collapsible/styled", "Styled"),
                            ("/collapsible/controlled", "Controlled"),
                            ("/collapsible/animated", "Animated"),
                            ("/collapsible/animated-horizontal", "Animated Horizontal"),
                            ("/collapsible/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Dialog" stories=vec![
                            ("/dialog/styled", "Styled"),
                            ("/dialog/non-modal", "Non Modal"),
                            ("/dialog/controlled", "Controlled"),
                            ("/dialog/focus-trap", "Focus Trap"),
                            ("/dialog/custom-focus", "Custom Focus"),
                            ("/dialog/no-escape-dismiss", "No Escape Dismiss"),
                            ("/dialog/no-pointer-down-outside-dismiss", "No Pointer Down Outside Dismiss"),
                            ("/dialog/with-portal-container", "With Portal Container"),
                            ("/dialog/animated", "Animated"),
                            ("/dialog/forced-mount", "Forced Mount"),
                            ("/dialog/inner-scrollable", "Inner Scrollable"),
                            ("/dialog/outer-scrollable", "Outer Scrollable"),
                            ("/dialog/chromatic", "Chromatic"),
                            ("/dialog/cypress", "Cypress"),
                        ] />
                        <NavSection title="Form" stories=vec![
                            ("/form/basic", "Basic"),
                            ("/form/cypress", "Cypress"),
                        ] />
                        <NavSection title="Hover Card" stories=vec![
                            ("/hover-card/basic", "Basic"),
                            ("/hover-card/contain-text-selection", "Contain Text Selection"),
                            ("/hover-card/async-update", "Async Update"),
                            ("/hover-card/custom-durations", "Custom Durations"),
                            ("/hover-card/controlled", "Controlled"),
                            ("/hover-card/layerable", "Layerable"),
                            ("/hover-card/animated", "Animated"),
                            ("/hover-card/forced-mount", "Forced Mount"),
                            ("/hover-card/nested", "Nested"),
                            ("/hover-card/non-portal", "Non Portal"),
                            ("/hover-card/with-slotted-trigger", "With Slotted Trigger"),
                            ("/hover-card/with-slotted-content", "With Slotted Content"),
                            ("/hover-card/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Label" stories=vec![
                            ("/label/styled", "Styled"),
                            ("/label/with-control", "With Control"),
                            ("/label/with-input-number", "With Input Number"),
                        ] />
                        // <NavSection title="Menu" stories=vec![
                        //     ("/menu/styled", "Styled"),
                        //     ("/menu/submenus", "Submenus"),
                        //     ("/menu/with-labels", "With Labels"),
                        //     ("/menu/typeahead", "Typeahead"),
                        //     ("/menu/checkbox-items", "Checkbox Items"),
                        //     ("/menu/radio-items", "Radio Items"),
                        //     ("/menu/animated", "Animated"),
                        // ] />
                        <NavSection title="One Time Password Field" tested=false stories=vec![
                            ("/one-time-password-field/uncontrolled", "Uncontrolled"),
                            ("/one-time-password-field/controlled", "Controlled"),
                        ] />
                        <NavSection title="Password Toggle Field" stories=vec![
                            ("/password-toggle-field/uncontrolled", "Uncontrolled"),
                            ("/password-toggle-field/controlled", "Controlled"),
                            ("/password-toggle-field/inside-form", "Inside Form"),
                        ] />
                        <NavSection title="Popover" stories=vec![
                            ("/popover/styled", "Styled"),
                            ("/popover/boundary", "Boundary"),
                            ("/popover/modality", "Modality"),
                            ("/popover/controlled", "Controlled"),
                            ("/popover/animated", "Animated"),
                            ("/popover/forced-mount", "Forced Mount"),
                            ("/popover/nested", "Nested"),
                            ("/popover/custom-anchor", "Custom Anchor"),
                            ("/popover/with-slotted-trigger", "With Slotted Trigger"),
                            ("/popover/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Progress" stories=vec![
                            ("/progress/styled", "Styled"),
                            ("/progress/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Radio Group" stories=vec![
                            ("/radio-group/styled", "Styled"),
                            ("/radio-group/controlled", "Controlled"),
                            ("/radio-group/unset", "Unset"),
                            ("/radio-group/within-form", "Within Form"),
                            ("/radio-group/animated", "Animated"),
                            ("/radio-group/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Scroll Area" stories=vec![
                            ("/scroll-area/basic", "Basic"),
                            ("/scroll-area/resizable", "Resizable"),
                            ("/scroll-area/content-change", "Content Change"),
                            ("/scroll-area/animated", "Animated"),
                            ("/scroll-area/chromatic", "Chromatic"),
                            ("/scroll-area/chromatic-dynamic-content-before-loaded", "Chromatic Dynamic Content Before Loaded"),
                            ("/scroll-area/chromatic-dynamic-content-after-loaded", "Chromatic Dynamic Content After Loaded"),
                        ] />
                        <NavSection title="Separator" stories=vec![
                            ("/separator/styled", "Styled"),
                        ] />
                        <NavSection title="Slider" stories=vec![
                            ("/slider/styled", "Styled"),
                            ("/slider/with-on-value-commit", "With On Value Commit"),
                            ("/slider/right-to-left", "Right To Left"),
                            ("/slider/horizontal", "Horizontal"),
                            ("/slider/vertical", "Vertical"),
                            ("/slider/inversions", "Inversions"),
                            ("/slider/with-minimum-steps-between-thumbs", "With Min Steps Between Thumbs"),
                            ("/slider/with-multiple-ranges", "With Multiple Ranges"),
                            ("/slider/small-steps", "Small Steps"),
                            ("/slider/within-form", "Within Form"),
                            ("/slider/strict", "Strict"),
                            ("/slider/chromatic", "Chromatic"),
                        ] />
                        // <NavSection title="Switch" stories=vec![
                        //     ("/switch/styled", "Styled"),
                        //     ("/switch/controlled", "Controlled"),
                        //     ("/switch/within-form", "Within Form"),
                        //     ("/switch/chromatic", "Chromatic"),
                        // ] />
                        <NavSection title="Tabs" stories=vec![
                            ("/tabs/styled", "Styled"),
                            ("/tabs/animated", "Animated"),
                            ("/tabs/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Toggle" stories=vec![
                            ("/toggle/styled", "Styled"),
                            ("/toggle/controlled", "Controlled"),
                            ("/toggle/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Toggle Group" stories=vec![
                            ("/toggle-group/single", "Single"),
                            ("/toggle-group/vertical", "Vertical"),
                            ("/toggle-group/multiple", "Multiple"),
                            ("/toggle-group/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Toolbar" tested=false stories=vec![
                            ("/toolbar/styled", "Styled"),
                            ("/toolbar/chromatic", "Chromatic"),
                        ] />
                    </NavTier>

                    // -- Utilities --
                    <NavTier title="Utilities">
                        <NavSection title="Accessible Icon" tested=false stories=vec![
                            ("/accessible-icon/styled", "Styled"),
                            ("/accessible-icon/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Arrow" stories=vec![
                            ("/arrow/styled", "Styled"),
                            ("/arrow/custom-sizes", "Custom Sizes"),
                            ("/arrow/custom-arrow", "Custom Arrow"),
                        ] />
                        <NavSection title="Collection" stories=vec![
                            ("/collection/basic", "Basic"),
                            ("/collection/with-element-in-between", "With Element In Between"),
                            ("/collection/with-wrapped-item", "With Wrapped Item"),
                            ("/collection/with-fragment", "With Fragment"),
                            ("/collection/dynamic-insertion", "Dynamic Insertion"),
                            ("/collection/with-changing-item", "With Changing Item"),
                            ("/collection/nested", "Nested"),
                        ] />
                        <NavSection title="Dismissable Layer" stories=vec![
                            ("/dismissable-layer/basic", "Basic"),
                            ("/dismissable-layer/nested", "Nested"),
                            ("/dismissable-layer/with-focus-scope", "With Focus Scope"),
                            ("/dismissable-layer/dialog-example", "Dialog Example"),
                            ("/dismissable-layer/popover-fully-modal", "Popover Fully Modal"),
                            ("/dismissable-layer/popover-semi-modal", "Popover Semi Modal"),
                            ("/dismissable-layer/popover-non-modal", "Popover Non Modal"),
                            ("/dismissable-layer/popover-in-dialog", "Popover In Dialog"),
                            ("/dismissable-layer/popover-nested", "Popover Nested"),
                        ] />
                        <NavSection title="Focus Scope" stories=vec![
                            ("/focus-scope/basic", "Basic"),
                            ("/focus-scope/multiple", "Multiple"),
                            ("/focus-scope/with-options", "With Options"),
                        ] />
                        <NavSection title="Popper" stories=vec![
                            ("/popper/styled", "Styled"),
                            ("/popper/with-custom-arrow", "With Custom Arrow"),
                            ("/popper/animated", "Animated"),
                            ("/popper/with-portal", "With Portal"),
                            ("/popper/with-update-position-strategy-always", "With Update Position Strategy Always"),
                            ("/popper/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Portal" stories=vec![
                            ("/portal/base", "Base"),
                            ("/portal/custom-container", "Custom Container"),
                            ("/portal/chromatic", "Chromatic"),
                        ] />
                        <NavSection title="Presence" stories=vec![
                            ("/presence/basic", "Basic"),
                            ("/presence/with-mount-animation", "With Mount Animation"),
                            ("/presence/with-unmount-animation", "With Unmount Animation"),
                            ("/presence/with-multiple-mount-animations", "With Multiple Mount Animations"),
                            ("/presence/with-open-and-close-animation", "With Open and Close Animation"),
                            ("/presence/with-multiple-open-and-close-animations", "With Multiple Open and Close Animations"),
                            ("/presence/with-deferred-mount-animation", "With Deferred Mount Animation"),
                        ] />
                        <NavSection title="Roving Focus" stories=vec![
                            ("/roving-focus/basic", "Basic"),
                            ("/roving-focus/nested", "Nested"),
                            ("/roving-focus/edge-cases", "Edge Cases"),
                        ] />
                        // <NavSection title="Slot" stories=vec![
                        //     ("/slot/without-slottable", "Without Slottable"),
                        //     ("/slot/with-slottable", "With Slottable"),
                        // ] />
                        <NavSection title="Visually Hidden" stories=vec![
                            ("/visually-hidden/basic", "Basic"),
                        ] />
                    </NavTier>
                </ul>
            </nav>
            <main style="position: fixed; top: 0; bottom: 0; left: 16rem; right: 0;">
                <StoryIframe />
            </main>
        </Router>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let is_embed = web_sys::window()
        .expect("Window should exist.")
        .location()
        .search()
        .unwrap_or_default()
        .contains("embed");

    if is_embed {
        EmbedApp().into_any()
    } else {
        ShellApp().into_any()
    }
}
