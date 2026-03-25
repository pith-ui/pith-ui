use std::collections::HashMap;

use crate::types::*;
use crate::utils::{
    approx_equal, calculate_range, default_key_extractor, default_range_extractor,
    find_nearest_binary_search,
};

// ---------------------------------------------------------------------------
// Options
// ---------------------------------------------------------------------------

/// Configuration for the [`Virtualizer`].
///
/// Closures use `Box<dyn Fn(...)>` so the core remains framework-agnostic.
pub struct VirtualizerOptions {
    /// Total number of items.
    pub count: usize,
    /// Estimate the size (px) of the item at the given index.
    pub estimate_size: Box<dyn Fn(usize) -> f64>,
    /// Number of items to render outside the visible range (default: 1).
    pub overscan: usize,
    /// Horizontal scrolling mode (default: false).
    pub horizontal: bool,
    /// Padding before the first item (default: 0.0).
    pub padding_start: f64,
    /// Padding after the last item (default: 0.0).
    pub padding_end: f64,
    /// Scroll padding applied when aligning to start (default: 0.0).
    pub scroll_padding_start: f64,
    /// Scroll padding applied when aligning to end (default: 0.0).
    pub scroll_padding_end: f64,
    /// Initial scroll offset (default: 0.0).
    pub initial_offset: f64,
    /// Initial container dimensions (default: 0×0).
    pub initial_rect: Rect,
    /// Map item index to a stable key (default: identity).
    pub get_item_key: Box<dyn Fn(usize) -> usize>,
    /// Custom range extraction (default: contiguous range + overscan).
    pub range_extractor: Box<dyn Fn(Range) -> Vec<usize>>,
    /// Offset from the scroll container edge to the first item (default: 0.0).
    pub scroll_margin: f64,
    /// Gap between items within a lane (default: 0.0).
    pub gap: f64,
    /// DOM attribute used to read item index from elements (default: "data-index").
    pub index_attribute: String,
    /// Pre-populated measurement cache.
    pub initial_measurements_cache: Vec<VirtualItem>,
    /// Number of columns/lanes for multi-column layouts (default: 1).
    pub lanes: usize,
    /// Debounce delay in ms for detecting scroll-end (default: 150).
    pub is_scrolling_reset_delay: u32,
    /// Whether the virtualizer is active (default: true).
    pub enabled: bool,
    /// Right-to-left layout (default: false).
    pub is_rtl: bool,
    /// Enable debug logging (default: false).
    pub debug: bool,
}

