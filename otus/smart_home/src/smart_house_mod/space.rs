use crate::devices::*;

use std::collections::BTreeMap;
use std::ops::{Add, Deref};

pub struct Space {
    pub name: String,
    pub devices: BTreeMap<String, Box<dyn DeviceInfoProvider>>,
}

impl Space {
    pub fn from_devices_vec(cnt: u32, devices_vec: Vec<Box<dyn DeviceInfoProvider>>) -> Self {
        let name = String::from("room_#").add(cnt.to_string().trim());
        let devices = devices_vec.into_iter().map(|d| (d.get_name(), d)).collect();
        Self { name, devices }
    }

    pub fn from_devices_map(cnt: u32, devices: BTreeMap<String, Box<dyn DeviceInfoProvider>>) -> Self {
        let name = String::from("room_#").add(cnt.to_string().trim());
        Self { name, devices }
    }

    pub fn get_device_names(&self) -> Vec<String> {
        self.devices
            .values()
            .clone()
            .map(|f| f.deref().get_name())
            .collect::<Vec<String>>()
    }

    pub fn get_devices(&self) -> &BTreeMap<String, Box<dyn DeviceInfoProvider>> {
        &self.devices
    }
}
