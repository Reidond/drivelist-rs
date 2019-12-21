#include "pch.h"

int main() {
  std::vector<Drivelist::DeviceDescriptor> devices;
  devices = Drivelist::ListStorageDevices();

  return 0;
}