use std::{thread, time::Duration};

use jetgpio_sys::{gpioInitialise, gpioSetMode, gpioTerminate, gpioWrite, JET_OUTPUT};

fn main() {
    unsafe {
        let result = gpioInitialise();
        if result < 0 {
            println!("Init Failed :( Error code {result}");
            return;
        }

        println!("Jetson initialized OK. Return code {result}");

        let gpio_mode = gpioSetMode(7, JET_OUTPUT);
        if gpio_mode < 0 {
            println!("Init Failed :( Error code {gpio_mode}");
            return;
        }

        println!("GPIO Pin 7 Set as Output. Return code {gpio_mode}");

        for _ in 0..500 {
            gpioWrite(7, 1);
            thread::sleep(Duration::from_millis(500));
            gpioWrite(7, 0);
            thread::sleep(Duration::from_millis(500));
        }

        gpioTerminate();
    }
}
