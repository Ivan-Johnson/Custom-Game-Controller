use evdev::Device;

pub fn read_main(node: &str) -> ! {
	let mut device = Device::open(node).unwrap();

	loop {
		for event in device.fetch_events().unwrap() {
			let event = event.destructure();

			match event {
				evdev::EventSummary::Synchronization(_, _, _) => println!("\nSYNC - {event:?}\n"),
				_ => println!("Got {event:?}"),
			}
		}
	}
}
