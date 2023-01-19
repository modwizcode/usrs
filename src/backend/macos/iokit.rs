//! Helpers for working with IOKit.

use std::ffi::{c_void, CStr, CString};

use core_foundation_sys::{
    number::{kCFNumberSInt64Type, CFNumberGetValue, CFNumberRef},
    string::{kCFStringEncodingUTF8, CFStringGetCStringPtr, CFStringRef},
    uuid::CFUUIDBytes,
};
use io_kit_sys::{
    kIORegistryIterateParents, kIORegistryIterateRecursively, keys::kIOServicePlane, ret::*,
    types::io_iterator_t, IOObjectRelease, IORegistryEntrySearchCFProperty, CFSTR,
};
use log::error;

use super::iokit_c::{
    self, kIOUSBFindInterfaceDontCare, kIOUSBNoAsyncPortErr, kIOUSBPipeStalled,
    kIOUSBTransactionTimeout, kIOUSBUnknownPipeErr, CFUUIDGetUUIDBytes, IOCFPlugInInterface,
    IOUSBDevRequest, IOUSBDevRequestTO, IOUSBFindInterfaceRequest, UInt16, UInt8,
};
use crate::error::{self, Error, UsbResult};

//
// Support declarations.
// These determine which versions of macOS we support, so they should be chosen carefully.
//

/// Alias that select the "version 500" (IOKit 5.0.0) version of UsbDevice, which means
/// that we support macOS versions back to 10.7.3, which is currently every version that Rust
/// supports. Use this instead of touching the iokit_c structure; this may be bumped to
/// (compatible) newer versions of the struct as Rust's support changes.
pub(crate) type UsbDevice = iokit_c::IOUSBDeviceStruct500;
pub(crate) type UsbInterface = iokit_c::IOUSBInterfaceStruct500;

pub(crate) fn usb_device_type_id() -> CFUUIDBytes {
    unsafe { CFUUIDGetUUIDBytes(iokit_c::kIOUSBDeviceInterfaceID500()) }
}

pub(crate) fn usb_interface_type_id() -> CFUUIDBytes {
    unsafe { CFUUIDGetUUIDBytes(iokit_c::kIOUSBInterfaceInterfaceID500()) }
}

//
// Helpers for working with IOKit types.
//

/// Helper for calling IOKit function pointers.
macro_rules! call_unsafe_iokit_function {
    ($ptr:expr, $function:ident) => {{
        unsafe {
            let func = (**$ptr).$function.expect("function pointer from IOKit was null");
            func($ptr as *mut c_void)
        }
    }};
    ($ptr:expr, $function:ident, $($args:expr),*) => {{
        unsafe {
            let func = (**$ptr).$function.expect("function pointer from IOKit was null");
            func($ptr as *mut c_void, $($args),*)
        }
    }};
}

//
// Wrappers around IOKit types.
//

/// Type alias to make it clear when our u32 handle is an IoIterator. It's clear, right?
pub(crate) type IoIterator = IoObject;

/// Type alias to make it clear(er) when our u32 handle is a handle to an IO service.
pub(crate) type IoService = IoObject;

/// Wrapper for an IOKit IO-object that automatically drops it.
#[derive(Debug)]
pub(crate) struct IoObject {
    object: u32,
}

impl IoObject {
    pub(crate) fn new(object: u32) -> Self {
        IoObject { object }
    }

    /// Fetches the inner handle for passing to IOKit functions.
    pub(crate) fn get(&self) -> u32 {
        self.object
    }

    /// Returns true iff the object has been created incorrectly.
    /// Use to maintain internal consistency.
    pub(crate) fn is_invalid(&self) -> bool {
        self.object == 0
    }
}

impl Drop for IoObject {
    fn drop(&mut self) {
        unsafe {
            IOObjectRelease(self.object);
        }
    }
}

/// Wrapper around a **IOCFPluginInterface that automatically releases it.
#[derive(Debug)]
pub(crate) struct PluginInterface {
    interface: *mut *mut IOCFPlugInInterface,
}

impl PluginInterface {
    pub(crate) fn new(interface: *mut *mut IOCFPlugInInterface) -> Self {
        Self { interface }
    }

    /// Fetches the inner pointer for passing to IOKit functions.
    pub(crate) fn get(&self) -> *mut *mut IOCFPlugInInterface {
        self.interface
    }
}

impl Drop for PluginInterface {
    fn drop(&mut self) {
        call_unsafe_iokit_function!(self.interface, Release);
    }
}

