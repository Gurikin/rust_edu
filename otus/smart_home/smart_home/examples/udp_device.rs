use crossbeam_channel::unbounded;
use cursive::align::Align;
use cursive::view::Resizable;
use cursive::views::ResizedView;
use cursive::Cursive;
use cursive::Printer;
use cursive::Vec2;
use cursive::View;
use cursive::{
    views::{CircularFocus, Dialog, TextView},
    With as _,
};
use rand::Rng;
use smart_home::devices::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use udp::connection::*;
use udp::error::ConnectError;

fn main() {
    let (ui_sender, ui_receiver) = unbounded::<String>(); //mpsc::channel::<String>();
    let mut ui = cursive::default();
    let cb_sink = ui.cb_sink().clone();
    ui.add_global_callback('q', |s| s.quit());
    ui.add_layer(create_ui(ui_receiver, ui_sender, cb_sink));
    ui.run();
}

fn create_threads(ui_sender: crossbeam_channel::Sender<String>, cb_sink: cursive::CbSink) {
    run_therm_thread();
    run_smart_house_server_thread(ui_sender, cb_sink);
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

fn run_therm_thread() -> JoinHandle<()> {
    let connection = Arc::new(Mutex::new(create_connection("127.0.0.1:55331")));
    let therm = Arc::new(Mutex::new(create_thermometer()));
    let therm_ref = therm.clone();
    thread::spawn(move || loop {
        update_themrmometer(
            &mut therm_ref.lock().unwrap(),
            rand::thread_rng().gen_range(0u32..50u32),
        );
        let conn_lock = connection.lock().unwrap();
        let _ = conn_lock.process_request(
            &therm.lock().unwrap().temperature.to_string(),
            "127.0.0.1:55330",
        );
    })
}

fn run_smart_house_server_thread(
    ui_sender: crossbeam_channel::Sender<String>,
    cb_sink: cursive::CbSink,
) -> JoinHandle<()> {
    let connection = Arc::new(Mutex::new(create_connection("127.0.0.1:55330")));
    thread::spawn(move || {
        let conn_lock = connection.lock().unwrap();
        loop {
            match conn_lock.process_response() {
                Ok(r) => {
                    // println!("Sent temp = {r}");
                    if ui_sender
                        .send(format!("Temperature in the kitchen: {}", r))
                        .is_err()
                    {
                        return;
                    }
                    cb_sink.send(Box::new(Cursive::noop)).unwrap();
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

//ui_sender: Sender<String>, ui_receiver: Arc<Mutex<Receiver<String>>>
fn create_ui(
    rx: crossbeam_channel::Receiver<String>,
    ui_sender: crossbeam_channel::Sender<String>,
    cb_sink: cursive::CbSink,
) -> ResizedView<CircularFocus<Dialog>> {
    // Creates a dialog with a single "Quit" button
    Dialog::around(TextView::new("Thermometer data!").align(Align::center()))
        .title("Cursive")
        .content(ThermView::new(20, rx).max_size(Vec2::new(20, 10)))
        .button("Start measure", move |_| {
            create_threads(ui_sender.clone(), cb_sink.clone())
        })
        .button("Quit", |s| s.quit())
        .wrap_with(CircularFocus::new)
        .wrap_tab()
        .min_size(Vec2::new(50, 15))
}

// Let's define a buffer view, that shows the last lines from a stream.
struct ThermView {
    // We'll use a ring buffer
    content: String,
    // Receiving end of the stream
    rx: crossbeam_channel::Receiver<String>,
}

impl ThermView {
    // Creates a new view with the given buffer size
    fn new(start_temp: u32, rx: crossbeam_channel::Receiver<String>) -> Self {
        let content = format!("Temperature in the kitchen: {}", start_temp);
        ThermView { content, rx }
    }

    // Reads available data from the stream into the buffer
    fn update(&mut self) {
        // Add each available line to the end of the buffer.
        while let Ok(line) = self.rx.try_recv() {
            self.content = line;
        }
    }
}

impl View for ThermView {
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the end of the buffer
        printer.print((0, printer.size.y - 1), &self.content);
    }
}
