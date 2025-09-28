use libc::input_event;
use std::fs::File;
use std::io::Read;
use std::mem;

pub struct EventFile {
	file: File,
}

impl EventFile {
	pub fn new(id: &str) -> std::io::Result<Self> {
		let path = format!("/dev/input/by-id/{id}");
		let file = File::open(path)?;

		Ok(Self { file })
	}

	pub fn read(&mut self, buffer: &mut Vec<input_event>) -> std::io::Result<()> {
		// TODO: use poll(2)?
		// https://www.man7.org/linux/man-pages/man2/poll.2.html

		let capacity_bytes = buffer.capacity() * mem::size_of::<input_event>();

		// Safety: Reading `eventX` devices will return a list of
		// `input_event`s.  It is therefore safe to read data from
		// `eventX` directly into our `Vec<input_event>` buffer.
		//
		// https://docs.kernel.org/input/input.html#event-interface
		let buffer_raw_byte_pointer = buffer.as_mut_ptr() as *mut u8;
		let byte_slice = unsafe { std::slice::from_raw_parts_mut(buffer_raw_byte_pointer, capacity_bytes) };

		let len_bytes = self.file.read(byte_slice)?;

		// This assertion is guarenteed to hold by Linux:
		// https://docs.kernel.org/input/input.html#event-interface
		//
		// > you’ll always get a whole number of input events on a read
		assert!(len_bytes % mem::size_of::<input_event>() == 0);
		let len_events = len_bytes / mem::size_of::<input_event>();

		// Safety: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.set_len
		//
		// * "`new_len` must be less than or equal to `capacity()`"
		//
		//   1. The `capacity()` was passed into the `File::read` function
		//
		//   2. The value returned by `File::read` is guarenteed to be no larger than the value passed in
		//
		//   3. We're passing that return value into `set_len`
		//
		//   4. Therefore `new_len` is less than or equal to `capacity()`
		//
		//   Just to be extra-safe though, we can add an assert:
		assert!(len_bytes <= capacity_bytes);
		//
		// * "The elements at `old_len..new_len` must be initialized."
		//
		//   The `File::read` did this initialization for us.
		unsafe { buffer.set_len(len_events) };

		Ok(())
	}
}
