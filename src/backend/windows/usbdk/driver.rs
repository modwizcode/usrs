//! Interfaces for communicating with the UsbDk kernel driver.

use std::{ffi::OsStr, iter, os::windows::prelude::*};

use winapi::{um::{fileapi::{CreateFileW, OPEN_EXISTING}, winnt::{GENERIC_READ, GENERIC_WRITE, FILE_ATTRIBUTE_NORMAL, HANDLE}, winbase::FILE_FLAG_OVERLAPPED, errhandlingapi::GetLastError, handleapi::{INVALID_HANDLE_VALUE, CloseHandle}, ioapiset::DeviceIoControl}, ctypes::c_void};

use crate::{UsbResult, Error};

/// A wrapper type to simplify communication with the driver.
#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct DriverHandle {
    pub(crate) handle: HANDLE
}

// DriverHandle represents ownership over the internal driver resource,
// thus it is Send + Sync, here we share this fact with the compiler.
unsafe impl Send for DriverHandle {}
unsafe impl Sync for DriverHandle {}

/// FIXME(Irides): This somewhat models the design of the C++ library, it should be refactored into a more rust
/// friendly design before release
/// NOTE: Probably want to rename this to OwnedDriver or similar (like `OwnedHandle` which this
/// subsumes), since it should be clear this is a /moved/ interface.
impl DriverHandle {
    /// Create from the provided [`HANDLE`].
    ///
    /// NOTE: This intentionally does not check the provided handle for validity.
    pub(crate) fn new(handle: HANDLE) -> Self {
        Self {
            handle
        }
    }

    /// Is the internal handle invalid?
    pub fn is_invalid(&self) -> bool {
        self.handle == INVALID_HANDLE_VALUE
    }

    /// Helper for submitting ioctl requests.
    /// FIXME(Irides): This isn't a very Rust-y interface, it's just temporary for testing
    /// This doesn't require `&mut` access, thus it's extra unsafe
    pub unsafe fn ioctl(&self, code: u32, in_buf: Option<&mut [u8]>, out_buf: Option<&mut [u8]>) -> UsbResult<u32> {
        // How many bytes were actually written to out_buf.
        let mut bytes_written: u32 = 0;

        // Gods this is a cursed and terrible way of doing this...
        let in_len = in_buf.as_ref().map_or(0, |slice| slice.len());
        let in_ptr = in_buf.map_or(std::ptr::null_mut(), |slice| slice.as_mut_ptr());

        let out_len = out_buf.as_ref().map_or(0, |slice| slice.len());
        let out_ptr = out_buf.map_or(std::ptr::null_mut(), |slice| slice.as_mut_ptr());

        // Issue the request...
        let result = DeviceIoControl(
            self.handle,
            code,
            in_ptr.cast::<c_void>(),
            in_len as u32,
            out_ptr.cast::<c_void>(),
            out_len as u32,
            &mut bytes_written as *mut u32,
            std::ptr::null_mut()
        );

        if result == 0 {
            return Err(Error::OsError(GetLastError().into()));
        }

        Ok(bytes_written)
    }
}

/// Close on drop if not invalid.
impl Drop for DriverHandle {
    fn drop(&mut self) {
        if !self.is_invalid() {
            unsafe { CloseHandle(self.handle); }
        }
    }
}

// We use the unicode versions of these functions for consistency.

// Helper shamelessly stolen from wdi-rs.
fn os_str_to_null_terminated_vec(s: &OsStr) -> Vec<u16> {
    s.encode_wide().chain(iter::once(0)).collect()
}

// Path to the UsbDk driver (from the UsbDk headers).
const DRIVER_DEVICE_PATH: &'static str = r#"\\.\UsbDk"#;

/// Open a handle to the UsbDk driver. Theoretically each process only needs one of these.
pub(crate) fn get_driver_handle(overlapped: bool) -> UsbResult<DriverHandle> {
    unsafe {
        // Oh Rust, why won't you let me make this a constant...
        let path = os_str_to_null_terminated_vec(AsRef::as_ref(DRIVER_DEVICE_PATH));

        // Try to attach to the driver's interface...
        let result = DriverHandle::new(
            CreateFileW(
                path.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                std::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL | (if overlapped { FILE_FLAG_OVERLAPPED } else { 0 }),
                std::ptr::null_mut()
            ) as HANDLE
        );
        if result.is_invalid() {
            return Err(Error::OsError(GetLastError().into()));
        }

        Ok(result)
    }
}
