//! Example driver program that blinks an LED

use jetson::{gpio::valid_pins::Pin7, Gpio};
use std::{thread, time::Duration};

fn main() {
    let gpio = Gpio::new().expect("Initialize GPIO");
    let mut led = gpio.get_output(Pin7).expect("Set GPIO pin 7 to output");

    for _ in 0..500 {
        led.toggle().expect("Toggle");
        thread::sleep(Duration::from_millis(500));
    }
}
