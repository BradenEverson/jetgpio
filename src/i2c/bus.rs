//! I2c Available Busses

/// A valid I2c Bus
#[cfg(feature = "orin")]
pub enum I2cBus {
    /// I2c Bus 1
    I2c1,
}

#[cfg(feature = "orin")]
impl I2cBus {
    /// Returns the underlying value for a Bus
    pub(crate) fn bus_val(&self) -> u32 {
        match self {
            I2cBus::I2c1 => 1,
        }
    }
}

/// A valid I2c Bus
#[cfg(not(feature = "orin"))]
pub enum I2cBus {
    /// I2c Bus 1
    I2c1,
    /// I2c Bus 0
    I2c0,
}

#[cfg(not(feature = "orin"))]
impl I2cBus {
    /// Returns the underlying value for a Bus
    pub(crate) fn bus_val(&self) -> u32 {
        match self {
            I2cBus::I2c1 => 1,
            I2cBus::I2c0 => 0,
        }
    }
}
