use crate::ntree::NTree;

pub struct Arrangement<'a, const N: usize> {
    ntree: NTree<'a, usize, N>,
}

impl<'a, const N: usize> Arrangement<'_, N> {
    pub fn new(locations: HashMap<[f64; N], usize>) -> Self {
        todo!()
    }

    pub fn get_closest(&self, loc: &Loc<N>, max_search_distance: f64) -> Option<usize> {
        let res = self.ntree.find_closest(loc.coords, max_search_distance);
        if let Ok(Some(index)) = res {
            return Some(index.data);
        } else {
            return None;
        }
    }

    pub fn get_within_radius(&self, loc: &Loc<N>, max_search_distance: f64) -> Vec<usize> {
        let res = self.ntree.find_in_radius(loc.coords, max_search_distance);
        return res.iter().map(|pt| pt.data).collect();
    }

    pub fn get_within_bounding_box(
        &self,
        loc: &Loc<N>,
        lower_corner: &[f64; N],
        upper_corner: &[f64; N],
    ) -> Vec<usize> {
        let res = self.ntree.find_in_box(lower_corner, upper_corner);
        return res.iter().map(|pt| pt.data).collect();
    }
}
