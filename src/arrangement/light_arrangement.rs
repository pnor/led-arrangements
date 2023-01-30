use super::arrangement::{Arrangement, ArrangementConfig};
use crate::{color::Color, light_strip::LightStrip, loc::Loc};

/// Uses Arrangement and LightStrip to assign to lights based on lcation in N dimensional space
pub struct LightArrangement<'a, T: LightStrip, const N: usize> {
    arrangement: Arrangement<'a, N>,
    light_strip: T,
}

impl<'a, T: LightStrip, const N: usize> LightArrangement<'a, T, N> {
    pub fn new(light_strip: T, arrangement_config: ArrangementConfig<N>) -> Self {
        LightArrangement {
            arrangement: Arrangement::new(&arrangement_config),
            light_strip,
        }
    }

    pub fn get_closest(&self, loc: &Loc<N>, max_search_distance: f64) -> Option<Color> {
        let index = self.arrangement.get_closest(loc, max_search_distance);
        if let Some(index) = index {
            return Some(self.light_strip.get(index));
        } else {
            return None;
        }
    }

    pub fn set_closest(&mut self, loc: &Loc<N>, color: &Color, max_set_distance: f64) {
        let index = self.arrangement.get_closest(loc, max_set_distance);
        if let Some(index) = index {
            self.light_strip.set(index, color);
        }
    }

    pub fn set_decreasing_intensity(&self, loc: &Loc<N>, color: &Color, set_distance: f64) {
        todo!()
    }

    pub fn fill(&mut self, color: &Color) {
        self.light_strip.fill(color)
    }

    pub fn show(&mut self) {
        self.light_strip.show()
    }
}
