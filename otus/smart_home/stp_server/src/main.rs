use chrono::offset;
use rand::Rng;
use smart_home::smart_house_mod::{Apartment, SmartHouse};
use std::error::Error;
use std::fs;
use stp::server::StpServer;

fn main() -> Result<(), Box<dyn Error>> {
    // Читаем IP-адрес сервера из файла или используем значение по умолчанию.
    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"));
    let server = StpServer::bind(addr)?;

    // Создаём новый чат.
    let apartments = vec![];
    let mut smart_house = SmartHouse::new(
        format!("Smart House #{}", rand::thread_rng().gen_range(1..=100)).trim(),
        apartments,
    );
    println!(
        "[{}]\tCreated {}",
        offset::Local::now(),
        smart_house.get_name()
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
                match smart_house.add(room) {
                    Ok(_) => {
                        let response = format!(
                            "[{}] Apartment {} added to {} successfully",
                            offset::Local::now(),
                            room_name,
                            smart_house.get_name().clone()
                        );
                        println!("{}", response);
                        return response;
                    }
                    Err(_) => {
                        println!(
                            "[{}] Could not add room {} to the smart house {}",
                            offset::Local::now(),
                            room_name,
                            smart_house.get_name().clone()
                        )
                    }
                }
            }

            // // Если запрос append, добавляем новое сообщение.
            // if let Some(msg) = req.strip_prefix("append:") {
            //     return chat.append(addr, msg.into());
            // }

            // Если запрос неизвестен, возвращаем сообщение об ошибке.
            format!("Unknown request: {}", req)
        })?;
    }
}
