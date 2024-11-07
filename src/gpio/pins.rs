//! Methods for getting pins and setting their type from GPIO

use jetgpio_sys::{gpioSetMode, JET_INPUT};

use super::Gpio;

pub struct InputPin(u8);
pub struct OutputPin(u8);

impl Gpio {
    pub fn get_input(&self, pin: u8) -> super::Result<InputPin> {
        let res = unsafe { gpioSetMode(pin as u32, JET_INPUT) };
        if res < 0 {
            Err(res)
        } else {
            Ok(InputPin(pin))
        }
    }
}
