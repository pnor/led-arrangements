use super::arrangement::Arrangement;
use crate::light_strip::LightStrip;

/// Uses Arrangement and LightStrip to assign to lights based on lcation in N dimensional space
pub struct LightArrangement<'a, const N: usize> {
    arrangement: Arrangement<'a, N>,
    light_strip: dyn LightStrip,
}
