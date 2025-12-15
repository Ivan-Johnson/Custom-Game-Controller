#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut led = pins.d13.into_output();

    loop {
        led.toggle();
        ufmt::uwriteln!(&mut serial, "Hello, World!").unwrap_infallible();
        arduino_hal::delay_ms(1000);
    }
}
