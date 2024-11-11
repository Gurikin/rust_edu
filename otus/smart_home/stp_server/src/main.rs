use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
};

use chrono::offset;
use rand::Rng;
use stp::server::StpServer;

use smart_home::{
    devices::{DeviceInfo, DeviceType, SmartSocket},
    info_provider_mod::OwningDeviceInfoProvider,
    smart_house_mod::{Apartment, SmartHouse},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr)?;

    // Создаём новый чат.
    let mut smart_house_data = stub_smart_house(format!(
        "Smart House #{}",
        rand::thread_rng().gen_range(1..=100)
    ));
    println!(
        "[{}]\tCreated {}",
        offset::Local::now(),
        smart_house_data.0.get_name()
    );

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(mut connection) = server.accept() else {
            continue;
        };

        let _addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        // Обрабатываем запрос.
        connection.process_request(|req| {
            // Если запрос add_room, то добавляем комнату в умный дом
            if let Some(room_name) = req.strip_prefix("add_room:") {
                let room = Apartment::new(room_name.to_string());
                match smart_house_data.0.add(room) {
                    Ok(_) => {
                        let response = format!(
                            "[{}] Apartment {} added to {} successfully",
                            offset::Local::now(),
                            room_name,
                            smart_house_data.0.get_name().clone()
                        );
                        println!("{}", response);
                        return response;
                    }
                    Err(_) => {
                        println!(
                            "[{}] Could not add room {} to the smart house {}",
                            offset::Local::now(),
                            room_name,
                            smart_house_data.0.get_name().clone()
                        )
                    }
                }
            }

            // Если запрос switch_state, то меняем состояние устройства в умном доме.
            if let Some(msg) = req.strip_prefix("switch_state:") {
                let mut args = msg.split("|||");
                let apart_name = String::from(args.next().unwrap_or("Unknown_room"));
                let device_name = String::from(args.next().unwrap_or("Unknown_device"));
                match smart_house_data.0.switch_state(
                    apart_name,
                    device_name.clone(),
                    &mut smart_house_data.1,
                ) {
                    Ok(state) => {
                        let response = format!(
                            "[{}] State for device \"{}\" is switched.\nNew State: {}",
                            offset::Local::now(),
                            device_name,
                            state
                        );
                        println!("{}", response);
                        return response;
                    }
                    Err(e) => {
                        println!(
                            "[{}] Could not switch state for device {}. Cause: {}",
                            offset::Local::now(),
                            device_name,
                            e
                        )
                    }
                }
            }
            
            // Если запрос get_state, то возвращаем состояние устройства
            if let Some(msg) = req.strip_prefix("get_state:") {
                let mut args = msg.split("|||");
                let apart_name = String::from(args.next().unwrap_or("Unknown_room"));
                let device_name = String::from(args.next().unwrap_or("Unknown_device"));
                match smart_house_data.0.get_state(
                    apart_name,
                    device_name.clone(),
                    &mut smart_house_data.1,
                ) {
                    Ok(state) => {
                        let response = format!(
                            "[{}] State for device \"{}\" is switched.\nNew State: {}",
                            offset::Local::now(),
                            device_name,
                            state
                        );
                        println!("{}", response);
                        return response;
                    }
                    Err(e) => {
                        println!(
                            "[{}] Could not get state for device {}. Cause: {}",
                            offset::Local::now(),
                            device_name,
                            e
                        )
                    }
                }
            }

            // Если запрос неизвестен, возвращаем сообщение об ошибке.
            format!("Unknown request: {}", req)
        })?;
    }
}

fn stub_smart_house(
    smart_house_name: String,
) -> (SmartHouse, OwningDeviceInfoProvider<SmartSocket>) {
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
        devices: sockets_map,
    };

    let apartments = vec![living_room];
    (
        SmartHouse::new(smart_house_name.trim(), apartments),
        info_provider,
    )
}
