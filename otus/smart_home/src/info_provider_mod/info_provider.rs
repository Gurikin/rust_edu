use crate::devices::*;
use std::collections::*;
use std::option::Option;

pub enum Device {
    SmartSocket(SmartSocket),
    Thermometer(SmartThermometer),
}

impl Device {
    fn get_state(&self) -> String {
        match self {
            Device::SmartSocket(s) => s.get_state(),
            Device::Thermometer(t) => t.get_state(),
        }
    }

    fn get_name(&self) -> String {
        match self {
            Device::SmartSocket(s) => s.get_name(),
            Device::Thermometer(t) => t.get_name(),
        }
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    pub devices: BTreeMap<String, BTreeMap<String, Device>>,
}
pub struct BorrowingDeviceInfoProvider<'a> {
    pub devices: BTreeMap<String, BTreeMap<String, &'a Device>>,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String> {
        let container = self
            .devices
            .get(&room_name)
            .and_then(|v| v.get(&device_name))
            .map(|v| v.get_state())?;

        Some(container)
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String> {
        let container = self
            .devices
            .get(&room_name)
            .and_then(|v| v.get(&device_name))
            .map(|v| v.get_state())?;

        Some(container)
    }
}

pub trait DeviceInfoStorage {
    fn add(&mut self, apart_name: String, device: Device) -> Result<bool, String>;
    fn remove(&mut self, apart_name: String, device: Device) -> Result<bool, String>;
}

pub trait DeviceInfoStorageRef<'a> {
    fn add(&mut self, apart_name: String, device: &'a Device) -> Result<bool, String>;
    fn remove(&mut self, apart_name: String, device: &'a Device) -> Result<bool, String>;
}

impl DeviceInfoStorage for OwningDeviceInfoProvider {
    fn add(&mut self, apart_name: String, device: Device) -> Result<bool, String> {
        if self.devices.contains_key(&apart_name) {
            self.devices
                .get_mut(&apart_name)
                .unwrap()
                .insert(device.get_name().clone(), device);
            Ok(true)
        } else {
            Err(format!(
                "В данном провайдере отсутствует комната {}",
                &apart_name
            ))
        }
    }

    fn remove(&mut self, apart_name: String, device: Device) -> Result<bool, String> {
        if self.devices.contains_key(&apart_name) {
            self.devices
                .get_mut(&apart_name)
                .unwrap()
                .remove(&device.get_name());
            Ok(true)
        } else {
            Err(format!(
                "В данном провайдере отсутствует комната {}",
                &apart_name
            ))
        }
    }
}

impl<'a> DeviceInfoStorageRef<'a> for BorrowingDeviceInfoProvider<'a> {
    fn add(&mut self, apart_name: String, device: &'a Device) -> Result<bool, String> {
        if self.devices.contains_key(&apart_name) {
            self.devices
                .get_mut(&apart_name)
                .unwrap()
                .insert(device.get_name().clone(), device);
            Ok(true)
        } else {
            Err(format!(
                "В данном провайдере отсутствует комната {}",
                &apart_name
            ))
        }
    }

    fn remove(&mut self, apart_name: String, device: &'a Device) -> Result<bool, String> {
        if self.devices.contains_key(&apart_name) {
            self.devices
                .get_mut(&apart_name)
                .unwrap()
                .remove(&device.get_name());
            Ok(true)
        } else {
            Err(format!(
                "В данном провайдере отсутствует комната {}",
                &apart_name
            ))
        }
    }
}

#[test]
fn test_owning_device_info_provider() {
    let socket_name = "Test Smart Socket".to_string();
    let smart_socket = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: socket_name.clone(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    };
    let devices_in_living_room = BTreeSet::from([socket_name.clone()]);
    let living_room = crate::smart_house_mod::Apartment::from_set(1, &devices_in_living_room);
    let mut sockets_map = BTreeMap::new();
    let mut room_devices = BTreeMap::new();
    room_devices.insert(socket_name.clone(), smart_socket);
    sockets_map.insert(living_room.get_name(), room_devices);
    let info_provider = OwningDeviceInfoProvider {
        devices: sockets_map,
    };
    assert!(info_provider
        .get_device_info(living_room.get_name(), socket_name.clone())
        .is_some());
    println!(
        "{}",
        info_provider
            .get_device_info(living_room.get_name(), socket_name.clone())
            .get_or_insert(String::from(""))
    );
}

