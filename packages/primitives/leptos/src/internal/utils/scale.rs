pub fn linear_scale(input: [f64; 2], output: [f64; 2]) -> impl Fn(f64) -> f64 {
    move |value: f64| {
        if input[0] == input[1] || output[0] == output[1] {
            return output[0];
        }
        let ratio = (output[1] - output[0]) / (input[1] - input[0]);
        output[0] + ratio * (value - input[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_scale_basic() {
        let scale = linear_scale([0.0, 100.0], [0.0, 1.0]);
        assert_eq!(scale(0.0), 0.0);
        assert_eq!(scale(50.0), 0.5);
        assert_eq!(scale(100.0), 1.0);
    }

    #[test]
    fn linear_scale_inverted_output() {
        let scale = linear_scale([0.0, 100.0], [1.0, 0.0]);
        assert_eq!(scale(0.0), 1.0);
        assert_eq!(scale(100.0), 0.0);
        assert_eq!(scale(50.0), 0.5);
    }

    #[test]
    fn linear_scale_non_zero_origin() {
        let scale = linear_scale([10.0, 20.0], [100.0, 200.0]);
        assert_eq!(scale(10.0), 100.0);
        assert_eq!(scale(15.0), 150.0);
        assert_eq!(scale(20.0), 200.0);
    }

    #[test]
    fn linear_scale_degenerate_input_returns_first_output() {
        let scale = linear_scale([5.0, 5.0], [10.0, 20.0]);
        assert_eq!(scale(5.0), 10.0);
        assert_eq!(scale(100.0), 10.0);
    }

    #[test]
    fn linear_scale_degenerate_output_returns_first_output() {
        let scale = linear_scale([0.0, 100.0], [7.0, 7.0]);
        assert_eq!(scale(50.0), 7.0);
    }

    #[test]
    fn linear_scale_extrapolates_beyond_range() {
        let scale = linear_scale([0.0, 10.0], [0.0, 100.0]);
        assert_eq!(scale(15.0), 150.0);
        assert_eq!(scale(-5.0), -50.0);
    }
}
