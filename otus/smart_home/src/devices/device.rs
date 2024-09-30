use std::fmt;
use std::ops::Add;

pub struct DeviceInfo {
    pub id: u8,
    pub name: String,
    pub device_type: DeviceType,
    pub description: String,
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

impl DeviceInfo {
    pub fn get_data(&self) -> String {
        String::from("Device data:\n")
            .add("\tId:\t")
            .add(self.id.to_string().trim())
            .add("\tName:\t")
            .add(self.name.trim())
            .add("\tType:\t")
            .add(self.device_type.to_string().trim())
            .add("\tDescription:\t")
            .add(self.description.to_string().trim())
    }
}

// Пользовательские устройства:
pub struct SmartSocket {
    pub info: DeviceInfo,
    pub is_switch_on: bool,
    pub current_power: u32,
}
pub struct SmartThermometer {
    pub info: DeviceInfo,
    pub temperature: u32,
}

pub trait DeviceInfoProvider {
    fn get_name(&self) -> String;
    fn get_state(&self) -> String;
}

impl DeviceInfoProvider for SmartSocket {
    fn get_name(&self) -> String {
        String::from(&self.info.name)
    }

    fn get_state(&self) -> String {
        self.info
            .get_data()
            .add("\tIs on:\t")
            .add(self.is_switch_on.to_string().trim())
            .add("\tCurrent power:\t")
            .add(self.current_power.to_string().trim())
            .add("\n")
    }
}

impl DeviceInfoProvider for SmartThermometer {
    fn get_name(&self) -> String {
        String::from(&self.info.name)
    }
    fn get_state(&self) -> String {
        self.info
            .get_data()
            .add("\tTemperature:\t")
            .add(self.temperature.to_string().trim())
            .add("\n")
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider<T: DeviceInfoProvider> {
    pub device: Box<T>,
}
pub struct BorrowingDeviceInfoProvider<'a> {
    pub device: &'a Box<dyn DeviceInfoProvider>,
}
