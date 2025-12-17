use crate::my_event_summary::summary_to_text;
use evdev::Device;

pub fn read_main(node: &str) -> ! {
	let mut device = Device::open(node).unwrap();

	loop {
		for event in device.fetch_events().unwrap() {
			let summary = event.destructure();
			println!("{}", summary_to_text(summary));
		}
	}
}
