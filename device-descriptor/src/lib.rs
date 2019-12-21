pub struct MountPoint {
    path: String
}

pub struct DeviceDescriptor {
    enumerator: String,
    bus_type: String,
    bus_version: Option<String>,
    device: String,
    device_path: Option<String>,
    raw: String,
    description: String,
    error: Option<String>,
    size: u64,
    block_size: u32,
    logical_block_size: u32,
    pub mountpoints: Vec<String>,
    mountpointsLabels: Vec<String>,
    is_read_only: bool,
    is_system: bool,
    is_virtual: bool,
    is_removable: bool,
    is_card: bool,
    is_scsi: bool,
    is_usb: bool,
    is_uas: Option<bool>,
}