use smart_home::devices::*;
use smart_home::smart_house_mod::*;

fn main() {
    let smart_socket1: Box<dyn DeviceInfoProvider> = Box::new(SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Smart Socket 1".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "EXAMPLE Smart Socket 1 in the bedroom".to_string(),
        },
        is_switch_on: false,
        current_power: 0,
    });
    let smart_socket1_2: Box<dyn DeviceInfoProvider> = Box::new(SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Smart Socket 2".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "EXAMPLE Smart Socket 2 in the bedroom".to_string(),
        },
        is_switch_on: true,
        current_power: 230,
    });
    let smart_socket2: Box<dyn DeviceInfoProvider> = Box::new(SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Smart Socket 2".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "EXAMPLE Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    });
    let therm: Box<dyn DeviceInfoProvider> = Box::new(SmartThermometer {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Street Thermometer".to_string(),
            device_type: DeviceType::Thermometer,
            description: "EXAMPLE Thermometer in the living room".to_string(),
        },
        temperature: 20,
    });
    let bedroom = Space::from_devices_vec(1, vec![smart_socket1, smart_socket1_2]);
    let living_room = Space::from_devices_vec(2, vec![smart_socket2, therm]);
    let smart_house = SmartHouse::new("EXAMPLE Cottage", vec![bedroom, living_room]);
    println!("EXAMPLE Rooms List:\t");
    for room in smart_house.get_rooms() {
        print!("{}:\t", room);
        smart_house
            .devices(room)
            .into_iter()
            .for_each(|d| print!("{},\t", d));
        println!();
    }

    let smart_socket3 = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Smart Socket 3".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "EXAMPLE Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    };
    let info_provider_1 = OwningDeviceInfoProvider {
        device: smart_socket3,
    };
    let report = smart_house.create_report(info_provider_1);
    println!("{}", report);

    let smart_socket4 = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Smart Socket 4".to_string(),
            device_type: DeviceType::PowerSocket,
            description: "EXAMPLE Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 380,
    };
    let therm2 = SmartThermometer {
        info: DeviceInfo {
            id: 0,
            name: "EXAMPLE Street Thermometer 2".to_string(),
            device_type: DeviceType::Thermometer,
            description: "EXAMPLE Thermometer in the living room".to_string(),
        },
        temperature: 40,
    };

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &smart_socket4,
        therm: &therm2,
    };
    let report2 = smart_house.create_report(info_provider_2);
    println!("{}", report2);
}