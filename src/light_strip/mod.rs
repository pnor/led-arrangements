mod test_strip;
/// Abstraction around the means to actually control the LED Strip. Establishes a common API across
/// each strip type
mod ws281x_strip;

use crate::color::Color;

use self::ws281x_strip::Ws281xStrip;
use rs_ws281x::WS2811Error;

pub trait LightStrip {
    fn get(&self, index: usize) -> Color;
    fn set(&mut self, index: usize, color: &Color);
    fn show(&mut self);
    fn fill(&mut self, color: &Color);
}

impl dyn LightStrip {
    pub fn ws281x(config: LightConfig) -> Result<Box<impl LightStrip>, WS2811Error> {
        Ok(Box::new(Ws281xStrip::new(config)?))
    }

    pub fn test() -> Box<Self> {
        todo!()
    }
}

pub struct LightConfig {
    pub number_lights: i32,
    pub io_pin: i32,
}
