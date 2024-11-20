use std::{
    error::Error,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
};
use udp::connection::UdpConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let connection = Arc::new(Mutex::new(UdpConnection::bind_port("127.0.0.1:55331")?));
    let t = thread::spawn(move || {
        let conn_ref = connection.clone();
        let result = process_connection(conn_ref).map_err(|_| "Error processing connection");
        println!("{}", result.unwrap());
    });
    println!("Main thread finished");
    t.join().unwrap();
    Ok(())
}

fn process_connection(connection: Arc<Mutex<UdpConnection>>) -> Result<String, Box<dyn Error>> {
    let request_conn = UdpConnection::bind_port("127.0.0.1:55332").unwrap();
    let message = "Hello, server";
    let bytes_sent = request_conn.process_request(&message, "127.0.0.1:55331");
    assert_eq!(message.len(), bytes_sent.unwrap());

    let connection_clone = connection.clone();
    thread::spawn(move || {
        let connection_lock = connection_clone.lock().unwrap();
        let response = connection_lock.process_response();
        assert!(response.is_ok());
        let response = response.unwrap();
        println!("Response: {}", &response);
        assert_eq!(response, message.to_string());
    })
    .join()
    .unwrap();

    Ok("Connection processed successfully.".to_string())
}
