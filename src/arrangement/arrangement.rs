use crate::loc::Loc;
use crate::ntree::NTree;

const NUM_CHILDREN_FOR_DIVISION: usize = 3;

/// Manages the mapping from light index to location in N-dimensional space
pub struct Arrangement<'a, const N: usize> {
    ntree: NTree<'a, usize, N>,
}

impl<'a, const N: usize> Arrangement<'a, N> {
    pub fn new(locations: &Vec<([f64; N], usize)>) -> Self {
        let mut ntree: NTree<'a, usize, N> =
            NTree::new(&|r| r.child_count() >= NUM_CHILDREN_FOR_DIVISION);
        for (loc, index) in locations {
            ntree.insert(*index, *loc);
        }
        return Arrangement { ntree };
    }

    pub fn get_closest(&self, loc: &Loc<N>, max_search_distance: f64) -> Option<usize> {
        let res = self.ntree.find_closest(&loc.coords, max_search_distance);
        if let Ok(Some(index)) = res {
            return Some(index.data);
        } else {
            return None;
        }
    }

    pub fn get_within_radius(&self, loc: &Loc<N>, max_search_distance: f64) -> Vec<usize> {
        let res = self.ntree.find_in_radius(&loc.coords, max_search_distance);
        return res.iter().map(|pt| pt.data).collect();
    }

    pub fn get_within_bounding_box(
        &self,
        lower_corner: &Loc<N>,
        upper_corner: &Loc<N>,
    ) -> Vec<usize> {
        let res = self
            .ntree
            .find_in_box(&lower_corner.coords, &upper_corner.coords);
        return res.iter().map(|pt| pt.data).collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::loc::Loc;

    #[test]
    fn get_closest() {
        let arr = Arrangement::new(&vec![
            ([0.1, 0.1, 0.1], 1),
            ([0.9, 0.9, 0.9], 2),
            ([0.5, 0.5, 0.5], 3),
            ([0.1, 0.9, 0.1], 4),
            ([0.1, 0.1, 0.9], 5),
        ]);
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.2, 0.2, 0.2]), 0.2),
            Some(1)
        );
        assert_eq!(arr.get_closest(&Loc::cartesian([0.3, 0.3, 0.3]), 0.1), None);
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.45, 0.45, 0.45]), 0.1),
            Some(3)
        );
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.1, 0.9, 0.1]), 0.3),
            Some(4)
        );
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.2, 0.2, 0.8]), 0.3),
            Some(5)
        );
    }

    #[test]
    fn get_radius() {
        let arr = Arrangement::new(&vec![
            ([0.1, 0.1, 0.1], 1),
            ([0.9, 0.9, 0.9], 2),
            ([0.5, 0.5, 0.5], 3),
            ([0.1, 0.9, 0.1], 4),
            ([0.1, 0.1, 0.9], 5),
            ([0.4, 0.4, 0.4], 6),
        ]);
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.2, 0.2, 0.2]), 0.2),
            vec![1]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.3, 0.3, 0.3]), 0.1),
            vec![]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.45, 0.45, 0.5]), 0.08),
            vec![3]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.5, 0.5, 0.5]), 0.5),
            vec![3, 6]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.5, 0.5, 0.5]), 0.9),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn get_bounding_box() {
        let arr = Arrangement::new(&vec![
            ([0.1, 0.1, 0.1], 1),
            ([0.9, 0.9, 0.9], 2),
            ([0.5, 0.5, 0.5], 3),
            ([0.1, 0.9, 0.1], 4),
            ([0.1, 0.1, 0.9], 5),
            ([0.4, 0.4, 0.4], 6),
        ]);
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([0.0, 0.0, 0.0]),
                &Loc::cartesian([0.3, 0.3, 0.3])
            ),
            vec![1]
        );
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([0.8, 0.8, 0.0]),
                &Loc::cartesian([1.0, 1.0, 1.0])
            ),
            vec![2]
        );
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([0.2, 0.2, 0.2]),
                &Loc::cartesian([0.3, 0.3, 0.3])
            ),
            vec![]
        );
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([-0.1, -0.1, -0.1]),
                &Loc::cartesian([1.1, 1.1, 1.1])
            ),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
