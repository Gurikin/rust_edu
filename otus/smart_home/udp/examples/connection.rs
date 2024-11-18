use std::{
    error::Error,
    net::UdpSocket,
    sync::{Arc, Mutex},
    thread,
};
use udp::server::UdpConnection;

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
    let client_socket = UdpSocket::bind("127.0.0.1:55332")?;
    let message = "Hello, server";
    let bytes_sent = client_socket.send_to(message.as_bytes(), "127.0.0.1:55331")?;
    assert_eq!(message.len(), bytes_sent);

    let connection_clone = connection.clone();
    thread::spawn(move || {
        let connection_lock = connection_clone.lock().unwrap();
        let response = connection_lock.process_response();
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), message.to_string());
    })
    .join()
    .unwrap();

    Ok("Connection processed successfully.".to_string())
}

// use std::io::{self, BufRead};
// use std::sync::mpsc::{self, TryRecvError};
// use std::thread;
// use std::time::Duration;

// fn main() {
//     println!("Press enter to terminate the child thread");
//     let (tx, rx) = mpsc::channel();

//     let t = thread::spawn(move || loop {
//         println!("Working...");
//         thread::sleep(Duration::from_millis(500));
//         match rx.try_recv() {
//             Ok(msg) => {
//                 println!("Terminating with message: {msg}");
//                 break;
//             },
//             Err(TryRecvError::Disconnected) => {
//                 println!("Disconnected.");
//                 break;
//             },
//             Err(TryRecvError::Empty) => {
//                 // println!("Empty.");
//                 // break;
//             }
//         }
//     });

//     let mut line = String::new();
//     let stdin = io::stdin();
//     let _ = stdin.lock().read_line(&mut line);

//     let _ = tx.send("Hello from main thread");
//     t.join().unwrap();
// }
