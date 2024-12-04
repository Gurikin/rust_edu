use std::error::Error;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server_addr();

    // Читаем аргументы командной строки.
    let mut cli_args = std::env::args().skip(1);
    let Some(action) = cli_args.next() else {
        return Err(String::from("No action provided, use 'append' or 'fetch'").into());
    };

    println!("Performing action: {action}...");

    // Соединяемся с сервером чата.
    let mut client = smart_house_tcp_client::SmartHouseTcpClient::new(addr).await?;

    if action == "switch_state" {
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        // Изменяем состояние устройства.
        match client.switch_state(&msg).await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("{}", e),
        };
        return Ok(());
    }

    if action == "get_state" {
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        // Получаем состояние устройства.
        match client.get_state(&msg).await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("{}", e),
        };

        return Ok(());
    }

    if action == "add_room" {
        // Отправляем новое сообщение.
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        //Добавляем комнату
        match client.add_room(&msg).await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("{}", e),
        };
        return Ok(());
    }

    if action == "add_device" {
        // Отправляем новое сообщение.
        let Some(msg) = cli_args.next() else {
            return Err(String::from("No message provided").into());
        };
        //Добавляем комнату
        match client.add_device(&msg).await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("{}", e),
        };
        return Ok(());
    }

    if action == "create_report" {
        //Получаем отчет о состоянии дома
        match client.create_report().await {
            Ok(response) => println!("{}", response),
            Err(e) => println!("{}", e),
        };
        return Ok(());
    }

    Err(String::from("Unknown action, use 'add_room' or 'switch_state'").into())
}

fn get_server_addr() -> String {
    fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55331"))
}
