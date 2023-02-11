/// Using the ws281x lighting library
use rs_ws281x::{self, ChannelBuilder, Controller, ControllerBuilder};

use super::{LightStrip, LightStripConfig, RealStrip};
use crate::{color::Color, LightArrangementError};

const CHANNEL: usize = 0;

pub struct Ws281xStrip {
    controller: Controller,
}

impl RealStrip for Ws281xStrip {
    fn new(config: LightStripConfig) -> Result<Self, LightArrangementError> {
        let controller = ControllerBuilder::new()
            .freq(800_000)
            .dma(10)
            .channel(
                CHANNEL,
                ChannelBuilder::new()
                    .pin(config.io_pin)
                    .count(config.number_lights)
                    .strip_type(rs_ws281x::StripType::Ws2811Rgb)
                    .build(),
            )
            .build();
        return match controller {
            Ok(controller) => Ok(Ws281xStrip { controller }),
            Err(error) => Err(LightArrangementError::from_error(error)),
        };
    }
}

impl LightStrip for Ws281xStrip {
    fn get(&self, index: usize) -> Color {
        let raw = self.controller.leds(CHANNEL)[index];
        return Color {
            red: raw[0],
            green: raw[1],
            blue: raw[2],
        };
    }

    fn set(&mut self, index: usize, color: &Color) {
        self.controller.leds_mut(CHANNEL)[index] = [color.red, color.green, color.blue, 0];
    }

    fn show(&mut self) {
        let _ = self.controller.render();
    }

    fn fill(&mut self, color: &Color) {
        self.controller
            .leds_mut(CHANNEL)
            .iter_mut()
            .for_each(|raw| {
                raw[0] = color.red;
                raw[1] = color.green;
                raw[2] = color.blue;
                raw[3] = 0;
            });
    }
}
