#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals)]
//! Types and defintions from the driver headers. This is tweaked bindgen output.
//! TODO(Irides): We should regenerate this with a few options set before doing the tweaks.

pub const MAX_DEVICE_ID_LEN: u32 = 200;
pub const USBDK_DRIVER_NAME: &[u8; 6usize] = b"UsbDk\0";
pub const USBDK_DRIVER_FILE_NAME: &[u8; 10usize] = b"UsbDk.sys\0";
pub const USBDK_DRIVER_INF_NAME: &[u8; 10usize] = b"UsbDk.inf\0";
pub const USBDK_DEVICE_NAME: &[u8; 14usize] = b"\\Device\\UsbDk\0";
pub const USBDK_DOSDEVICE_NAME: &[u8; 18usize] = b"\\DosDevices\\UsbDk\0";
pub const USBDK_USERMODE_NAME: &[u8; 10usize] = b"\\\\.\\UsbDk\0";
pub const _USBDK_HIDER_NAME: &[u8; 11usize] = b"UsbDkHider\0";
pub const USBDK_HIDER_DEVICE_NAME: &[u8; 19usize] = b"\\Device\\UsbDkHider\0";
pub const USBDK_DOS_HIDER_DEVICE_NAME: &[u8; 23usize] = b"\\DosDevices\\UsbDkHider\0";
pub const USBDK_USERMODE_HIDER_NAME: &[u8; 15usize] = b"\\\\.\\UsbDkHider\0";
pub const USBDK_DEVICE_TYPE: u32 = 50000;
pub type WCHAR = ::std::os::raw::c_ushort;
pub type PCWCHAR = *mut ::std::os::raw::c_ushort;
pub type USHORT = ::std::os::raw::c_ushort;
pub type ULONG64 = ::std::os::raw::c_ulonglong;
pub type PVOID64 = *mut ::std::os::raw::c_void;
pub type UCHAR = ::std::os::raw::c_uchar;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _USB_CONFIGURATION_DESCRIPTOR {
    pub bLength: UCHAR,
    pub bDescriptorType: UCHAR,
    pub wTotalLength: USHORT,
    pub bNumInterfaces: UCHAR,
    pub bConfigurationValue: UCHAR,
    pub iConfiguration: UCHAR,
    pub bmAttributes: UCHAR,
    pub MaxPower: UCHAR,
}
#[test]
fn bindgen_test_layout__USB_CONFIGURATION_DESCRIPTOR() {
    const UNINIT: ::std::mem::MaybeUninit<_USB_CONFIGURATION_DESCRIPTOR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_USB_CONFIGURATION_DESCRIPTOR>(),
        9usize,
        concat!("Size of: ", stringify!(_USB_CONFIGURATION_DESCRIPTOR))
    );
    assert_eq!(
        ::std::mem::align_of::<_USB_CONFIGURATION_DESCRIPTOR>(),
        1usize,
        concat!("Alignment of ", stringify!(_USB_CONFIGURATION_DESCRIPTOR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bLength) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(bLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bDescriptorType) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(bDescriptorType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wTotalLength) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(wTotalLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bNumInterfaces) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(bNumInterfaces)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bConfigurationValue) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(bConfigurationValue)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iConfiguration) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(iConfiguration)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bmAttributes) as usize - ptr as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(bmAttributes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).MaxPower) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_USB_CONFIGURATION_DESCRIPTOR),
            "::",
            stringify!(MaxPower)
        )
    );
}
pub type USB_CONFIGURATION_DESCRIPTOR = _USB_CONFIGURATION_DESCRIPTOR;
pub type PUSB_CONFIGURATION_DESCRIPTOR = *mut _USB_CONFIGURATION_DESCRIPTOR;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct USB_DEVICE_DESCRIPTOR {
    pub bLength: UCHAR,
    pub bDescriptorType: UCHAR,
    pub bcdUSB: USHORT,
    pub bDeviceClass: UCHAR,
    pub bDeviceSubClass: UCHAR,
    pub bDeviceProtocol: UCHAR,
    pub bMaxPacketSize0: UCHAR,
    pub idVendor: USHORT,
    pub idProduct: USHORT,
    pub bcdDevice: USHORT,
    pub iManufacturer: UCHAR,
    pub iProduct: UCHAR,
    pub iSerialNumber: UCHAR,
    pub bNumConfigurations: UCHAR,
}
#[test]
fn bindgen_test_layout_USB_DEVICE_DESCRIPTOR() {
    const UNINIT: ::std::mem::MaybeUninit<USB_DEVICE_DESCRIPTOR> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<USB_DEVICE_DESCRIPTOR>(),
        18usize,
        concat!("Size of: ", stringify!(USB_DEVICE_DESCRIPTOR))
    );
    assert_eq!(
        ::std::mem::align_of::<USB_DEVICE_DESCRIPTOR>(),
        1usize,
        concat!("Alignment of ", stringify!(USB_DEVICE_DESCRIPTOR))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bLength) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bDescriptorType) as usize - ptr as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bDescriptorType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bcdUSB) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bcdUSB)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bDeviceClass) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bDeviceClass)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bDeviceSubClass) as usize - ptr as usize },
        5usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bDeviceSubClass)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bDeviceProtocol) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bDeviceProtocol)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bMaxPacketSize0) as usize - ptr as usize },
        7usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bMaxPacketSize0)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).idVendor) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(idVendor)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).idProduct) as usize - ptr as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(idProduct)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bcdDevice) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bcdDevice)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iManufacturer) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(iManufacturer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iProduct) as usize - ptr as usize },
        15usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(iProduct)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).iSerialNumber) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(iSerialNumber)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bNumConfigurations) as usize - ptr as usize },
        17usize,
        concat!(
            "Offset of field: ",
            stringify!(USB_DEVICE_DESCRIPTOR),
            "::",
            stringify!(bNumConfigurations)
        )
    );
}
pub type PUSB_DEVICE_DESCRIPTOR = *mut USB_DEVICE_DESCRIPTOR;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_DEVICE_ID {
    pub DeviceID: [WCHAR; 200usize],
    pub InstanceID: [WCHAR; 200usize],
}
#[test]
fn bindgen_test_layout_tag_USB_DK_DEVICE_ID() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_DEVICE_ID> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_DEVICE_ID>(),
        800usize,
        concat!("Size of: ", stringify!(tag_USB_DK_DEVICE_ID))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_DEVICE_ID>(),
        2usize,
        concat!("Alignment of ", stringify!(tag_USB_DK_DEVICE_ID))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DeviceID) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_ID),
            "::",
            stringify!(DeviceID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).InstanceID) as usize - ptr as usize },
        400usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_ID),
            "::",
            stringify!(InstanceID)
        )
    );
}
pub type USB_DK_DEVICE_ID = tag_USB_DK_DEVICE_ID;
pub type PUSB_DK_DEVICE_ID = *mut tag_USB_DK_DEVICE_ID;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_DEVICE_INFO {
    pub ID: USB_DK_DEVICE_ID,
    pub FilterID: ULONG64,
    pub Port: ULONG64,
    pub Speed: ULONG64,
    pub DeviceDescriptor: USB_DEVICE_DESCRIPTOR,
}
#[test]
fn bindgen_test_layout_tag_USB_DK_DEVICE_INFO() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_DEVICE_INFO> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_DEVICE_INFO>(),
        848usize,
        concat!("Size of: ", stringify!(tag_USB_DK_DEVICE_INFO))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_DEVICE_INFO>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_USB_DK_DEVICE_INFO))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ID) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_INFO),
            "::",
            stringify!(ID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).FilterID) as usize - ptr as usize },
        800usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_INFO),
            "::",
            stringify!(FilterID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Port) as usize - ptr as usize },
        808usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_INFO),
            "::",
            stringify!(Port)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Speed) as usize - ptr as usize },
        816usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_INFO),
            "::",
            stringify!(Speed)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).DeviceDescriptor) as usize - ptr as usize },
        824usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_DEVICE_INFO),
            "::",
            stringify!(DeviceDescriptor)
        )
    );
}
pub type USB_DK_DEVICE_INFO = tag_USB_DK_DEVICE_INFO;
pub type PUSB_DK_DEVICE_INFO = *mut tag_USB_DK_DEVICE_INFO;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST {
    pub ID: USB_DK_DEVICE_ID,
    pub Index: ULONG64,
}
#[test]
fn bindgen_test_layout_tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST>(),
        808usize,
        concat!(
            "Size of: ",
            stringify!(tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ID) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST),
            "::",
            stringify!(ID)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Index) as usize - ptr as usize },
        800usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST),
            "::",
            stringify!(Index)
        )
    );
}
pub type USB_DK_CONFIG_DESCRIPTOR_REQUEST = tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST;
pub type PUSB_DK_CONFIG_DESCRIPTOR_REQUEST = *mut tag_USB_DK_CONFIG_DESCRIPTOR_REQUEST;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_ISO_TRANSFER_RESULT {
    pub ActualLength: ULONG64,
    pub TransferResult: ULONG64,
}
#[test]
fn bindgen_test_layout_tag_USB_DK_ISO_TRANSFER_RESULT() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_ISO_TRANSFER_RESULT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_ISO_TRANSFER_RESULT>(),
        16usize,
        concat!("Size of: ", stringify!(tag_USB_DK_ISO_TRANSFER_RESULT))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_ISO_TRANSFER_RESULT>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_USB_DK_ISO_TRANSFER_RESULT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ActualLength) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_ISO_TRANSFER_RESULT),
            "::",
            stringify!(ActualLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TransferResult) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_ISO_TRANSFER_RESULT),
            "::",
            stringify!(TransferResult)
        )
    );
}
pub type USB_DK_ISO_TRANSFER_RESULT = tag_USB_DK_ISO_TRANSFER_RESULT;
pub type PUSB_DK_ISO_TRANSFER_RESULT = *mut tag_USB_DK_ISO_TRANSFER_RESULT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_GEN_TRANSFER_RESULT {
    pub BytesTransferred: ULONG64,
    pub UsbdStatus: ULONG64,
}
#[test]
fn bindgen_test_layout_tag_USB_DK_GEN_TRANSFER_RESULT() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_GEN_TRANSFER_RESULT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_GEN_TRANSFER_RESULT>(),
        16usize,
        concat!("Size of: ", stringify!(tag_USB_DK_GEN_TRANSFER_RESULT))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_GEN_TRANSFER_RESULT>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_USB_DK_GEN_TRANSFER_RESULT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).BytesTransferred) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_GEN_TRANSFER_RESULT),
            "::",
            stringify!(BytesTransferred)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).UsbdStatus) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_GEN_TRANSFER_RESULT),
            "::",
            stringify!(UsbdStatus)
        )
    );
}
pub type USB_DK_GEN_TRANSFER_RESULT = tag_USB_DK_GEN_TRANSFER_RESULT;
pub type PUSB_DK_GEN_TRANSFER_RESULT = *mut tag_USB_DK_GEN_TRANSFER_RESULT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_TRANSFER_RESULT {
    pub GenResult: USB_DK_GEN_TRANSFER_RESULT,
    pub IsochronousResultsArray: PVOID64,
}
#[test]
fn bindgen_test_layout_tag_USB_DK_TRANSFER_RESULT() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_TRANSFER_RESULT> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_TRANSFER_RESULT>(),
        24usize,
        concat!("Size of: ", stringify!(tag_USB_DK_TRANSFER_RESULT))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_TRANSFER_RESULT>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_USB_DK_TRANSFER_RESULT))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).GenResult) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_RESULT),
            "::",
            stringify!(GenResult)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsochronousResultsArray) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_RESULT),
            "::",
            stringify!(IsochronousResultsArray)
        )
    );
}
pub type USB_DK_TRANSFER_RESULT = tag_USB_DK_TRANSFER_RESULT;
pub type PUSB_DK_TRANSFER_RESULT = *mut tag_USB_DK_TRANSFER_RESULT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USB_DK_TRANSFER_REQUEST {
    pub EndpointAddress: ULONG64,
    pub Buffer: PVOID64,
    pub BufferLength: ULONG64,
    pub TransferType: ULONG64,
    pub IsochronousPacketsArraySize: ULONG64,
    pub IsochronousPacketsArray: PVOID64,
    pub Result: USB_DK_TRANSFER_RESULT,
}
#[test]
fn bindgen_test_layout_tag_USB_DK_TRANSFER_REQUEST() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USB_DK_TRANSFER_REQUEST> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USB_DK_TRANSFER_REQUEST>(),
        72usize,
        concat!("Size of: ", stringify!(tag_USB_DK_TRANSFER_REQUEST))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USB_DK_TRANSFER_REQUEST>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_USB_DK_TRANSFER_REQUEST))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).EndpointAddress) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(EndpointAddress)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Buffer) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(Buffer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).BufferLength) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(BufferLength)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).TransferType) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(TransferType)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsochronousPacketsArraySize) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(IsochronousPacketsArraySize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).IsochronousPacketsArray) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(IsochronousPacketsArray)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).Result) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USB_DK_TRANSFER_REQUEST),
            "::",
            stringify!(Result)
        )
    );
}
pub type USB_DK_TRANSFER_REQUEST = tag_USB_DK_TRANSFER_REQUEST;
pub type PUSB_DK_TRANSFER_REQUEST = *mut tag_USB_DK_TRANSFER_REQUEST;
pub const TransferResult_TransferFailure: TransferResult = 0;
pub const TransferResult_TransferSuccess: TransferResult = 1;
pub const TransferResult_TransferSuccessAsync: TransferResult = 2;
pub type TransferResult = ::std::os::raw::c_uint;
pub const USB_DK_DEVICE_SPEED_NoSpeed: USB_DK_DEVICE_SPEED = 0;
pub const USB_DK_DEVICE_SPEED_LowSpeed: USB_DK_DEVICE_SPEED = 1;
pub const USB_DK_DEVICE_SPEED_FullSpeed: USB_DK_DEVICE_SPEED = 2;
pub const USB_DK_DEVICE_SPEED_HighSpeed: USB_DK_DEVICE_SPEED = 3;
pub const USB_DK_DEVICE_SPEED_SuperSpeed: USB_DK_DEVICE_SPEED = 4;
pub type USB_DK_DEVICE_SPEED = ::std::os::raw::c_uint;
pub const USB_DK_TRANSFER_TYPE_ControlTransferType: USB_DK_TRANSFER_TYPE = 0;
pub const USB_DK_TRANSFER_TYPE_BulkTransferType: USB_DK_TRANSFER_TYPE = 1;
pub const USB_DK_TRANSFER_TYPE_InterruptTransferType: USB_DK_TRANSFER_TYPE = 2;
pub const USB_DK_TRANSFER_TYPE_IsochronousTransferType: USB_DK_TRANSFER_TYPE = 3;
pub type USB_DK_TRANSFER_TYPE = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tag_USBDK_ALTSETTINGS_IDXS {
    pub InterfaceIdx: ULONG64,
    pub AltSettingIdx: ULONG64,
}
#[test]
fn bindgen_test_layout_tag_USBDK_ALTSETTINGS_IDXS() {
    const UNINIT: ::std::mem::MaybeUninit<tag_USBDK_ALTSETTINGS_IDXS> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tag_USBDK_ALTSETTINGS_IDXS>(),
        16usize,
        concat!("Size of: ", stringify!(tag_USBDK_ALTSETTINGS_IDXS))
    );
    assert_eq!(
        ::std::mem::align_of::<tag_USBDK_ALTSETTINGS_IDXS>(),
        8usize,
        concat!("Alignment of ", stringify!(tag_USBDK_ALTSETTINGS_IDXS))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).InterfaceIdx) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USBDK_ALTSETTINGS_IDXS),
            "::",
            stringify!(InterfaceIdx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).AltSettingIdx) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tag_USBDK_ALTSETTINGS_IDXS),
            "::",
            stringify!(AltSettingIdx)
        )
    );
}
pub type USBDK_ALTSETTINGS_IDXS = tag_USBDK_ALTSETTINGS_IDXS;
pub type PUSBDK_ALTSETTINGS_IDXS = *mut tag_USBDK_ALTSETTINGS_IDXS;
