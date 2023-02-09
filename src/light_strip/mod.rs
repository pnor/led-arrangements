/// Abstraction around the means to actually control the LED Strip. Establishes a common API across
/// each strip type
mod test_strip;
mod ws281x_strip;

use crate::color::Color;

pub use test_strip::{TestStrip, TestStripDisplayConfig};
pub use ws281x_strip::Ws281xStrip;

pub trait LightStrip {
    fn get(&self, index: usize) -> Color;
    fn set(&mut self, index: usize, color: &Color);
    fn show(&mut self);
    fn fill(&mut self, color: &Color);
}

pub struct LightConfig {
    number_lights: i32,
    io_pin: i32,
}

impl LightConfig {
    pub fn new(number_lights: i32, io_pin: i32) -> Self {
        LightConfig {
            number_lights,
            io_pin,
        }
    }
}
