use std::collections::BTreeSet;
use std::ops::Add;

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
}
