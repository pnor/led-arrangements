use super::arrangement::Arrangement;
use super::arrangement_config::ArrangementConfig;
use crate::LightArrangementError;
use crate::{color::Color, light_strip::LightStrip, loc::Loc, math::distance};

/// Uses Arrangement and LightStrip to assign to lights based on lcation in N dimensional space
pub struct LightArrangement<T: LightStrip, const N: usize> {
    arrangement: Arrangement<N>,
    light_strip: T,
}

impl<T: LightStrip, const N: usize> LightArrangement<T, N> {
    pub fn new(
        light_strip: T,
        arrangement_config: ArrangementConfig<N>,
    ) -> Result<Self, LightArrangementError> {
        Ok(LightArrangement {
            arrangement: Arrangement::new(&arrangement_config)?,
            light_strip,
        })
    }

    pub fn get_closest(&self, loc: &Loc<N>, max_search_distance: f64) -> Option<Color> {
        let datapoint = self.arrangement.get_closest(loc, max_search_distance);
        if let Some(datapoint) = datapoint {
            return Some(self.light_strip.get(datapoint.data));
        } else {
            return None;
        }
    }

    pub fn set_closest(&mut self, loc: &Loc<N>, max_set_distance: f64, color: &Color) {
        let datapoint = self.arrangement.get_closest(loc, max_set_distance);
        if let Some(datapoint) = datapoint {
            self.light_strip.set(datapoint.data, color);
        }
    }

    pub fn set_decreasing_intensity(&mut self, loc: &Loc<N>, set_distance: f64, color: &Color) {
        let datapoints = self.arrangement.get_within_radius(loc, set_distance);
        for pt in datapoints.iter() {
            let distance = distance(&pt.point, &loc.coords);

            let mut color = color.clone();
            color.dim(1.0 - (distance / set_distance));

            self.light_strip.set(pt.data, &color);
        }
    }

    /// Sets lights at `loc` with a decreasing intensity outward
    /// When setting, will merge `color` and the color of the light using a simple max method
    pub fn set_decreasing_intensity_merge(
        &mut self,
        loc: &Loc<N>,
        set_distance: f64,
        color: &Color,
    ) {
        let datapoints = self.arrangement.get_within_radius(loc, set_distance);
        for pt in datapoints.iter() {
            let distance = distance(&pt.point, &loc.coords);

            let mut color = color.clone();
            color.dim(1.0 - (distance / set_distance));

            let cur_color = self.light_strip.get(pt.data);
            color.merge(cur_color);
            self.light_strip.set(pt.data, &color);
        }
    }

    pub fn set_all_in_box(&mut self, lower_corner: &Loc<N>, upper_corner: &Loc<N>, color: &Color) {
        for pt in self
            .arrangement
            .get_within_bounding_box(lower_corner, upper_corner)
        {
            self.light_strip.set(pt.data, color);
        }
    }

    pub fn set_all_in_radius(&mut self, center: &Loc<N>, radius: f64, color: &Color) {
        for pt in self.arrangement.get_within_radius(center, radius) {
            self.light_strip.set(pt.data, color);
        }
    }

    pub fn get_by_index(&mut self, index: usize) -> Color {
        self.light_strip.get(index)
    }

    pub fn set_by_index(&mut self, index: usize, color: &Color) {
        self.light_strip.set(index, color);
    }

    pub fn fill(&mut self, color: &Color) {
        self.light_strip.fill(color)
    }

    pub fn show(&mut self) {
        self.light_strip.show()
    }

