extern crate tpntree;

mod intersection;
mod spatial_data_tree;

use intersection::cube_intersection;

use tpntree::{
    tpntree::{SpatialTree, TpnTree},
    TpnTreeError,
};

use self::spatial_data_tree::{insert_by_coordinates, DataPoint, SpatialDataTree};

pub struct NTree<'a, T, const N: usize> {
    root: SpatialDataTree<T, N>,
    division_condition: &'a dyn Fn(&SpatialDataTree<T, N>) -> bool,
}

impl<'a, T: 'a, const N: usize> NTree<'a, T, N> {
    fn new(division_condition: &'a dyn Fn(&SpatialDataTree<T, N>) -> bool) -> Self {
        NTree {
            root: SpatialTree::new([0.5; N], [0.5; N], 0),
            division_condition,
        }
    }

    fn insert(&mut self, data: T, point: [f64; N]) -> Result<(), TpnTreeError> {
        insert_by_coordinates(
            &mut self.root,
            DataPoint { data, point },
            self.division_condition,
        )
    }

    fn find_closest(&self, point: &[f64; N]) -> Result<Option<[f64; N]>, TpnTreeError> {
        // let node = find_by_coordinates(self.root, point);
        todo!()
    }

    fn find_in_cube(&self, corner_1: [f64; N], corner_2: [f64; N]) {
        todo!()
    }

    fn trees_in_cube(
        root: &SpatialDataTree<T, N>,
        corner1: [f64; N],
        corner2: [f64; N],
    ) -> Vec<&SpatialDataTree<T, N>> {
        let span = root.span();
        let center = root.coordinates();

        let mut root_corner1 = center;
        for i in 0..N {
            root_corner1[i] -= span[i] / 2.0;
        }
        let mut root_corner2 = center;
        for i in 0..N {
            root_corner2[i] += span[i] / 2.0;
        }

        if cube_intersection(&root_corner1, &root_corner2, &corner1, &corner2) {
            if root.child_count() == 0 {
                return vec![root];
            } else {
                let mut v = vec![];
                for child in root.iter_children() {
                    let mut res = Self::trees_in_cube(child, corner1, corner2);
                    v.append(&mut res);
                }
                return v;
            }
        } else {
            return vec![];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_tree() {
        let _root: NTree<i32, 1> = NTree::new(&|_| false);
    }

    #[test]
    fn insert_1_d() {
        let mut root: NTree<i32, 1> = NTree::new(&|_| false);
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
        let mut root: NTree<i32, 2> = NTree::new(&|_| false);
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
        let mut root: NTree<i32, 4> = NTree::new(&|_| false);
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
}
