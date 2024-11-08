//! Methods for getting pins and setting their type from GPIO

use jetgpio_sys::{gpioRead, gpioSetMode, gpioWrite, JET_INPUT, JET_OUTPUT};

use super::{jetgpio_code_to_result, valid_pins::ValidPin, Gpio};

/// An input GPIO pin that can read a HIGH or LOW signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputPin {
    /// The physical pin number
    pin: u32,
}

/// An output GPIO pin that can send a HIGH or LOW signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputPin {
    /// The physical pin number
    pin: u32,
    /// Current pin state (true for high, false for low)
    state: bool,
}

impl Gpio {
    /// Returns a GPIO InputPin on the given valid pin
    pub fn get_input<PIN: ValidPin>(&self, pin: PIN) -> super::Result<InputPin> {
        let pin = pin.pin();
        let res = unsafe { gpioSetMode(pin, JET_INPUT) };
        jetgpio_code_to_result(res)?;
        Ok(InputPin { pin })
    }

    /// Returns a GPIO OutputPin on the given valid pin
    pub fn get_output<PIN: ValidPin>(&self, pin: PIN) -> super::Result<OutputPin> {
        let pin = pin.pin();
        let res = unsafe { gpioSetMode(pin, JET_OUTPUT) };
        jetgpio_code_to_result(res)?;
        Ok(OutputPin { pin, state: false })
    }
}

impl InputPin {
    /// Read the current value at this pin
    pub fn read(&self) -> super::Result<bool> {
        let level = unsafe { gpioRead(self.pin) };
        jetgpio_code_to_result(level)?;
        Ok(level == 1)
    }
}

impl OutputPin {
    /// Toggles the GPIO pin's current state between LOW and HIGH
    pub fn toggle(&mut self) -> super::Result<()> {
        let result = unsafe { gpioWrite(self.pin, !self.state as u32) };
        jetgpio_code_to_result(result)?;
        self.state = !self.state;
        Ok(())
    }

    /// Sets the GPIO pin to HIGH
    pub fn set_high(&mut self) -> super::Result<()> {
        let result = unsafe { gpioWrite(self.pin, 1) };
        jetgpio_code_to_result(result)?;
        self.state = true;
        Ok(())
    }

    /// Sets the GPIO pin to LOW
    pub fn set_low(&mut self) -> super::Result<()> {
        let result = unsafe { gpioWrite(self.pin, 0) };
        jetgpio_code_to_result(result)?;
        self.state = false;
        Ok(())
    }
}
