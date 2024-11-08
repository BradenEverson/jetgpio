//! PWM control over a servo on physical pin 32

use jetgpio::{gpio::valid_pins::Pin32, Pwm};
use std::{io, thread, time::Duration};

fn main() {
    let pwm = Pwm::new(Pin32).expect("Initialize PWM");
    pwm.set_frequency(255).expect("Set PWM Frequency");

    loop {
        println!("Enter a PWM value: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let pwm_value: u32 = match input.trim().parse() {
            Ok(num) => num,
            _ => {
                println!("Please enter a valid u32.");
                continue;
            }
        };

        pwm.set_duty_cycle(pwm_value).expect("Set duty cycle");

        thread::sleep(Duration::from_millis(500));
    }
}
