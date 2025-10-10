pub fn main() -> std::io::Result<()> {
	let mut device = evdev::Device::open(
		"/dev/input/by-id/usb-Microsoft_Xbox_Adaptive_Joystick_0Y3DD3R24223Q8-event-joystick",
	)
	.unwrap();
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
