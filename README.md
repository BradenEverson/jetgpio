# JetGpio
This crate provides a peripheral abstraction layer over the [jetgpio-sys](https://github.com/raoz/jetgpio-sys) crate to provide memory-mapped access to NvidiaⓇ Jetson Nano™ or Jetson Orin Nano™ GPIO interfaces.

This library is built on top of [jetgpio-sys](https://github.com/raoz/jetgpio-sys), which iteslf is built on top of the [JETGPIO](https://github.com/Rubberazer/JETGPIO) package.

## Requirements

* Clang
    - Can be installed with `sudo apt-get install clang`
* sudo
    - In order for the crate to work properly, you must run the code with sudo level permissions. The workflow I have personally been using so far is as follows:
    ```bash
    cargo build
    sudo ./target/debug/{MY_CRATE_HERE}
    ```
    - The examples can be built and run in a very similar way.
* PWM
    - For PWM to work currently, the `pwm.sh` shell script must be run

## Currently Supported Interfaces (and those in progress)
- [x] GPIO Input and Output Pins
- [ ] PWM

## Licence

This crate is published under the [UNLICENSE](LICENSE), just as both of the underlying libraries.
