/// Abstraction around the means to actually control the LED Strip. Establishes a common API across
/// each strip type
mod ws281x_strip;

use crate::color::Color;

trait LightStrip {
    pub fn get(&self, index: usize) -> Color;
    pub fn set(&mut self, index: usize, color: Color);
    pub fn show(&self);
    pub fn fill(&mut self, color: Color);

    // Constructor
    pub fn ws281x(config: LightConfig) -> Box<Self> {
        todo!()
    }

    pub fn test() -> Box<Self> {
        todo!()
    }
}

struct LightConfig {
    pub number_lights: i32,
    pub io_pin: i32,
}
