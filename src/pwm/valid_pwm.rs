//! Trait implementations for the valid Pins that can carry PWM signals

use crate::gpio::valid_pins::{Pin15, Pin32, Pin33};

/// Marker Trait for a valid PWM pin
pub trait ValidPwmPin {}

/// Creates a valid pwm pin impl for an existing GPIO pin
macro_rules! define_valid_pwm {
    ($($pin_name:ident),*) => {
        $(
            impl ValidPwmPin for $pin_name {}
        )*
    };
}

define_valid_pwm!(Pin32, Pin33, Pin15);
