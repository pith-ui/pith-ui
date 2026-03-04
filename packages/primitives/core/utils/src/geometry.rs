#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Determine if a point is inside of a polygon.
/// Based on https://github.com/substack/point-in-polygon
pub fn is_point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    let (x, y) = (point.x, point.y);
    let mut inside = false;
    let len = polygon.len();
    if len == 0 {
        return false;
    }
    let mut j = len - 1;
    for i in 0..len {
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if intersect {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Returns a new array of points representing the convex hull of the given set of points.
/// https://www.nayuki.io/page/convex-hull-algorithm
pub fn get_hull(points: &[Point]) -> Vec<Point> {
    let mut new_points: Vec<Point> = points.to_vec();
    new_points.sort_by(|a, b| {
        a.x.partial_cmp(&b.x)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
    });
    get_hull_presorted(&new_points)
}

/// Returns the convex hull, assuming that each points[i] <= points[i + 1]. Runs in O(n) time.
pub fn get_hull_presorted(points: &[Point]) -> Vec<Point> {
    if points.len() <= 1 {
        return points.to_vec();
    }

    let mut upper_hull: Vec<Point> = Vec::new();
    for p in points {
        while upper_hull.len() >= 2 {
            let q = &upper_hull[upper_hull.len() - 1];
            let r = &upper_hull[upper_hull.len() - 2];
            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                upper_hull.pop();
            } else {
                break;
            }
        }
        upper_hull.push(p.clone());
    }
    upper_hull.pop();

    let mut lower_hull: Vec<Point> = Vec::new();
    for p in points.iter().rev() {
        while lower_hull.len() >= 2 {
            let q = &lower_hull[lower_hull.len() - 1];
            let r = &lower_hull[lower_hull.len() - 2];
            if (q.x - r.x) * (p.y - r.y) >= (q.y - r.y) * (p.x - r.x) {
                lower_hull.pop();
            } else {
                break;
            }
        }
        lower_hull.push(p.clone());
    }
    lower_hull.pop();

    if upper_hull.len() == 1
        && lower_hull.len() == 1
        && (upper_hull[0].x - lower_hull[0].x).abs() < f64::EPSILON
        && (upper_hull[0].y - lower_hull[0].y).abs() < f64::EPSILON
    {
        return upper_hull;
    }

    upper_hull.extend(lower_hull);
    upper_hull
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── is_point_in_polygon ─────────────────────────────────

    fn triangle() -> Vec<Point> {
        vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 5.0, y: 10.0 },
        ]
    }

    fn rectangle() -> Vec<Point> {
        vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
        ]
    }

    #[test]
    fn point_inside_triangle() {
        assert!(is_point_in_polygon(&Point { x: 5.0, y: 3.0 }, &triangle()));
    }

    #[test]
    fn point_inside_triangle_center() {
        assert!(is_point_in_polygon(&Point { x: 5.0, y: 5.0 }, &triangle()));
    }

    #[test]
    fn point_outside_triangle() {
        assert!(!is_point_in_polygon(
            &Point { x: 0.0, y: 10.0 },
            &triangle()
        ));
        assert!(!is_point_in_polygon(
            &Point { x: 20.0, y: 5.0 },
            &triangle()
        ));
        assert!(!is_point_in_polygon(
            &Point { x: 20.0, y: 20.0 },
            &triangle()
        ));
    }

    #[test]
    fn point_inside_rectangle() {
        assert!(is_point_in_polygon(&Point { x: 5.0, y: 5.0 }, &rectangle()));
        assert!(is_point_in_polygon(&Point { x: 1.0, y: 1.0 }, &rectangle()));
        assert!(is_point_in_polygon(&Point { x: 9.0, y: 9.0 }, &rectangle()));
    }

    #[test]
    fn point_outside_rectangle() {
        assert!(!is_point_in_polygon(
            &Point { x: -1.0, y: 5.0 },
            &rectangle()
        ));
        assert!(!is_point_in_polygon(
            &Point { x: 11.0, y: 5.0 },
            &rectangle()
        ));
        assert!(!is_point_in_polygon(
            &Point { x: 5.0, y: -1.0 },
            &rectangle()
        ));
        assert!(!is_point_in_polygon(
            &Point { x: 5.0, y: 11.0 },
            &rectangle()
        ));
    }

    #[test]
    fn point_on_edge_of_rectangle() {
        // Ray-casting algorithms are inconsistent on exact edges;
        // the important contract is no panic. The result may be true or false.
        let _ = is_point_in_polygon(&Point { x: 0.0, y: 5.0 }, &rectangle());
        let _ = is_point_in_polygon(&Point { x: 5.0, y: 0.0 }, &rectangle());
        let _ = is_point_in_polygon(&Point { x: 10.0, y: 5.0 }, &rectangle());
    }

    #[test]
    fn point_in_empty_polygon() {
        assert!(!is_point_in_polygon(&Point { x: 0.0, y: 0.0 }, &[]));
    }

    #[test]
    fn point_in_single_vertex_polygon() {
        let polygon = vec![Point { x: 5.0, y: 5.0 }];
        assert!(!is_point_in_polygon(&Point { x: 5.0, y: 5.0 }, &polygon));
    }

    #[test]
    fn point_far_outside_large_polygon() {
        let pentagon = vec![
            Point { x: 5.0, y: 0.0 },
            Point { x: 10.0, y: 4.0 },
            Point { x: 8.0, y: 10.0 },
            Point { x: 2.0, y: 10.0 },
            Point { x: 0.0, y: 4.0 },
        ];
        assert!(!is_point_in_polygon(
            &Point { x: 100.0, y: 100.0 },
            &pentagon
        ));
        assert!(is_point_in_polygon(&Point { x: 5.0, y: 5.0 }, &pentagon));
    }

    // ── get_hull / get_hull_presorted ───────────────────────

    #[test]
    fn hull_empty() {
        let hull = get_hull(&[]);
        assert!(hull.is_empty());
    }

    #[test]
    fn hull_single_point() {
        let points = vec![Point { x: 3.0, y: 4.0 }];
        let hull = get_hull(&points);
        assert_eq!(hull.len(), 1);
        assert!((hull[0].x - 3.0).abs() < f64::EPSILON);
        assert!((hull[0].y - 4.0).abs() < f64::EPSILON);
    }

    #[test]
    fn hull_two_points() {
        let points = vec![Point { x: 0.0, y: 0.0 }, Point { x: 5.0, y: 5.0 }];
        let hull = get_hull(&points);
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn hull_triangle_already_convex() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 5.0, y: 10.0 },
        ];
        let hull = get_hull(&points);
        assert_eq!(hull.len(), 3);
    }

    #[test]
    fn hull_square_with_interior_point() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 0.0, y: 10.0 },
            Point { x: 5.0, y: 5.0 }, // interior — should be excluded
        ];
        let hull = get_hull(&points);
        assert_eq!(hull.len(), 4);
        // Interior point should not be in the hull
        assert!(
            hull.iter()
                .all(|p| !((p.x - 5.0).abs() < f64::EPSILON && (p.y - 5.0).abs() < f64::EPSILON))
        );
    }

    #[test]
    fn hull_collinear_points() {
        let points = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 5.0, y: 0.0 },
            Point { x: 10.0, y: 0.0 },
        ];
        let hull = get_hull(&points);
        // Collinear points reduce to endpoints
        assert_eq!(hull.len(), 2);
    }

    #[test]
    fn hull_duplicate_points() {
        let points = vec![
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: 1.0 },
            Point { x: 1.0, y: 1.0 },
        ];
        let hull = get_hull(&points);
        assert_eq!(hull.len(), 1);
    }

    #[test]
    fn hull_presorted_identity() {
        // Already sorted by x then y
        let sorted = vec![
            Point { x: 0.0, y: 0.0 },
            Point { x: 5.0, y: 10.0 },
            Point { x: 10.0, y: 0.0 },
        ];
        let hull = get_hull_presorted(&sorted);
        assert_eq!(hull.len(), 3);
    }
}
