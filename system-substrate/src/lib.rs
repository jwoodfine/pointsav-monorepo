#![no_std]

use system_core::{PointSavResult, CoreError};

//! # system-substrate
//! Updated with Wireless Substrate for Air-Gapped Bootstrap.

pub trait Substrate {
    fn boot_sequence(&self) -> PointSavResult<()>;
}

/// New trait for machines requiring WiFi connectivity (Laptop B).
pub trait WirelessHardware {
    /// Connect to a SSID using WPA2-PSK.
    fn connect_wifi(&self, ssid: &str, psk: &str) -> PointSavResult<()>;
    /// Return the signal strength.
    fn get_signal_dbm(&self) -> i8;
}

pub struct SeL4Substrate;

impl Substrate for SeL4Substrate {
    fn boot_sequence(&self) -> PointSavResult<()> {
        Ok(())
    }
}
