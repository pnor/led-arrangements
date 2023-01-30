use super::{LightConfig, LightStrip};
use crate::color::Color;

pub struct TestStrip {
    // TODO
}

impl TestStrip {
    // TODO
}

impl LightStrip for TestStrip {
    fn get(&self, index: usize) -> Color {
        todo!()
    }

    fn set(&mut self, index: usize, color: &Color) {
        todo!()
    }

    fn show(&mut self) {
        todo!()
    }

    fn fill(&mut self, color: &Color) {
        todo!()
    }
}
