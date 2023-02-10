//! Core, low-level functionality for UsbDk.

#![allow(dead_code, unused)]

use log::{trace, debug};

use crate::{
    UsbResult,
    backend::{Backend, BackendDevice},
    device::Device,
    DeviceInformation,
};

use self::{
    driver::{DriverHandle, get_driver_handle},
    bindings::USB_DK_DEVICE_INFO,
    bindings::{IOCTL_USBDK_COUNT_DEVICES, IOCTL_USBDK_ENUM_DEVICES},
};

mod driver;
mod enumeration;
// TODO(Irides): This will be renamed to something like ioctl_c or driver_c when we have brain to
// regenerate and clean it up.
mod bindings;

/// Internal data for the UsbDk backend.
#[derive(Debug)]
pub struct UsbDkBackend {
    pub(crate) driver: DriverHandle
}

impl UsbDkBackend {
    pub fn new() -> UsbResult<Self> {
        trace!("Attempting to open a driver handle");

        /// FIXME: figure out if it's okay to always open the handle in overlapped mode.
        let driver = get_driver_handle(true);
        if driver.is_err() {
            debug!("Encountered error opening driver handle: {}", (driver.as_ref()).unwrap_err())
        }
        let driver = driver?;

        Ok(Self {
           driver
        })
    }
}

impl Backend for UsbDkBackend {
    fn get_devices(&self) -> UsbResult<Vec<DeviceInformation>> {
        // FIXME(Irides): This should be replaced with ioctl operations directly defined on the
        // driver handle.
        unsafe {
            // Learn how many devices to expect...
            let device_count: u32 = self.driver.ioctl_read(IOCTL_USBDK_COUNT_DEVICES)
                .expect("could not read device count");
            debug!("Driver reports {} devices to enumerate", device_count);

            // ... enumerate them ...
            let driver_devices: Vec<USB_DK_DEVICE_INFO> = self.driver.ioctl_read_vec(IOCTL_USBDK_ENUM_DEVICES, device_count)
                .expect("could not enumerate devices");

            // ... and convert them to our representation of a device.
            let mut output: Vec<DeviceInformation> = Vec::new();
            for driver_info in driver_devices {
                let driver_descriptor = driver_info.DeviceDescriptor;

                output.push(DeviceInformation::new(
                        driver_descriptor.idVendor,
                        driver_descriptor.idProduct,
                        None,
                        None,
                        None,
                ));
            }
            Ok(output)
        }
    }

    fn open(&self, information: &DeviceInformation) -> UsbResult<Box<dyn BackendDevice>> {
        todo!()
    }

    fn release_kernel_driver(&self, device: &mut Device, interface: u8) -> UsbResult<()> {
        todo!()
    }

    fn claim_interface(&self, device: &mut Device, interface: u8) -> UsbResult<()> {
        todo!()
    }

    fn unclaim_interface(&self, device: &mut Device, interface: u8) -> UsbResult<()> {
        todo!()
    }

    fn active_configuration(&self, device: &Device) -> UsbResult<u8> {
        todo!()
    }

    fn set_active_configuration(&self, device: &Device, configuration_index: u8) -> UsbResult<()> {
        todo!()
    }

    fn reset_device(&self, device: &Device) -> UsbResult<()> {
        todo!()
    }

    fn clear_stall(&self, device: &Device, endpoint_address: u8) -> UsbResult<()> {
        todo!()
    }

    fn set_alternate_setting(&self, device: &Device, interface: u8, setting: u8) -> UsbResult<()> {
        todo!()
    }

    fn current_bus_frame(&self, device: &Device) -> UsbResult<(u64, std::time::SystemTime)> {
        todo!()
    }

    fn control_read(
        &self,
        device: &Device,
        request_type: u8,
        request_number: u8,
        value: u16,
        index: u16,
        target: &mut [u8],
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<usize> {
        todo!()
    }

    fn control_read_nonblocking(
        &self,
        device: &Device,
        request_type: u8,
        request_number: u8,
        value: u16,
        index: u16,
        target: crate::ReadBuffer,
        callback: Box<dyn FnOnce(UsbResult<usize>)>,
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<()> {
        todo!()
    }

    fn control_write(
        &self,
        device: &Device,
        request_type: u8,
        request_number: u8,
        value: u16,
        index: u16,
        data: &[u8],
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<()> {
        todo!()
    }

    fn control_write_nonblocking(
        &self,
        device: &Device,
        request_type: u8,
        request_number: u8,
        value: u16,
        index: u16,
        data: crate::WriteBuffer,
        callback: Box<dyn FnOnce(UsbResult<usize>)>,
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<()> {
        todo!()
    }

    fn read(
        &self,
        device: &Device,
        endpoint: u8,
        buffer: &mut [u8],
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<usize> {
        todo!()
    }

    fn write(
        &self,
        device: &Device,
        endpoint: u8,
        data: &[u8],
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<()> {
        todo!()
    }

    fn read_nonblocking(
        &self,
        device: &Device,
        endpoint: u8,
        buffer: crate::ReadBuffer,
        callback: Box<dyn FnOnce(UsbResult<usize>)>,
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<()> {
        todo!()
    }

    fn write_nonblocking(
        &self,
        device: &Device,
        endpoint: u8,
        data: crate::WriteBuffer,
        callback: Box<dyn FnOnce(UsbResult<usize>)>,
        timeout: Option<std::time::Duration>,
    ) -> UsbResult<()> {
        todo!()
    }
}
