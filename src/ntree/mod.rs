extern crate tpntree;

mod intersection;
mod spatial_data_tree;

use intersection::box_intersection;

use tpntree::{tpntree::SpatialTree, TpnTreeError};

use crate::math::{array_map, array_zip, distance};

use self::{
    intersection::point_intersection,
    spatial_data_tree::{insert_by_coordinates, spans, SpatialDataTree},
};

pub use spatial_data_tree::DataPoint;

pub struct NTree<'a, T, const N: usize> {
    root: SpatialDataTree<T, N>,
    division_condition: &'a dyn Fn(&SpatialDataTree<T, N>) -> bool,
}

impl<'a: 'b, 'b, T: 'a, const N: usize> NTree<'a, T, N> {
    /// Creates an `NTree` spanning 0..1 in all dimensions
    pub fn new(division_condition: &'a dyn Fn(&SpatialDataTree<T, N>) -> bool) -> Self {
        NTree {
            root: SpatialTree::new([0.5; N], [0.5; N], 0),
            division_condition,
        }
    }

    /// Inserts a datapoint with `data` locoated at `point`
    /// `point` should be within 0..1 on all dimensions
    pub fn insert(&mut self, data: T, point: [f64; N]) -> Result<(), TpnTreeError> {
        insert_by_coordinates(
            &mut self.root,
            DataPoint { data, point },
            self.division_condition,
        )
    }

    /// Returns the datapoint closest to `point` that is <= `max_distance` away from `point`
    pub fn find_closest(
        &self,
        point: &[f64; N],
        max_distance: f64,
    ) -> Result<Option<&DataPoint<T, N>>, TpnTreeError> {
        if !spans(&self.root, point) {
            return Err(TpnTreeError::DoesNotSpan);
        }

        let corner1 = array_map(point, &|x| x - max_distance);
        let corner2 = array_map(point, &|x| x + max_distance);

        let points = self.find_in_box(&corner1, &corner2);

        let mut closest = None;
        let mut closest_distance = f64::MAX;
        for p in points {
            let d = distance(&p.point, point);
            if d < closest_distance && d < max_distance {
                closest = Some(p);
                closest_distance = d;
            }
        }
        return Ok(closest);
    }

    /// Returns all datapoints within the box region described by the corner points `corner1` and `corner2`.
    pub fn find_in_box(
        &'a self,
        corner1: &'b [f64; N],
        corner2: &'b [f64; N],
    ) -> Vec<&'a DataPoint<T, N>> {
        let mut datapoints = vec![];
        for tree in trees_in_box(&self.root, corner1, corner2) {
            if let Some(tree_data) = tree.data() {
                for child in tree_data {
                    if point_intersection(&child.point, &corner1, &corner2) {
                        datapoints.push(child);
                    }
                }
            }
        }
        return datapoints;
    }

    pub fn find_in_radius(&'a self, point: &'b [f64; N], radius: f64) -> Vec<&'a DataPoint<T, N>> {
        let corner1 = array_map(&point, &|x| x - radius);
        let corner2 = array_map(&point, &|x| x + radius);
        let points = self.find_in_box(&corner1, &corner2);

        if points.len() >= 1 {
            let p = points[0];
            assert!(p.point[0] > -999.0);
        }

        return points
            .into_iter()
            .filter(|x| -> bool { distance(point, &x.point) < radius })
            .collect();
    }
}

