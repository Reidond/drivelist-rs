#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::*;

include!("./bindings.rs");

pub fn list() {
  let storage_devices: std_vector = unsafe { Drivelist_ListStorageDevices() };
  println!("{:?}", storage_devices);
}
