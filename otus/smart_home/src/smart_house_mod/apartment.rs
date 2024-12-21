use std::collections::BTreeSet;
use std::ops::Add;

use super::smart_house_err::ApartmentError;

pub struct Apartment {
    name: String,
    devices: BTreeSet<String>,
}

impl Apartment {
    pub fn from_vec(cnt: u32, devices_vec: Vec<String>) -> Self {
        let name = String::from("room_#").add(cnt.to_string().trim());
        let devices = devices_vec.into_iter().collect();
        Self { name, devices }
    }

    pub fn from_set(cnt: u32, devices_set: &BTreeSet<String>) -> Self {
        let name = String::from("room_#").add(cnt.to_string().trim());
        let devices = devices_set.clone().into_iter().collect();
        Self { name, devices }
    }

    pub fn get_name(&self) -> String {
        let local_name = &self.name;
        local_name.to_string()
    }

    pub fn get_devices(&self) -> BTreeSet<String> {
        let local_devices = Box::new(&self.devices);
        local_devices.iter().cloned().collect()
    }

    pub fn add(&mut self, device_name: String) -> Result<bool, ApartmentError> {
        match self.devices.insert(device_name.clone()) {
            true => Ok(true),
            false => Err(ApartmentError::AddError(self.get_name(), device_name)),
        }
    }

    pub fn remove(&mut self, device_name: String) -> Result<bool, ApartmentError> {
        match self.devices.remove(&device_name) {
            true => Ok(true),
            false => Err(ApartmentError::RemoveError(self.get_name(), device_name)),
        }
    }
}

#[test]
fn test_owning_device_info_provider() {
    let devices_in_living_room = BTreeSet::from([]);
    let mut living_room = Apartment::from_set(1, &devices_in_living_room);
    assert_eq!(
        false,
        living_room.get_devices().contains("computer_socket_name")
    );
    assert!(living_room
        .add(String::from("computer_socket_name"))
        .is_ok());
    assert_eq!(
        true,
        living_room
            .add(String::from("monitor_socket_name"))
            .ok()
            .unwrap()
    );

    let add_err = living_room.add(String::from("computer_socket_name"));
    assert!(add_err.is_err());
    println!("{}", add_err.err().unwrap());

    let remove_err = living_room.remove(String::from("unknown_device"));
    assert!(remove_err.is_err());
    println!("{}", remove_err.err().unwrap());
}
