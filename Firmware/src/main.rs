#![no_std]
#![no_main]
/// In this version, you need to have two push buttons; one on A5, one on D13.
/// They should pull the pins to ground when they are pushed.
///
/// When D13 is pressed, The keycode for the 'F2' keycode is emitted. When A5 is
/// pressed, the keycode for 'F1' is emitted. These keycodes were choosen to
/// reduce the risk of conflicts in typical videogame keybindings.

use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use arduino_hal::Peripherals;
use itj_videogame_controller_firmware::my_pin::MyPin;
use panic_halt as _;
use usb_device::bus::UsbBusAllocator;
use usb_device::device::StringDescriptors;
use usb_device::device::UsbDeviceBuilder;
use usb_device::device::UsbVidPid;
use usb_device::LangID;
use usbd_hid::descriptor::KeyboardReport;
use usbd_hid::hid_class::HIDClass;
use usbd_hid::descriptor::SerializedDescriptor;

// TODO short term: I should be using a custom descriptor instead of
// a keyboard report. Examples:
//
// * https://github.com/twitchyliquid64/usbd-hid/blob/master/src/descriptor.rs#L61-L86
// * https://github.com/twitchyliquid64/usbd-hid/blob/master/macros/src/lib.rs#L51-L64
//
// TODO long term: would it make sense to merge such a descriptor upstream?
// Perhaps not, given that different video game controllers support different
// keys. Then again, you could say the same thing about keyboards and
// mice. Maybe merge many descriptors upstream? e.g. xbox, playstation, generic.

#[arduino_hal::entry]
fn main() -> ! {
	let dp: Peripherals = Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);
	let mut pin1 = MyPin::new(pins.a5.into_pull_up_input());
	let mut pin2 = MyPin::new(pins.d13.into_pull_up_input());
	let mut serial_hw = arduino_hal::default_serial!(dp, pins, 57600);
	ufmt::uwriteln!(&mut serial_hw, "Hello from Arduino!").unwrap_infallible();

	let usb_bus = arduino_hal::default_usb_bus_with_pll_macro!(dp);
	let usb_bus_allocator = UsbBusAllocator::new(usb_bus);

	let poll_period_ms = 5;
	let mut hid_class = HIDClass::new(&usb_bus_allocator, KeyboardReport::desc(), poll_period_ms);

	let string_descriptors = StringDescriptors::new(LangID::EN_US)
		.manufacturer("test manufacturer")
		.product("test product")
		.serial_number("test serial number");

	let rand_ids = UsbVidPid(0x1ea7, 0x4a09);

	let mut usb_dev = UsbDeviceBuilder::new(&usb_bus_allocator, rand_ids)
		.strings(&[string_descriptors])
		.unwrap()
		.max_packet_size_0(64)
		.unwrap()
		.build();

	// TODO: only in dev builds
	usb_dev.force_reset().unwrap();

	// Even if things go horribly wrong and the device starts spamming
	// keyboard events, it should still be trivial to flash it: just press
	// the reset button.
	//
	// I might be overlooking some edge case where it's hard to flash the
	// device. This delay will, hopefully, help with that.
	delay_ms(500);

	loop {
		usb_dev.poll(&mut [&mut hid_class]);

		if !pin1.has_data() && !pin2.has_data() {
			continue;
		}

		let state1 = !pin1.is_high();
		let state2 = !pin2.is_high();
		let mut keycodes = [0u8; 6];
		if state1 {
			// Keycode obtained from:
			// https://gist.github.com/mildsunrise/4e231346e2078f440969cdefb6d4caa3
			//
			// TODO: Create an enum upstream?
			keycodes[0] = 0x3A; // F1
		}
		if state2 {
			keycodes[1] = 0x3B; // F2
		}
		ufmt::uwriteln!(&mut serial_hw, "{}, {}", state1, state2).unwrap_infallible();

		hid_class
			.push_input(&KeyboardReport {
				keycodes,
				leds: 0,
				modifier: 0,
				reserved: 0,
			})
			.unwrap();
	}
}