    pub fn number_lights(&self) -> usize {
        self.arrangement.number_lights()
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::{Loc, TestStrip, TestStripDisplayConfig};

    use super::*;

    fn clamp(number: f64, lower: f64, upper: f64) -> f64 {
        if number < lower {
            return lower;
        } else if number > upper {
            return upper;
        } else {
            return number;
        }
    }

    fn make_light_arrangement() -> Result<LightArrangement<TestStrip, 2>, LightArrangementError> {
        let arrangement_config = ArrangementConfig {
            light_locations: vec![
                ([0.2, 0.2], 0),
                ([0.4, 0.2], 1),
                ([0.6, 0.2], 2),
                ([0.8, 0.2], 3),
                ([1.0, 0.2], 4),
                ([0.2, 0.4], 5),
                ([0.4, 0.4], 6),
                ([0.6, 0.4], 7),
                ([0.8, 0.4], 8),
                ([1.0, 0.4], 9),
                ([0.2, 0.6], 10),
                ([0.4, 0.6], 11),
                ([0.6, 0.6], 12),
                ([0.8, 0.6], 13),
                ([1.0, 0.6], 14),
                ([0.2, 0.8], 15),
                ([0.4, 0.8], 16),
                ([0.6, 0.8], 17),
                ([0.8, 0.8], 18),
                ([1.0, 0.8], 19),
                ([0.2, 1.0], 20),
                ([0.4, 1.0], 21),
                ([0.6, 1.0], 22),
                ([0.8, 1.0], 23),
                ([1.0, 1.0], 24),
            ],
            number_children_for_division: 2,
        };

        let light_strip = TestStrip::new(&arrangement_config, &TestStripDisplayConfig::default());
        let light_arrangement = LightArrangement::new(light_strip, arrangement_config)?;
        return Ok(light_arrangement);
    }

    #[test]
    fn fill() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let mut fill_color = Color {
            red: 0,
            green: 0,
            blue: 0,
        };

        light_arrangement.fill(&fill_color);

        for i in [0.2, 0.4, 0.6, 0.8, 1.0] {
            for j in [0.2, 0.4, 0.6, 0.8, 1.0] {
                assert_eq!(
                    light_arrangement.get_closest(&Loc::cartesian([i, j]), 0.2),
                    Some(fill_color)
                );
            }
        }

        fill_color.red = 200;
        fill_color.green = 100;
        fill_color.blue = 50;

        light_arrangement.fill(&fill_color);

        for i in [0.2, 0.4, 0.6, 0.8, 1.0] {
            for j in [0.2, 0.4, 0.6, 0.8, 1.0] {
                assert_eq!(
                    light_arrangement.get_closest(&Loc::cartesian([i, j]), 0.2),
                    Some(fill_color)
                );
            }
        }

        return Ok(());
    }

