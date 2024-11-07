//! Example driver program that turns an LED on when a button is held
//! Led - Physical Pin 7
//! Button - Physical Pin 5

use jetson::{
    gpio::valid_pins::{Pin5, Pin7},
    Gpio,
};
use std::{thread, time::Duration};

fn main() {
    let gpio = Gpio::new().expect("Initialize GPIO");
    let mut led = gpio.get_output(Pin7).expect("Set GPIO pin 7 to output");
    let button = gpio.get_input(Pin5).expect("Get input button");

    loop {
        if button.read().expect("Set High") {
            led.set_high().unwrap();
        } else {
            led.set_low().expect("Set Low");
        }
        thread::sleep(Duration::from_millis(50));
    }
}
