//! GPIO Struct Definitions

use jetgpio_sys::{gpioInitialise, gpioTerminate};
use thiserror::Error;

pub mod pins;
pub mod valid_pins;

/// The host for all GPIO accessibility
#[derive(Debug)]
pub struct Gpio;

/// A result that JetGpio may return
pub type Result<T> = std::result::Result<T, Error>;

/// Any error that may be caused by GPIO
#[derive(Debug, Error)]
pub enum Error {
    /// Raw JetGPIO Error Code
    #[error("Error Code from JETGPIO: {0}")]
    JetGpioSysError(i32),
}

/// Converts a JETGPIO returned code to a corresponding result
pub fn jetgpio_code_to_result(code: i32) -> Result<()> {
    if code < 0 {
        Err(Error::JetGpioSysError(code))
    } else {
        Ok(())
    }
}

impl Gpio {
    /// Initializes the Jetson's GPIO
    pub fn new() -> Result<Self> {
        let res = unsafe { gpioInitialise() };
        jetgpio_code_to_result(res)?;
        Ok(Self)
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        unsafe { gpioTerminate() }
    }
}
