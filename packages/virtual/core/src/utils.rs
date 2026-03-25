use crate::types::{ComputedRange, Range, VirtualItem};

// ---------------------------------------------------------------------------
// Memoization
// ---------------------------------------------------------------------------

/// A memoized computation that caches its result and recomputes only when
/// dependencies change. Replaces the TS `memo()` utility.
#[allow(dead_code)]
pub(crate) struct Memo<D: PartialEq, R> {
    deps: Option<D>,
    result: Option<R>,
}

#[allow(dead_code)]
impl<D: PartialEq, R> Memo<D, R> {
    pub fn new() -> Self {
        Self {
            deps: None,
            result: None,
        }
    }

    /// Get the cached result, recomputing if `current_deps` differ from the
    /// previously stored deps.
    pub fn get(&mut self, current_deps: D, compute: impl FnOnce(&D) -> R) -> &R {
        let changed = match &self.deps {
            Some(prev) => prev != &current_deps,
            None => true,
        };
        if changed {
            let result = compute(&current_deps);
            self.deps = Some(current_deps);
            self.result = Some(result);
        }
        self.result.as_ref().unwrap()
    }

    /// Force-update the stored deps without recomputing. Used to keep
    /// the memoization in sync when deps are updated externally (mirrors
    /// the TS `memoizedFunction.updateDeps()`).
    pub fn update_deps(&mut self, new_deps: D) {
        self.deps = Some(new_deps);
    }

    /// Invalidate the cache so the next `get` call always recomputes.
    pub fn invalidate(&mut self) {
        self.deps = None;
        self.result = None;
    }
}

// ---------------------------------------------------------------------------
// Numeric helpers
// ---------------------------------------------------------------------------

/// Returns true if `a` and `b` are within ~1px of each other.
/// Mirrors the TS `approxEqual`.
pub(crate) fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.01
}

// ---------------------------------------------------------------------------
// Binary search
// ---------------------------------------------------------------------------

/// Find the index of the nearest item whose value (obtained via
/// `get_value(index)`) is <= `target`. Mirrors the TS
/// `findNearestBinarySearch`.
pub(crate) fn find_nearest_binary_search(
    mut low: usize,
    mut high: usize,
    get_value: impl Fn(usize) -> f64,
    target: f64,
) -> usize {
    while low <= high {
        let middle = (low + high) / 2;
        let value = get_value(middle);

        if value < target {
            low = middle + 1;
        } else if value > target {
            if middle == 0 {
                return 0;
            }
            high = middle - 1;
        } else {
            return middle;
        }
    }

    if low > 0 { low - 1 } else { 0 }
}

// ---------------------------------------------------------------------------
// Range extraction
// ---------------------------------------------------------------------------

/// Default key extractor — identity function.
pub fn default_key_extractor(index: usize) -> usize {
    index
}

/// Default range extractor — returns a contiguous range with overscan applied.
pub fn default_range_extractor(range: Range) -> Vec<usize> {
    let start = range.start_index.saturating_sub(range.overscan);
    let end = (range.end_index + range.overscan).min(range.count.saturating_sub(1));

    (start..=end).collect()
}

// ---------------------------------------------------------------------------
// Range calculation
// ---------------------------------------------------------------------------

