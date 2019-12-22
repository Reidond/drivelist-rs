#include "pch.h"
#include <iostream>

int main() {
  std::vector<DeviceDescriptor> devices;
  devices = ListStorageDevices();

  for (DeviceDescriptor device : devices) {
    std::cout << device.description << std::endl;
  }

  return 0;
}
