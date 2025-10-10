use evdev::uinput::VirtualDevice;
use evdev::Device;

use crate::constants::get_input_id_merged_controller;

/// This is a HelloWorld virtual controller.
///
/// It takes a list of identical physical controllers as input, and uses it to
/// create a single virtual controller. Inputs from any of the physical
/// controllers are forwarded to the virtual controller.
pub struct MergedController {
	input_device: Device,
	virtual_device: VirtualDevice,
	// device: Device,
}

impl MergedController {
	pub fn new(input_controllers: &[&str]) -> Self {
		assert!(input_controllers.len() == 1);
		let input_device = Device::open(input_controllers[0]).unwrap();

		let mut virtual_device = VirtualDevice::builder()
			.unwrap()
			.name("VirtualController")
			.input_id(get_input_id_merged_controller())
			.build()
			.unwrap();
		println!("Virtual device = {virtual_device:?}");
		let syspath = virtual_device.get_syspath().unwrap();
		println!("syspath = {syspath:?}");

		let devnodes = virtual_device.enumerate_dev_nodes_blocking().unwrap();
		let mut devnodes: Vec<_> = devnodes.collect();
		assert!(devnodes.len() == 1);
		let node = devnodes.pop().unwrap().unwrap();
		println!("node = {node:?}");

		// TODO: ~~setup udev rules so that this doesn't crash~~
		// Now that I'm in the input group, this should no longer crash?
		// let device = evdev::Device::open(node).unwrap();
		Self {
			input_device,
			virtual_device,
			// device,
		}
	}

	pub fn get_device(&mut self) -> &mut Device {
		todo!();
		// &mut self.device
	}

	pub fn poll(&mut self) {
		for event in self.input_device.fetch_events().unwrap() {
			let summary = event.destructure();
			println!("Mirroring - {summary:?}");

			self.virtual_device.emit(&[event]).unwrap();
		}
	}

	pub fn poll_loop(&mut self) -> ! {
		loop {
			self.poll();
		}
	}
}
