use chrono::NaiveTime;
use leptos::{context::Provider, ev, html, prelude::*};
use leptos_node_ref::AnyNodeRef;

use crate::support::compose_refs::use_composed_refs;
use crate::support::primitive::{Primitive, data_attr, prop_or, prop_or_default};

use super::{
    DigitResult, HourCycle, SegmentInfo, SegmentType, TimeFieldContextValue, TimeGranularity,
    compute_segments, decompose_time, editable_segment_count, focus_segment,
    format_segment_value, process_digit, reconstruct_time, segment_bounds,
    segment_value_text, wrap_value,
};

// ── TimeField (Root) ─────────────────────────────────────────────────

/// Root time field component.
///
/// Manages the time value, segment state, and provides context for all
/// child components. Wrap a [`TimeFieldInput`] inside this.
#[component]
pub fn TimeField(
    /// The controlled time value.
    #[prop(into, optional)]
    value: MaybeProp<NaiveTime>,
    /// Default time value (uncontrolled).
    #[prop(into, optional)]
    default_value: MaybeProp<NaiveTime>,
    /// Fires when the time changes (all segments filled).
    #[prop(into, optional)]
    on_value_change: Option<Callback<NaiveTime>>,
    /// 12-hour or 24-hour format (default: H12).
    #[prop(into, optional)]
    hour_cycle: MaybeProp<HourCycle>,
    /// Which segments to display (default: Minute).
    #[prop(into, optional)]
    granularity: MaybeProp<TimeGranularity>,
    /// Earliest allowed time.
    #[prop(into, optional)]
    min_value: MaybeProp<NaiveTime>,
    /// Latest allowed time.
    #[prop(into, optional)]
    max_value: MaybeProp<NaiveTime>,
    /// When `true`, all segments are disabled.
    #[prop(into, optional)]
    disabled: MaybeProp<bool>,
    /// When `true`, values cannot be changed.
    #[prop(into, optional)]
    read_only: MaybeProp<bool>,
    /// When `true`, the field is required for form validation.
    #[prop(into, optional)]
    required: MaybeProp<bool>,
    /// Form field name. When set, a hidden `<input>` is rendered for
    /// native form submission.
    #[prop(into, optional)]
    name: MaybeProp<String>,
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
    children: ChildrenFn,
) -> impl IntoView {
    let children = StoredValue::new(children);

    let hour_cycle = prop_or(hour_cycle, HourCycle::H12);
    let granularity = prop_or(granularity, TimeGranularity::Minute);
    let disabled = prop_or_default(disabled);
    let read_only = prop_or_default(read_only);
    let required = prop_or_default(required);
    let min_val = Signal::derive(move || min_value.get());
    let max_val = Signal::derive(move || max_value.get());

    // ── Initialize segment state from initial value ──

    let initial_time = value
        .get_untracked()
        .or_else(|| default_value.get_untracked());
    let initial_hc = hour_cycle.get_untracked();

    let (init_h, init_m, init_s, init_p) = match initial_time {
        Some(t) => {
            let (h, m, s, p) = decompose_time(t, initial_hc);
            (Some(h), Some(m), Some(s), Some(p))
        }
        None => (None, None, None, None),
    };

    let hour_val = RwSignal::new(init_h);
    let minute_val = RwSignal::new(init_m);
    let second_val = RwSignal::new(init_s);
    let day_period_val = RwSignal::new(init_p);
    let digit_buffer = RwSignal::new(String::new());

    // ── Controlled / uncontrolled value ──

    let is_controlled = on_value_change.is_some();

    // Callback for when a complete time is reconstructed from segments.
    let on_complete = Callback::new(move |time: NaiveTime| {
        if is_controlled {
            if let Some(cb) = on_value_change {
                cb.run(time);
            }
        }
    });

    // Sync controlled value prop → segment signals.
    if is_controlled {
        Effect::new(move |prev: Option<Option<NaiveTime>>| {
            let current = value.get();
            // Only sync if the prop actually changed (avoid overwriting in-progress edits).
            if prev.is_some() && current != prev.unwrap() {
                let hc = hour_cycle.get_untracked();
                match current {
                    Some(time) => {
                        let (h, m, s, p) = decompose_time(time, hc);
                        if hour_val.get_untracked() != Some(h) {
                            hour_val.set(Some(h));
                        }
                        if minute_val.get_untracked() != Some(m) {
                            minute_val.set(Some(m));
                        }
                        if second_val.get_untracked() != Some(s) {
                            second_val.set(Some(s));
                        }
                        if hc == HourCycle::H12 && day_period_val.get_untracked() != Some(p) {
                            day_period_val.set(Some(p));
                        }
                    }
                    None => {
                        hour_val.set(None);
                        minute_val.set(None);
                        second_val.set(None);
                        day_period_val.set(None);
                    }
                }
            }
            current
        });
    }

    // ── Input ref ──

    let input_ref = AnyNodeRef::new();

    // ── Context ──

    let context = TimeFieldContextValue {
        hour_cycle,
        granularity,
        disabled,
        read_only,
        required,
        min_value: min_val,
        max_value: max_val,
        hour: hour_val,
        minute: minute_val,
        second: second_val,
        day_period: day_period_val,
        digit_buffer,
        input_ref,
        on_complete,
    };

    // ── Hidden form input ──

    let has_name = Signal::derive(move || name.get().is_some());
    let form_value = Signal::derive(move || {
        let hc = hour_cycle.get();
        let gran = granularity.get();
        let time = reconstruct_time(
            hour_val.get(),
            minute_val.get(),
            second_val.get(),
            day_period_val.get(),
            hc,
            gran,
        );
        match time {
            Some(t) => t.format("%H:%M:%S").to_string(),
            None => String::new(),
        }
    });

    view! {
        <Provider value=context>
            <Primitive
                element=html::div
                as_child=as_child
                node_ref=node_ref
                attr:data-disabled=data_attr(disabled)
            >
                {children.with_value(|children| children())}
                <Show when=move || has_name.get()>
                    <input
                        type="hidden"
                        name=move || name.get().unwrap_or_default()
                        value=form_value
                    />
                </Show>
            </Primitive>
        </Provider>
    }
}

