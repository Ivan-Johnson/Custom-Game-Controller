#![no_std]
#![no_main]

use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::port::PinOps;
use arduino_hal::prelude::_unwrap_infallible_UnwrapInfallible;
use panic_halt as _;

// Pointless little animation.
//
// Mostly because I don't want the LED to blink non-stop (it's distracting), but
// I also (for some inexplicably reason) don't want to delete the LED code
// entirely.
fn play_boot_animation<T>(led: &mut Pin<Output, T>)
where
	T: PinOps,
{
	// This is an "animation". Each value represents how long the LED is on for, in ms.
	// For simplicity, I've made it so that the total length of the animation is one second.
	let mini_animation = [175, 175, 350];
	let repeat_count = 1;

	let delay_off = 100;

	for delay in mini_animation
		.iter()
		.cycle()
		.take(mini_animation.len() * repeat_count)
	{
		led.set_high();
		arduino_hal::delay_ms(*delay);

		led.set_low();
		arduino_hal::delay_ms(delay_off);
	}
}

#[arduino_hal::entry]
fn main() -> ! {
	let dp = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);
	let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

	let mut led = pins.d13.into_output();
	play_boot_animation(&mut led);

	let button = pins.d11.into_floating_input();

	let mut was_high = button.is_high();

	loop {
		let is_high = button.is_high();

		let event = if !was_high && is_high {
			Some('D')
		} else if was_high && !is_high {
			Some('d')
		} else {
			None
		};

		if let Some(event) = event {
			ufmt::uwrite!(&mut serial, "{}", event).unwrap_infallible();
		};

		was_high = is_high;
	}
}
