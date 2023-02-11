/// Abstraction around the means to actually control the Light Strip. Establishes a common API across
/// each strip type
mod test_strip;
mod ws281x_strip;

use crate::{color::Color, LightArrangementError};

pub use test_strip::{TestStrip, TestStripDisplayConfig};
pub use ws281x_strip::Ws281xStrip;

pub trait LightStrip {
    fn get(&self, index: usize) -> Color;
    fn set(&mut self, index: usize, color: &Color);
    fn show(&mut self);
    fn fill(&mut self, color: &Color);
}

/// implemented by Light Strips that are not simulations, such as Ws281x strips.
pub trait RealStrip {
    fn new(config: LightStripConfig) -> Result<Self, LightArrangementError>
    where
        Self: Sized;
}

pub struct LightStripConfig {
    number_lights: i32,
    io_pin: i32,
}

impl LightStripConfig {
    pub fn new(number_lights: i32, io_pin: i32) -> Self {
        LightStripConfig {
            number_lights,
            io_pin,
        }
    }
}
