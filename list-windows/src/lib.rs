extern crate device_descriptor;

use core::ptr;
use std::borrow::Borrow;
use std::ffi::CString;

use widestring::U16String;
use winapi::_core::mem::size_of_val;
use winapi::ctypes::{c_char, c_uchar, c_void};
use winapi::shared::minwindef::{BOOL, HLOCAL, LPBYTE, MAX_PATH, PBYTE, DWORD};
use winapi::shared::ntdef::{ANSI_NULL, NULL};
use winapi::shared::winerror::S_OK;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::setupapi::{HDEVINFO, SetupDiGetDeviceRegistryPropertyA, SP_DEVINFO_DATA, SPDRP_HARDWAREID, SPDRP_REMOVAL_POLICY};
use winapi::um::shlobj::SHGetKnownFolderPath;
use winapi::um::winbase::{FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, FormatMessageW, LocalFree};
use winapi::um::winnt::{LPWSTR, PWSTR, RtlZeroMemory};

use device_descriptor::DeviceDescriptor;

use crate::constants::{GENERIC_STORAGE_DRIVERS, KNOWN_FOLDER_IDS, USB_STORAGE_DRIVERS, VHD_HARDWARE_IDS};

mod constants;

fn is_scsi_device(enumerator_name: &str) -> bool {
    for driver_name in GENERIC_STORAGE_DRIVERS.iter() {
        if enumerator_name == *driver_name {
            return true;
        }
    }

    false
}

fn is_usb_device(enumerator_name: &str) -> bool {
    for driver_name in USB_STORAGE_DRIVERS.iter() {
        if enumerator_name == *driver_name {
            return true;
        }
    }

    false
}

fn is_virtual_hard_drive(h_device_info: HDEVINFO, device_info_data: *mut SP_DEVINFO_DATA) -> bool {
    let buffer: PBYTE = 0 as PBYTE;

    let has_hardware_id: BOOL = unsafe { SetupDiGetDeviceRegistryPropertyA(h_device_info, device_info_data, SPDRP_HARDWAREID, ptr::null_mut(), buffer, size_of_val(&buffer) as u32, ptr::null_mut()) };

    if !(has_hardware_id != 0) {
        return false;
    }

    let hardware_id = unsafe { CString::from_raw(buffer as *mut i8).into_string().unwrap() };

    for vhd_hardware_id in VHD_HARDWARE_IDS.iter() {
        if hardware_id.find(vhd_hardware_id) != None {
            return true;
        }
    }

    false
}

fn is_system_device(h_device_info: HDEVINFO, device: &DeviceDescriptor) -> bool {
    let folder_path: PWSTR = ptr::null_mut();
    let mut is_system = false;

    for folder_id in KNOWN_FOLDER_IDS.iter() {
        let result = unsafe { SHGetKnownFolderPath(folder_id, 0, NULL, folder_path as *mut *mut u16) };

        if result == S_OK {
            let u16str: U16String;
            unsafe {
                let mut buffer: LPWSTR = ptr::null_mut();
                let strlen = FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM |
                                                FORMAT_MESSAGE_ALLOCATE_BUFFER |
                                                FORMAT_MESSAGE_IGNORE_INSERTS,
                                            ptr::null(),
                                            GetLastError(),
                                            0,
                                            (&mut buffer as *mut LPWSTR) as LPWSTR,
                                            0,
                                            ptr::null_mut());
                u16str = U16String::from_ptr(buffer, strlen as usize);
                LocalFree(buffer as HLOCAL);
            }
            let system_path = u16str.to_string().unwrap();
            for mountpoint in &device.mountpoints {
                if system_path.contains(mountpoint) {
                    is_system = true;
                    break;
                }
            }
        }
    }

    is_system
}

fn is_removable_device(h_device_info: HDEVINFO, device_info_data: SP_DEVINFO_DATA) -> bool {
    let result: DWORD = 0;

    let has_remove_policy: BOOL = unsafe {
        SetupDiGetDeviceRegistryPropertyA(h_device_info,
                                          device_info_data as *mut SP_DEVINFO_DATA,
                                          SPDRP_REMOVAL_POLICY,
                                          ptr::null_mut(),
                                          &result as PBYTE,
                                          size_of_val(&result) as u32,
                                          ptr::null_mut())
    };

    match result {

    }

    false
}

pub fn list(x: i32) -> i32 {
    x + 1
}
