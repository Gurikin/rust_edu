use crate::smart_house_mod::space::Space;

use crate::devices::DeviceInfoProvider;
use std::collections::BTreeMap;
use std::ops::Add;

pub struct SmartHouse {
    name: String,
    spaces: BTreeMap<String, Space>,
}

impl SmartHouse {
    pub fn new(uniq_name: &str, spaces_vec: Vec<Space>) -> Self {
        let name = String::from("Smart_Home_").add(uniq_name);
        let mut space_map: BTreeMap<String, Space> = BTreeMap::new();
        let mut cnt = 1;
        for space in spaces_vec {
            space_map.insert(space.name, Space::from_devices_map(cnt, space.devices));
            cnt += 1;
        }
        Self {
            name,
            spaces: space_map,
        }
    }

    pub fn get_rooms(&self) -> Vec<&String> {
        self.spaces.keys().clone().collect::<Vec<&String>>()
    }

    pub fn devices(&self, room: &str) -> Vec<String> {
        self.spaces
            .get(room)
            .expect("Room not found")
            .get_device_names()
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, info_provider: T) -> String {
        let mut report = String::from("Report for Smart House:\t")
            .add(self.name.trim())
            .add("\n");
        report = report.add(info_provider.get_state().trim()).add("\n");
        report
    }
}
