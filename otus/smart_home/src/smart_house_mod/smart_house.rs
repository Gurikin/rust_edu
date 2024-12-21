use crate::info_provider_mod::info_provider::DeviceInfoProvider;
use crate::smart_house_mod::apartment::Apartment;

use std::collections::BTreeMap;
use std::ops::Add;

use super::smart_house_err::SmartHouseError;

pub struct SmartHouse<'a> {
    name: String,
    apartments: BTreeMap<String, &'a Apartment>,
}

impl<'a> SmartHouse<'a> {
    pub fn new(uniq_name: &str, spaces_vec: &'a Vec<Apartment>) -> Self {
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

    pub fn get_apartments(&self) -> BTreeMap<String, &Apartment> {
        self.apartments
            .iter()
            .map(|(n, a)| (n.clone(), *a))
            .collect()
    }

    pub fn add(&mut self, apartment: &'a Apartment) -> Result<bool, SmartHouseError> {
        match self.apartments.contains_key(&apartment.get_name()) {
            true => Err(SmartHouseError::Add(
                apartment.get_name(),
                self.get_name().clone(),
            )),
            false => {
                self.apartments.insert(apartment.get_name(), apartment);
                Ok(true)
            }
        }
    }

    pub fn remove(&mut self, room_name: String) -> Result<bool, SmartHouseError> {
        match self.apartments.contains_key(&room_name) {
            false => Err(SmartHouseError::Remove(room_name, self.name.clone())),
            true => {
                self.apartments.remove(&room_name);
                Ok(true)
            }
        }
    }

    pub fn create_report<T: DeviceInfoProvider>(
        &self,
        info_provider: T,
    ) -> Result<String, SmartHouseError> {
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
                        return Err(SmartHouseError::GetReport(device, self.get_name().clone()));
                    }
                }
            }
        }

        Ok(report)
    }
}

#[test]
fn test_owning_device_info_provider() {
    let devices_in_living_room = std::collections::BTreeSet::from([]);
    let mut living_room = Apartment::from_set(1, &devices_in_living_room);
    assert!(living_room
        .add(String::from("smart socket in living room"))
        .is_ok());
    assert!(living_room
        .get_devices()
        .contains("smart socket in living room"));
    let binding = vec![];
    let mut smart_house = SmartHouse::new("Test", &binding);
    assert!(smart_house.add(&living_room).is_ok());
    assert!(smart_house
        .get_apartments()
        .contains_key(&living_room.get_name()));

    let add_err = smart_house.add(&living_room);
    assert!(add_err.is_err());
    println!("{}", add_err.err().unwrap());

    let remove_err = smart_house.remove("Unknown_apartment".to_string());
    assert!(remove_err.is_err());
    println!("{}", remove_err.err().unwrap());
}
