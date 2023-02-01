use crate::loc::Loc;
use crate::ntree::DataPoint;
use crate::ntree::NTree;

const NUM_CHILDREN_FOR_DIVISION: usize = 3;

/// Manages the mapping from light index to location in N-dimensional space
pub struct Arrangement<'a, const N: usize> {
    ntree: NTree<'a, usize, N>,
}

impl<'a, const N: usize> Arrangement<'a, N> {
    pub fn new(config: &ArrangementConfig<N>) -> Self {
        let mut ntree: NTree<'a, usize, N> =
            NTree::new(&|r| r.child_count() >= NUM_CHILDREN_FOR_DIVISION);
        for (loc, index) in config.light_locations.iter() {
            ntree.insert(*index, *loc);
        }
        return Arrangement { ntree };
    }

    pub fn get_closest(
        &self,
        loc: &Loc<N>,
        max_search_distance: f64,
    ) -> Option<&DataPoint<usize, N>> {
        let res = self.ntree.find_closest(&loc.coords, max_search_distance);
        if let Ok(opt_datapoint) = res {
            return opt_datapoint;
        } else {
            return None;
        }
    }

    pub fn get_within_radius(
        &self,
        loc: &Loc<N>,
        max_search_distance: f64,
    ) -> Vec<&DataPoint<usize, N>> {
        self.ntree.find_in_radius(&loc.coords, max_search_distance)
    }

    pub fn get_within_bounding_box(
        &self,
        lower_corner: &Loc<N>,
        upper_corner: &Loc<N>,
    ) -> Vec<&DataPoint<usize, N>> {
        self.ntree
            .find_in_box(&lower_corner.coords, &upper_corner.coords)
    }
}

pub struct ArrangementConfig<const N: usize> {
    pub light_locations: Vec<([f64; N], usize)>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::loc::Loc;

    #[test]
    fn get_closest() {
        let arr = Arrangement::new(&ArrangementConfig {
            light_locations: vec![
                ([0.1, 0.1, 0.1], 1),
                ([0.9, 0.9, 0.9], 2),
                ([0.5, 0.5, 0.5], 3),
                ([0.1, 0.9, 0.1], 4),
                ([0.1, 0.1, 0.9], 5),
            ],
        });
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.2, 0.2, 0.2]), 0.2)
                .unwrap()
                .data,
            1
        );
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.2, 0.2, 0.2]), 0.2)
                .unwrap()
                .point,
            [0.1, 0.1, 0.1]
        );
        assert!(arr
            .get_closest(&Loc::cartesian([0.3, 0.3, 0.3]), 0.1)
            .is_none());
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.45, 0.45, 0.45]), 0.1)
                .unwrap()
                .data,
            3
        );
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.1, 0.9, 0.1]), 0.3)
                .unwrap()
                .data,
            4
        );
        assert_eq!(
            arr.get_closest(&Loc::cartesian([0.2, 0.2, 0.8]), 0.3)
                .unwrap()
                .data,
            5
        );
    }

    #[test]
    fn get_radius() {
        let arr = Arrangement::new(&ArrangementConfig {
            light_locations: vec![
                ([0.1, 0.1, 0.1], 1),
                ([0.9, 0.9, 0.9], 2),
                ([0.5, 0.5, 0.5], 3),
                ([0.1, 0.9, 0.1], 4),
                ([0.1, 0.1, 0.9], 5),
                ([0.4, 0.4, 0.4], 6),
            ],
        });
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.2, 0.2, 0.2]), 0.2)
                .iter()
                .map(|pt| pt.data)
                .collect::<Vec<usize>>(),
            vec![1]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.3, 0.3, 0.3]), 0.1)
                .iter()
                .map(|pt| pt.data)
                .collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.45, 0.45, 0.5]), 0.08)
                .iter()
                .map(|pt| pt.data)
                .collect::<Vec<usize>>(),
            vec![3]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.5, 0.5, 0.5]), 0.5)
                .iter()
                .map(|pt| pt.data)
                .collect::<Vec<usize>>(),
            vec![3, 6]
        );
        assert_eq!(
            arr.get_within_radius(&Loc::cartesian([0.5, 0.5, 0.5]), 0.9)
                .iter()
                .map(|pt| pt.data)
                .collect::<Vec<usize>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn get_bounding_box() {
        let arr = Arrangement::new(&ArrangementConfig {
            light_locations: vec![
                ([0.1, 0.1, 0.1], 1),
                ([0.9, 0.9, 0.9], 2),
                ([0.5, 0.5, 0.5], 3),
                ([0.1, 0.9, 0.1], 4),
                ([0.1, 0.1, 0.9], 5),
                ([0.4, 0.4, 0.4], 6),
            ],
        });
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([0.0, 0.0, 0.0]),
                &Loc::cartesian([0.3, 0.3, 0.3])
            )
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>(),
            vec![1]
        );
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([0.8, 0.8, 0.0]),
                &Loc::cartesian([1.0, 1.0, 1.0])
            )
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>(),
            vec![2]
        );
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([0.2, 0.2, 0.2]),
                &Loc::cartesian([0.3, 0.3, 0.3])
            )
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>(),
            vec![]
        );
        assert_eq!(
            arr.get_within_bounding_box(
                &Loc::cartesian([-0.1, -0.1, -0.1]),
                &Loc::cartesian([1.1, 1.1, 1.1])
            )
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