// ── TimeFieldInput ───────────────────────────────────────────────────

/// Container for time field segments.
///
/// Renders as a `<div>` with `role="group"` containing all time segments
/// (both editable spinbuttons and literal separators). Segments are
/// auto-rendered based on the parent [`TimeField`]'s `hour_cycle` and
/// `granularity` configuration.
#[component]
pub fn TimeFieldInput(
    /// Accessible label for the group.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)]
    as_child: MaybeProp<bool>,
    #[prop(into, optional)]
    node_ref: AnyNodeRef,
) -> impl IntoView {
    let context = expect_context::<TimeFieldContextValue>();
    let composed_refs = use_composed_refs(vec![node_ref, context.input_ref]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            node_ref=composed_refs
            attr:role="group"
            attr:aria-label=move || aria_label.get().unwrap_or_else(|| "Time".to_string())
            attr:data-disabled=data_attr(context.disabled)
            attr:data-readonly=move || context.read_only.get().then_some("")
        >
            {move || {
                let hc = context.hour_cycle.get();
                let gran = context.granularity.get();
                let segments = compute_segments(hc, gran);

                segments
                    .into_iter()
                    .map(|seg_info| {
                        view! {
                            <TimeFieldSegment info=seg_info />
                        }
                    })
                    .collect_view()
            }}
        </Primitive>
    }
}

// ── TimeFieldSegment (internal) ──────────────────────────────────────

