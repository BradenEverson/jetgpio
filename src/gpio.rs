//! GPIO Struct Definitions

use jetgpio_sys::{gpioInitialise, gpioTerminate};

pub mod pins;
pub mod valid_pins;

/// The host for all GPIO accessibility
#[derive(Debug)]
pub struct Gpio;

/// A result that JetGpio may return
pub type Result<T> = std::result::Result<T, i32>;

impl Gpio {
    /// Initializes the Jetson's GPIO
    pub fn new() -> Result<Self> {
        let res = unsafe { gpioInitialise() };
        if res < 0 {
            Err(res)
        } else {
            Ok(Self)
        }
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        unsafe { gpioTerminate() }
    }
}
