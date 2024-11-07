//! PWM control over a servo (completely jetgpio_sys for now)

use jetgpio_sys::{gpioInitialise, gpioPWM, gpioSetPWMfrequency};
use std::{io, thread, time::Duration};

fn main() {
    unsafe {
        let init = gpioInitialise();
        if init < 0 {
            panic!("INIT FAIL VERY BAD {init}");
        }

        let pwm_stat = gpioSetPWMfrequency(32, 255);
        if pwm_stat < 0 {
            panic!("Setting PWM failed");
        }

        println!("PWM frequency set properly");

        loop {
            println!("Enter a PWM value (0 to 255): ");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let pwm_value: u32 = match input.trim().parse() {
                Ok(num) if num <= 255 => num,
                _ => {
                    println!("Please enter a valid u32 value between 0 and 255.");
                    continue;
                }
            };

            let result = gpioPWM(32, pwm_value as u32);
            if result < 0 {
                println!("Setting duty cycle failed");
            } else {
                println!("PWM duty cycle set to {}", pwm_value);
            }

            thread::sleep(Duration::from_millis(500)); // Optional delay between reads
        }
    }
}
