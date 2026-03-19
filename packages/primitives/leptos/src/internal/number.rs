pub fn clamp(value: f64, [min, max]: [f64; 2]) -> f64 {
    value.max(min).min(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_within_range() {
        assert_eq!(clamp(5.0, [0.0, 10.0]), 5.0);
    }

    #[test]
    fn clamp_below_min() {
        assert_eq!(clamp(-5.0, [0.0, 10.0]), 0.0);
    }

    #[test]
    fn clamp_above_max() {
        assert_eq!(clamp(15.0, [0.0, 10.0]), 10.0);
    }

    #[test]
    fn clamp_at_min_boundary() {
        assert_eq!(clamp(0.0, [0.0, 10.0]), 0.0);
    }

    #[test]
    fn clamp_at_max_boundary() {
        assert_eq!(clamp(10.0, [0.0, 10.0]), 10.0);
    }

    #[test]
    fn clamp_negative_range() {
        assert_eq!(clamp(-5.0, [-10.0, -1.0]), -5.0);
        assert_eq!(clamp(-15.0, [-10.0, -1.0]), -10.0);
        assert_eq!(clamp(0.0, [-10.0, -1.0]), -1.0);
    }

    #[test]
    fn clamp_equal_min_max() {
        assert_eq!(clamp(5.0, [3.0, 3.0]), 3.0);
        assert_eq!(clamp(1.0, [3.0, 3.0]), 3.0);
    }

    #[test]
    fn clamp_fractional_values() {
        assert_eq!(clamp(0.5, [0.0, 1.0]), 0.5);
        assert_eq!(clamp(0.001, [0.01, 0.99]), 0.01);
        assert_eq!(clamp(0.999, [0.01, 0.99]), 0.99);
    }

    #[test]
    fn clamp_nan_returns_min() {
        // f64::NAN.max(min) returns min in Rust, so NaN input clamps to min.
        assert_eq!(clamp(f64::NAN, [0.0, 10.0]), 0.0);
    }

    #[test]
    fn clamp_infinity() {
        assert_eq!(clamp(f64::INFINITY, [0.0, 10.0]), 10.0);
        assert_eq!(clamp(f64::NEG_INFINITY, [0.0, 10.0]), 0.0);
    }
}
