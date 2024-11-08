//! PWM Access with ability to set Frequency and Duty Cycle

use jetgpio_sys::{gpioInitialise, gpioPWM, gpioSetPWMfrequency};
use valid_pwm::ValidPwmPin;

use crate::gpio::{jetgpio_code_to_result, valid_pins::ValidPin};

pub mod valid_pwm;

/// A PWM Channel with respect to its physical Pin
pub struct Pwm {
    /// The physical pin attached to this PWM channel
    pin: u32,
}

impl Pwm {
    /// Constructs a new PWM instance with respect to a valid pin
    pub fn new<PIN: ValidPwmPin + ValidPin>(pin: PIN) -> super::gpio::Result<Self> {
        let pin = pin.pin();
        let init = unsafe { gpioInitialise() };
        jetgpio_code_to_result(init)?;
        Ok(Self { pin })
    }

    /// Sets the frequency of the PWM channel
    pub fn set_frequency(&self, freq: u32) -> super::gpio::Result<()> {
        let pwm_set_status = unsafe { gpioSetPWMfrequency(self.pin, freq) };
        jetgpio_code_to_result(pwm_set_status)?;
        Ok(())
    }

    /// Sets the duty cycle for the PWM channel
    pub fn set_duty_cycle(&self, duty_cycle: u32) -> super::gpio::Result<()> {
        let res = unsafe { gpioPWM(self.pin, duty_cycle) };
        jetgpio_code_to_result(res)?;
        Ok(())
    }
}
