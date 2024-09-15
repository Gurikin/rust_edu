use rppal::gpio::OutputPin;
use std::collections::HashMap;
use std::fmt;

pub struct Device {
    pub id: u8,
    pub device_type: DeviceType,
    pub name: String,
    pub description: String,
    pub data: HashMap<String, String>,
    pub device: OutputPin,
}

#[derive(Debug)]
pub enum DeviceType {
    PowerSocket,
    Thermometer,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl Device {
    pub fn description(&self) {
        println!("Device description: {}", self.description);
    }
    pub fn get_data(&self) {
        println!("Device data:");
        println!("\tId:\t{}", self.id);
        println!("\tName:\t{}", self.name);
        println!("\tType:\t{}", self.device_type);
        println!("\tData:");
        for datum in &self.data {
            println!("\t\t{}:\t{}", datum.0, datum.1);
        }
    }
}
