use super::arrangement::{Arrangement, ArrangementConfig};
use crate::{color::Color, light_strip::LightStrip, loc::Loc, math::distance};

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
        let datapoint = self.arrangement.get_closest(loc, max_search_distance);
        if let Some(datapoint) = datapoint {
            return Some(self.light_strip.get(datapoint.data));
        } else {
            return None;
        }
    }

    pub fn set_closest(&mut self, loc: &Loc<N>, color: &Color, max_set_distance: f64) {
        let datapoint = self.arrangement.get_closest(loc, max_set_distance);
        if let Some(datapoint) = datapoint {
            self.light_strip.set(datapoint.data, color);
        }
    }

    pub fn set_decreasing_intensity(&mut self, loc: &Loc<N>, color: &Color, set_distance: f64) {
        let datapoints = self.arrangement.get_within_radius(loc, set_distance);
        for pt in datapoints.iter() {
            let distance = distance(&pt.point, &loc.coords);

            let mut color = color.clone();
            color.dim(1.0 - (distance / set_distance));

            self.light_strip.set(pt.data, &color);
        }
    }

    pub fn fill(&mut self, color: &Color) {
        self.light_strip.fill(color)
    }

    pub fn show(&mut self) {
        self.light_strip.show()
    }
}
