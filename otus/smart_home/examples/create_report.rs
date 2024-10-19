use smart_home::devices::*;
use smart_home::info_provider_mod::*;
use smart_home::smart_house_mod::apartment::*;
use smart_home::smart_house_mod::smart_house::*;
use std::collections::*;

fn main() {
    let socket_name = "Smart Socket In Living Room".to_string();
    let smart_socket = SmartSocket {
        info: DeviceInfo {
            id: 0,
            name: socket_name.clone(),
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    };
    let devices_in_living_room = BTreeSet::from([socket_name.clone()]);
    let living_room = Apartment::from_set(1, &devices_in_living_room);
    let mut sockets_map = BTreeMap::new();
    let mut room_devices = BTreeMap::new();
    room_devices.insert(socket_name.clone(), smart_socket);
    sockets_map.insert(living_room.get_name(), room_devices);
    let info_provider = OwningDeviceInfoProvider {
        sockets: sockets_map,
    };

    let apartments = vec![living_room];
    let smart_house = SmartHouse::new("Cottage", &apartments);
    match smart_house.create_report(info_provider) {
        Ok(r) => println!("{}", r),
        Err(e) => eprint!("{}", e),
    }
}
