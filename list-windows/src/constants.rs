extern crate winapi;

use std::collections::BTreeSet;
use winapi::shared::guiddef::GUID;
use winapi::um::knownfolders::FOLDERID_Profile;
use winapi::um::knownfolders::FOLDERID_ProgramFiles;
use winapi::um::knownfolders::FOLDERID_ProgramFilesX86;
use winapi::um::knownfolders::FOLDERID_Windows;

const GUID_DEVICE_INTERFACE_DISK: GUID = GUID {
  Data1: 0x53F56307u32,
  Data2: 0xB6BF,
  Data3: 0x11D0,
  Data4: [0x94, 0xF2, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const GUID_DEVICE_INTERFACE_CDROM: GUID = GUID {
  Data1: 0x53F56308u32,
  Data2: 0xB6BF,
  Data3: 0x11D0,
  Data4: [0x94, 0xF2, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const GUID_DEVICE_INTERFACE_USB_HUB: GUID = GUID {
  Data1: 0xF18A0E88u32,
  Data2: 0xC30C,
  Data3: 0x11D0,
  Data4: [0x88, 0x15, 0x00, 0xA0, 0xC9, 0x06, 0xBE, 0xD8],
};

const GUID_DEVICE_INTERFACE_FLOPPY: GUID = GUID {
  Data1: 0x53F56311u32,
  Data2: 0xB6BF,
  Data3: 0x11D0,
  Data4: [0x94, 0xF2, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const GUID_DEVICE_INTERFACE_WRITEONCEDISK: GUID = GUID {
  Data1: 0x53F5630Cu32,
  Data2: 0xB6BF,
  Data3: 0x11D0,
  Data4: [0x94, 0xF2, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const GUID_DEVICE_INTERFACE_TAPE: GUID = GUID {
  Data1: 0x53F5630Bu32,
  Data2: 0xB6BF,
  Data3: 0x11D0,
  Data4: [0x94, 0xF2, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const GUID_DEVICE_INTERFACE_USB_DEVICE: GUID = GUID {
  Data1: 0xA5DCBF10u32,
  Data2: 0x6530,
  Data3: 0x11D2,
  Data4: [0x90, 0x1F, 0x00, 0xC0, 0x4F, 0xB9, 0x51, 0xED],
};

const GUID_DEVICE_INTERFACE_VOLUME: GUID = GUID {
  Data1: 0x53F5630Du32,
  Data2: 0xB6BF,
  Data3: 0x11D0,
  Data4: [0x94, 0xF2, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const GUID_DEVICE_INTERFACE_STORAGEPORT: GUID = GUID {
  Data1: 0x2ACCFE60u32,
  Data2: 0xC130,
  Data3: 0x11D2,
  Data4: [0xB0, 0x82, 0x00, 0xA0, 0xC9, 0x1E, 0xFB, 0x8B],
};

const USB_STORAGE_DRIVERS: BTreeSet<&str> = vec![
  "USBSTOR",
  "UASPSTOR",
  "VUSBSTOR",
  "RTUSER",
  "CMIUCR",
  "EUCR",
  "ETRONSTOR",
  "ASUSSTPT",
]
.into_iter()
.collect();

const GENERIC_STORAGE_DRIVERS: BTreeSet<&str> = vec![
  "SCSI", "SD", "PCISTOR", "RTSOR", "JMCR", "JMCF", "RIMMPTSK", "RIMSPTSK", "RIXDPTSK", "TI21SONY",
  "ESD7SK", "ESM7SK", "O2MD", "O2SD", "VIACR",
]
.into_iter()
.collect();

const VHD_HARDWARE_IDS: BTreeSet<&str> = vec![
  "Arsenal_________Virtual_",
  "KernSafeVirtual_________",
  "Msft____Virtual_Disk____",
  "VMware__VMware_Virtual_S",
]
.into_iter()
.collect();

const KNOWN_FOLDER_IDS: [GUID; 4] = [
  FOLDERID_Windows,
  FOLDERID_Profile,
  FOLDERID_ProgramFiles,
  FOLDERID_ProgramFilesX86,
];
