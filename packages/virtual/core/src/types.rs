/// Direction of scrolling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Forward,
    Backward,
}

/// Alignment when scrolling to an item or offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAlignment {
    Start,
    Center,
    End,
    #[default]
    Auto,
}

/// Scroll animation behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollBehavior {
    #[default]
    Auto,
    Smooth,
    Instant,
}

/// Options for scroll-to operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct ScrollToOptions {
    pub align: ScrollAlignment,
    pub behavior: ScrollBehavior,
}

/// A command returned to the adapter to execute a scroll operation.
#[derive(Debug, Clone, Copy)]
pub struct ScrollCommand {
    pub offset: f64,
    pub behavior: Option<ScrollBehavior>,
    pub adjustments: Option<f64>,
}

/// Result from `resize_item` telling the adapter what to do.
#[derive(Debug, Clone)]
pub struct ResizeItemResult {
    /// Whether the visible range changed (adapter should re-render).
    pub should_notify: bool,
    /// If scroll position needs correcting, the command to execute.
    pub scroll_correction: Option<ScrollCommand>,
}

/// A single virtual item with computed layout information.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualItem {
    /// Unique key for this item (typically the index).
    pub key: usize,
    /// Index in the full list.
    pub index: usize,
    /// Start position in pixels (from the scroll origin).
    pub start: f64,
    /// End position in pixels.
    pub end: f64,
    /// Size in pixels (height for vertical, width for horizontal).
    pub size: f64,
    /// Lane assignment (for multi-column layouts).
    pub lane: usize,
}

/// Dimensions of a scroll container.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
}

/// Input to the range extractor function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start_index: usize,
    pub end_index: usize,
    pub overscan: usize,
    pub count: usize,
}

/// The computed visible range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputedRange {
    pub start_index: usize,
    pub end_index: usize,
}

/// Internal scroll reconciliation state.
#[derive(Debug, Clone)]
pub(crate) struct ScrollState {
    /// Target index (None if scrolling to an offset).
    pub index: Option<usize>,
    /// Alignment preference.
    pub align: ScrollAlignment,
    /// Scroll animation behavior.
    pub behavior: ScrollBehavior,
    /// Timestamp when scroll started (milliseconds).
    pub started_at: f64,
    /// Last computed target offset.
    pub last_target_offset: f64,
    /// Number of consecutive stable frames.
    pub stable_frames: u32,
}
