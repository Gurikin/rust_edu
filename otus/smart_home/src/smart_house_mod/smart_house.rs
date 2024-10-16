use crate::info_provider_mod::info_provider::DeviceInfoProvider;
use crate::smart_house_mod::apartment::Apartment;

use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;

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

    pub fn get_apartments(&self) -> BTreeMap<String, &Apartment> {
        self.apartments
            .iter()
            .map(|(n, a)| (n.clone(), *a))
            .collect()
    }

    pub fn devices(&self, apartment: String) -> BTreeSet<String> {
        self.apartments
            .get(&apartment)
            .expect("Apartment not found")
            .get_devices()
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, info_provider: T) -> String {
        let mut report = String::from("Report for Smart House:\t")
            .add(self.name.trim())
            .add("\n");
        for room in self.get_apartments().values() {
            for device in self.devices(room.get_name()) {
                match info_provider.get_device_info(room.get_name(), device.clone()) {
                    Some(dr) => report = report.add(&dr).add("\n"),
                    None => println!("Device {} was not found in the provided provider", device),
                }
            }
        }

        report
    }
}
