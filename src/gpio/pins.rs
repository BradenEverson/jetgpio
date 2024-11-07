//! Methods for getting pins and setting their type from GPIO

use jetgpio_sys::{gpioRead, gpioSetMode, gpioWrite, JET_INPUT, JET_OUTPUT};

use super::{valid_pins::ValidPin, Gpio};

/// An input GPIO pin that can read a HIGH or LOW signal
pub struct InputPin {
    pin: u32,
}

/// An output GPIO pin that can send a HIGH or LOW signal
pub struct OutputPin {
    pin: u32,
    state: bool,
}

impl Gpio {
    pub fn get_input<PIN: ValidPin>(&self, pin: PIN) -> super::Result<InputPin> {
        let pin = pin.pin();
        let res = unsafe { gpioSetMode(pin, JET_INPUT) };
        if res < 0 {
            Err(res)
        } else {
            Ok(InputPin { pin })
        }
    }

    pub fn get_output<PIN: ValidPin>(&self, pin: PIN) -> super::Result<OutputPin> {
        let pin = pin.pin();
        let res = unsafe { gpioSetMode(pin, JET_OUTPUT) };
        if res < 0 {
            Err(res)
        } else {
            Ok(OutputPin { pin, state: false })
        }
    }
}

impl InputPin {
    pub fn read(&self) -> super::Result<bool> {
        let level = unsafe { gpioRead(self.pin) };
        if level < 0 {
            Err(level)
        } else {
            Ok(level == 1)
        }
    }
}

impl OutputPin {
    /// Toggles the GPIO pin's current state between LOW and HIGH
    pub fn toggle(&mut self) -> super::Result<()> {
        let result = unsafe { gpioWrite(self.pin, !self.state as u32) };
        if result < 0 {
            Err(result)
        } else {
            self.state = !self.state;
            Ok(())
        }
    }

    /// Sets the GPIO pin to HIGH
    pub fn set_high(&mut self) -> super::Result<()> {
        let result = unsafe { gpioWrite(self.pin, 1) };
        if result < 0 {
            Err(result)
        } else {
            self.state = true;
            Ok(())
        }
    }

    /// Sets the GPIO pin to LOW
    pub fn set_low(&mut self) -> super::Result<()> {
        let result = unsafe { gpioWrite(self.pin, 0) };
        if result < 0 {
            Err(result)
        } else {
            self.state = false;
            Ok(())
        }
    }
}
