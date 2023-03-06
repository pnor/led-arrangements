#[cfg(feature = "visualizer")]
extern crate kiss3d;

pub mod arrangement;
mod color;
mod error;
mod light_strip;
mod loc;
mod math;
mod ntree;

pub use arrangement::ArrangementConfig;
pub use arrangement::LightArrangement;
pub use color::Color;
pub use error::LightArrangementError;
pub use light_strip::{
    ColorOrder, LightStrip, LightStripConfig, RealStrip, TestStrip, TestStripDisplayConfig,
    Ws281xStrip,
};
pub use loc::Loc;