// Wrapper around a **UsbDevice that helps us poke at its innards.
#[derive(Debug)]
pub(crate) struct OsDevice {
    device: *mut *mut UsbDevice,

    /// True iff the device is currently open.
    is_open: bool,
}

#[allow(dead_code)]
impl OsDevice {
    pub(crate) fn new(device: *mut *mut UsbDevice) -> Self {
        Self {
            device,
            is_open: false,
        }
    }

    /// Opens the device, allowing the other functions on this type to be used.
    pub fn open(&mut self) -> UsbResult<()> {
        // If we're already open, we're done!
        if self.is_open {
            return Ok(());
        }

        // Otherwise, open the device.
        UsbResult::from_io_return(call_unsafe_iokit_function!(self.device, USBDeviceOpen))?;

        self.is_open = true;
        Ok(())
    }

    /// Applies a configuration to the device.
    pub fn set_configuration(&mut self, index: u8) -> UsbResult<()> {
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.device,
            SetConfiguration,
            index
        ))
    }

    /// Attempts to perform a Bus Reset on the device.
    pub fn reset(&mut self) -> UsbResult<()> {
        UsbResult::from_io_return(call_unsafe_iokit_function!(self.device, ResetDevice))
    }

    /// Performs a control request on the device, without wrapping the unsafe behavior of
    /// the contained IOUSbDevRequest. See also [[device_request_with_timeout]].
    pub fn device_request(&self, request: &mut IOUSBDevRequest) -> UsbResult<()> {
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.device,
            DeviceRequest,
            request
        ))
    }

    /// Performs a control request on the device, without wrapping the unsafe behavior of
    /// the contained IOUSbDevRequest. See also [[device_request]].
    pub fn device_request_with_timeout(&self, request: &mut IOUSBDevRequestTO) -> UsbResult<()> {
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.device,
            DeviceRequestTO,
            request
        ))
    }

    /// Aborts any active transfer on EP0.
    pub fn abort_ep0(&mut self) -> UsbResult<()> {
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.device,
            USBDeviceAbortPipeZero
        ))
    }

    /// Places the device into power-save mode, or takes it out.
    /// A value of true places the device into suspend.
    pub fn suspend(&mut self, suspend: bool) -> UsbResult<()> {
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.device,
            USBDeviceSuspend,
            suspend as u8
        ))
    }

    /// Returns an IOKit iterator that can be used to iterate over all interfaces on this device.
    pub fn create_interface_iterator(&mut self) -> UsbResult<IoObject> {
        let mut iterator: io_iterator_t = 0;

        // For our purposes, we don't want macOS to filter the interface list
        // by anything in particular (e.g. by device class), so we'll just construct
        // a big ol' list of Don't Cares.
        let mut dont_care = IOUSBFindInterfaceRequest {
            bInterfaceClass: kIOUSBFindInterfaceDontCare,
            bInterfaceSubClass: kIOUSBFindInterfaceDontCare,
            bInterfaceProtocol: kIOUSBFindInterfaceDontCare,
            bAlternateSetting: kIOUSBFindInterfaceDontCare,
        };

        // Finally, actually ask macOS to give us that iterator...
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.device,
            CreateInterfaceIterator,
            &mut dont_care,
            &mut iterator
        ))?;

        // ... and pack it all up nicely in an IoObject for our user.
        Ok(IoObject::new(iterator))
    }

    /// Closes the active USB device.
    pub fn close(&mut self) {
        if !self.is_open {
            return;
        }

        if call_unsafe_iokit_function!(self.device, USBDeviceClose) == kIOReturnSuccess {
            self.is_open = false;
        }
    }
}

impl Drop for OsDevice {
    fn drop(&mut self) {
        // If the device is still open, close it...
        self.close();

        // ... and decrease macOS's refcount, so the device can be dealloc'd.
        call_unsafe_iokit_function!(self.device, Release);
    }
}

/// Helper for fetching endpoint metadata from our OsInterface.
/// At some point, a caller will convert this up into OS-agnostic metadata.
#[allow(dead_code)]
pub(crate) struct EndpointMetadata {
    pub(crate) direction: u8,
    pub(crate) number: u8,
    pub(crate) transfer_type: u8,
    pub(crate) max_packet_size: u16,
    pub(crate) interval: u8,
    pub(crate) max_burst: u8,
    pub(crate) mult: u8,
    pub(crate) bytes_per_interval: u16,
}

/// Wrapper around a **UsbInterface that helps us poke at its contained function pointers.
#[derive(Debug)]
pub(crate) struct OsInterface {
    interface: *mut *mut UsbInterface,

