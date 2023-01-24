extern crate tpntree;

use std::iter::once;

use tpntree::{tpntree::TpnTree, TpnTreeError};

pub type SpatialDataTree<T, const N: usize> = TpnTree<Vec<DataPoint<T, N>>, N>;

pub struct DataPoint<T, const N: usize> {
    pub point: [f64; N],
    pub data: T,
}

pub fn spans<T, const N: usize>(tree: &SpatialDataTree<T, N>, point: &[f64; N]) -> bool {
    tree.coordinates()
        .iter()
        .enumerate()
        .all(|(dimension, &coordinate)| {
            point[dimension] <= coordinate + tree.span()[dimension]
                && point[dimension] >= coordinate - tree.span()[dimension]
        })
}

pub fn insert_by_coordinates<T, const N: usize>(
    tree: &mut SpatialDataTree<T, N>,
    datapoint: DataPoint<T, N>,
    division_condition: &dyn Fn(&SpatialDataTree<T, N>) -> bool,
) -> Result<(), TpnTreeError> {
    if tree.is_root() && !spans(tree, &datapoint.point) {
        return Err(TpnTreeError::DoesNotSpan);
    }

    if tree.is_leaf() {
        if division_condition(tree) {
            tree.divide()?;

            for d in tree
                .data_mut()
                .take()
                .unwrap_or_default()
                .into_iter()
                .chain(once(datapoint))
            {
                insert_into_children(tree, d, division_condition)?
            }
            return Ok(());
        } else {
            tree.data_mut().get_or_insert(Vec::new()).push(datapoint);
            return Ok(());
        }
    } else {
        return insert_into_children(tree, datapoint, division_condition);
    }
}

fn insert_into_children<T, const N: usize>(
    tree: &mut SpatialDataTree<T, N>,
    datapoint: DataPoint<T, N>,
    division_condition: &dyn Fn(&SpatialDataTree<T, N>) -> bool,
) -> Result<(), TpnTreeError> {
    let res = tree
        .iter_children_mut()
        .find(|child| spans(&child, &datapoint.point))
        .map(|child| insert_by_coordinates(child, datapoint, division_condition))
        .unwrap();
    return res;
}
