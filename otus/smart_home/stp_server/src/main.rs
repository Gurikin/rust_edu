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
    info_provider_mod::{DeviceInfoStorageB, OwningDeviceInfoProvider},
    smart_house_mod::{Apartment, SmartHouse},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr).await?;

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
        let Ok(mut connection) = server.accept().await else {
            continue;
        };

        let _addr = match connection.peer_addr() {
            Ok(addr) => addr.to_string(),
            Err(_) => "unknown".into(),
        };

        // Обрабатываем запрос.
        connection.process_request(|req| {
            println!("Handle request {}", &req);

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
                        let response = format!(
                            "[{}] Could not add room {} to the smart house {}. Maybe it's already exists",
                            offset::Local::now(),
                            room_name,
                            smart_house_data.0.get_name().clone()
                        );
                        println!("{}", &response);
                        return response;
                    }
                }
            }

            // Если запрос add_room, то добавляем комнату в умный дом
            if let Some(msg) = req.strip_prefix("add_device:") {
                let mut args = msg.split("|||");
                let apart_name = String::from(args.next().unwrap_or("Unknown_room"));
                let device_name = String::from(args.next().unwrap_or("Unknown_device"));
                let id = smart_house_data.0.get_apartments().keys().count() + 1;
                match smart_house_data.1.add(apart_name.clone(), stub_smart_socket(id as u8, device_name.clone())) {
                    Ok(_) => {
                        if !smart_house_data.0.get_apartments().contains_key(&apart_name) {
                            println!("Smart house DO NOT contains room {}", apart_name);
                            let mut apart = Apartment::new(apart_name.clone());
                            match apart.add(device_name.clone()) {
                                Ok(_) => println!("OK 89"),
                                Err(_) => eprintln!("BAD 90"),
                            }
                            match smart_house_data.0.add(apart) {
                                Ok(_) => println!("OK 93"),
                                Err(_) => eprintln!("BAD 94"),
                            }
                        } else {
                            println!("Smart house contains room {}", apart_name);
                            match smart_house_data.0.add_device(apart_name.clone(), device_name.clone()) {
                                Ok(_) => {
                                    println!("Inserted");
                                    assert!(smart_house_data.1.devices.contains_key(&apart_name));
                                    println!("{}", if smart_house_data.1.devices.get(&apart_name).unwrap().contains_key(&device_name) {"InfProv OK"} else {"InfProv Empty"});
                                },
                                Err(_) => println!("Failed insert"),
                            };
                        }
                        let response = format!(
                            "[{}] Device {} added to apart {} successfully",
                            offset::Local::now(),
                            device_name,
                            apart_name
                        );
                        println!("{}", response);
                        return response;
                    },
                    Err(e) => {
                        let response = format!(
                            "[{}] Could not add device {} to the room {}. Try to use add_room command. Cause: {}",
                            offset::Local::now(),
                            device_name,
                            apart_name,
                            e
                        );
                        println!("{}", &response);
                        return response;
                    },
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
                        let response = format!(
                            "[{}] Could not switch state for device {}. Cause: {}",
                            offset::Local::now(),
                            device_name,
                            e
                        );
                        println!("{}", &response);
                        return response;
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
                        let response = format!(
                            "[{}] Could not get state for device {}. Cause: {}",
                            offset::Local::now(),
                            device_name,
                            e
                        );
                        println!("{}", &response);
                        return response;
                    }
                }
            }

            // Если запрос get_report, то возвращаем полное состояние всех устройств и комнат умного дома
            if req == *"create_report" {
                match smart_house_data.0.create_report(&smart_house_data.1) {
                    Ok(report) => {
                        let response = format!(
                            "[{}] {}",
                            offset::Local::now(),
                            report
                        );
                        println!("{}", response);
                        return response;
                    }
                    Err(e) => {
                        let response = format!(
                            "[{}] Could not create report for smart_house {}. Cause: {}",
                            offset::Local::now(),
                            smart_house_data.0.get_name(),
                            e
                        );
                        println!("{}", &response);
                        return response;
                    }
                }
            }
            // Если запрос неизвестен, возвращаем сообщение об ошибке.
            format!("Unknown request: {}", req)
        }).await?;
    }
}

fn stub_smart_house(
    smart_house_name: String,
) -> (SmartHouse, OwningDeviceInfoProvider<SmartSocket>) {
    let socket_name = "Smart Socket In Living Room".to_string();
    let smart_socket = stub_smart_socket(1, socket_name.clone());
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

fn stub_smart_socket(id: u8, name: String) -> SmartSocket {
    SmartSocket {
        info: DeviceInfo {
            id,
            name,
            device_type: DeviceType::PowerSocket,
            description: "Smart Socket in the living room".to_string(),
        },
        is_switch_on: true,
        current_power: 220,
    }
}
