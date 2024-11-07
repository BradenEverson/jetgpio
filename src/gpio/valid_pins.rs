//! Trait and struct implementations for all valid GPIO pins

/// A valid pin and its numeric representation
pub trait ValidPin {
    /// Returns the physical pin number for the given pin
    fn pin(&self) -> u32;
}

/// Creates a valid pin struct givent a name and physical pin number
macro_rules! define_valid_pins {
    ($($pin_name:ident, $pin_number:literal),*) => {
        $(
            #[doc = concat!("GPIO pin ", stringify!($pin_number), ".")]
            pub struct $pin_name;

            impl $pin_name {
                #[doc = concat!("Returns GPIO pin number ", stringify!($pin_number), ".")]
                pub fn number() -> u8 {
                    $pin_number
                }
            }

            impl ValidPin for $pin_name {
                fn pin(&self) -> u32 {
                    $pin_number
                }
            }
        )*
    };
}

// Valid GPIO pins on Jetson. Possible todo: Block this macro behind several different feature
// flags for the different nanos.
define_valid_pins!(
    Pin3, 3, Pin4, 4, Pin5, 5, Pin7, 7, Pin8, 8, Pin10, 10, Pin11, 11, Pin12, 12, Pin14, 14, Pin15,
    15, Pin16, 16, Pin17, 17, Pin18, 18, Pin19, 19, Pin21, 21, Pin22, 22, Pin23, 23, Pin24, 24,
    Pin26, 26, Pin27, 27, Pin28, 28, Pin29, 29, Pin31, 31, Pin32, 32, Pin33, 33, Pin35, 35, Pin36,
    36, Pin37, 37, Pin38, 38, Pin40, 40
);
