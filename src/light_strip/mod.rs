/// Abstraction around the means to actually control the LED Strip. Establishes a common API across
/// each strip type
mod test_strip;
mod ws281x_strip;

use crate::{arrangement::ArrangementConfig, color::Color};

use rs_ws281x::WS2811Error;

pub use test_strip::TestStrip;
pub use ws281x_strip::Ws281xStrip;

pub trait LightStrip {
    fn get(&self, index: usize) -> Color;
    fn set(&mut self, index: usize, color: &Color);
    fn show(&mut self);
    fn fill(&mut self, color: &Color);
}

pub struct LightStripBuilder;

impl LightStripBuilder {
    pub fn ws281x(config: LightConfig) -> Result<Ws281xStrip, WS2811Error> {
        Ok(Ws281xStrip::new(config)?)
    }

    pub fn test<const N: usize>(
        arrangement_info: &ArrangementConfig<N>,
        dimension_mask: &[u8; 3],
    ) -> TestStrip {
        TestStrip::new(arrangement_info, dimension_mask)
    }
}

pub struct LightConfig {
    pub number_lights: i32,
    pub io_pin: i32,
}