/// Returns all child TpnTrees whose span intersects the box region described by the corner points
/// `corner1` and `corner2`
fn trees_in_box<'a: 'b, 'b, T, const N: usize>(
    root: &'a SpatialDataTree<T, N>,
    corner1: &'b [f64; N],
    corner2: &'b [f64; N],
) -> Vec<&'a SpatialDataTree<T, N>> {
    let span = root.span();
    let center = root.coordinates();

    let root_corner1 = array_zip(&center, &span, &|(x, s)| x - s);
    let root_corner2 = array_zip(&center, &span, &|(x, s)| x + s);

    if box_intersection(&root_corner1, &root_corner2, &corner1, &corner2) {
        if root.child_count() == 0 {
            return vec![root];
        } else {
            let mut v = vec![];
            for child in root.iter_children() {
                let mut res = trees_in_box(child, corner1, corner2);
                v.append(&mut res);
            }
            return v;
        }
    } else {
        return vec![];
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_tree() {
        let _root: NTree<i32, 1> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);
    }

    #[test]
    fn insert_1_d() {
        let mut root: NTree<i32, 1> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);
        // insert inside span
        assert!(root.insert(1, [0.2]).is_ok());
        assert!(root.insert(1, [0.3]).is_ok());
        assert!(root.insert(1, [0.0]).is_ok());
        assert!(root.insert(1, [1.0]).is_ok());
        // insert outside span
        assert!(root.insert(1, [1.4]).is_err());
        assert!(root.insert(1, [-0.2]).is_err());
    }

    #[test]
    fn insert_2_d() {
        let mut root: NTree<i32, 2> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);
        // insert inside span
        assert!(root.insert(1, [0.5, 0.5]).is_ok());
        assert!(root.insert(1, [0.1, 0.8]).is_ok());
        assert!(root.insert(1, [0.0, 0.0]).is_ok());
        assert!(root.insert(1, [1.0, 1.0]).is_ok());
        // insert outside span
        assert!(root.insert(1, [1.1, 0.2]).is_err());
        assert!(root.insert(1, [-0.1, 0.2]).is_err());
        assert!(root.insert(1, [-0.1, -0.1]).is_err());
        assert!(root.insert(1, [0.2, -0.2]).is_err());
    }

    #[test]
    fn insert_4_d() {
        let mut root: NTree<i32, 4> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);
        // insert inside span
        assert!(root.insert(1, [0.0, 0.0, 0.0, 0.0]).is_ok());
        assert!(root.insert(1, [1.0, 1.0, 1.0, 1.0]).is_ok());
        assert!(root.insert(1, [0.0, 1.0, 0.0, 1.0]).is_ok());
        assert!(root.insert(1, [1.0, 0.0, 1.0, 0.0]).is_ok());
        assert!(root.insert(1, [0.1, 0.2, 0.3, 0.4]).is_ok());
        // insert outside span
        assert!(root.insert(1, [0.0, -0.1, 0.1, 0.2]).is_err());
        assert!(root.insert(1, [2.0, 2.0, 2.0, 2.0]).is_err());
        assert!(root.insert(1, [-0.5, -0.5, -0.5, -0.5]).is_err());
    }

    #[test]
    fn closest_1d() -> Result<(), TpnTreeError> {
        let mut root: NTree<i32, 1> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);
        // insert into the tree
        assert!(root.insert(1, [0.2]).is_ok());
        assert!(root.insert(2, [0.4]).is_ok());
        assert!(root.insert(3, [0.6]).is_ok());
        assert!(root.insert(4, [0.8]).is_ok());

        // query points
        let q1 = root.find_closest(&[0.1], 1.0)?;
        assert_eq!(q1.unwrap().data, 1);
        let q2 = root.find_closest(&[0.2999], 1.0)?;
        assert_eq!(q2.unwrap().data, 1);
        let q3 = root.find_closest(&[0.35], 1.0)?;
        assert_eq!(q3.unwrap().data, 2);
        let q4 = root.find_closest(&[0.3999], 1.0)?;
        assert_eq!(q4.unwrap().data, 2);
        let q5 = root.find_closest(&[0.59], 1.0)?;
        assert_eq!(q5.unwrap().data, 3);
        let q6 = root.find_closest(&[1.0], 1.0)?;
        assert_eq!(q6.unwrap().data, 4);

        Ok(())
    }

    #[test]
    fn closest_3d() -> Result<(), TpnTreeError> {
        let mut root: NTree<i32, 3> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);
        // insert into the tree
        assert!(root.insert(1, [0.0, 0.0, 0.0]).is_ok());
        assert!(root.insert(2, [1.0, 1.0, 1.0]).is_ok());
        assert!(root.insert(3, [0.5, 0.5, 0.5]).is_ok());
        assert!(root.insert(4, [0.0, 1.0, 0.0]).is_ok());

        // query points
        let q1 = root.find_closest(&[0.5, 0.5, 0.5], 1.0)?;
        assert_eq!(q1.unwrap().data, 3);
        let q2 = root.find_closest(&[0.1, 0.1, 0.1], 1.0)?;
        assert_eq!(q2.unwrap().data, 1);
        let q3 = root.find_closest(&[0.9, 0.9, 0.9], 1.0)?;
        assert_eq!(q3.unwrap().data, 2);
        let q4 = root.find_closest(&[0.2, 0.9, 0.1], 1.0)?;
        assert_eq!(q4.unwrap().data, 4);

        Ok(())
    }

    #[test]
    fn find_in_box_1d() {
        let mut root: NTree<i32, 1> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);

        assert!(root.insert(1, [0.0]).is_ok());
        assert!(root.insert(2, [0.2]).is_ok());
        assert!(root.insert(3, [0.4]).is_ok());
        assert!(root.insert(4, [0.6]).is_ok());
        assert!(root.insert(5, [0.8]).is_ok());
        assert!(root.insert(6, [1.0]).is_ok());

        let mut res: Vec<i32> = root
            .find_in_box(&[0.0], &[1.0])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6]);

        let mut res: Vec<i32> = root
            .find_in_box(&[0.5], &[1.0])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![4, 5, 6]);

        let mut res: Vec<i32> = root
            .find_in_box(&[0.0], &[0.4])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1, 2, 3]);

        let mut res: Vec<i32> = root
            .find_in_box(&[0.1], &[0.15])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn find_in_box_3d() {
        let mut root: NTree<i32, 3> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);

        assert!(root.insert(1, [0.0, 0.0, 0.0]).is_ok());
        assert!(root.insert(2, [1.0, 1.0, 1.0]).is_ok());
        assert!(root.insert(3, [0.5, 0.5, 0.5]).is_ok());
        assert!(root.insert(4, [0.0, 1.0, 0.0]).is_ok());

        let mut res: Vec<i32> = root
            .find_in_box(&[0.0, 0.0, 0.0], &[0.51, 0.51, 0.51])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1, 3]);

        let mut res: Vec<i32> = root
            .find_in_box(&[0.0, 0.0, 0.0], &[0.0, 1.0, 0.2])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1, 4]);

        let mut res: Vec<i32> = root
            .find_in_box(&[0.1, 0.1, 0.1], &[0.2, 0.2, 0.2])
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn find_in_radius_3d() {
        let mut root: NTree<i32, 3> = NTree::new(&|t| t.data().unwrap_or(&vec![]).len() > 1);

        assert!(root.insert(1, [0.0, 0.0, 0.0]).is_ok());
        assert!(root.insert(2, [1.0, 1.0, 1.0]).is_ok());
        assert!(root.insert(3, [0.5, 0.5, 0.5]).is_ok());
        assert!(root.insert(4, [0.0, 1.0, 0.0]).is_ok());

        let mut res: Vec<i32> = root
            .find_in_radius(&[0.0, 0.0, 0.0], 0.25)
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1]);

        let mut res: Vec<i32> = root
            .find_in_radius(&[0.0, 0.0, 0.0], 0.89)
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1, 3]);

        let mut res: Vec<i32> = root
            .find_in_radius(&[0.5, 0.5, 0.5], 0.4)
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![3]);

        let mut res: Vec<i32> = root
            .find_in_radius(&[0.0, 0.75, 0.0], 0.3)
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![4]);

        let mut res: Vec<i32> = root
            .find_in_radius(&[0.5, 0.5, 0.5], 1.0)
            .iter()
            .map(|p| p.data)
            .collect();
        res.sort();
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