/// Calculate which items are visible given measurements, viewport size,
/// and scroll offset. Mirrors the TS `calculateRange` function.
pub(crate) fn calculate_range(
    measurements: &[VirtualItem],
    outer_size: f64,
    scroll_offset: f64,
    lanes: usize,
) -> Option<ComputedRange> {
    if measurements.is_empty() || outer_size <= 0.0 {
        return None;
    }

    let last_index = measurements.len() - 1;

    // Handle case when item count is less than or equal to lanes.
    if measurements.len() <= lanes {
        return Some(ComputedRange {
            start_index: 0,
            end_index: last_index,
        });
    }

    let get_offset = |index: usize| measurements[index].start;

    let mut start_index = find_nearest_binary_search(0, last_index, get_offset, scroll_offset);
    let mut end_index = start_index;

    if lanes == 1 {
        while end_index < last_index && measurements[end_index].end < scroll_offset + outer_size {
            end_index += 1;
        }
    } else {
        // Multi-lane: expand forward until we include visible items from all lanes.
        let mut end_per_lane = vec![0.0_f64; lanes];
        while end_index < last_index
            && end_per_lane
                .iter()
                .any(|&pos| pos < scroll_offset + outer_size)
        {
            let item = &measurements[end_index];
            end_per_lane[item.lane] = item.end;
            end_index += 1;
        }

        // Expand backward until we include all lanes' visible items.
        let mut start_per_lane = vec![scroll_offset + outer_size; lanes];
        while start_index > 0 && start_per_lane.iter().any(|&pos| pos >= scroll_offset) {
            let item = &measurements[start_index];
            start_per_lane[item.lane] = item.start;
            start_index -= 1;
        }

        // Align to lane boundaries.
        start_index = start_index.saturating_sub(start_index % lanes);
        end_index = (end_index + (lanes - 1 - (end_index % lanes))).min(last_index);
    }

    Some(ComputedRange {
        start_index,
        end_index,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_equal() {
        assert!(approx_equal(10.0, 10.0));
        assert!(approx_equal(10.0, 10.5));
        assert!(approx_equal(10.0, 11.0));
        assert!(!approx_equal(10.0, 11.02));
        assert!(!approx_equal(10.0, 12.0));
    }

    #[test]
    fn test_binary_search_exact() {
        // Items at positions 0, 50, 100, 150, 200.
        let get_value = |i: usize| (i * 50) as f64;
        assert_eq!(find_nearest_binary_search(0, 4, get_value, 100.0), 2);
    }

    #[test]
    fn test_binary_search_between() {
        let get_value = |i: usize| (i * 50) as f64;
        // 75 is between index 1 (50) and index 2 (100) → should return 1.
        assert_eq!(find_nearest_binary_search(0, 4, get_value, 75.0), 1);
    }

    #[test]
    fn test_binary_search_before_start() {
        let get_value = |i: usize| ((i + 1) * 50) as f64;
        // All values start at 50, target is 0 → should return 0.
        assert_eq!(find_nearest_binary_search(0, 4, get_value, 0.0), 0);
    }

    #[test]
    fn test_binary_search_past_end() {
        let get_value = |i: usize| (i * 50) as f64;
        assert_eq!(find_nearest_binary_search(0, 4, get_value, 999.0), 4);
    }

    #[test]
    fn test_default_range_extractor() {
        let range = Range {
            start_index: 5,
            end_index: 10,
            overscan: 2,
            count: 100,
        };
        let result = default_range_extractor(range);
        assert_eq!(result, (3..=12).collect::<Vec<_>>());
    }

    #[test]
    fn test_default_range_extractor_clamps() {
        let range = Range {
            start_index: 0,
            end_index: 3,
            overscan: 5,
            count: 10,
        };
        let result = default_range_extractor(range);
        assert_eq!(result, (0..=8).collect::<Vec<_>>());
    }

    #[test]
    fn test_calculate_range_empty() {
        assert_eq!(calculate_range(&[], 600.0, 0.0, 1), None);
    }

    #[test]
    fn test_calculate_range_zero_size() {
        let items = vec![VirtualItem {
            key: 0,
            index: 0,
            start: 0.0,
            end: 50.0,
            size: 50.0,
            lane: 0,
        }];
        assert_eq!(calculate_range(&items, 0.0, 0.0, 1), None);
    }

    #[test]
    fn test_calculate_range_single_lane() {
        let items: Vec<VirtualItem> = (0..20)
            .map(|i| VirtualItem {
                key: i,
                index: i,
                start: (i as f64) * 50.0,
                end: (i as f64) * 50.0 + 50.0,
                size: 50.0,
                lane: 0,
            })
            .collect();

        // Viewport of 200px at offset 0 → items 0..3 visible.
        let range = calculate_range(&items, 200.0, 0.0, 1).unwrap();
        assert_eq!(range.start_index, 0);
        assert_eq!(range.end_index, 3);

        // Scroll to 100 → items 2..5 visible.
        let range = calculate_range(&items, 200.0, 100.0, 1).unwrap();
        assert_eq!(range.start_index, 2);
        assert_eq!(range.end_index, 5);
    }

    #[test]
    fn test_calculate_range_few_items_less_than_lanes() {
        let items: Vec<VirtualItem> = (0..2)
            .map(|i| VirtualItem {
                key: i,
                index: i,
                start: 0.0,
                end: 50.0,
                size: 50.0,
                lane: i,
            })
            .collect();

        let range = calculate_range(&items, 600.0, 0.0, 3).unwrap();
        assert_eq!(range.start_index, 0);
        assert_eq!(range.end_index, 1);
    }

    #[test]
    fn test_memo_caches() {
        let mut memo = Memo::new();
        let mut call_count = 0;

        let result = memo.get(42, |d| {
            call_count += 1;
            d * 2
        });
        assert_eq!(*result, 84);
        assert_eq!(call_count, 1);

        // Same deps → cached.
        let result = memo.get(42, |d| {
            call_count += 1;
            d * 2
        });
        assert_eq!(*result, 84);
        assert_eq!(call_count, 1);

        // Different deps → recompute.
        let result = memo.get(10, |d| {
            call_count += 1;
            d * 2
        });
        assert_eq!(*result, 20);
        assert_eq!(call_count, 2);
    }

    #[test]
    fn test_memo_invalidate() {
        let mut memo = Memo::new();
        let mut call_count = 0;

        memo.get(1, |_| {
            call_count += 1;
            "a"
        });
        assert_eq!(call_count, 1);

        memo.invalidate();

        memo.get(1, |_| {
            call_count += 1;
            "b"
        });
        assert_eq!(call_count, 2);
    }
}
