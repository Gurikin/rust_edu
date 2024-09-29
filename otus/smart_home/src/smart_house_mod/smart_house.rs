use crate::smart_house_mod::space::Space;

use std::collections::BTreeMap;
use std::ops::Add;

pub struct SmartHouse {
    name: String,
    spaces: BTreeMap<String, Space>,
}

trait DeviceInfoProvider {
    fn device_info(&self, room: &str, device: &str) -> String;
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    // кмк, здесь закралась ошибка. Если судить по вот этим примерам использования
    //     // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    //     let info_provider_1 = OwningDeviceInfoProvider {
    //         socket: socket1,
    //     };
    //     // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    //     let report1 = house.create_report(/* &info_provider_1 */);
    // тут мы явно не "возвращаем состояние устройства по имени комнаты и имени устройства"
    // мы просто возвращаем состояние известного, полученного нами ранее (любым способом) устройства
}

impl DeviceInfoProvider for SmartHouse {
    fn device_info(&self, room: &str, device: &str) -> String {
        String::from(
            &self
                .spaces
                .get(room)
                .unwrap()
                .get_devices()
                .get(device)
                .unwrap()
                .get_state(),
        )
    }
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
        //infoProvider: Box<dyn DeviceInfoProvider> // Не совсем понял зачем тут обобщенный тип, если нам нужно хранить информацию об устройствах уже в обобщенном типе
        //Если мы используем DeviceInfoProvider, то сюда должны поступать данные о названиях комнаты и устройства (см. строку 13 to_do как должно было реализовать метод)
        //Если мы используем какой-то другой обобщенный тип, то я не совсем понимаю какой именно и как его лучше здесь использовать.
    ) -> String {
        // todo!("перебор комнат и устройств в них для составления отчёта");
        let mut report = String::from("Report for Smart House:\t")
            .add(self.name.trim())
            .add("\n");
        for room in self.spaces.iter().clone() {
            report = report.add("Room:\t").add(room.1.name.trim()).add("\n");
            for device in room.1.get_devices() {
                report = report.add(device.1.get_state().trim()).add("\n");
            }
        }
        report
    }
}
