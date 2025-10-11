use std::io::ErrorKind;

use crate::my_event_summary::summary_to_text;
use evdev::uinput::VirtualDevice;
use evdev::Device;
use evdev::EventSummary;
use evdev::UinputAbsSetup;

use crate::constants::get_input_id_merged_controller;

/// This is a HelloWorld virtual controller.
///
/// It takes a list of identical physical controllers as input, and uses it to
/// create a single virtual controller. Inputs from any of the physical
/// controllers are forwarded to the virtual controller.
pub struct MergedController {
	input_device: Vec<Device>,
	virtual_device: VirtualDevice,
}

impl MergedController {
	pub fn new(input_controllers: &[&str]) -> Self {
		assert!(!input_controllers.is_empty());
		let input_device: Vec<Device> = input_controllers
			.iter()
			.map(|path| Device::open(path).unwrap())
			.collect();

		for device in &input_device {
			device.set_nonblocking(true).unwrap();
		}

		let primary = &input_device[0];
		let properties = primary.properties();
		println!("PROPERTIES: {properties:?}");
		let keys = primary.supported_keys().unwrap();

		let mut builder = VirtualDevice::builder()
			.unwrap()
			.name("Two One Handed Controllers")
			.input_id(get_input_id_merged_controller())
			.with_properties(properties)
			.unwrap()
			.with_keys(keys)
			.unwrap();

		// for device in input_device[1..] {
		// 	// TODO: assert device.properties() == properties, etc
		// }

		for (code, info) in primary.get_absinfo().unwrap() {
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
		for device in &mut self.input_device {
			let events = device.fetch_events();
			if let Err(err) = events {
				assert_eq!(err.kind(), ErrorKind::WouldBlock);
				continue;
			}
			// TODO: finish processing inner loop before calling emit. Send all events at once? (assuming we don't get any `sync`s in the middle...)

			// TODO: add fance logic. if button a is pressed on controller 1 then 2, insert fake release event.
			for event in events.unwrap() {
				let summary = event.destructure();
				let should_forward = match summary {
					EventSummary::Key(_, _, _) => true,
					EventSummary::Synchronization(_, _, _) => false, /* unnecessary; https://docs.rs/evdev/latest/evdev/uinput/struct.VirtualDevice.html#method.emit*/
					EventSummary::AbsoluteAxis(_, _, _) => true,
					EventSummary::ForceFeedback(_, _, _) => false,
					summary => panic!("Unsupported event summary: {summary:?}"),
				};

				let result = if should_forward {
					self.virtual_device.emit(&[event]).unwrap();
					"Forward"
				} else {
					"IGNORE"
				};
				let text = summary_to_text(summary);
				println!("{result:10} - {text}");
			}
		}
	}

	pub fn poll_loop(&mut self) -> ! {
		loop {
			self.poll();
		}
	}
}
