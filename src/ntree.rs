extern crate tpntree;

use tpntree::{
    tpntree::{SpatialTree, TpnTree},
    TpnTreeError,
};

pub struct NTree<'a, const N: usize> {
    root: TpnTree<Vec<[f64; N]>, N>,
    division_condition: &'a dyn Fn(&Self) -> bool,
}

impl<'a, const N: usize> NTree<'a, N> {
    fn new(division_condition: &'a dyn Fn(&Self) -> bool) -> Self {
        NTree {
            root: SpatialTree::new([0.5; N], [0.5; N], 0),
            division_condition,
        }
    }

    fn insert(&mut self, point: [f64; N]) -> Result<(), TpnTreeError> {
        self.root.insert_by_coordinates(point, &|_| false)
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    use super::NTree;

    #[test]
    fn create_tree() {
        let _root: NTree<1> = NTree::new(&|_| false);
    }

    #[test]
    fn insert_1_d() {
        let mut root: NTree<1> = NTree::new(&|_| false);
        // insert inside span
        assert!(root.insert([0.2]).is_ok());
        assert!(root.insert([0.3]).is_ok());
        assert!(root.insert([0.0]).is_ok());
        assert!(root.insert([1.0]).is_ok());
        // insert outside span
        assert!(root.insert([1.4]).is_err());
        assert!(root.insert([-0.2]).is_err());
    }

    #[test]
    fn insert_2_d() {
        let mut root: NTree<2> = NTree::new(&|_| false);
        // insert inside span
        assert!(root.insert([0.5, 0.5]).is_ok());
        assert!(root.insert([0.1, 0.8]).is_ok());
        assert!(root.insert([0.0, 0.0]).is_ok());
        assert!(root.insert([1.0, 1.0]).is_ok());
        // insert outside span
        assert!(root.insert([1.1, 0.2]).is_err());
        assert!(root.insert([-0.1, 0.2]).is_err());
        assert!(root.insert([-0.1, -0.1]).is_err());
        assert!(root.insert([0.2, -0.2]).is_err());
    }

    #[test]
    fn insert_4_d() {
        let mut root: NTree<4> = NTree::new(&|_| false);
        // insert inside span
        assert!(root.insert([0.0, 0.0, 0.0, 0.0]).is_ok());
        assert!(root.insert([1.0, 1.0, 1.0, 1.0]).is_ok());
        assert!(root.insert([0.0, 1.0, 0.0, 1.0]).is_ok());
        assert!(root.insert([1.0, 0.0, 1.0, 0.0]).is_ok());
        assert!(root.insert([0.1, 0.2, 0.3, 0.4]).is_ok());
        // insert outside span
        assert!(root.insert([0.0, -0.1, 0.1, 0.2]).is_err());
        assert!(root.insert([2.0, 2.0, 2.0, 2.0]).is_err());
        assert!(root.insert([-0.5, -0.5, -0.5, -0.5]).is_err());
    }
}
