mod devices;

use std::error::Error;

use crate::devices::{Device, DeviceType};
use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;
use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
pub const GPIO_POWER_SOCKET: u8 = 17;
pub const GPIO_THERMOMETER: u8 = 22;

fn read_user_input() -> Result<i32, ParseIntError> {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let number = match input_line.trim().parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    Ok(number)
}

fn switch_on(device: &mut Device) {
    device.device.set_high();
}

fn switch_off(device: &mut Device) {
    device.device.set_low();
}

fn main() -> Result<(), Box<dyn Error>> {
    //Для повышения интереса эмулировал работу с устройствами через GPIO на Малинке
    println!(
        "Smart home devices emulation on a {}.",
        DeviceInfo::new()?.model()
    );

    let pin17: OutputPin = Gpio::new()?.get(GPIO_POWER_SOCKET)?.into_output();
    let pin22: OutputPin = Gpio::new()?.get(GPIO_THERMOMETER)?.into_output();
    let power_socket_data = HashMap::from([
        (
            String::from("Устройво включено"),
            pin17.is_set_high().to_string(),
        ),
        (
            String::from("Потребляемая мощность"),
            String::from(if pin17.is_set_high() {
                "220 ватт"
            } else {
                "0 ватт"
            }),
        ),
    ]);
    let thermometer_data = HashMap::from([(
        String::from("Температура"),
        String::from(if pin22.is_set_high() { "+18" } else { "-20" }),
    )]);
    let mut power_socket = Device {
        id: pin17.pin(),
        device_type: DeviceType::PowerSocket,
        name: String::from("Розетка"),
        description: String::from("Розетка для компьютера"),
        data: power_socket_data,
        device: pin17,
    };
    power_socket.description();

    let mut thermometer = Device {
        id: pin22.pin(),
        device_type: DeviceType::Thermometer,
        name: String::from("Термометр"),
        description: String::from("Уличный термометр на кухне"),
        data: thermometer_data,
        device: pin22,
    };
    thermometer.description();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    loop {
        let input = read_user_input().unwrap();
        match input {
            1 => {
                switch_on(&mut power_socket);
                update_power_socket_data(&mut power_socket);
                power_socket.get_data()
            }
            2 => {
                switch_off(&mut power_socket);
                update_power_socket_data(&mut power_socket);
                power_socket.get_data();
            }
            3 => {
                switch_on(&mut thermometer);
                update_thermometer_data(&mut thermometer);
                thermometer.get_data();
            }
            4 => {
                switch_off(&mut thermometer);
                update_thermometer_data(&mut thermometer);
                thermometer.get_data()
            }
            _ => {
                switch_off(&mut power_socket);
                switch_off(&mut thermometer);
                power_socket.get_data();
                thermometer.get_data();
                break;
            }
        }
    }
    Ok(())
}

fn update_power_socket_data(device: &mut Device) {
    let data = HashMap::from([
        (
            String::from("Устройво включено"),
            device.device.is_set_high().to_string(),
        ),
        (
            String::from("Потребляемая мощность"),
            String::from(if device.device.is_set_high() {
                "220 ватт"
            } else {
                "0 ватт"
            }),
        ),
    ]);
    device.data = data;
}

fn update_thermometer_data(device: &mut Device) {
    let data = HashMap::from([(
        String::from("Температура"),
        String::from(if device.device.is_set_high() {
            "+18"
        } else {
            "-20"
        }),
    )]);
    device.data = data;
}
