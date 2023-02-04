use rs_ws281x::WS2811Error;

use crate::{
    light_strip::{TestStrip, Ws281xStrip},
    ArrangementConfig, LightConfig, TestStripDisplayConfig,
};

pub fn ws281x(config: LightConfig) -> Result<Ws281xStrip, WS2811Error> {
    Ok(Ws281xStrip::new(config)?)
}

pub fn test<const N: usize>(
    arrangement_config: &ArrangementConfig<N>,
    display_config: &TestStripDisplayConfig,
) -> TestStrip {
    TestStrip::new(arrangement_config, display_config)
}
