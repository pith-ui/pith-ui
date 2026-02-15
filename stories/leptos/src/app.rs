use leptos::prelude::*;
use leptos_router::{
    components::{A, Route, Router, Routes, ToHref},
    path,
};

use crate::primitives::{
    accessible_icon, accordion, alert_dialog, arrow, aspect_ratio, collapsible, collection, dialog,
    dismissable_layer, focus_scope, form, label, one_time_password_field, password_toggle_field,
    popper, portal, presence, progress, radio_group, roving_focus, scroll_area, separator, slider,
    tabs, toggle, toggle_group, toolbar, visually_hidden,
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

#[component]
fn Index() -> impl IntoView {
    view! {
        <h1>Radix Leptos Stories</h1>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 box-border overflow-y-auto leading-normal">
                <ul class="list-none m-0 p-0">
                    <li>
                        <NavLink href="/">Index</NavLink>
                    </li>
                    <li>
                        Accordion

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/accordion/single">Single</NavLink></li>
                            <li><NavLink href="/accordion/multiple">Multiple</NavLink></li>
                            <li><NavLink href="/accordion/animated">Animated</NavLink></li>
                            <li><NavLink href="/accordion/animated-2d">Animated 2D</NavLink></li>
                            <li><NavLink href="/accordion/animated-controlled">Animated Controlled</NavLink></li>
                            <li><NavLink href="/accordion/outside-viewport">Outside Viewport</NavLink></li>
                            <li><NavLink href="/accordion/horizontal">Horizontal</NavLink></li>
                            <li><NavLink href="/accordion/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Accessible Icon

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/accessible-icon/styled">Styled</NavLink></li>
                            <li><NavLink href="/accessible-icon/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Alert Dialog

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/alert-dialog/styled">Styled</NavLink></li>
                            <li><NavLink href="/alert-dialog/controlled">Controlled</NavLink></li>
                            <li><NavLink href="/alert-dialog/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Arrow

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/arrow/styled">Styled</NavLink></li>
                            <li><NavLink href="/arrow/custom-sizes">Custom Sizes</NavLink></li>
                            <li><NavLink href="/arrow/custom-arrow">Custom Arrow</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Aspect Ratio

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/aspect-ratio/styled">Styled</NavLink></li>
                            <li><NavLink href="/aspect-ratio/custom-ratios">Custom Ratios</NavLink></li>
                            <li><NavLink href="/aspect-ratio/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Collapsible

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/collapsible/styled">Styled</NavLink></li>
                            <li><NavLink href="/collapsible/controlled">Controlled</NavLink></li>
                            <li><NavLink href="/collapsible/animated">Animated</NavLink></li>
                            <li><NavLink href="/collapsible/animated-horizontal">Animated Horizontal</NavLink></li>
                            <li><NavLink href="/collapsible/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Dialog

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/dialog/styled">Styled</NavLink></li>
                            <li><NavLink href="/dialog/non-modal">Non Modal</NavLink></li>
                            <li><NavLink href="/dialog/controlled">Controlled</NavLink></li>
                            <li><NavLink href="/dialog/focus-trap">Focus Trap</NavLink></li>
                            <li><NavLink href="/dialog/custom-focus">Custom Focus</NavLink></li>
                            <li><NavLink href="/dialog/no-escape-dismiss">No Escape Dismiss</NavLink></li>
                            <li><NavLink href="/dialog/no-pointer-down-outside-dismiss">No Pointer Down Outside Dismiss</NavLink></li>
                            <li><NavLink href="/dialog/with-portal-container">With Portal Container</NavLink></li>
                            <li><NavLink href="/dialog/animated">Animated</NavLink></li>
                            <li><NavLink href="/dialog/forced-mount">Forced Mount</NavLink></li>
                            <li><NavLink href="/dialog/inner-scrollable">Inner Scrollable</NavLink></li>
                            <li><NavLink href="/dialog/outer-scrollable">Outer Scrollable</NavLink></li>
                            <li><NavLink href="/dialog/chromatic">Chromatic</NavLink></li>
                            <li><NavLink href="/dialog/cypress">Cypress</NavLink></li>
                        </ul>
                    </li>
                    // <li>
                    //     Avatar

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/avatar/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/avatar/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    // <li>
                    //     Checkbox

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/checkbox/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/checkbox/controlled">Controlled</NavLink></li>
                    //         <li><NavLink href="/checkbox/indeterminate">Indeterminate</NavLink></li>
                    //         <li><NavLink href="/checkbox/within-form">Within Form</NavLink></li>
                    //         <li><NavLink href="/checkbox/animated">Animated</NavLink></li>
                    //         <li><NavLink href="/checkbox/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    <li>
                        Dismissable Layer

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/dismissable-layer/basic">Basic</NavLink></li>
                            <li><NavLink href="/dismissable-layer/nested">Nested</NavLink></li>
                            <li><NavLink href="/dismissable-layer/with-focus-scope">With Focus Scope</NavLink></li>
                            <li><NavLink href="/dismissable-layer/dialog-example">Dialog Example</NavLink></li>
                            <li><NavLink href="/dismissable-layer/popover-fully-modal">Popover Fully Modal</NavLink></li>
                            <li><NavLink href="/dismissable-layer/popover-semi-modal">Popover Semi Modal</NavLink></li>
                            <li><NavLink href="/dismissable-layer/popover-non-modal">Popover Non Modal</NavLink></li>
                            <li><NavLink href="/dismissable-layer/popover-in-dialog">Popover In Dialog</NavLink></li>
                            <li><NavLink href="/dismissable-layer/popover-nested">Popover Nested</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Collection

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/collection/basic">Basic</NavLink></li>
                            <li><NavLink href="/collection/with-element-in-between">With Element In Between</NavLink></li>
                            <li><NavLink href="/collection/with-wrapped-item">With Wrapped Item</NavLink></li>
                            <li><NavLink href="/collection/with-fragment">With Fragment</NavLink></li>
                            <li><NavLink href="/collection/dynamic-insertion">Dynamic Insertion</NavLink></li>
                            <li><NavLink href="/collection/with-changing-item">With Changing Item</NavLink></li>
                            <li><NavLink href="/collection/nested">Nested</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Focus Scope

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/focus-scope/basic">Basic</NavLink></li>
                            <li><NavLink href="/focus-scope/multiple">Multiple</NavLink></li>
                            <li><NavLink href="/focus-scope/with-options">With Options</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Form

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/form/basic">Basic</NavLink></li>
                            <li><NavLink href="/form/cypress">Cypress</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Label

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/label/styled">Styled</NavLink></li>
                            <li><NavLink href="/label/with-control">With Control</NavLink></li>
                            <li><NavLink href="/label/with-input-number">With Input Number</NavLink></li>
                        </ul>
                    </li>
                    // <li>
                    //     Menu

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/menu/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/menu/submenus">Submenus</NavLink></li>
                    //         <li><NavLink href="/menu/with-labels">With Labels</NavLink></li>
                    //         <li><NavLink href="/menu/typeahead">Typeahead</NavLink></li>
                    //         <li><NavLink href="/menu/checkbox-items">Checkbox Items</NavLink></li>
                    //         <li><NavLink href="/menu/radio-items">Radio Items</NavLink></li>
                    //         <li><NavLink href="/menu/animated">Animated</NavLink></li>
                    //     </ul>
                    // </li>
                    <li>
                        {"One Time Password Field"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/one-time-password-field/uncontrolled">Uncontrolled</NavLink></li>
                            <li><NavLink href="/one-time-password-field/controlled">Controlled</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        {"Password Toggle Field"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/password-toggle-field/uncontrolled">Uncontrolled</NavLink></li>
                            <li><NavLink href="/password-toggle-field/controlled">Controlled</NavLink></li>
                            <li><NavLink href="/password-toggle-field/inside-form">Inside Form</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Popper

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/popper/styled">Styled</NavLink></li>
                            <li><NavLink href="/popper/with-custom-arrow">With Custom Arrow</NavLink></li>
                            <li><NavLink href="/popper/animated">Animated</NavLink></li>
                            <li><NavLink href="/popper/with-portal">With Portal</NavLink></li>
                            <li><NavLink href="/popper/with-update-position-strategy-always">With Update Position Strategy Always</NavLink></li>
                            <li><NavLink href="/popper/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Portal

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/portal/base">Base</NavLink></li>
                            <li><NavLink href="/portal/custom-container">Custom Container</NavLink></li>
                            <li><NavLink href="/portal/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Presence

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/presence/basic">Basic</NavLink></li>
                            <li><NavLink href="/presence/with-mount-animation">With Mount Animation</NavLink></li>
                            <li><NavLink href="/presence/with-unmount-animation">With Unmount Animation</NavLink></li>
                            <li><NavLink href="/presence/with-multiple-mount-animations">With Multiple Mount Animations</NavLink></li>
                            <li><NavLink href="/presence/with-open-and-close-animation">With Open and Close Animation</NavLink></li>
                            <li><NavLink href="/presence/with-multiple-open-and-close-animations">With Multiple Open and Close Animations</NavLink></li>
                            <li><NavLink href="/presence/with-deferred-mount-animation">With Deferred Mount Animation</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Progress

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/progress/styled">Styled</NavLink></li>
                            <li><NavLink href="/progress/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Radio Group

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/radio-group/styled">Styled</NavLink></li>
                            <li><NavLink href="/radio-group/controlled">Controlled</NavLink></li>
                            <li><NavLink href="/radio-group/unset">Unset</NavLink></li>
                            <li><NavLink href="/radio-group/within-form">Within Form</NavLink></li>
                            <li><NavLink href="/radio-group/animated">Animated</NavLink></li>
                            <li><NavLink href="/radio-group/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Roving Focus

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/roving-focus/basic">Basic</NavLink></li>
                            <li><NavLink href="/roving-focus/nested">Nested</NavLink></li>
                            <li><NavLink href="/roving-focus/edge-cases">Edge Cases</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Scroll Area

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/scroll-area/basic">Basic</NavLink></li>
                            <li><NavLink href="/scroll-area/resizable">Resizable</NavLink></li>
                            <li><NavLink href="/scroll-area/content-change">Content Change</NavLink></li>
                            <li><NavLink href="/scroll-area/animated">Animated</NavLink></li>
                            <li><NavLink href="/scroll-area/chromatic">Chromatic</NavLink></li>
                            <li><NavLink href="/scroll-area/chromatic-dynamic-content-before-loaded">Chromatic Dynamic Content Before Loaded</NavLink></li>
                            <li><NavLink href="/scroll-area/chromatic-dynamic-content-after-loaded">Chromatic Dynamic Content After Loaded</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Separator

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/separator/styled">Styled</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Slider

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/slider/styled">Styled</NavLink></li>
                            <li><NavLink href="/slider/with-on-value-commit">With On Value Commit</NavLink></li>
                            <li><NavLink href="/slider/right-to-left">Right To Left</NavLink></li>
                            <li><NavLink href="/slider/horizontal">Horizontal</NavLink></li>
                            <li><NavLink href="/slider/vertical">Vertical</NavLink></li>
                            <li><NavLink href="/slider/inversions">Inversions</NavLink></li>
                            <li><NavLink href="/slider/with-minimum-steps-between-thumbs">With Min Steps Between Thumbs</NavLink></li>
                            <li><NavLink href="/slider/with-multiple-ranges">With Multiple Ranges</NavLink></li>
                            <li><NavLink href="/slider/small-steps">Small Steps</NavLink></li>
                            <li><NavLink href="/slider/within-form">Within Form</NavLink></li>
                            <li><NavLink href="/slider/strict">Strict</NavLink></li>
                            <li><NavLink href="/slider/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Tabs

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/tabs/styled">Styled</NavLink></li>
                            <li><NavLink href="/tabs/animated">Animated</NavLink></li>
                            <li><NavLink href="/tabs/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    // <li>
                    //     Slot

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/slot/without-slottable">Without Slottable</NavLink></li>
                    //         <li><NavLink href="/slot/with-slottable">With Slottable</NavLink></li>
                    //         // TODO
                    //     </ul>
                    // </li>
                    // <li>
                    //     Switch

                    //     <ul class="list-none m-0 ms-4 p-0">
                    //         <li><NavLink href="/switch/styled">Styled</NavLink></li>
                    //         <li><NavLink href="/switch/controlled">Controlled</NavLink></li>
                    //         <li><NavLink href="/switch/within-form">Within Form</NavLink></li>
                    //         <li><NavLink href="/switch/chromatic">Chromatic</NavLink></li>
                    //     </ul>
                    // </li>
                    <li>
                        Toggle

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/toggle/styled">Styled</NavLink></li>
                            <li><NavLink href="/toggle/controlled">Controlled</NavLink></li>
                            <li><NavLink href="/toggle/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Toggle Group

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/toggle-group/single">Single</NavLink></li>
                            <li><NavLink href="/toggle-group/vertical">Vertical</NavLink></li>
                            <li><NavLink href="/toggle-group/multiple">Multiple</NavLink></li>
                            <li><NavLink href="/toggle-group/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Toolbar

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/toolbar/styled">Styled</NavLink></li>
                            <li><NavLink href="/toolbar/chromatic">Chromatic</NavLink></li>
                        </ul>
                    </li>
                    <li>
                        Visually Hidden

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink href="/visually-hidden/basic">Basic</NavLink></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
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
