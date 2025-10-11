use evdev::uinput::VirtualDevice;
use evdev::Device;
use evdev::UinputAbsSetup;

use crate::constants::get_input_id_merged_controller;

/// This is a HelloWorld virtual controller.
///
/// It takes a list of identical physical controllers as input, and uses it to
/// create a single virtual controller. Inputs from any of the physical
/// controllers are forwarded to the virtual controller.
pub struct MergedController {
	input_device: Device,
	virtual_device: VirtualDevice,
}

impl MergedController {
	pub fn new(input_controllers: &[&str]) -> Self {
		assert!(input_controllers.len() == 1);
		let input_device = Device::open(input_controllers[0]).unwrap();

		let mut builder = VirtualDevice::builder()
			.unwrap()
			.name("Two One Handed Controllers")
			.input_id(get_input_id_merged_controller())
			.with_properties(input_device.properties())
			.unwrap()
			.with_keys(input_device.supported_keys().unwrap())
			.unwrap();

		for (code, info) in input_device.get_absinfo().unwrap() {
			builder = builder
				.with_absolute_axis(&UinputAbsSetup::new(code, info))
				.unwrap();
		}

		let mut virtual_device = builder.build().unwrap();

		println!("Virtual device = {virtual_device:?}");
		let syspath = virtual_device.get_syspath().unwrap();
		println!("syspath = {syspath:?}");

		let devnodes = virtual_device.enumerate_dev_nodes_blocking().unwrap();
		let mut devnodes: Vec<_> = devnodes.collect();
		assert!(devnodes.len() == 1);
		let node = devnodes.pop().unwrap().unwrap();
		println!("node = {node:?}");

		Self {
			input_device,
			virtual_device,
		}
	}

	pub fn poll(&mut self) {
		for event in self.input_device.fetch_events().unwrap() {
			let summary = event.destructure();

			self.virtual_device.emit(&[event]).unwrap();
			println!("Mirrored - {summary:?}");
		}
	}

	pub fn poll_loop(&mut self) -> ! {
		loop {
			self.poll();
		}
	}
}
