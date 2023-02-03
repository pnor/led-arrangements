#[cfg(feature = "visualizer")]
extern crate kiss3d;

pub mod arrangement;
mod color;
mod light_strip;
mod loc;
mod math;
mod ntree;
pub mod strip_builder;

pub use arrangement::ArrangementConfig;
pub use arrangement::LightArrangement;
pub use color::Color;
pub use light_strip::{LightConfig, LightStrip, TestStrip, Ws281xStrip};
pub use loc::Loc;
pub use strip_builder::{test, ws281x};
