use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr();

    // Читаем аргументы командной строки.
    let mut cli_args = std::env::args().skip(1);
    let Some(action) = cli_args.next() else {
        return Err(String::from("No action provided, use 'append' or 'fetch'").into());
    };

    println!("Performing action: {action}...");

    // Соединяемся с сервером чата.
    let mut client = smart_house_tcp_client::SmartHouseTcpClient::new(addr)?;

    if action == "switch_state" {
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        // Изменяем состояние устройства.
        client.switch_state(&msg)?;
        return Ok(());
    }

    if action == "get_state" {
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        // Изменяем состояние устройства.
        client.get_state(&msg)?;
        return Ok(());
    }

    if action == "add_room" {
        // Отправляем новое сообщение.
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        client.add_room(&msg)?;
        return Ok(());
    }

    Err(String::from("Unknown action, use 'add_room' or 'switch_state'").into())
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
