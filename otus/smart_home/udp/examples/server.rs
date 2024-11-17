use std::{
    error::Error, net::UdpSocket, os::windows::io::AsHandle, sync::{Arc, Mutex}, thread
};
use stp::server::{UdpConnection, UdpServer};

fn main() -> Result<(), Box<dyn Error>> {
    let server = Arc::new(Mutex::new(UdpServer::bind("127.0.0.1:55331")));
    let server_ref = server.clone();
    let t = thread::spawn(move || {
        let server_lock = server_ref.lock().unwrap();
        let conn = Arc::new(Mutex::new(server_lock.as_ref().unwrap().accept().unwrap()));
        let conn_ref = conn.clone();
        let result = process_connection(conn_ref)
            .map_err(|_| "Error processing connection");
        println!("{}", result.unwrap());
    });
    println!("Main thread finished");
    t.join().unwrap();
    Ok(())
}

fn process_connection(conn: Arc<Mutex<UdpConnection>>) -> Result<String, Box<dyn Error>> {
    let conn_ref= conn.clone();
    let size = UdpSocket::bind("127.0.0.1:55332").unwrap().send_to("Hello, server".as_bytes(), "127.0.0.1:55331").map_err(|_| 0usize).unwrap();
    assert_eq!(13, size);
    thread::spawn(move || {
        let conn_lock = conn_ref.try_lock().unwrap();
        let response = conn_lock.process_response();
        assert!(response.is_ok());
        assert_eq!(response.unwrap(), String::from("Hello, server"));
    }).join().unwrap();

    Ok(String::from("Connection processed successfully."))
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
