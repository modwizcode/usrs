//! Routines for querying UsbDk for USB devices.

#[allow(unused)]
use log::{debug, trace};

use crate::UsbResult;

/// Helper to retrieve the number of devices present.
fn get_device_count() -> UsbResult<u64> {
    todo!()
}
