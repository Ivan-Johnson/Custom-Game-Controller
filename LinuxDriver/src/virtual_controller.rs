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
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::mem::Discriminant;
use std::process::Child;
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;

/// A virtual controller.
///
/// This controller is driven by a plain-text UART device, e.g. /dev/ttyUSB0.
/// Each character of text that is read from that UART device is mapped to a
/// different virtual key event. Uppercase characters correspond to buttons
/// being pressed, lowercase letter correspond to buttons being released.
///
/// * L -> D-pad left
/// * R -> D-pad right
/// * U -> D-pad up
/// * D -> D-pad down
pub struct VirtualController {
	virtual_device: VirtualDevice,
	child: Child,
	stdout: ChildStdout,
}

impl Drop for VirtualController {
	fn drop(&mut self) {
		self.child.kill().unwrap();
	}
}

impl VirtualController {
	fn make_virtual_device() -> VirtualDevice {
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

		virtual_device
	}

	fn make_child() -> Result<(Child, ChildStdout), ()> {
		let uart = env!("RAVEDUDE_PORT");
		let mut child = Command::new("bash")
			.arg("-c")
			.arg(format!(
				"python3 -m serial.tools.miniterm --quiet \"{uart}\" 57600"
			))
			.stdout(Stdio::piped())
			// TODO?
			// .stdin(Stdio::piped())
			.stderr(Stdio::null())
			.spawn()
			.unwrap();

		let stdout = child.stdout.take().expect("bash's stdout is None??");

		Ok((child, stdout))
	}

	pub fn new() -> Self {
		let (child, stdout) = Self::make_child().unwrap();
		let virtual_device = Self::make_virtual_device();

		Self {
			virtual_device,
			child,
			stdout,
		}
	}

	pub fn poll(&mut self) {
		let exit_status = self.child.try_wait().unwrap();
		assert!(exit_status.is_none());

		let mapping = BTreeMap::from_iter([
			('U', KeyEvent::new(KeyCode::BTN_NORTH, 1)),
			('u', KeyEvent::new(KeyCode::BTN_NORTH, 0)),
			('R', KeyEvent::new(KeyCode::BTN_EAST, 1)),
			('r', KeyEvent::new(KeyCode::BTN_EAST, 0)),
			('D', KeyEvent::new(KeyCode::BTN_SOUTH, 1)),
			('d', KeyEvent::new(KeyCode::BTN_SOUTH, 0)),
			('L', KeyEvent::new(KeyCode::BTN_WEST, 1)),
			('l', KeyEvent::new(KeyCode::BTN_WEST, 0)),
		]);

		loop {
			let mut event: [u8; 1] = [0];

			let num_events = self.stdout.read(&mut event).unwrap();
			if num_events == 0 {
				break;
			}

			let event = event[0];
			let event_c = event as char;
			let Some(keycode) = mapping.get(&event_c) else {
				println!("ERROR: The char {event_c:?} ({event:?}) could not be mapped to  keycode");
				continue;
			};
			let event: [InputEvent; 1] = [InputEvent::from(*keycode)];
			println!("{event_c} -> {event:?}");
			self.virtual_device.emit(&event).unwrap();
		}
	}

	pub fn poll_loop(&mut self) -> ! {
		loop {
			std::thread::sleep(Duration::from_micros(10));
			self.poll();
		}
	}
}