    /// The interface number associated with the given OS interface.
    interface_number: u8,

    /// If set, all function calls on this interface will return PermissionDenied.
    ///
    /// This allows us to act like permission is being denied on the individual calls,
    /// rather than on creating the interface object. This makes the macOS backend have
    /// the same behavior as other backends, which don't try to "open" interfaces in an early step.
    deny_all: bool,

    /// True iff the interface is currently open.
    is_open: bool,
}

#[allow(dead_code)]
impl OsInterface {
    pub(crate) fn new(interface: *mut *mut UsbInterface, interface_number: u8) -> Self {
        Self {
            interface,
            interface_number,
            deny_all: false,
            is_open: false,
        }
    }

    pub(crate) fn new_denying_placeholder(interface_number: u8) -> Self {
        Self {
            interface: std::ptr::null_mut(),
            interface_number,
            deny_all: true,
            is_open: false,
        }
    }

    pub fn interface_number(&self) -> UsbResult<u8> {
        Ok(self.interface_number)
    }

    /// Opens the interface, allowing the other functions on this type to be used.
    pub fn open(&mut self) -> UsbResult<()> {
        if self.deny_all {
            return Err(Error::PermissionDenied);
        }

        // If we're already open, we're done!
        if self.is_open {
            return Ok(());
        }

        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.interface,
            USBInterfaceOpen
        ))
    }

    /// Returns the number of endpoints associated with the interface.
    pub fn endpoint_count(&mut self) -> UsbResult<u8> {
        let mut count: UInt8 = 0;

        // If we won't allow access to any actual functions,
        // lie that we have no endpoints, for lack of a better thing to do.
        if self.deny_all {
            return Ok(0);
        }

        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.interface,
            GetNumEndpoints,
            &mut count
        ))?;

        Ok(count as u8)
    }

    pub fn endpoint_properties(&mut self, pipe_ref: u8) -> UsbResult<EndpointMetadata> {
        if self.deny_all {
            return Err(Error::PermissionDenied);
        }

        let mut direction: UInt8 = 0;
        let mut number: UInt8 = 0;
        let mut transfer_type: UInt8 = 0;
        let mut max_packet_size: UInt16 = 0;
        let mut interval: UInt8 = 0;
        let mut max_burst: UInt8 = 0;
        let mut mult: UInt8 = 0;
        let mut bytes_per_interval: UInt16 = 0;

        // We have entered hell, it's real, and it is this IOKit function signature.
        UsbResult::from_io_return(call_unsafe_iokit_function!(
            self.interface,
            GetPipePropertiesV2,
            pipe_ref,
            &mut direction,
            &mut number,
            &mut transfer_type,
            &mut max_packet_size,
            &mut interval,
            &mut max_burst,
            &mut mult,
            &mut bytes_per_interval
        ))?;

        Ok(EndpointMetadata {
            direction,
            number,
            transfer_type,
            max_packet_size,
            interval,
            max_burst,
            mult,
            bytes_per_interval,
        })
    }

    /// Closes the active USB interface.
    pub fn close(&mut self) {
        if !self.is_open {
            return;
        }

        if self.deny_all {
            panic!("internal consistency: somehow, we have an open deny_all interface? what have we _done_");
        }

        if call_unsafe_iokit_function!(self.interface, USBInterfaceClose) == kIOReturnSuccess {
            self.is_open = false;
        }
    }
}

impl Drop for OsInterface {
    fn drop(&mut self) {
        // If the device is still open, close it...
        self.close();

        // ... and decrease macOS's refcount, so the device can be dealloc'd.
        call_unsafe_iokit_function!(self.interface, Release);
    }
}

//
// Helpers for working with CoreFoundation / IOKit types.
//

/// Rustified version of the CFSTR C macro.
macro_rules! cfstr {
    ($string:expr) => {{
        let cstr = CString::new($string).unwrap();
        CFSTR(cstr.as_ptr())
    }};
}
pub(crate) use cfstr;

/// Translates an IOReturn error to its USRs equivalent.
#[allow(non_upper_case_globals, non_snake_case)]
fn io_return_to_error(rc: IOReturn) -> error::Error {
    match rc {
        // Substitute IOKit messages for our equivalent...
        kIOReturnNotOpen => Error::DeviceNotOpen,
        kIOReturnNoDevice => Error::DeviceNotFound,
        kIOReturnExclusiveAccess => Error::DeviceReserved,
        kIOReturnBadArgument => Error::InvalidArgument,
        kIOReturnAborted => Error::Aborted,
        kIOReturnOverrun => Error::Overrun,
        kIOReturnNoResources => Error::PermissionDenied,
        kIOUSBNoAsyncPortErr => Error::DeviceNotOpen,
        kIOUSBUnknownPipeErr => Error::InvalidEndpoint,
        kIOUSBPipeStalled => Error::Stalled,
        kIOUSBTransactionTimeout => Error::TimedOut,
        _ => Error::OsError(rc as i64),
    }
}

