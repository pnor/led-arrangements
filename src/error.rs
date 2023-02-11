use std::{error::Error, fmt};

use rs_ws281x::WS2811Error;

#[derive(Debug)]
pub struct LightArrangementError {
    reason: String,
}

impl LightArrangementError {
    pub fn new(reason: String) -> Self {
        LightArrangementError { reason }
    }

    pub fn from_error(err: WS2811Error) -> Self {
        LightArrangementError {
            reason: err.to_string(),
        }
    }

    pub fn reason(&self) -> String {
        String::from(&self.reason)
    }
}

impl Error for LightArrangementError {}

impl fmt::Display for LightArrangementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to create ArrangementConfig: {}", self.reason)
    }
}
