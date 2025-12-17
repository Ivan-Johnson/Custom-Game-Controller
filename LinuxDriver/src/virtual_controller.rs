use std::io::ErrorKind;

use crate::constants::get_input_id_virtual_controller;
use crate::my_event_summary::summary_to_text;
use evdev::uinput::VirtualDevice;
use evdev::AttributeSet;
use evdev::AttributeSetRef;
use evdev::Device;
use evdev::EventSummary;
use evdev::InputEvent;
use evdev::KeyCode;
use evdev::KeyEvent;
use evdev::UinputAbsSetup;
use std::time::Duration;

/// This is a HelloWorld virtual controller.
///
/// It takes a list of identical physical controllers as input, and uses it to
/// create a single virtual controller. Inputs from any of the physical
/// controllers are forwarded to the virtual controller.
pub struct VirtualController {
	virtual_device: VirtualDevice,
}

impl VirtualController {
	pub fn new() -> Self {
		// For reference, here is the config info from one of my Microsoft XBox Adaptive Joysticks.
		//
		// PROPERTIES: {}
		// KEYS: {BTN_SOUTH, BTN_EAST, BTN_NORTH, BTN_WEST, BTN_TL, BTN_TR, BTN_SELECT, BTN_START, BTN_MODE, BTN_THUMBL, BTN_THUMBR}
		// ABSINFO: ABS_X, AbsInfo(input_absinfo { value: -64, minimum: -32768, maximum: 32767, fuzz: 16, flat: 128, resolution: 0 })
		// ABSINFO: ABS_Y, AbsInfo(input_absinfo { value: 1317, minimum: -32768, maximum: 32767, fuzz: 16, flat: 128, resolution: 0 })
		// ABSINFO: ABS_Z, AbsInfo(input_absinfo { value: 0, minimum: 0, maximum: 1023, fuzz: 0, flat: 0, resolution: 0 })
		// ABSINFO: ABS_RX, AbsInfo(input_absinfo { value: 0, minimum: -32768, maximum: 32767, fuzz: 16, flat: 128, resolution: 0 })
		// ABSINFO: ABS_RY, AbsInfo(input_absinfo { value: 0, minimum: -32768, maximum: 32767, fuzz: 16, flat: 128, resolution: 0 })
		// ABSINFO: ABS_RZ, AbsInfo(input_absinfo { value: 0, minimum: 0, maximum: 1023, fuzz: 0, flat: 0, resolution: 0 })
		// ABSINFO: ABS_HAT0X, AbsInfo(input_absinfo { value: 0, minimum: -1, maximum: 1, fuzz: 0, flat: 0, resolution: 0 })
		// ABSINFO: ABS_HAT0Y, AbsInfo(input_absinfo { value: 0, minimum: -1, maximum: 1, fuzz: 0, flat: 0, resolution: 0 })

		let keys = [
			KeyCode::BTN_SOUTH,
			KeyCode::BTN_EAST,
			KeyCode::BTN_NORTH,
			KeyCode::BTN_WEST,
		];
		let keys = AttributeSet::from_iter(keys);

		let mut virtual_device = VirtualDevice::builder()
			.unwrap()
			.name("Hello World")
			.input_id(get_input_id_virtual_controller())
			.with_keys(&keys)
			.unwrap()
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

		Self { virtual_device }
	}

	pub fn poll(&mut self) {
		todo!();
	}

	pub fn poll_loop(&mut self) -> ! {
		loop {
			std::thread::sleep(Duration::from_millis(1_000));

			let event: [InputEvent; 1] = [InputEvent::from(KeyEvent::new(KeyCode::BTN_SOUTH, 0))];
			self.virtual_device.emit(&event).unwrap();

			std::thread::sleep(Duration::from_millis(1_000));

			let event: [InputEvent; 1] = [InputEvent::from(KeyEvent::new(KeyCode::BTN_SOUTH, 1))];
			self.virtual_device.emit(&event).unwrap();
		}
	}
}
