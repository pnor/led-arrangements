/// true if the cube A represented by the corner points `a_corner1` and `a_corner2` intersects the
/// cube B represented by corner points `b_corner1` and `b_corner2`
/// Every value in `a_corner1` should be <= values in `a_corner2`
/// Every value in `b_corner1` should be <= values in `b_corner2`
pub fn cube_intersection<const N: usize>(
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

mod test {
    use super::*;

    #[test]
    fn box_intersects_3d() {
        // [ { } ] in every axis
        assert!(cube_intersection(
            &[0.0, 0.0, 0.0],
            &[2.0, 2.0, 2.0],
            &[0.5, 0.5, 0.5],
            &[1.5, 1.5, 1.5]
        ));
        // [ { ] } in every axis
        assert!(cube_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[0.5, 0.5, 0.5],
            &[1.5, 1.5, 1.5]
        ));
        // { [ ] } in every axis
        assert!(cube_intersection(
            &[0.5, 0.5, 0.5],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.5, 1.5, 1.5]
        ));
        // { [ } ] in every axis
        assert!(cube_intersection(
            &[0.5, 0.5, 0.5],
            &[1.5, 1.5, 1.5],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // Smae Box
        assert!(cube_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // Overlapping Edges
        assert!(cube_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[0.5, 0.5, 0.5]
        ));
        assert!(cube_intersection(
            &[0.5, 0.5, 0.5],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        assert!(cube_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0]
        ));
        assert!(cube_intersection(
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // Overlap on not every axis
        assert!(cube_intersection(
            &[0.0, 0.0, 0.0],
            &[2.0, 2.0, 2.0],
            &[0.0, 0.5, 0.0],
            &[1.0, 1.0, 1.0]
        ));
    }

    #[test]
    fn no_intersection_3d() {
        // [ ] {  } and { } [ ] : no overlap in any dimension
        assert!(!cube_intersection(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0],
            &[3.0, 3.0, 3.0]
        ));
        assert!(!cube_intersection(
            &[2.0, 2.0, 2.0],
            &[3.0, 3.0, 3.0],
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0]
        ));
        // [ { ] } in one dimension
        assert!(!cube_intersection(
            &[0.0, 0.0, 7.0],
            &[1.0, 1.0, 8.0],
            &[0.5, 2.0, 9.0],
            &[1.5, 3.0, 10.0]
        ));
    }
}
