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
use usbd_hid::descriptor::MouseReport;
use usbd_hid::descriptor::SerializedDescriptor;
use usbd_hid::hid_class::HIDClass;

#[arduino_hal::entry]
fn main() -> ! {
	let dp: Peripherals = Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(dp);
	let mut serial_hw = arduino_hal::default_serial!(dp, pins, 57600);
	ufmt::uwriteln!(&mut serial_hw, "Hello from Arduino!").unwrap_infallible();

	let usb_bus = arduino_hal::default_usb_bus_with_pll_macro!(dp);
	let usb_bus_allocator = UsbBusAllocator::new(usb_bus);

	let mut hid_class = HIDClass::new(&usb_bus_allocator, MouseReport::desc(), 1);

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
	loop {
		counter += 1;
		ufmt::uwriteln!(&mut serial_hw, "Cycle {}", counter).unwrap_infallible();

		usb_dev.poll(&mut [&mut hid_class]);

		if counter % 1000 == 0 {
			counter = 0;
			hid_class
				.push_input(&MouseReport {
					x: 0,
					y: 40,
					buttons: 0,
					pan: 0,
					wheel: 0,
				})
				.unwrap();
		}
	}
}
