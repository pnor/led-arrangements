#[cfg(feature = "visualizer")]
extern crate kiss3d;

pub mod arrangement;
pub mod builder;
mod color;
mod light_strip;
mod loc;
mod math;
mod ntree;

pub use arrangement::ArrangementConfig;
pub use arrangement::LightArrangement;
pub use builder::{test, ws281x};
pub use color::Color;
pub use light_strip::{LightConfig, LightStrip, TestStrip, Ws281xStrip};
pub use loc::Loc;

pub fn add(left: usize, right: usize) -> usize {
    return left + right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
