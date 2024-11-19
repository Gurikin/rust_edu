use rand::Error;
use rand::RngCore;
use smart_home::devices::*;
use smart_home::info_provider_mod::*;
use smart_home::smart_house_mod::apartment::*;
use smart_home::smart_house_mod::smart_house::*;
use std::collections::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use udp::connection::*;
use udp::error::ConnectError;

fn main() {
    let (temp_sender, term_receiver) = mpsc::channel::<u32>();
    run_therm_thread(temp_sender, term_receiver);
}

/// Create a new smart thermometer
fn create_thermometer() -> SmartThermometer {
    SmartThermometer {
        info: DeviceInfo {
            id: 0,
            name: "Bedroom's therm".to_string(),
            device_type: DeviceType::Thermometer,
            description: "Outside thermometer in the bedroom".to_string(),
        },
        temperature: 0,
    }
}

/// Create new UdpConnection for send temperature to the server
fn create_connection() -> UdpConnection {
    UdpConnection::bind_port("127.0.0.1:808080")
        .map_err(|e| ConnectError::from(e))
        .unwrap()
}

fn run_therm_thread(tx: Sender<u32>, rx: Receiver<u32>) -> i32 {
    let connection = Arc::new(Mutex::new(create_connection()));
    let therm = Arc::new(Mutex::new(create_thermometer()));
    thread::spawn(move || loop {
        let received = match rx.recv() {
            Ok(r) => Some(r),
            Err(_) => None,
        };
        if received.is_none() {
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        let conn_lock = connection.lock().unwrap();
        let _ = conn_lock.process_request(
            &therm.lock().unwrap().temperature.to_string(),
            conn_lock.peer_addr(),
        );
    });
    for _ in 0..100 {
        tx.send(rand::thread_rng().next_u32()).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
    0
}
