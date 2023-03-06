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
    pub number_lights: i32,
    pub io_pin: i32,
    pub brightness: u8,
    pub order: ColorOrder,
    pub frequency: u32,
}

impl LightStripConfig {
    /// Creates the config for a light strip
    /// `number_lights`: number of lights on the strip
    /// `io_pin`: GPIO pin on the rasberry pi used to control the lights
    /// `brightness`: 0..255 brightness value to set lights
    /// `order`: Color order of the strip
    /// `frequency`: Frequency of the signal to the LEDs, usually 800K
    pub fn new(
        number_lights: i32,
        io_pin: i32,
        brightness: u8,
        order: ColorOrder,
        frequency: u32,
    ) -> Self {
        LightStripConfig {
            number_lights,
            io_pin,
            brightness,
            order,
            frequency,
        }
    }
}

/// Color order of the light strip
pub enum ColorOrder {
    Rgb,
    Rbg,
    Grb,
    Gbr,
    Brg,
    Bgr,
}