// Extend UsbResult with IOKit conversions.
pub(crate) trait IOKitEmptyResultExtension {
    fn from_io_return(io_return: IOReturn) -> UsbResult<()>;
}

pub(crate) trait IOKitResultExtension<T> {
    fn from_io_return_and_value(io_return: IOReturn, ok_value: T) -> UsbResult<T>;
}

impl IOKitEmptyResultExtension for UsbResult<()> {
    /// Creates s UsbResult from an IOKit return code.
    fn from_io_return(io_return: IOReturn) -> UsbResult<()> {
        // If this wasn't a success, translate our error.
        if io_return != kIOReturnSuccess {
            Err(io_return_to_error(io_return))
        } else {
            Ok(())
        }
    }
}

impl<T> IOKitResultExtension<T> for UsbResult<T> {
    /// Creates s UsbResult from an IOKit return code.
    fn from_io_return_and_value(io_return: IOReturn, ok_value: T) -> UsbResult<T> {
        // If this wasn't a success, translate our error.
        if io_return != kIOReturnSuccess {
            Err(io_return_to_error(io_return))
        }
        // Otherwise, package up our Ok value.
        else {
            Ok(ok_value)
        }
    }
}

/// Converts a CFNumberRef to a Rust integer.
pub(crate) fn number_from_cf_number<T: TryFrom<u64>>(number_ref: CFNumberRef) -> UsbResult<T> {
    unsafe {
        let mut result: u64 = 0;

        // Promote a null pointer error to a slightly nicer panic.
        if number_ref.is_null() {
            panic!("something passed a null pointer to number_from_cf_number T_T");
        }

        let succeeded = CFNumberGetValue(
            number_ref,
            kCFNumberSInt64Type,
            &mut result as *mut u64 as *mut c_void,
        );
        if !succeeded {
            error!("Failed to convert a NumberRef into a CFNumber!");
            return Err(Error::UnspecifiedOsError);
        }

        result.try_into().map_err(|_| Error::UnspecifiedOsError)
    }
}

/// Converts a raw CFString into a Rust string.
pub(crate) fn string_from_cf_string(string_ref: CFStringRef) -> UsbResult<Option<String>> {
    unsafe {
        // Promote a null pointer error to a slightly nicer panic.
        if string_ref.is_null() {
            panic!("something passed a null pointer to string_from_cf_string T_T");
        }

        let c_string = CFStringGetCStringPtr(string_ref, kCFStringEncodingUTF8);
        if c_string.is_null() {
            return Ok(None);
        }

        Ok(Some(CStr::from_ptr(c_string).to_string_lossy().to_string()))
    }
}

/// Queries IOKit and fetches a device property from the IORegistry.
/// Accepts a usb_device_iterator and the property name.
pub(crate) fn get_iokit_numeric_device_property<T: TryFrom<u64>>(
    device: io_iterator_t,
    property: &str,
) -> UsbResult<T> {
    unsafe {
        let service_plane: *mut i8 = kIOServicePlane as *mut i8;

        let raw_value = IORegistryEntrySearchCFProperty(
            device,
            service_plane,
            cfstr!(property),
            std::ptr::null(),
            kIORegistryIterateRecursively | kIORegistryIterateParents,
        ) as CFNumberRef;
        if raw_value.is_null() {
            error!("Failed to read numeric device property {}!", property);
            return Err(Error::UnspecifiedOsError);
        }
        number_from_cf_number::<T>(raw_value)
    }
}

/// Queries IOKit and fetches a device property from the IORegistry.
/// Accepts a usb_device_iterator and the property name.
pub(crate) fn get_iokit_string_device_property(
    device: io_iterator_t,
    property: &str,
) -> UsbResult<Option<String>> {
    unsafe {
        let service_plane: *mut i8 = kIOServicePlane as *mut i8;

        let raw_value = IORegistryEntrySearchCFProperty(
            device,
            service_plane,
            cfstr!(property),
            std::ptr::null(),
            kIORegistryIterateRecursively | kIORegistryIterateParents,
        ) as CFStringRef;
        if raw_value.is_null() {
            return Ok(None);
        }

        string_from_cf_string(raw_value)
    }
}
