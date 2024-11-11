use crate::devices::device::*;
use crate::info_provider_mod::info_provider::*;
use crate::smart_house_mod::apartment::Apartment;

use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::ops::Add;

pub struct SmartHouse {
    name: String,
    apartments: BTreeMap<String, Apartment>,
}

impl<'a> SmartHouse {
    pub fn new(uniq_name: &str, spaces_vec: Vec<Apartment>) -> Self {
        let local_name = String::from("Smart_Home_").add(uniq_name);
        let mut apartments_map = BTreeMap::new();
        for apartment in spaces_vec {
            apartments_map.insert(apartment.get_name(), apartment);
        }
        Self {
            name: local_name,
            apartments: apartments_map,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_apartments(&self) -> &BTreeMap<String, Apartment> {
        &self.apartments
    }

    pub fn add(&mut self, apartment: Apartment) -> Result<bool, bool> {
        match self.apartments.contains_key(&apartment.get_name()) {
            true => Err(false),
            false => {
                self.apartments.insert(apartment.get_name(), apartment);
                Ok(true)
            }
        }
    }

    pub fn remove(&mut self, room_name: String) -> Result<bool, bool> {
        match self.apartments.contains_key(&room_name) {
            false => Err(false),
            true => {
                self.apartments.remove(&room_name);
                Ok(true)
            }
        }
    }

    pub fn create_report<T>(
        &self,
        info_provider: &'a dyn DeviceInfoProvider<T>,
    ) -> Result<String, String>
    where
        T: Borrow<dyn Device> + Device + 'a,
    {
        let mut report = String::from("Report for Smart House:\t")
            .add(self.name.trim())
            .add("\n");
        for room in self.get_apartments().values() {
            for device in room.get_devices() {
                match info_provider.get_device_info(room.get_name(), device.clone()) {
                    Some(dr) => {
                        report = report.add(&dr).add("\n");
                    }
                    None => {
                        return Err(format!("In a devices in smart_house {}, was not found devices from info_provider.", self.get_name()));
                    }
                }
            }
        }

        Ok(report)
    }

    pub fn switch_state<T>(
        &self,
        apart_name: String,
        device_name: String,
        device_provider: &'a mut dyn DeviceInfoStorageB<'a, T, dyn Device>,
    ) -> Result<String, String>
    where
        T: Borrow<dyn Device> + Device + 'a,
    {
        match device_provider.get(apart_name, device_name) {
            Ok(device) => Ok(device.switch_state()),
            Err(_) => Err("Device not found".to_string()),
        }
    }

    pub fn get_state<T>(
        &self,
        apart_name: String,
        device_name: String,
        device_provider: &'a mut dyn DeviceInfoStorageB<'a, T, dyn Device>,
    ) -> Result<String, String>
    where
        T: Borrow<dyn Device> + Device + 'a,
    {
        match device_provider.get(apart_name, device_name) {
            Ok(device) => Ok(device.get_state()),
            Err(_) => Err("Device not found".to_string()),
        }
    }
}

#[test]
fn test_owning_device_info_provider() {
    let devices_in_living_room = std::collections::BTreeSet::from([]);
    let mut living_room = Apartment::from_set(1, &devices_in_living_room);
    let room_name = living_room.get_name().clone();
    assert!(living_room
        .add(String::from("smart socket in living room"))
        .is_ok());
    assert!(living_room
        .get_devices()
        .contains("smart socket in living room"));
    let binding = vec![];
    let mut smart_house = SmartHouse::new("Test", binding);
    assert!(smart_house.add(living_room).is_ok());
    assert!(smart_house.get_apartments().contains_key(&room_name));
}
