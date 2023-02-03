use rs_ws281x::WS2811Error;

use crate::{
    light_strip::{TestStrip, Ws281xStrip},
    ArrangementConfig, LightConfig,
};

pub fn ws281x(config: LightConfig) -> Result<Ws281xStrip, WS2811Error> {
    Ok(Ws281xStrip::new(config)?)
}

pub fn test<const N: usize>(
    arrangement_info: &ArrangementConfig<N>,
    dimension_mask: &[u8; 3],
) -> TestStrip {
    TestStrip::new(arrangement_info, dimension_mask)
}