    #[test]
    fn get_and_set_closest() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let mut color = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        light_arrangement.set_closest(&Loc::cartesian([0.2, 0.2]), 0.2, &color);
        light_arrangement.set_closest(&Loc::cartesian([0.4, 0.2]), 0.2, &color);
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.2, 0.2]), 0.2),
            Some(color)
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.4, 0.2]), 0.2),
            Some(color)
        );

        color.blue = 255;
        light_arrangement.set_closest(&Loc::cartesian([0.8, 0.8]), 0.2, &color);
        light_arrangement.set_closest(&Loc::cartesian([0.4, 0.2]), 0.2, &color);
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.8, 0.8]), 0.2),
            Some(color)
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.4, 0.2]), 0.2),
            Some(color)
        );

        return Ok(());
    }

    #[test]
    fn set_decreasing_intensity() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let color = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        light_arrangement.set_decreasing_intensity(&Loc::cartesian([0.5, 0.5]), 1.0, &color);

        for i in [0.2, 0.4, 0.6, 0.8, 1.0] {
            for j in [0.2, 0.4, 0.6, 0.8, 1.0] {
                let dist = distance(&[i, j], &[0.5, 0.5]);
                let expected_color_val = ((1.0 - clamp(dist / 1.0, 0.0, 1.0)) * 255.0) as u8;

                let color = light_arrangement.get_closest(&Loc::cartesian([i, j]), 0.1);
                assert_eq!(
                    color.unwrap(),
                    Color {
                        red: expected_color_val,
                        green: 0,
                        blue: 0
                    }
                );
            }
        }

        return Ok(());
    }

    #[test]
    fn set_decreasing_intensity_merging() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let color1 = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        let color2 = Color {
            red: 0,
            green: 255,
            blue: 0,
        };
        light_arrangement.set_decreasing_intensity_merge(&Loc::cartesian([0.5, 0.5]), 1.0, &color1);
        light_arrangement.set_decreasing_intensity_merge(&Loc::cartesian([0.5, 0.5]), 1.0, &color2);

        for i in [0.2, 0.4, 0.6, 0.8, 1.0] {
            for j in [0.2, 0.4, 0.6, 0.8, 1.0] {
                let dist = distance(&[i, j], &[0.5, 0.5]);
                let expected_color_val = ((1.0 - clamp(dist / 1.0, 0.0, 1.0)) * 255.0) as u8;

                let color = light_arrangement.get_closest(&Loc::cartesian([i, j]), 0.1);
                assert_eq!(
                    color.unwrap(),
                    Color {
                        red: expected_color_val,
                        green: expected_color_val,
                        blue: 0
                    }
                );
            }
        }

        return Ok(());
    }

    #[test]
    fn set_all_in_radius() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let color = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        light_arrangement.set_all_in_radius(&Loc::cartesian([0.5, 0.5]), 0.6, &color);

        for i in [0.2, 0.4, 0.6, 0.8, 1.0] {
            for j in [0.2, 0.4, 0.6, 0.8, 1.0] {
                let dist = distance(&[i, j], &[0.5, 0.5]);

                if dist < 0.6 {
                    assert_eq!(
                        light_arrangement.get_closest(&Loc::cartesian([i, j]), 0.2),
                        Some(Color {
                            red: 255,
                            green: 0,
                            blue: 0
                        })
                    );
                } else {
                    assert_eq!(
                        light_arrangement.get_closest(&Loc::cartesian([i, j]), 0.2),
                        Some(Color {
                            red: 0,
                            green: 0,
                            blue: 0
                        })
                    );
                }
            }
        }

        return Ok(());
    }

    #[test]
    fn set_all_in_box() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let color = Color {
            red: 255,
            green: 0,
            blue: 0,
        };
        light_arrangement.set_all_in_box(
            &Loc::cartesian([0.4, 0.4]),
            &Loc::cartesian([0.6, 0.6]),
            &color,
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.5, 0.5]), 0.2),
            Some(Color {
                red: 255,
                green: 0,
                blue: 0
            })
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.45, 0.6]), 0.2),
            Some(Color {
                red: 255,
                green: 0,
                blue: 0
            })
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.4, 0.5]), 0.2),
            Some(Color {
                red: 255,
                green: 0,
                blue: 0
            })
        );

        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.1, 0.1]), 0.2),
            Some(Color {
                red: 0,
                green: 0,
                blue: 0
            })
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.9, 0.1]), 0.2),
            Some(Color {
                red: 0,
                green: 0,
                blue: 0
            })
        );
        assert_eq!(
            light_arrangement.get_closest(&Loc::cartesian([0.2, 0.8]), 0.2),
            Some(Color {
                red: 0,
                green: 0,
                blue: 0
            })
        );

        return Ok(());
    }

    #[test]
    fn get_and_set_by_index() -> Result<(), Box<dyn Error>> {
        let mut light_arrangement = make_light_arrangement()?;

        let color = Color {
            red: 255,
            green: 0,
            blue: 0,
        };

        light_arrangement.set_by_index(2, &color);
        assert_eq!(
            light_arrangement.get_by_index(2),
            Color {
                red: 255,
                green: 0,
                blue: 0
            }
        );
        Ok(())
    }

    #[test]
    fn number_lights() -> Result<(), Box<dyn Error>> {
        let light_arrangement = make_light_arrangement()?;
        assert_eq!(light_arrangement.number_lights(), 25);
        Ok(())
    }
}
