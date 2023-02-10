//! Interfaces for communicating with the UsbDk kernel driver.

use std::{ffi::{OsStr, c_void}, iter, os::windows::prelude::*, mem::MaybeUninit};

use winapi::um::{fileapi::{CreateFileW, OPEN_EXISTING}, winnt::{GENERIC_READ, GENERIC_WRITE, FILE_ATTRIBUTE_NORMAL, HANDLE}, winbase::FILE_FLAG_OVERLAPPED, errhandlingapi::GetLastError, handleapi::{INVALID_HANDLE_VALUE, CloseHandle}, ioapiset::DeviceIoControl};

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

    /// Temporary helper for submitting typed buffer ioctls.
    /// TODO: This should be moved to a macro helper.
    pub unsafe fn ioctl_typed_buf<T: Sized>(&self, code: u32, out: &mut [MaybeUninit<T>], in_ptr: *mut c_void, in_len: u32) -> UsbResult<()> {
        // This is the size of the buffer for output; expect exactly this much data.
        let expected_size: u32 = (std::mem::size_of::<T>() * out.len()).try_into().unwrap();
        // The actual number of bytes returned from the ioctl; must match expected_size after ioctl.
        let mut actual_size: u32 = 0;

        // Issue the operation...
        let result = DeviceIoControl(
            self.handle,
            code,
            in_ptr,
            in_len,
            out.as_mut_ptr().cast(),
            expected_size,
            &mut actual_size,
            std::ptr::null_mut()
        );

        // Non-zero result indicates success.
        if result == 0 {
            return Err(Error::OsError(GetLastError().into()));
        }

        // Check post-conditions, size driver returned must be same as size of type.
        assert_eq!(expected_size, actual_size, "ioctl {:#x} returned {} bytes, expected {}",
            code, actual_size, expected_size);

        Ok(())
    }

    /// Temporary helper for submitting typed ioctls.
    /// TODO: This should be moved to a macro helper used to define ioctl functions for each of the
    /// relevant types.
    pub unsafe fn ioctl_typed<T: Sized>(&self, code: u32, in_ptr: *mut c_void, in_len: u32) -> UsbResult<T> {
        // This is the of the type we expect the ioctl to provide.
        let expected_size: u32 = std::mem::size_of::<T>().try_into().unwrap();
        let mut out: MaybeUninit<T> = MaybeUninit::uninit();

        // The actual number of bytes returned from the ioctl, we panic if this doesn't match
        // expected_size.
        let mut actual_size: u32 = 0;

        // Issue the operation...
        let result = DeviceIoControl(
            self.handle,
            code,
            in_ptr,
            in_len,
            out.as_mut_ptr().cast(),
            expected_size,
            &mut actual_size,
            std::ptr::null_mut()
        );

        // Non-zero result indicates success.
        if result == 0 {
            return Err(Error::OsError(GetLastError().into()));
        }

        // Check post-conditions, size driver returned must be same as size of type.
        assert_eq!(expected_size, actual_size, "ioctl {:#x} returned {} bytes, expected {}",
            code, actual_size, expected_size);

        Ok(out.assume_init())
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
