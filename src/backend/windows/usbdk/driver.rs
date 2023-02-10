//! Interfaces for communicating with the UsbDk kernel driver.

use std::{ffi::{OsStr, c_void}, iter, os::windows::prelude::*, mem::MaybeUninit};

use winapi::um::{fileapi::{CreateFileW, OPEN_EXISTING}, winnt::{GENERIC_READ, GENERIC_WRITE, FILE_ATTRIBUTE_NORMAL, HANDLE}, winbase::FILE_FLAG_OVERLAPPED, errhandlingapi::GetLastError, handleapi::{INVALID_HANDLE_VALUE, CloseHandle}, ioapiset::DeviceIoControl, minwinbase::OVERLAPPED};

use crate::{UsbResult, Error};

/// A wrapper type to simplify communication with the driver.
#[repr(transparent)]
#[derive(Debug)]
pub(crate) struct DriverHandle {
    pub(crate) handle: HANDLE
}

// Windows defines IO operations on a `HANDLE` to be thread-safe. Thus our type is *also* thread-safe.
unsafe impl Send for DriverHandle {}
unsafe impl Sync for DriverHandle {}

/// Owned handle to a kernel driver. Releases the internal handle on drop.
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

    /// Internal helper for submitting raw `DeviceIoControl` requests, checking the result for failure.
    unsafe fn ioctl_raw(&self, ctl_code: u32,
                        in_ptr: *mut c_void, in_size: u32,
                        out_ptr: *mut c_void, out_size: u32,
                        overlapped: *mut OVERLAPPED) -> UsbResult<u32> {
        // The actual number of bytes returned from the ioctl; must match expected_size after ioctl.
        let mut actual_size: u32 = 0;

        // Issue the operation...
        let result = DeviceIoControl(
            self.handle,
            ctl_code,
            in_ptr,
            in_size,
            out_ptr,
            out_size,
            &mut actual_size,
            overlapped
        );

        // Non-zero result indicates success.
        if result == 0 {
            return Err(Error::OsError(GetLastError().into()));
        }

        Ok(actual_size)
    }

    /// Helper for submitting IOCTLs that simply read a value of the provided type.
    pub unsafe fn ioctl_read<T>(&self, ctl_code: u32) -> UsbResult<T> {
        let mut out: MaybeUninit<T> = MaybeUninit::uninit();

        // Safety: Windows only accepts a u32, so types used should never be able to exceed that anyway.
        let expected_size: u32 = std::mem::size_of::<T>()
            .try_into()
            .expect(&format!("internal consistency: expected_size somehow did not fit in a u32"))
        ;

        let actual_size = self.ioctl_raw(
            ctl_code,
            std::ptr::null_mut(),
            0,
            out.as_mut_ptr().cast(),
            expected_size,
            std::ptr::null_mut()
        )?;

        // Check post-conditions, size driver returned must be same as size of type.
        assert_eq!(expected_size, actual_size, "ioctl ctl_code = {:#x} returned {} bytes, expected {}",
            ctl_code, actual_size, expected_size);

        // Safety: we've checked that the driver claims to have fully initialized our type.
        Ok(out.assume_init())
    }

    /// Helper for submitting IOCTLs that simply read multiple values of the provided type.
    pub unsafe fn ioctl_read_vec<T: Copy>(&self, ctl_code: u32, count: u32) -> UsbResult<Vec<T>> {
        let mut out: Vec<MaybeUninit<T>> = vec![MaybeUninit::uninit(); count as usize];

        // Safety: Windows only accepts a u32, so types used should never be able to exceed that anyway.
        let expected_size: u32 = (std::mem::size_of::<T>() * count as usize)
            .try_into()
            .expect(&format!("internal consistency: expected_size somehow did not fit in a u32"))
        ;

        let actual_size = self.ioctl_raw(
            ctl_code,
            std::ptr::null_mut(),
            0,
            out.as_mut_ptr().cast(),
            expected_size,
            std::ptr::null_mut()
        )?;

        // Check post-conditions, size driver returned must be same as size of type.
        assert_eq!(expected_size, actual_size, "ioctl ctl_code = {:#x} returned {} bytes, expected {}",
            ctl_code, actual_size, expected_size);

        // Safety: we've checked that the driver claims to have fully initialized our type.
        // `MaybeUninit<T>` and `T` are defined to have the same representation, so this transmute
        // is defined to be safe.
        Ok(std::mem::transmute(out))
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
