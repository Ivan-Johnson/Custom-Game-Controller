use embedded_hal::digital::InputPin;

pub struct MyPin<GPIO: InputPin> {
	pin: GPIO,
	was_high: bool,
}

impl<GPIO: InputPin> MyPin<GPIO> {
	pub fn new(mut pin: GPIO) -> Self {
		Self {
			was_high: pin.is_high().unwrap(),
			pin,
		}
	}

	pub fn has_data(&mut self) -> bool {
		self.pin.is_high().unwrap() != self.was_high
	}

	pub fn is_high(&mut self) -> bool {
		let is_high = self.pin.is_high().unwrap();
		self.was_high = is_high;
		is_high
	}
}
