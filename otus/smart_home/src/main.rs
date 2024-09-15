use std::error::Error;

use rppal::gpio::{Gpio, OutputPin};
use rppal::system::DeviceInfo;
use std::io;
use std::num::ParseIntError;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 17;

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

fn switch_on(pin: &mut OutputPin) {
    pin.set_high();
}

fn switch_off(pin: &mut OutputPin) {
    pin.set_low();
}

fn main() -> Result<(), Box<dyn Error>> {
    //Для повышения интереса эмулировал работу с устройствами через GPIO на Малинке
    println!("Smart home devices emulation on a {}.", DeviceInfo::new()?.model());

    let mut pin: OutputPin = Gpio::new()?.get(GPIO_LED)?.into_output();
    println!("TypeId: {}", pin.pin());

    // Blink the LED by setting the pin's logic level high for 500 ms.
    loop {
        let input = read_user_input().unwrap();
        match input {
            0 => switch_off(&mut pin),
            1 => switch_on(&mut pin),
            _ => {
                switch_off(&mut pin);
                break;
            }
        }
    }
    switch_off(&mut pin);
    Ok(())
}
