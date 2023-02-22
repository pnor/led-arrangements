use super::arrangement_config::ArrangementConfig;
use crate::loc::Loc;
use crate::ntree::DataPoint;
use crate::ntree::NTree;
use crate::LightArrangementError;

/// Manages the mapping from light index to location in N-dimensional space
pub struct Arrangement<const N: usize> {
    ntree: NTree<usize, N>,
    number_lights: usize,
}

impl<const N: usize> Arrangement<N> {
    pub fn new(config: &ArrangementConfig<N>) -> Result<Self, LightArrangementError> {
        let mut ntree: NTree<usize, N> = NTree::new(config.number_children_for_division);
        let mut number_lights = 0;
        for (loc, index) in config.light_locations.iter() {
            let res = ntree.insert(*index, *loc);
            number_lights += 1;
            if res.is_err() {
                return Err(LightArrangementError::new(format!(
                    "Tried to insert index {}
    outside of range",
                    *index
                )));
            }
        }
        return Ok(Arrangement {
            ntree,
            number_lights,
        });
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

    pub fn number_lights(&self) -> usize {
        self.number_lights
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

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
            number_children_for_division: 1,
        })
        .unwrap();
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
    fn get_radius() -> Result<(), Box<dyn Error>> {
        let arr = Arrangement::new(&ArrangementConfig {
            light_locations: vec![
                ([0.1, 0.1, 0.1], 1),
                ([0.9, 0.9, 0.9], 2),
                ([0.5, 0.5, 0.5], 3),
                ([0.1, 0.9, 0.1], 4),
                ([0.1, 0.1, 0.9], 5),
                ([0.4, 0.4, 0.4], 6),
            ],
            number_children_for_division: 1,
        })?;
        let mut v = arr
            .get_within_radius(&Loc::cartesian([0.2, 0.2, 0.2]), 0.2)
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>();
        v.sort();
        assert_eq!(v, vec![1]);
        let mut v = arr
            .get_within_radius(&Loc::cartesian([0.3, 0.3, 0.3]), 0.1)
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>();
        v.sort();
        assert_eq!(v, vec![]);
        let mut v = arr
            .get_within_radius(&Loc::cartesian([0.45, 0.45, 0.5]), 0.08)
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>();
        v.sort();
        assert_eq!(v, vec![3]);
        let mut v = arr
            .get_within_radius(&Loc::cartesian([0.5, 0.5, 0.5]), 0.5)
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>();
        v.sort();
        assert_eq!(v, vec![3, 6]);
        let mut v = arr
            .get_within_radius(&Loc::cartesian([0.5, 0.5, 0.5]), 0.9)
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>();
        v.sort();
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);

        return Ok(());
    }

    #[test]
    fn get_bounding_box() -> Result<(), Box<dyn Error>> {
        let arr = Arrangement::new(&ArrangementConfig {
            light_locations: vec![
                ([0.1, 0.1, 0.1], 1),
                ([0.9, 0.9, 0.9], 2),
                ([0.5, 0.5, 0.5], 3),
                ([0.1, 0.9, 0.1], 4),
                ([0.1, 0.1, 0.9], 5),
                ([0.4, 0.4, 0.4], 6),
            ],

            number_children_for_division: 1,
        })?;
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
        let mut res = arr
            .get_within_bounding_box(
                &Loc::cartesian([-0.1, -0.1, -0.1]),
                &Loc::cartesian([1.1, 1.1, 1.1]),
            )
            .iter()
            .map(|pt| pt.data)
            .collect::<Vec<usize>>();
        res.sort();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);

        return Ok(());
    }

    #[test]
    fn number_lights() -> Result<(), Box<dyn Error>> {
        let arr = Arrangement::new(&ArrangementConfig {
            light_locations: vec![
                ([0.1, 0.1, 0.1], 1),
                ([0.9, 0.9, 0.9], 2),
                ([0.5, 0.5, 0.5], 3),
                ([0.1, 0.9, 0.1], 4),
                ([0.1, 0.1, 0.9], 5),
            ],
            number_children_for_division: 1,
        })?;
        assert_eq!(arr.number_lights(), 5);
        return Ok(());
    }
}
