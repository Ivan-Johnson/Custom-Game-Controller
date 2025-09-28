use driver::EventFile;
use libc::input_event;

pub fn main() -> std::io::Result<()> {
	let mut controller1 = EventFile::new("usb-Microsoft_Xbox_Adaptive_Joystick_0Y3DCGX24243Q8-event-joystick")?;

	let mut read_buffer: Vec<input_event> = vec![];
	read_buffer.reserve(1000);

	loop {
		controller1.read(&mut read_buffer)?;
		if read_buffer.is_empty() {
			// TODO: if/when `read` is updated to block with `poll`,
			// this `continue` can be changed to a panic.
			continue;
		}
		println!("Got {} events", read_buffer.len());
		for event in &read_buffer {
			let type_ = event.type_;
			let code = event.code;
			let value = event.value;
			let time = event.time;
			println!(
				"\ttype: {type_}, code: {code}, value: {value}, time_sec: {}",
				time.tv_sec
			);
		}
		println!("\n\n\n");
		read_buffer.clear();
	}
}
