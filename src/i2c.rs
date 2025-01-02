//! Control over I2C protocol

use bus::I2cBus;
use jetgpio_sys::{i2cClose, i2cReadByteData, i2cWriteByteData};
use thiserror::Error;

pub mod bus;

/// A result that JetGpio may return
pub type Result<T> = std::result::Result<T, Error>;

/// Any error that may be caused by GPIO
#[derive(Debug, Error)]
pub enum Error {
    /// Raw JetGPIO Error Code
    #[error("Error Code from JETGPIO: {0}")]
    JetGpioSysError(i32),
    /// Uninitialized slave address,
    #[error("Uninitialized Slave Address in i2c")]
    UninitializedSlaveAddr,
}

/// Peripheral i2c struct
pub struct I2c {
    /// The underlying bus
    bus: u32,
    /// The i2c handle
    handle: u32,
    /// The slave address
    slave_addr: Option<u32>,
}

impl I2c {
    /// Initializes and opens an I2c Bus
    pub fn init(bus: I2cBus, flags: u32) -> Result<Self> {
        let bus = bus.bus_val();
        let handle = unsafe { jetgpio_sys::i2cOpen(bus, flags) };
        if handle < 0 {
            return Err(Error::JetGpioSysError(handle));
        }

        Ok(Self {
            bus,
            handle: handle as u32,
            slave_addr: None,
        })
    }

    /// Sets the slave address for the I2c Channel
    pub fn set_slave_address(&mut self, addr: u32) {
        self.slave_addr = Some(addr)
    }

    /// Writes to the channel
    pub fn write_byte(&mut self, register: u8, data: u8) -> Result<()> {
        let addr = self.slave_addr.ok_or(Error::UninitializedSlaveAddr)?;

        let val = unsafe { i2cWriteByteData(self.handle, addr, register as u32, data as u32) };
        if val < 0 {
            return Err(Error::JetGpioSysError(val));
        }

        Ok(())
    }

    /// Writes a slice to a register
    pub fn write(&mut self, register: u8, data: &[u8]) -> Result<()> {
        for val in data {
            self.write_byte(register, *val)?
        }

        Ok(())
    }

    /// Reads a single byte from a register
    pub fn read_byte(&mut self, register: u8) -> Result<u8> {
        let addr = self.slave_addr.ok_or(Error::UninitializedSlaveAddr)?;
        let read = unsafe { i2cReadByteData(self.handle, addr, register as u32) };
        if read < 0 {
            return Err(Error::JetGpioSysError(read));
        } else {
            Ok(read as u8)
        }
    }

    /// Reads into a mutable slice and returns the number of bytes successfully read
    pub fn read(&mut self, register: u8, into: &mut [u8]) -> Result<i32> {
        self.write_byte(register, 0x0)?;
        let mut read = 0;
        for val in into {
            *val = self.read_byte(register)?;
            read += 1
        }

        Ok(read)
    }
}

impl Drop for I2c {
    fn drop(&mut self) {
        unsafe { i2cClose(self.bus) };
    }
}
