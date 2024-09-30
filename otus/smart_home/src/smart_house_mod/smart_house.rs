use crate::smart_house_mod::space::Space;

use std::collections::BTreeMap;
use std::ops::Add;
use crate::devices::DeviceInfoProvider;

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
        // let space_map: HashMap<String, Space> = spaces_vec.into_iter().map(|Space { name, devices }| (&name, Space::)).collect();
        Self {
            name,
            spaces: space_map,
        }
        // todo!("реализовать инициализацию дома")
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

    pub fn create_report(
        &self,
        info_provider: Box<dyn DeviceInfoProvider>
    ) -> String {
        // todo!("перебор комнат и устройств в них для составления отчёта");
        let mut report = String::from("Report for Smart House:\t")
            .add(self.name.trim())
            .add("\n");
        // for room in self.spaces.iter().clone() {
        //     report = report.add("Room:\t").add(room.1.name.trim()).add("\n");
        //     for device in room.1.get_devices() {
        //         report = report.add(device.1.get_state().trim()).add("\n");
        //     }
        // }
        report = report.add(info_provider.get_name().trim()).add("\n");
        report = report.add(info_provider.get_state().trim()).add("\n");
        report
    }
}
