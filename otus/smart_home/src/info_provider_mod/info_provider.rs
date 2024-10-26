use crate::devices::*;
use std::collections::*;
use std::option::Option;

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    pub sockets: BTreeMap<String, BTreeMap<String, SmartSocket>>,
}
pub struct BorrowingDeviceInfoProvider<'a> {
    pub sockets: BTreeMap<String, BTreeMap<String, &'a SmartSocket>>,
    pub therms: BTreeMap<String, BTreeMap<String, &'a SmartThermometer>>,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String> {
        Option::from(self.sockets.get(&room_name)?.get(&device_name)?.get_state())
    }
}

impl<'a> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a> {
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String> {
        let container = self
            .sockets
            .get(&room_name)
            .and_then(|v| v.get(&device_name))
            .map(|v| v.get_state())
            .or_else(|| {
                self.therms
                    .get(&room_name)
                    .and_then(|v| v.get(&device_name))
                    .map(|v| v.get_state())
            })?;

        Some(container)
    }
}

pub trait DeviceInfoStorage<T> {
    fn add(&mut self, apart_name: String, device: T) -> Result<bool, String>;
    fn remove(&mut self, apart_name: String, device: T) -> Result<bool, String>;
}

pub trait DeviceInfoStorageRef<'a, T> {
    fn add(&mut self, apart_name: String, device: &'a T) -> Result<bool, String>;
    fn remove(&mut self, apart_name: String, device: &'a T) -> Result<bool, String>;
}

impl DeviceInfoStorage<SmartSocket> for OwningDeviceInfoProvider {
    fn add(&mut self, apart_name: String, device: SmartSocket) -> Result<bool, String> {
        match DeviceType::PowerSocket.eq(&device.info.device_type) {
            true => {
                if self.sockets.contains_key(&apart_name) {
                    self.sockets
                        .get_mut(&apart_name)
                        .unwrap()
                        .insert(device.get_name().clone(), device);
                } else {
                    return Err(format!(
                        "В данном провайдере отсутствует комната {}",
                        &apart_name
                    ));
                }
                Ok(true)
            }
            false => Err(String::from(
                "Неизвестный тип устройства для данного провайдера",
            )),
        }
    }

    fn remove(&mut self, apart_name: String, device: SmartSocket) -> Result<bool, String> {
        match DeviceType::PowerSocket == device.info.device_type {
            true => {
                if self.sockets.contains_key(&apart_name) {
                    self.sockets
                        .get_mut(&apart_name)
                        .unwrap()
                        .remove(&device.get_name());
                } else {
                    return Err(format!(
                        "В данном провайдере отсутствует комната {}",
                        &apart_name
                    ));
                }
                Ok(true)
            }
            false => Err(format!(
                "Неизвестный тип устройства {} для данного провайдера",
                device.info.device_type
            )),
        }
    }
}

impl<'a> DeviceInfoStorageRef<'a, SmartSocket> for BorrowingDeviceInfoProvider<'a> {
    fn add(&mut self, apart_name: String, device: &'a SmartSocket) -> Result<bool, String> {
        match DeviceType::PowerSocket == device.info.device_type {
            true => {
                if self.sockets.contains_key(&apart_name) {
                    self.sockets
                        .get_mut(&apart_name)
                        .unwrap()
                        .insert(device.get_name().clone(), device);
                } else {
                    return Err(format!(
                        "В данном провайдере отсутствует комната {}",
                        &apart_name
                    ));
                }
                Ok(true)
            }
            false => Err(String::from(
                "Неизвестный тип устройства для данного провайдера",
            )),
        }
    }

    fn remove(&mut self, apart_name: String, device: &'a SmartSocket) -> Result<bool, String> {
        match DeviceType::PowerSocket == device.info.device_type {
            true => {
                if self.sockets.contains_key(&apart_name) {
                    self.sockets
                        .get_mut(&apart_name)
                        .unwrap()
                        .remove(&device.get_name());
                } else {
                    return Err(format!(
                        "В данном провайдере отсутствует комната {}",
                        &apart_name
                    ));
                }
                Ok(true)
            }
            false => Err(format!(
                "Неизвестный тип устройства {} для данного провайдера",
                device.info.device_type
            )),
        }
    }
}

impl<'a> DeviceInfoStorageRef<'a, SmartThermometer> for BorrowingDeviceInfoProvider<'a> {
    fn add(&mut self, apart_name: String, device: &'a SmartThermometer) -> Result<bool, String> {
        match DeviceType::Thermometer == device.info.device_type {
            true => {
                if self.therms.contains_key(&apart_name) {
                    self.therms
                        .get_mut(&apart_name)
                        .unwrap()
                        .insert(device.get_name().clone(), device);
                } else {
                    return Err(format!(
                        "В данном провайдере отсутствует комната {}",
                        &apart_name
                    ));
                }
                Ok(true)
            }
            false => Err(String::from(
                "Неизвестный тип устройства для данного провайдера",
            )),
        }
    }

    fn remove(&mut self, apart_name: String, device: &'a SmartThermometer) -> Result<bool, String> {
        match DeviceType::Thermometer == device.info.device_type {
            true => {
                if self.therms.contains_key(&apart_name) {
                    self.therms
                        .get_mut(&apart_name)
                        .unwrap()
                        .remove(&device.get_name());
                } else {
                    return Err(format!(
                        "В данном провайдере отсутствует комната {}",
                        &apart_name
                    ));
                }
                Ok(true)
            }
            false => Err(format!(
                "Неизвестный тип устройства {} для данного провайдера",
                device.info.device_type
            )),
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
        sockets: sockets_map,
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
        sockets: sockets_map,
        therms: terms_map,
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
        sockets: sockets_map,
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
        sockets: sockets_map,
        therms: terms_map,
    };
    assert!(info_provider
        .sockets
        .get(&kitchen.get_name())
        .unwrap()
        .is_empty());
    assert!(info_provider
        .sockets
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
