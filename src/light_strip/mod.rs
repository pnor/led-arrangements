/// Abstraction around the means to actually control the LED Strip. Establishes a common API across
/// each strip type
mod ws281x_strip;

use crate::color::Color;

use self::ws281x_strip::Ws281xStrip;
use rs_ws281x::WS2811Error;

trait LightStrip {
    pub fn get(&self, index: usize) -> Color;
    pub fn set(&mut self, index: usize, color: Color);
    pub fn show(&self);
    pub fn fill(&mut self, color: Color);
}

impl dyn LightStrip {
    pub fn ws281x(config: LightConfig) -> Result<Box<impl LightStrip>, WS2811Error> {
        Ok(Box::new(Ws281xStrip::new(config)?))
    }

    pub fn test() -> Box<Self> {
        todo!()
    }
}

struct LightConfig {
    pub number_lights: i32,
    pub io_pin: i32,
}
