//! Macro helpers for building IOCTL numbers.
//!
//! Definitions are derrived from `devioctl.h` from the Windows SDK.

use winapi::shared::{ntdef::{UCHAR, USHORT, WCHAR}, basetsd::ULONG64};

/// Takes a device_type, function, method, and access
///
/// This is equivilant to the `CTL_CODE()` macro in `devioctl.h`.
macro_rules! ctl_code {
    ($ty:expr, $func:expr, $method:expr, $access:expr) => {
        ((($ty as u32) << 16) |
         (($access as u32) << 14) |
         (($func as u32) << 2) |
         ($method as u32))
    };
}

pub(crate) const METHOD_BUFFERED:   u8 = 0;
pub(crate) const METHOD_IN_DIRECT:  u8 = 1;
pub(crate) const METHOD_OUT_DIRECT: u8 = 2;
pub(crate) const METHOD_NEITHER:    u8 = 4;

pub(crate) const FILE_ANY_ACCESS:     u16 = 0;
pub(crate) const FILE_SPECIAL_ACCESS: u16 = FILE_ANY_ACCESS;
pub(crate) const FILE_READ_ACCESS:    u16 = 0x0001;
pub(crate) const FILE_WRITE_ACCESS:   u16 = 0x0002;

// IOCTLs from UsbDk itself

pub(crate) const USBDK_DEVICE_TYPE: u16 = 50000;

macro_rules! usbdk_ioctl {
    ($name:ident, $func:expr, $access:expr) => {
        pub(crate) const $name: u32 = ctl_code!(USBDK_DEVICE_TYPE, $func, METHOD_BUFFERED, $access);
    }
 }

macro_rules! usbdk_read {
    ($name:ident, $func:expr) => {
        usbdk_ioctl!($name, $func, FILE_READ_ACCESS);
    }
}

macro_rules! usbdk_write {
    ($name:ident, $func:expr) => {
        usbdk_ioctl!($name, $func, FILE_WRITE_ACCESS);
    }
}

macro_rules! usbdk_readwrite {
    ($name:ident, $func:expr) => {
        usbdk_ioctl!($name, $func, FILE_READ_ACCESS | FILE_WRITE_ACCESS);
    }
}

usbdk_read!(USBDK_COUNT_DEVICES, 0x851);
usbdk_read!(USBDK_ENUM_DEVICES,  0x852);

// Data structures returned and used by ioctls.
// TODO(Irides): Clean up this mess, like everything else...
#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct USB_DEVICE_DESCRIPTOR {
    pub(crate) bLength: UCHAR,
    pub(crate) bDescriptorType: UCHAR,
    pub(crate) bcdUSB: USHORT,
    pub(crate) bDeviceClass: UCHAR,
    pub(crate) bDeviceSubClass: UCHAR,
    pub(crate) bDeviceProtocol: UCHAR,
    pub(crate) bMaxPacketSize0: UCHAR,
    pub(crate) idVendor: USHORT,
    pub(crate) idProduct: USHORT,
    pub(crate) bcdDevice: USHORT,
    pub(crate) iManufacturer: UCHAR,
    pub(crate) iProduct: UCHAR,
    pub(crate) iSerialNumber: UCHAR,
    pub(crate) bNumConfigurations: UCHAR
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DEVICE_ID {
    pub(crate) device_id: [WCHAR; 200],
    pub(crate) instance_id: [WCHAR; 200]
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DEVICE_INFO {
    pub(crate) ID: DEVICE_ID,
    pub(crate) FilterID: ULONG64,
    pub(crate) Port: ULONG64,
    pub(crate) Speed: ULONG64,
    pub(crate) DeviceDescriptor: USB_DEVICE_DESCRIPTOR
}