#[test]
fn test_borrowing_device_info_provider() {
    let socket_name = "Test Smart Socket".to_string();
    let smart_socket = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: socket_name.clone(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 380,
    };

    let term_name = "Test Thermometer".to_string();
    let test_therm = SmartThermometer {
        info: DeviceInfo {
            id: 0,
            name: term_name.clone(),
            device_type: DeviceType::Thermometer,
            description: "Thermometer in the living room".to_string(),
        },
        temperature: 40,
    };

    let devices_in_kitchen = vec![socket_name.clone(), term_name.clone()];
    let kitchen = crate::smart_house_mod::Apartment::from_vec(1, devices_in_kitchen);

    let mut sockets_map = BTreeMap::new();
    let mut socket_devices = BTreeMap::new();
    socket_devices.insert(socket_name.clone(), &smart_socket);
    sockets_map.insert(kitchen.get_name(), socket_devices);

    let mut terms_map = BTreeMap::new();
    let mut therm_devices = BTreeMap::new();
    therm_devices.insert(term_name.clone(), &test_therm);
    terms_map.insert(kitchen.get_name(), therm_devices);

    let info_provider = BorrowingDeviceInfoProvider {
        devices: sockets_map,
    };

    assert!(info_provider
        .get_device_info(kitchen.get_name(), socket_name.clone())
        .is_some());
    assert!(info_provider
        .get_device_info(kitchen.get_name(), term_name.clone())
        .is_some());
    println!(
        "{}",
        info_provider
            .get_device_info(kitchen.get_name(), term_name.clone())
            .get_or_insert(String::from("where is my info?"))
    );
}

#[test]
fn test_owning_device_info_storage() {
    let socket_name = "Test Smart Socket".to_string();
    let smart_socket = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: socket_name.clone(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    };
    let devices_in_living_room = BTreeSet::from([socket_name.clone()]);
    let living_room = crate::smart_house_mod::Apartment::from_set(1, &devices_in_living_room);
    let mut sockets_map = BTreeMap::new();
    let room_devices = BTreeMap::new();
    sockets_map.insert(living_room.get_name(), room_devices);
    let mut info_provider = OwningDeviceInfoProvider {
        devices: sockets_map,
    };
    assert!(info_provider
        .add(living_room.get_name(), smart_socket)
        .is_ok());
    assert!(info_provider
        .get_device_info(living_room.get_name(), socket_name.clone())
        .is_some());
    println!(
        "{}",
        info_provider
            .get_device_info(living_room.get_name(), socket_name.clone())
            .get_or_insert(String::from(""))
    );
}

#[test]
fn test_borrowing_device_info_storage() {
    let socket_name = "Test Smart Socket".to_string();
    let smart_socket = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: socket_name.clone(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 380,
    };

    let term_name = "Test Thermometer".to_string();
    let test_therm = SmartThermometer {
        info: DeviceInfo {
            id: 0,
            name: term_name.clone(),
            device_type: DeviceType::Thermometer,
            description: "Thermometer in the living room".to_string(),
        },
        temperature: 40,
    };

    let devices_in_kitchen = vec![socket_name.clone(), term_name.clone()];
    let kitchen = crate::smart_house_mod::Apartment::from_vec(1, devices_in_kitchen);

    let mut sockets_map = BTreeMap::new();
    let socket_devices = BTreeMap::new();
    // socket_devices.insert(socket_name.clone(), &smart_socket);
    sockets_map.insert(kitchen.get_name(), socket_devices);

    let mut terms_map = BTreeMap::new();
    let therm_devices = BTreeMap::new();
    // therm_devices.insert(term_name.clone(), &test_therm);
    terms_map.insert(kitchen.get_name(), therm_devices);

    let mut info_provider = BorrowingDeviceInfoProvider {
        devices: sockets_map,
        therms: terms_map,
    };
    assert!(info_provider
        .devices
        .get(&kitchen.get_name())
        .unwrap()
        .is_empty());
    assert!(info_provider
        .devices
        .get(&kitchen.get_name())
        .unwrap()
        .is_empty());

    assert!(info_provider.add(kitchen.get_name(), &smart_socket).is_ok());
    assert!(info_provider.add(kitchen.get_name(), &test_therm).is_ok());

    assert!(info_provider
        .get_device_info(kitchen.get_name(), socket_name.clone())
        .is_some());
    assert!(info_provider
        .get_device_info(kitchen.get_name(), term_name.clone())
        .is_some());
    println!(
        "{}",
        info_provider
            .get_device_info(kitchen.get_name(), term_name.clone())
            .get_or_insert(String::from("where is my info?"))
    );
}
