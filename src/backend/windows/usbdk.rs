//! Core, low-level functionality for UsbDk.

use std::mem::{transmute, MaybeUninit};

use log::trace;

use crate::{UsbResult, backend::{Backend, BackendDevice, windows::usbdk::ioctl::DEVICE_INFO}, DeviceInformation, device::Device};

use self::driver::{DriverHandle, get_driver_handle};

mod driver;
mod enumeration;
mod ioctl;

/// Internal data for the UsbDk backend.
#[derive(Debug)]
pub struct UsbDkBackend {
    pub(crate) driver: DriverHandle
}

impl UsbDkBackend {
    pub fn new() -> UsbResult<Self> {
        trace!("Attempting to open a driver handle");

        // TODO(Irides): Remove some of this debug tracing.
        let driver = get_driver_handle(false);
        if driver.is_err() {
            trace!("Encountered error opening driver handle: {}", (driver.as_ref()).unwrap_err())
        }
        let driver = driver?;

        Ok(Self {
           driver
        })
    }
}

impl Backend for UsbDkBackend {
    fn get_devices(&self) -> UsbResult<Vec<DeviceInformation>> {
        // FIXME(Irides): This implementation is incredibly cursed and literally only exists for
        // first pass testing.
        let mut out_buf: Vec<_> = vec![0u8; 4];
        unsafe {
            let out_size = self.driver.ioctl(ioctl::USBDK_COUNT_DEVICES, None, Some(&mut out_buf))
                .expect("Retrieving device count failed");
            assert_eq!(out_size, 4, "Output was {} bytes instead of {}.", out_size, 4);
            let device_count = *(out_buf.as_ptr().cast::<u32>());
            trace!("Found {} devices", device_count);

            // Allocate enough buffers for all of them
            let mut out_buf: Vec<MaybeUninit<DEVICE_INFO>> = Vec::new();
            out_buf.resize(device_count as usize, MaybeUninit::uninit());

            // Ahhhhh fucking cursed (seriously what the fuck is this)
            let buf_ptr = out_buf.as_mut_ptr();
            let buf_size = out_buf.len() * std::mem::size_of::<DEVICE_INFO>();
            let mut slice = std::slice::from_raw_parts_mut(buf_ptr.cast::<u8>(), buf_size);

            let out_size = self.driver.ioctl(ioctl::USBDK_ENUM_DEVICES, None, Some(slice))
                .expect("Enumerating devices failed");
            assert_eq!(out_size as usize / std::mem::size_of::<DEVICE_INFO>(), device_count as usize, "Returned data didn't match device_count");

            let mut output: Vec<DeviceInformation> = Vec::new();
            for driver_info in out_buf {
                let driver_info = driver_info.assume_init();
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
