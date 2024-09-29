use crate::devices::{Device, DeviceInfo, DeviceType, SmartSocket, SmartThermometer};
use crate::smart_house_mod::{SmartHouse, Space};

mod devices;
mod smart_house_mod;

// use std::{collections::HashMap, error::Error};

// use crate::devices::{Device, DeviceType};

fn main() {
    let smart_socket1: Box<dyn Device> = Box::new(SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "Smart Socket 1".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket 1 in the bedroom".to_string(),
        },
        is_switch_on: false,
        current_power: 0,
    });
    let smart_socket1_2: Box<dyn Device> = Box::new(SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "Smart Socket 2".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket 2 in the bedroom".to_string(),
        },
        is_switch_on: true,
        current_power: 230,
    });
    let smart_socket2: Box<dyn Device> = Box::new(SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "Smart Socket 2".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    });
    let therm: Box<dyn Device> = Box::new(SmartThermometer {
        info: DeviceInfo {
            id: 0,
            name: "Street Thermometer".to_string(),
            device_type: DeviceType::Thermometer,
            description: "Thermometer in the living room".to_string(),
        },
        temperature: 20,
    });
    let bedroom = Space::from_devices_vec(1, vec![smart_socket1, smart_socket1_2]);
    let living_room = Space::from_devices_vec(2, vec![smart_socket2, therm]);
    let smart_house = SmartHouse::new("Cottage", vec![bedroom, living_room]);
    println!("Rooms List:\t");
    for room in smart_house.get_rooms() {
        print!("{}:\t", room);
        smart_house
            .devices(room)
            .into_iter()
            .for_each(|d| print!("{},\t", d));
        println!();
    }
    let report = smart_house.create_report();
    println!("{}", report);
}

// fn main() {
//     // Инициализация устройств
//     let socket1 = SmartSocket {};
//     let socket2 = SmartSocket {};
//     let thermo = SmartThermometer {};
//
//     // Инициализация дома
//     let house = SmartHouse::new();
//
//
//     // Строим отчёт с использованием `OwningDeviceInfoProvider`.
//     let info_provider_1 = OwningDeviceInfoProvider {
//         socket: socket1,
//     };
//     // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
//     let report1 = house.create_report(/* &info_provider_1 */);
//
//     // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
//     let info_provider_2 = BorrowingDeviceInfoProvider {
//         socket: &socket2,
//         thermo: &thermo,
//     };
//     // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
//     let report2 = house.create_report(/* &info_provider_2 */);
//
//     // Выводим отчёты на экран:
//     println!("Report #1: {report1}");
//     println!("Report #2: {report2}");
// }
