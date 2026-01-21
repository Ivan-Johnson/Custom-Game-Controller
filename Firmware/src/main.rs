#![no_std]
#![no_main]
use arduino_hal::delay_ms;
use arduino_hal::prelude::*;
use arduino_hal::Peripherals;
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

	delay_ms(500);

	let mut counter = 0;
	let mut button_is_pressed = false;

	// This will hold down the "A" button for a second or two, release it for a second or two, and then repeat infinitely.
	loop {
		counter += 1;
		ufmt::uwriteln!(&mut serial_hw, "Cycle {}", counter).unwrap_infallible();

		usb_dev.poll(&mut [&mut hid_class]);

		if counter % 1000 == 0 {
			counter = 0;
			button_is_pressed = !button_is_pressed;

			let mut keycodes = [0u8; 6];
			if button_is_pressed {
				// Keycode obtained from:
				// https://gist.github.com/mildsunrise/4e231346e2078f440969cdefb6d4caa3
				//
				// TODO: Create an enum upstream?
				keycodes[0] = 0x04; // 'a'
			}

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
}
