/// true if the box A represented by the corner points `a_corner1` and `a_corner2` intersects the
/// box B represented by corner points `b_corner1` and `b_corner2`
/// Every value in `a_corner1` should be <= values in `a_corner2`
/// Every value in `b_corner1` should be <= values in `b_corner2`
pub fn box_intersection<const N: usize>(
    a_corner1: &[f64; N],
    a_corner2: &[f64; N],
    b_corner1: &[f64; N],
    b_corner2: &[f64; N],
) -> bool {
    for i in 0..N {
        // a: []
        // b: {}
        if (a_corner1[i] <= b_corner1[i] && a_corner2[i] >= b_corner2[i]) // [ { } ]
            || (a_corner1[i] <= b_corner1[i] && (b_corner1[i]..=b_corner2[i]).contains(&a_corner2[i])) // [ { ] }
            || (a_corner1[i] >= b_corner1[i] && a_corner2[i] <= b_corner2[i]) //  { [ ] }
            || ((b_corner1[i]..=b_corner2[i]).contains(&a_corner1[i]) && a_corner2[i] >= b_corner2[i])
        //  { [ } ]
        {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

/// true if `point` is inside the box represented by corner points `corner1` and `corner2`
pub fn point_intersection<const N: usize>(
    point: &[f64; N],
    corner1: &[f64; N],
    corner2: &[f64; N],
) -> bool {
    corner1
        .iter()
        .zip(corner2.iter())
        .zip(point)
        .find(|((c1, c2), p)| !(c1..=c2).contains(&p))
        .is_none()
}

mod test {
    use super::*;

    #[test]
    fn box_intersects_3d() {
        // [ { } ] in every axis
        assert!(box_intersection(
            &[0.0, 0.0, 0.0],
            &[2.0, 2.0, 2.0],
            &[0.5, 0.5, 0.5],
            &[1.5, 1.5, 1.5]
        ));
        // [ { ] } in every axis
        assert!(box_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[0.5, 0.5, 0.5],
            &[1.5, 1.5, 1.5]
        ));
        // { [ ] } in every axis
        assert!(box_intersection(
            &[0.5, 0.5, 0.5],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.5, 1.5, 1.5]
        ));
        // { [ } ] in every axis
        assert!(box_intersection(
            &[0.5, 0.5, 0.5],
            &[1.5, 1.5, 1.5],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // Smae Box
        assert!(box_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // Overlapping Edges
        assert!(box_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[0.5, 0.5, 0.5]
        ));
        assert!(box_intersection(
            &[0.5, 0.5, 0.5],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        assert!(box_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0]
        ));
        assert!(box_intersection(
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // Overlap on not every axis
        assert!(box_intersection(
            &[0.0, 0.0, 0.0],
            &[2.0, 2.0, 2.0],
            &[0.0, 0.5, 0.0],
            &[1.0, 1.0, 1.0]
        ));
    }

    #[test]
    fn no_intersection_3d() {
        // [ ] {  } and { } [ ] : no overlap in any dimension
        assert!(!box_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0],
            &[3.0, 3.0, 3.0]
        ));
        assert!(!box_intersection(
            &[2.0, 2.0, 2.0],
            &[3.0, 3.0, 3.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // [ { ] } in one dimension
        assert!(!box_intersection(
            &[0.0, 0.0, 7.0],
            &[1.0, 1.0, 8.0],
            &[0.5, 2.0, 9.0],
            &[1.5, 3.0, 10.0]
        ));
    }

    #[test]
    fn point_in_box_2d() {
        // on border
        assert!(point_intersection(&[0.0], &[0.0], &[1.0]));
        assert!(point_intersection(&[1.0], &[0.0], &[1.0]));

        // point in box
        assert!(point_intersection(&[0.2], &[0.0], &[1.0]));
        assert!(point_intersection(&[0.5, 0.5], &[0.0, 0.0], &[1.0, 1.0]));
        assert!(point_intersection(
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        assert!(point_intersection(
            &[0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0, 1.0]
        ));

        // point not in box
        assert!(!point_intersection(&[-0.2], &[0.0], &[1.0]));
        assert!(!point_intersection(&[1.5, 1.5], &[0.0, 0.0], &[1.0, 1.0]));
    }
}
