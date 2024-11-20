use rand::Rng;
use smart_home::devices::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use udp::connection::*;
use udp::error::ConnectError;

fn main() {
    let (tx, rx) = mpsc::channel::<u32>();
    let therm_thread = run_therm_thread(rx);
    let smart_house_thread = run_smart_house_server_thread();
    for _ in 0..5 {
        let temperature = rand::thread_rng().gen_range(0u32..50u32);
        match tx.send(temperature.clone()) {
            Ok(_) => (),
            Err(e) => eprintln!("{e}"),
        };
        thread::sleep(Duration::from_millis(1000));
    }
    smart_house_thread.join().unwrap();
    therm_thread.join().unwrap();
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

fn update_themrmometer(therm: &mut SmartThermometer, temp: u32) {
    therm.temperature = temp
}

/// Create new UdpConnection for send temperature to the server
fn create_connection(local_address: &str) -> UdpConnection {
    UdpConnection::bind_port(local_address)
        .map_err(|e| ConnectError::from(e))
        .unwrap()
}

fn run_therm_thread(rx: Receiver<u32>) -> JoinHandle<()> {
    let connection = Arc::new(Mutex::new(create_connection("127.0.0.1:55331")));
    let therm = Arc::new(Mutex::new(create_thermometer()));
    let therm_ref = therm.clone();
    thread::spawn(move || {
        for _ in 0..5 {
            match rx.recv() {
                Ok(r) => {update_themrmometer(&mut therm_ref.lock().unwrap(), r)},
                Err(_) => {
                    println!("err update temperature on therm");
                    thread::sleep(Duration::from_millis(500));
                }
            };
            let conn_lock = connection.lock().unwrap();
            let _ = conn_lock.process_request(
                &therm.lock().unwrap().temperature.to_string(),
                "127.0.0.1:55330",
            );
        }
    })
}

fn run_smart_house_server_thread() -> JoinHandle<()> {
    let connection = Arc::new(Mutex::new(create_connection("127.0.0.1:55330")));
    thread::spawn(move || {
        let conn_lock = connection.lock().unwrap();
        for _ in 0..5 {
            match conn_lock.process_response() {
                Ok(r) => {
                    println!("Temperature in the kitchen: {r}");
                    thread::sleep(Duration::from_millis(500));
                }
                Err(_) => {
                    println!("Sleep in server thread");
                    thread::sleep(Duration::from_millis(500));
                    continue;
                }
            };
        }
    })
}