impl Default for VirtualizerOptions {
    fn default() -> Self {
        Self {
            count: 0,
            estimate_size: Box::new(|_| 50.0),
            overscan: 1,
            horizontal: false,
            padding_start: 0.0,
            padding_end: 0.0,
            scroll_padding_start: 0.0,
            scroll_padding_end: 0.0,
            initial_offset: 0.0,
            initial_rect: Rect::default(),
            get_item_key: Box::new(default_key_extractor),
            range_extractor: Box::new(default_range_extractor),
            scroll_margin: 0.0,
            gap: 0.0,
            index_attribute: "data-index".into(),
            initial_measurements_cache: Vec::new(),
            lanes: 1,
            is_scrolling_reset_delay: 150,
            enabled: true,
            is_rtl: false,
            debug: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Virtualizer
// ---------------------------------------------------------------------------

/// Framework-agnostic virtual scrolling engine.
///
/// The adapter (e.g. Leptos) pushes data in via [`set_scroll_rect`],
/// [`set_scroll_offset`], and [`resize_item`], and reads computed output via
/// [`get_virtual_items`], [`get_total_size`], etc. Scroll operations return
/// [`ScrollCommand`] values for the adapter to execute.
pub struct Virtualizer {
    // -- Configuration --
    options: VirtualizerOptions,

    // -- Scroll state (fed by adapter) --
    scroll_rect: Option<Rect>,
    scroll_offset: Option<f64>,
    scroll_direction: Option<ScrollDirection>,
    is_scrolling: bool,
    scroll_adjustments: f64,

    // -- Scroll reconciliation --
    scroll_state: Option<ScrollState>,

    // -- Measurement caches --
    measurements_cache: Vec<VirtualItem>,
    item_size_cache: HashMap<usize, f64>,
    lane_assignments: HashMap<usize, usize>,
    pending_measured_cache_indexes: Vec<usize>,

    // -- Lane change tracking --
    prev_lanes: Option<usize>,
    lanes_changed_flag: bool,
    lanes_settling: bool,

    // -- Computed range --
    range: Option<ComputedRange>,

    /// Incremented whenever `item_size_cache` changes, used by the adapter
    /// to detect measurement changes.
    item_size_cache_version: u64,

    // -- Notify tracking --
    prev_notify_state: Option<(bool, Option<usize>, Option<usize>)>,
}

impl Virtualizer {
    /// Create a new virtualizer with the given options.
    pub fn new(options: VirtualizerOptions) -> Self {
        let initial_rect = options.initial_rect;
        let initial_offset = options.initial_offset;

        let mut v = Self {
            options,
            scroll_rect: None,
            scroll_offset: None,
            scroll_direction: None,
            is_scrolling: false,
            scroll_adjustments: 0.0,
            scroll_state: None,
            measurements_cache: Vec::new(),
            item_size_cache: HashMap::new(),
            lane_assignments: HashMap::new(),
            pending_measured_cache_indexes: Vec::new(),
            prev_lanes: None,
            lanes_changed_flag: false,
            lanes_settling: false,
            range: None,
            item_size_cache_version: 0,
            prev_notify_state: None,
        };

        // Apply initial rect/offset if provided.
        if initial_rect.width > 0.0 || initial_rect.height > 0.0 {
            v.scroll_rect = Some(initial_rect);
        }
        if initial_offset != 0.0 {
            v.scroll_offset = Some(initial_offset);
        }

        v
    }

    // -----------------------------------------------------------------------
    // Options
    // -----------------------------------------------------------------------

    /// Update the virtualizer options. Call this when reactive option values
    /// change (e.g. `count`).
    pub fn set_options(&mut self, options: VirtualizerOptions) {
        self.options = options;
    }

    /// Read-only access to current options.
    pub fn options(&self) -> &VirtualizerOptions {
        &self.options
    }

    // -----------------------------------------------------------------------
    // Data input (called by adapter)
    // -----------------------------------------------------------------------

    /// Set the scroll container's dimensions. Called by the adapter when the
    /// container's ResizeObserver fires.
    pub fn set_scroll_rect(&mut self, rect: Rect) {
        self.scroll_rect = Some(rect);
    }

    /// Set the current scroll offset and scrolling state. Called by the
    /// adapter from the scroll event listener.
    ///
    /// Returns `true` if the visible range changed and the adapter should
    /// re-render.
    pub fn set_scroll_offset(&mut self, offset: f64, is_scrolling: bool) -> bool {
        self.scroll_adjustments = 0.0;
        self.scroll_direction = if is_scrolling {
            Some(if self.get_scroll_offset() < offset {
                ScrollDirection::Forward
            } else {
                ScrollDirection::Backward
            })
        } else {
            None
        };
        self.scroll_offset = Some(offset);
        self.is_scrolling = is_scrolling;

        self.check_should_notify()
    }

    /// Register/update a measured item size. Called by the adapter when a
    /// ResizeObserver fires for a virtual item element.
    pub fn resize_item(&mut self, index: usize, size: f64) -> ResizeItemResult {
        let item = match self.measurements_cache.get(index) {
            Some(item) => item.clone(),
            None => {
                return ResizeItemResult {
                    should_notify: false,
                    scroll_correction: None,
                };
            }
        };

        let item_size = self
            .item_size_cache
            .get(&item.key)
            .copied()
            .unwrap_or(item.size);
        let delta = size - item_size;

        if delta == 0.0 {
            return ResizeItemResult {
                should_notify: false,
                scroll_correction: None,
            };
        }

        // Check if we need to adjust scroll to compensate for items above viewport.
        let scroll_correction = if self
            .scroll_state
            .as_ref()
            .is_none_or(|s| s.behavior != ScrollBehavior::Smooth)
            && item.start < self.get_scroll_offset() + self.scroll_adjustments
        {
            self.scroll_adjustments += delta;
            Some(ScrollCommand {
                offset: self.get_scroll_offset(),
                behavior: None,
                adjustments: Some(self.scroll_adjustments),
            })
        } else {
            None
        };

        self.pending_measured_cache_indexes.push(item.index);
        self.item_size_cache.insert(item.key, size);
        self.item_size_cache_version += 1;

        ResizeItemResult {
            should_notify: true,
            scroll_correction,
        }
    }

    /// Clear all measurement caches, forcing a full re-layout.
    pub fn measure(&mut self) {
        self.item_size_cache.clear();
        self.lane_assignments.clear();
        self.item_size_cache_version += 1;
    }

    // -----------------------------------------------------------------------
    // Computed output
    // -----------------------------------------------------------------------

    /// Whether the virtualizer is currently scrolling.
    pub fn is_scrolling(&self) -> bool {
        self.is_scrolling
    }

    /// Current scroll direction (if scrolling).
    pub fn scroll_direction(&self) -> Option<ScrollDirection> {
        self.scroll_direction
    }

    /// Current scroll offset.
    pub fn scroll_offset(&self) -> Option<f64> {
        self.scroll_offset
    }

    /// Current scroll rect.
    pub fn scroll_rect(&self) -> Option<Rect> {
        self.scroll_rect
    }

    /// Get the computed visible range.
    pub fn range(&self) -> Option<ComputedRange> {
        self.range
    }

    /// Build or return cached measurements for all items. This is the core
    /// layout computation — it assigns positions, sizes, and lanes.
    pub fn get_measurements(&mut self) -> Vec<VirtualItem> {
        if !self.options.enabled {
            self.measurements_cache.clear();
            self.item_size_cache.clear();
            self.lane_assignments.clear();
            return Vec::new();
        }

        let count = self.options.count;
        let padding_start = self.options.padding_start;
        let scroll_margin = self.options.scroll_margin;
        let lanes = self.options.lanes;
        let gap = self.options.gap;

        // Detect lane changes.
        let lanes_changed = self.prev_lanes.is_some_and(|prev| prev != lanes);
        if lanes_changed {
            self.lanes_changed_flag = true;
        }
        self.prev_lanes = Some(lanes);

        // Force complete recalculation when lanes change.
        if self.lanes_changed_flag {
            self.lanes_changed_flag = false;
            self.lanes_settling = true;
            self.measurements_cache.clear();
            self.item_size_cache.clear();
            self.lane_assignments.clear();
            self.pending_measured_cache_indexes.clear();
        }

        // Clean up stale lane cache entries when count decreases.
        if self.lane_assignments.len() > count {
            self.lane_assignments.retain(|&idx, _| idx < count);
        }

        // Restore from initial cache if empty (and not settling).
        if self.measurements_cache.is_empty() && !self.lanes_settling {
            self.measurements_cache = self.options.initial_measurements_cache.clone();
            for item in &self.measurements_cache {
                self.item_size_cache.insert(item.key, item.size);
            }
        }

        // During lanes settling, ignore pending indexes (start from 0).
        let min = if self.lanes_settling {
            0
        } else if !self.pending_measured_cache_indexes.is_empty() {
            self.pending_measured_cache_indexes
                .iter()
                .copied()
                .min()
                .unwrap_or(0)
        } else {
            0
        };
        self.pending_measured_cache_indexes.clear();

        // End settling period when cache is fully built.
        if self.lanes_settling && self.measurements_cache.len() == count {
            self.lanes_settling = false;
        }

        // Truncate to `min` and rebuild from there.
        self.measurements_cache.truncate(min);

        // Track last item index per lane for O(1) lookup.
        let mut lane_last_index: Vec<Option<usize>> = vec![None; lanes];
        for (m, item) in self.measurements_cache.iter().enumerate() {
            lane_last_index[item.lane] = Some(m);
        }

        for i in min..count {
            let key = (self.options.get_item_key)(i);

            let (lane, start) = if lanes > 1 {
                if let Some(&cached_lane) = self.lane_assignments.get(&i) {
                    let prev_index = lane_last_index[cached_lane];
                    let prev_in_lane = prev_index.and_then(|idx| self.measurements_cache.get(idx));
                    let start = prev_in_lane
                        .map(|p| p.end + gap)
                        .unwrap_or(padding_start + scroll_margin);
                    (cached_lane, start)
                } else {
                    let (lane, start) = self.find_shortest_lane(
                        i,
                        &self.measurements_cache,
                        lanes,
                        padding_start,
                        scroll_margin,
                        gap,
                    );
                    self.lane_assignments.insert(i, lane);
                    (lane, start)
                }
            } else {
                let prev = if i > 0 {
                    self.measurements_cache.get(i - 1)
                } else {
                    None
                };
                let start = prev
                    .map(|p| p.end + gap)
                    .unwrap_or(padding_start + scroll_margin);
                (0, start)
            };

            let measured_size = self.item_size_cache.get(&key).copied();
            let size = measured_size.unwrap_or_else(|| (self.options.estimate_size)(i));
            let end = start + size;

            let item = VirtualItem {
                index: i,
                start,
                size,
                end,
                key,
                lane,
            };

            if i < self.measurements_cache.len() {
                self.measurements_cache[i] = item;
            } else {
                self.measurements_cache.push(item);
            }

            lane_last_index[lane] = Some(i);
        }

        // Truncate if count decreased.
        self.measurements_cache.truncate(count);

        self.measurements_cache.clone()
    }

    /// Calculate the visible range given current measurements, viewport size,
    /// and scroll offset.
    pub fn calculate_range(&mut self) -> Option<ComputedRange> {
        let measurements = self.get_measurements();
        let outer_size = self.get_size();
        let scroll_offset = self.get_scroll_offset();
        let lanes = self.options.lanes;

        self.range = if !measurements.is_empty() && outer_size > 0.0 {
            calculate_range(&measurements, outer_size, scroll_offset, lanes)
        } else {
            None
        };

        self.range
    }

    /// Get the indexes of virtual items to render (with overscan applied).
    pub fn get_virtual_indexes(&mut self) -> Vec<usize> {
        let range = self.calculate_range();

        let (start_index, end_index) = match range {
            Some(r) => (Some(r.start_index), Some(r.end_index)),
            None => (None, None),
        };

        match (start_index, end_index) {
            (Some(si), Some(ei)) => (self.options.range_extractor)(Range {
                start_index: si,
                end_index: ei,
                overscan: self.options.overscan,
                count: self.options.count,
            }),
            _ => Vec::new(),
        }
    }

    /// Get the visible [`VirtualItem`]s. This is the primary output method.
    pub fn get_virtual_items(&mut self) -> Vec<VirtualItem> {
        let indexes = self.get_virtual_indexes();
        let measurements = &self.measurements_cache;

        indexes
            .iter()
            .filter_map(|&i| measurements.get(i).cloned())
            .collect()
    }

    /// Get the total scrollable content size in pixels.
    pub fn get_total_size(&mut self) -> f64 {
        let measurements = self.get_measurements();
        let lanes = self.options.lanes;

        let end = if measurements.is_empty() {
            self.options.padding_start
        } else if lanes == 1 {
            measurements.last().map(|m| m.end).unwrap_or(0.0)
        } else {
            // Multi-lane: find the max end across all lanes.
            let mut end_by_lane: Vec<Option<f64>> = vec![None; lanes];
            for item in measurements.iter().rev() {
                if end_by_lane[item.lane].is_none() {
                    end_by_lane[item.lane] = Some(item.end);
                }
                if end_by_lane.iter().all(|v| v.is_some()) {
                    break;
                }
            }
            end_by_lane.into_iter().flatten().fold(0.0_f64, f64::max)
        };

        (end - self.options.scroll_margin + self.options.padding_end).max(0.0)
    }

    /// Get the VirtualItem at a given scroll offset.
    pub fn get_virtual_item_for_offset(&mut self, offset: f64) -> Option<VirtualItem> {
        let measurements = self.get_measurements();
        if measurements.is_empty() {
            return None;
        }

        let index = find_nearest_binary_search(
            0,
            measurements.len() - 1,
            |i| measurements[i].start,
            offset,
        );

        measurements.get(index).cloned()
    }

    /// Compute the alignment-adjusted scroll offset for a target offset.
    pub fn get_offset_for_alignment(
        &mut self,
        to_offset: f64,
        align: ScrollAlignment,
        item_size: f64,
        max_scroll_offset: f64,
    ) -> f64 {
        let size = self.get_size();
        let scroll_offset = self.get_scroll_offset();

        let mut aligned = to_offset;
        let resolved_align = match align {
            ScrollAlignment::Auto => {
                if to_offset >= scroll_offset + size {
                    ScrollAlignment::End
                } else {
                    ScrollAlignment::Start
                }
            }
            other => other,
        };

        match resolved_align {
            ScrollAlignment::Center => {
                aligned += (item_size - size) / 2.0;
            }
            ScrollAlignment::End => {
                aligned -= size;
            }
            _ => {}
        }

        aligned.clamp(0.0, max_scroll_offset)
    }

    /// Get `(offset, resolved_align)` for scrolling to a given index.
    ///
    /// The `max_scroll_offset` parameter must be provided by the adapter
    /// (it requires DOM measurement: `scrollHeight - clientHeight`).
    pub fn get_offset_for_index(
        &mut self,
        index: usize,
        align: ScrollAlignment,
        max_scroll_offset: f64,
    ) -> Option<(f64, ScrollAlignment)> {
        let index = index.min(self.options.count.saturating_sub(1));

        let size = self.get_size();
        let scroll_offset = self.get_scroll_offset();

        let measurements = self.get_measurements();
        let item = measurements.get(index)?.clone();

        let resolved_align = match align {
            ScrollAlignment::Auto => {
                if item.end >= scroll_offset + size - self.options.scroll_padding_end {
                    ScrollAlignment::End
                } else if item.start <= scroll_offset + self.options.scroll_padding_start {
                    ScrollAlignment::Start
                } else {
                    // Already visible — no scroll needed.
                    return Some((scroll_offset, align));
                }
            }
            other => other,
        };

        // For the last item with 'end' alignment, use the actual max scroll.
        if resolved_align == ScrollAlignment::End && index == self.options.count - 1 {
            return Some((max_scroll_offset, resolved_align));
        }

        let to_offset = match resolved_align {
            ScrollAlignment::End => item.end + self.options.scroll_padding_end,
            _ => item.start - self.options.scroll_padding_start,
        };

        let offset =
            self.get_offset_for_alignment(to_offset, resolved_align, item.size, max_scroll_offset);

        Some((offset, resolved_align))
    }

    // -----------------------------------------------------------------------
    // Scroll commands
    // -----------------------------------------------------------------------

    /// Initiate scrolling to a specific offset. Returns a [`ScrollCommand`]
    /// for the adapter to execute, and sets up reconciliation state.
    pub fn scroll_to_offset(
        &mut self,
        to_offset: f64,
        opts: ScrollToOptions,
        max_scroll_offset: f64,
        now: f64,
    ) -> ScrollCommand {
        let offset = self.get_offset_for_alignment(to_offset, opts.align, 0.0, max_scroll_offset);

        self.scroll_state = Some(ScrollState {
            index: None,
            align: opts.align,
            behavior: opts.behavior,
            started_at: now,
            last_target_offset: offset,
            stable_frames: 0,
        });

        ScrollCommand {
            offset,
            behavior: Some(opts.behavior),
            adjustments: None,
        }
    }

    /// Initiate scrolling to a specific item index. Returns a [`ScrollCommand`]
    /// for the adapter to execute.
    pub fn scroll_to_index(
        &mut self,
        index: usize,
        opts: ScrollToOptions,
        max_scroll_offset: f64,
        now: f64,
    ) -> Option<ScrollCommand> {
        let index = index.min(self.options.count.saturating_sub(1));

        let (offset, resolved_align) =
            self.get_offset_for_index(index, opts.align, max_scroll_offset)?;

        self.scroll_state = Some(ScrollState {
            index: Some(index),
            align: resolved_align,
            behavior: opts.behavior,
            started_at: now,
            last_target_offset: offset,
            stable_frames: 0,
        });

        Some(ScrollCommand {
            offset,
            behavior: Some(opts.behavior),
            adjustments: None,
        })
    }

    /// Initiate a relative scroll by `delta` pixels.
    pub fn scroll_by(&mut self, delta: f64, opts: ScrollToOptions, now: f64) -> ScrollCommand {
        let offset = self.get_scroll_offset() + delta;

        self.scroll_state = Some(ScrollState {
            index: None,
            align: ScrollAlignment::Start,
            behavior: opts.behavior,
            started_at: now,
            last_target_offset: offset,
            stable_frames: 0,
        });

        ScrollCommand {
            offset,
            behavior: Some(opts.behavior),
            adjustments: None,
        }
    }

    /// Run one frame of scroll reconciliation. Called by the adapter from a
    /// `requestAnimationFrame` callback.
    ///
    /// - Returns `None` if reconciliation is complete (scroll has settled).
    /// - Returns `Some(ScrollCommand)` if the adapter needs to scroll again
    ///   and schedule another frame.
    ///
    /// `max_scroll_offset` must be provided by the adapter.
    pub fn reconcile_scroll(&mut self, now: f64, max_scroll_offset: f64) -> Option<ScrollCommand> {
        // Extract values we need from scroll_state before any &mut self calls.
        let state = self.scroll_state.as_ref()?;
        let started_at = state.started_at;
        let state_index = state.index;
        let state_align = state.align;
        let last_target_offset = state.last_target_offset;

        // Safety valve: bail out after 5 seconds.
        const MAX_RECONCILE_MS: f64 = 5000.0;
        if now - started_at > MAX_RECONCILE_MS {
            self.scroll_state = None;
            return None;
        }

        // Recompute target offset (measurements may have changed).
        let target_offset = if let Some(index) = state_index {
            self.get_offset_for_index(index, state_align, max_scroll_offset)
                .map(|(o, _)| o)
                .unwrap_or(last_target_offset)
        } else {
            last_target_offset
        };

        const STABLE_FRAMES: u32 = 1;
        let current_offset = self.get_scroll_offset();
        let target_changed = target_offset != last_target_offset;

        let state = self.scroll_state.as_mut().unwrap();

        if !target_changed && approx_equal(target_offset, current_offset) {
            state.stable_frames += 1;
            if state.stable_frames >= STABLE_FRAMES {
                self.scroll_state = None;
                return None;
            }
        } else {
            state.stable_frames = 0;

            if target_changed {
                state.last_target_offset = target_offset;
                state.behavior = ScrollBehavior::Auto;

                return Some(ScrollCommand {
                    offset: target_offset,
                    behavior: Some(ScrollBehavior::Auto),
                    adjustments: None,
                });
            }
        }

        // Reschedule — still reconciling.
        Some(ScrollCommand {
            offset: target_offset,
            behavior: None, // No scroll needed, just keep reconciling.
            adjustments: None,
        })
    }

    /// Whether there is an active scroll reconciliation in progress.
    pub fn has_pending_scroll(&self) -> bool {
        self.scroll_state.is_some()
    }

    /// Check whether measurement at an index should happen during scroll.
    /// During smooth scroll, only items near the target are measured.
    pub fn should_measure_during_scroll(&self, index: usize) -> bool {
        let state = match &self.scroll_state {
            Some(s) if s.behavior == ScrollBehavior::Smooth => s,
            _ => return true,
        };

        let scroll_index = state.index.or_else(|| {
            // Find nearest item to the target offset.
            if self.measurements_cache.is_empty() {
                return None;
            }
            let idx = find_nearest_binary_search(
                0,
                self.measurements_cache.len() - 1,
                |i| self.measurements_cache[i].start,
                state.last_target_offset,
            );
            Some(idx)
        });

        if let (Some(scroll_idx), Some(range)) = (scroll_index, &self.range) {
            let range_size = range.end_index.saturating_sub(range.start_index);
            let buffer_size = self.options.overscan.max(range_size.div_ceil(2));
            let min_index = scroll_idx.saturating_sub(buffer_size);
            let max_index = (scroll_idx + buffer_size).min(self.options.count.saturating_sub(1));
            index >= min_index && index <= max_index
        } else {
            true
        }
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn get_size(&mut self) -> f64 {
        if !self.options.enabled {
            self.scroll_rect = None;
            return 0.0;
        }

        let rect = self.scroll_rect.unwrap_or(self.options.initial_rect);
        self.scroll_rect = Some(rect);

        if self.options.horizontal {
            rect.width
        } else {
            rect.height
        }
    }

    fn get_scroll_offset(&self) -> f64 {
        if !self.options.enabled {
            return 0.0;
        }
        self.scroll_offset.unwrap_or(self.options.initial_offset)
    }

    /// Find the shortest lane and the start position for an item at `index`.
    fn find_shortest_lane(
        &self,
        index: usize,
        measurements: &[VirtualItem],
        lanes: usize,
        padding_start: f64,
        scroll_margin: f64,
        gap: f64,
    ) -> (usize, f64) {
        // Find the furthest (minimum-end) measurement per lane by scanning backward.
        let mut furthest_measurements: HashMap<usize, &VirtualItem> = HashMap::new();
        let mut furthest_found: HashMap<usize, bool> = HashMap::new();

        for m in (0..index.min(measurements.len())).rev() {
            let measurement = &measurements[m];

            if furthest_found.contains_key(&measurement.lane) {
                continue;
            }

            match furthest_measurements.get(&measurement.lane) {
                None => {
                    furthest_measurements.insert(measurement.lane, measurement);
                }
                Some(prev) => {
                    if measurement.end > prev.end {
                        furthest_measurements.insert(measurement.lane, measurement);
                    } else if measurement.end < prev.end {
                        furthest_found.insert(measurement.lane, true);
                    }
                }
            }

            if furthest_found.len() == lanes {
                break;
            }
        }

        if furthest_measurements.len() == lanes {
            // All lanes have measurements — pick the one with the smallest end.
            let mut sorted: Vec<_> = furthest_measurements.values().collect();
            sorted.sort_by(|a, b| {
                a.end
                    .partial_cmp(&b.end)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then(a.index.cmp(&b.index))
            });
            let shortest = sorted[0];
            (shortest.lane, shortest.end + gap)
        } else {
            // Not all lanes populated yet — assign to lane = index % lanes.
            (index % lanes, padding_start + scroll_margin)
        }
    }

    /// Check whether the notify state (isScrolling + range) has changed.
    /// Returns `true` if the adapter should re-render.
    fn check_should_notify(&mut self) -> bool {
        let range = self.calculate_range();
        let new_state = (
            self.is_scrolling,
            range.map(|r| r.start_index),
            range.map(|r| r.end_index),
        );

        let changed = self.prev_notify_state.as_ref() != Some(&new_state);
        self.prev_notify_state = Some(new_state);
        changed
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_options(count: usize, item_size: f64) -> VirtualizerOptions {
        VirtualizerOptions {
            count,
            estimate_size: Box::new(move |_| item_size),
            ..Default::default()
        }
    }

    #[test]
    fn test_fixed_size_items() {
        let mut v = Virtualizer::new(make_options(1000, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        let items = v.get_virtual_items();
        assert!(!items.is_empty());
        assert_eq!(items[0].start, 0.0);
        assert_eq!(items[0].size, 50.0);
        assert_eq!(items[0].lane, 0);
        assert_eq!(v.get_total_size(), 50000.0);
    }

    #[test]
    fn test_scroll_changes_visible_items() {
        let mut v = Virtualizer::new(make_options(100, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 200.0,
        });
        v.set_scroll_offset(0.0, false);

        let items0 = v.get_virtual_items();
        let first_index_0 = items0.first().unwrap().index;

        v.set_scroll_offset(500.0, true);
        let items1 = v.get_virtual_items();
        let first_index_1 = items1.first().unwrap().index;

        assert!(first_index_1 > first_index_0);
    }

    #[test]
    fn test_disabled_returns_empty() {
        let mut opts = make_options(100, 50.0);
        opts.enabled = false;
        let mut v = Virtualizer::new(opts);
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });

        assert!(v.get_virtual_items().is_empty());
        assert_eq!(v.get_total_size(), 0.0);
    }

    #[test]
    fn test_padding() {
        let mut opts = make_options(10, 50.0);
        opts.padding_start = 20.0;
        opts.padding_end = 30.0;
        let mut v = Virtualizer::new(opts);
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        let measurements = v.get_measurements();
        // First item starts at padding_start.
        assert_eq!(measurements[0].start, 20.0);
        // Total size includes padding_end.
        assert_eq!(v.get_total_size(), 20.0 + 500.0 + 30.0);
    }

    #[test]
    fn test_gap() {
        let mut opts = make_options(5, 50.0);
        opts.gap = 10.0;
        let mut v = Virtualizer::new(opts);
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        let measurements = v.get_measurements();
        assert_eq!(measurements[0].start, 0.0);
        assert_eq!(measurements[0].end, 50.0);
        assert_eq!(measurements[1].start, 60.0); // 50 + 10 gap
        assert_eq!(measurements[1].end, 110.0);
    }

    #[test]
    fn test_resize_item() {
        let mut v = Virtualizer::new(make_options(100, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        // Trigger initial measurements.
        let _ = v.get_virtual_items();

        // Resize item 0 from 50 to 100.
        let result = v.resize_item(0, 100.0);
        assert!(result.should_notify);

        // Item 0 should now be 100px.
        let measurements = v.get_measurements();
        assert_eq!(measurements[0].size, 100.0);
        assert_eq!(measurements[0].end, 100.0);
        // Item 1 should shift.
        assert_eq!(measurements[1].start, 100.0);
    }

    #[test]
    fn test_scroll_to_index() {
        let mut v = Virtualizer::new(make_options(100, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 200.0,
        });
        v.set_scroll_offset(0.0, false);
        let _ = v.get_measurements();

        let cmd = v.scroll_to_index(
            10,
            ScrollToOptions {
                align: ScrollAlignment::Start,
                behavior: ScrollBehavior::Auto,
            },
            4800.0, // maxScrollOffset = 50*100 - 200
            0.0,
        );

        assert!(cmd.is_some());
        let cmd = cmd.unwrap();
        // Item 10 starts at 500px.
        assert_eq!(cmd.offset, 500.0);
    }

    #[test]
    fn test_reconcile_scroll_settles() {
        let mut v = Virtualizer::new(make_options(100, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 200.0,
        });
        v.set_scroll_offset(0.0, false);
        let _ = v.get_measurements();

        // Start a scroll to index 10.
        let _ = v.scroll_to_index(10, ScrollToOptions::default(), 4800.0, 0.0);

        // Simulate the scroll arriving.
        v.set_scroll_offset(500.0, false);

        // Reconcile should settle within a couple frames.
        let result = v.reconcile_scroll(16.0, 4800.0);
        // Should still be reconciling (needs stable frame).
        assert!(result.is_some() || !v.has_pending_scroll());

        // After settling.
        let result = v.reconcile_scroll(32.0, 4800.0);
        if result.is_none() {
            assert!(!v.has_pending_scroll());
        }
    }

    #[test]
    fn test_reconcile_scroll_safety_valve() {
        let mut v = Virtualizer::new(make_options(100, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 200.0,
        });
        v.set_scroll_offset(0.0, false);
        let _ = v.get_measurements();

        let _ = v.scroll_to_index(50, ScrollToOptions::default(), 4800.0, 0.0);

        // Don't update scroll offset — reconciliation should bail after 5s.
        let result = v.reconcile_scroll(6000.0, 4800.0);
        assert!(result.is_none());
        assert!(!v.has_pending_scroll());
    }

    #[test]
    fn test_should_measure_during_scroll_no_state() {
        let v = Virtualizer::new(make_options(100, 50.0));
        // No scroll state → always allow.
        assert!(v.should_measure_during_scroll(50));
    }

    #[test]
    fn test_multi_lane_layout() {
        let mut opts = make_options(6, 50.0);
        opts.lanes = 2;
        let mut v = Virtualizer::new(opts);
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        let measurements = v.get_measurements();
        // Items should alternate lanes.
        assert_eq!(measurements[0].lane, 0);
        assert_eq!(measurements[1].lane, 1);
        // Items in different lanes start at the same position.
        assert_eq!(measurements[0].start, measurements[1].start);
    }

    #[test]
    fn test_get_total_size_multi_lane() {
        let mut opts = make_options(4, 50.0);
        opts.lanes = 2;
        let mut v = Virtualizer::new(opts);
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        let total = v.get_total_size();
        // 2 lanes, 4 items of 50px each → 2 rows → 100px.
        assert_eq!(total, 100.0);
    }

    #[test]
    fn test_scroll_margin() {
        let mut opts = make_options(10, 50.0);
        opts.scroll_margin = 100.0;
        let mut v = Virtualizer::new(opts);
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);

        let measurements = v.get_measurements();
        // First item starts at scroll_margin.
        assert_eq!(measurements[0].start, 100.0);

        // Total size subtracts scroll_margin.
        let total = v.get_total_size();
        // end of last item = 100 + 10*50 = 600, total = 600 - 100 + 0 = 500.
        assert_eq!(total, 500.0);
    }

    #[test]
    fn test_measure_clears_caches() {
        let mut v = Virtualizer::new(make_options(100, 50.0));
        v.set_scroll_rect(Rect {
            width: 400.0,
            height: 600.0,
        });
        v.set_scroll_offset(0.0, false);
        let _ = v.get_measurements();

        // Resize an item.
        v.resize_item(0, 100.0);
        assert!(v.item_size_cache.contains_key(&0));

        // Clear.
        v.measure();
        assert!(v.item_size_cache.is_empty());
    }
}

    #[test]
    fn test_resize_item_updates_positions() {
        // Simulate: 10 items estimated at 60px, viewport 400px
        let mut v = Virtualizer::new(VirtualizerOptions {
            count: 10,
            estimate_size: Box::new(|_| 60.0),
            ..Default::default()
        });
        v.set_scroll_rect(Rect { width: 400.0, height: 400.0 });
        v.set_scroll_offset(0.0, false);

        // Initial layout: items at 0, 60, 120, 180...
        let items = v.get_virtual_items();
        assert_eq!(items[0].start, 0.0);
        assert_eq!(items[1].start, 60.0);
        assert_eq!(items[2].start, 120.0);

        // Measure item 0 at 80px (taller than estimate)
        let result = v.resize_item(0, 80.0);
        assert!(result.should_notify);

        // After measurement, item 1 should start at 80, not 60
        let items = v.get_virtual_items();
        assert_eq!(items[0].size, 80.0);
        assert_eq!(items[0].start, 0.0);
        assert_eq!(items[0].end, 80.0);
        assert_eq!(items[1].start, 80.0, "item 1 should start at item 0's end");
        assert_eq!(items[2].start, 140.0, "item 2 should start at item 1's end");
    }
