//! Color constants from [Radix Colors](https://www.radix-ui.com/colors).
//!
//! A comprehensive color system with 12-step scales for light and dark themes.
//! Each scale is a `[&str; 12]` array of hex or `color(display-p3 ...)` values,
//! indexed from step 1 (lightest/darkest) to step 12 (most prominent).
//!
//! # Modules
//!
//! - Light theme scales (e.g., `GRAY`, `BLUE`, `RED`) and their P3 variants
//! - Dark theme scales (e.g., `GRAY_DARK`, `BLUE_DARK`) and their P3 variants
//! - Alpha scales (`BLACK_A`, `WHITE_A`) for transparent overlays

mod black_a;
mod dark;
mod light;
mod white_a;

pub use black_a::*;
pub use dark::*;
pub use light::*;
pub use white_a::*;
