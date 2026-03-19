/// Wraps an array around itself at a given start index.
/// Example: `wrap_array(&mut ['a', 'b', 'c', 'd'], 2) == &['c', 'd', 'a', 'b']`
pub fn wrap_array<T>(array: &mut [T], start_index: usize) -> &[T] {
    array.rotate_left(start_index);
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_array_empty() {
        let mut arr: [i32; 0] = [];
        assert_eq!(wrap_array(&mut arr, 0), &[] as &[i32]);
    }

    #[test]
    fn wrap_array_at_zero() {
        let mut arr = ['a', 'b', 'c'];
        assert_eq!(wrap_array(&mut arr, 0), &['a', 'b', 'c']);
    }

    #[test]
    fn wrap_array_at_one() {
        let mut arr = [1, 2, 3, 4, 5];
        assert_eq!(wrap_array(&mut arr, 1), &[2, 3, 4, 5, 1]);
    }

    #[test]
    fn wrap_array_basic() {
        let mut arr = ['a', 'b', 'c', 'd'];
        assert_eq!(wrap_array(&mut arr, 2), &['c', 'd', 'a', 'b']);
    }

    #[test]
    fn wrap_array_at_last() {
        let mut arr = ['a', 'b', 'c', 'd'];
        assert_eq!(wrap_array(&mut arr, 3), &['d', 'a', 'b', 'c']);
    }

    #[test]
    fn wrap_array_at_end() {
        let mut arr = ['a', 'b', 'c'];
        assert_eq!(wrap_array(&mut arr, 2), &['c', 'a', 'b']);
    }

    #[test]
    fn wrap_array_single_element() {
        let mut arr = [42];
        assert_eq!(wrap_array(&mut arr, 0), &[42]);
    }
}
