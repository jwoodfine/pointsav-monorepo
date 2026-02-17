#![no_std]
use system_core::{PointSavResult, CoreError};

pub trait WirelessSubstrate {
    fn scan_ssids(&self) -> PointSavResult<()>;
    fn connect(&self, ssid: &str, psk: &str) -> PointSavResult<()>;
}
