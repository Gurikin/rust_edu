use crate::devices::*;
use std::borrow::{Borrow, BorrowMut};
use std::collections::*;

fn add_device<'a, T>(
    apart_name: String,
    devices: &mut BTreeMap<String, BTreeMap<String, T>>,
    device: T,
) -> Result<bool, String>
where
    T: Borrow<dyn Device + 'a>,
{
    if devices.contains_key(&apart_name) {
        devices
            .get_mut(&apart_name)
            .unwrap()
            .insert(device.borrow().get_name(), device);
    } else {
        return Err(format!(
            "В данном провайдере отсутствует комната {}",
            &apart_name
        ));
    }
    Ok(true)
}

fn remove_device<'a, T>(
    apart_name: String,
    devices: &mut BTreeMap<String, BTreeMap<String, T>>,
    device_name: String,
) -> Result<bool, String>
where
    T: Borrow<dyn Device + 'a> + device::Device + 'a,
{
    if devices.contains_key(&apart_name) {
        devices.get_mut(&apart_name).unwrap().remove(&device_name);
    } else {
        return Err(format!(
            "В данном провайдере отсутствует комната {}",
            &apart_name
        ));
    }
    Ok(true)
}

fn get_device<'a, T>(
    apart_name: String,
    devices: &BTreeMap<String, BTreeMap<String, T>>,
    device_name: String,
) -> Result<&T, String>
where
    T: Borrow<dyn Device + 'a>,
{
    if devices.contains_key(&apart_name) {
        Ok(devices
            .get(&apart_name)
            .unwrap()
            .get(device_name.as_str())
            .unwrap())
    } else {
        Err(format!(
            "В данном провайдере отсутствует комната {}",
            &apart_name
        ))
    }
}

fn missed_device_err(device_name: String) -> Result<bool, String> {
    Err(format!(
        "Неизвестное устройство {} для данного провайдера",
        device_name
    ))
}

pub trait DeviceInfoProvider<T>
where
    T: Borrow<dyn Device>,
{
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String>;
}

pub struct OwningDeviceInfoProvider<T>
where
    T: Borrow<dyn Device>,
{
    pub devices: BTreeMap<String, BTreeMap<String, T>>,
}

pub trait DeviceInfoStorageB<'b, T, V: ?Sized>
where
    T: Borrow<V>,
{
    fn add(&'b mut self, apart_name: String, device: T) -> Result<bool, String>;
    fn remove(&'b mut self, apart_name: String, device_name: String) -> Result<bool, String>;
    fn get(&'b self, apart_name: String, device_name: String) -> Result<&T, String>;
}

impl<T> DeviceInfoProvider<T> for OwningDeviceInfoProvider<T>
where
    T: Borrow<dyn Device>,
{
    fn get_device_info(&self, room_name: String, device_name: String) -> Option<String> {
        Option::from(
            self.devices
                .get(&room_name)?
                .get(&device_name)?
                .borrow()
                .get_state(),
        )
    }
}

impl<'b, T> DeviceInfoStorageB<'b, T, dyn Device> for OwningDeviceInfoProvider<T>
where
    T: Borrow<dyn Device> + Device + 'static,
{
    fn add(&mut self, apart_name: String, device: T) -> Result<bool, String> {
        match add_device(apart_name, self.devices.borrow_mut(), device) {
            Ok(_) => Ok(true),
            Err(err_text) => Err(err_text),
        }
    }

    fn remove(&mut self, apart_name: String, device_name: String) -> Result<bool, String> {
        match remove_device(apart_name, self.devices.borrow_mut(), device_name.clone()) {
            Ok(_) => Ok(true),
            Err(_) => missed_device_err(device_name),
        }
    }

    fn get(&'b self, apart_name: String, device_name: String) -> Result<&T, String> {
        match get_device(apart_name, &self.devices, device_name.clone()) {
            Ok(device) => Ok(device),
            Err(_) => Err(format!(
                "Неизвестное устройство {} для данного провайдера",
                device_name
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