/// A single segment in the time field.
///
/// Editable segments render as `role="spinbutton"` divs with full
/// keyboard support. Literal segments render as `aria-hidden` spans.
#[component]
fn TimeFieldSegment(info: SegmentInfo) -> impl IntoView {
    let context = expect_context::<TimeFieldContextValue>();

    if info.segment_type == SegmentType::Literal {
        return view! {
            <span
                aria-hidden="true"
                data-type="literal"
            >
                {info.literal}
            </span>
        }
        .into_any();
    }

    // ── Editable segment ──

    let seg_type = info.segment_type;
    let editable_index = info.editable_index.unwrap_or(0);

    // Signal that reads the appropriate segment value.
    let seg_value: Signal<Option<u32>> = Signal::derive(move || match seg_type {
        SegmentType::Hour => context.hour.get(),
        SegmentType::Minute => context.minute.get(),
        SegmentType::Second => context.second.get(),
        SegmentType::DayPeriod => context.day_period.get(),
        SegmentType::Literal => None,
    });

    // Setter for this segment's value.
    let set_seg_value = move |val: Option<u32>| {
        match seg_type {
            SegmentType::Hour => context.hour.set(val),
            SegmentType::Minute => context.minute.set(val),
            SegmentType::Second => context.second.set(val),
            SegmentType::DayPeriod => context.day_period.set(val),
            SegmentType::Literal => {}
        }
    };

    let (min, max) = segment_bounds(seg_type, context.hour_cycle.get_untracked());

    let display_text = Signal::derive(move || {
        format_segment_value(seg_type, seg_value.get(), context.hour_cycle.get())
    });

    let value_text = Signal::derive(move || {
        segment_value_text(seg_type, seg_value.get(), context.hour_cycle.get())
    });

    let is_placeholder = Signal::derive(move || seg_value.get().is_none());

    // Try to reconstruct and emit the complete time.
    let try_emit = move || {
        let hc = context.hour_cycle.get_untracked();
        let gran = context.granularity.get_untracked();
        if let Some(time) = reconstruct_time(
            context.hour.get_untracked(),
            context.minute.get_untracked(),
            context.second.get_untracked(),
            context.day_period.get_untracked(),
            hc,
            gran,
        ) {
            context.on_complete.run(time);
        }
    };

    let total_editable = Signal::derive(move || {
        editable_segment_count(context.hour_cycle.get(), context.granularity.get())
    });

    // ── Keyboard handler ──

    let on_keydown = move |event: ev::KeyboardEvent| {
        if context.disabled.get_untracked() || context.read_only.get_untracked() {
            return;
        }

        let key = event.key();
        let hc = context.hour_cycle.get_untracked();
        let (seg_min, seg_max) = segment_bounds(seg_type, hc);

        match key.as_str() {
            "ArrowUp" => {
                event.prevent_default();
                let new_val = match seg_value.get_untracked() {
                    Some(current) => wrap_value(current, 1, seg_min, seg_max),
                    None => seg_min, // From placeholder, start at min
                };
                set_seg_value(Some(new_val));
                context.digit_buffer.set(String::new());
                try_emit();
            }
            "ArrowDown" => {
                event.prevent_default();
                let new_val = match seg_value.get_untracked() {
                    Some(current) => wrap_value(current, -1, seg_min, seg_max),
                    None => seg_max, // From placeholder, start at max
                };
                set_seg_value(Some(new_val));
                context.digit_buffer.set(String::new());
                try_emit();
            }
            "ArrowLeft" => {
                event.prevent_default();
                // Commit any pending digits.
                context.digit_buffer.set(String::new());
                if editable_index > 0 {
                    focus_segment(context.input_ref, editable_index - 1);
                }
            }
            "ArrowRight" => {
                event.prevent_default();
                context.digit_buffer.set(String::new());
                if editable_index + 1 < total_editable.get_untracked() {
                    focus_segment(context.input_ref, editable_index + 1);
                }
            }
            "Home" => {
                event.prevent_default();
                set_seg_value(Some(seg_min));
                context.digit_buffer.set(String::new());
                try_emit();
            }
            "End" => {
                event.prevent_default();
                set_seg_value(Some(seg_max));
                context.digit_buffer.set(String::new());
                try_emit();
            }
            "Backspace" | "Delete" => {
                event.prevent_default();
                set_seg_value(None);
                context.digit_buffer.set(String::new());
            }
            "a" | "A" if seg_type == SegmentType::DayPeriod => {
                event.prevent_default();
                set_seg_value(Some(0)); // AM
                context.digit_buffer.set(String::new());
                try_emit();
            }
            "p" | "P" if seg_type == SegmentType::DayPeriod => {
                event.prevent_default();
                set_seg_value(Some(1)); // PM
                context.digit_buffer.set(String::new());
                try_emit();
            }
            digit if digit.len() == 1 && digit.as_bytes()[0].is_ascii_digit() => {
                event.prevent_default();
                if seg_type == SegmentType::DayPeriod {
                    return; // Digits don't apply to AM/PM.
                }
                let ch = digit.as_bytes()[0] as char;
                let buffer = context.digit_buffer.get_untracked();
                let (result, new_buffer) = process_digit(&buffer, ch, seg_max);
                context.digit_buffer.set(new_buffer);

                match result {
                    DigitResult::Wait(val) => {
                        set_seg_value(Some(val));
                    }
                    DigitResult::Advance(val) => {
                        set_seg_value(Some(val));
                        try_emit();
                        // Advance to next segment.
                        if editable_index + 1 < total_editable.get_untracked() {
                            focus_segment(context.input_ref, editable_index + 1);
                        }
                    }
                    DigitResult::Reject => {}
                }
            }
            _ => {}
        }
    };

    // Clear digit buffer when focus leaves this segment.
    let on_blur = move |_: ev::FocusEvent| {
        context.digit_buffer.set(String::new());
    };

    // ── Render ──

    let aria_valuenow = Signal::derive(move || {
        seg_value
            .get()
            .map(|v| v.to_string())
            .unwrap_or_default()
    });

    view! {
        <div
            role="spinbutton"
            tabindex=move || if context.disabled.get() { "-1" } else { "0" }
            aria-label=seg_type.aria_label()
            aria-valuenow=aria_valuenow
            aria-valuemin=min.to_string()
            aria-valuemax=max.to_string()
            aria-valuetext=value_text
            data-type=seg_type.data_attr()
            data-editable-index=editable_index.to_string()
            data-placeholder=move || is_placeholder.get().then_some("")
            data-disabled=move || context.disabled.get().then_some("")
            data-readonly=move || context.read_only.get().then_some("")
            on:keydown=on_keydown
            on:blur=on_blur
        >
            {display_text}
        </div>
    }
    .into_any()
}
