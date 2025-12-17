use evdev::BusType;
use evdev::InputId;

pub const INPUT_ID_BUS_TYPE: BusType = BusType::BUS_VIRTUAL;
pub const INPUT_ID_VENDOR: u16 = 0xac0b;
pub const INPUT_ID_VERSION: u16 = 0x0000;
pub const INPUT_ID_PRODUCT_VIRTUAL_CONTROLLER: u16 = 0x0000;

fn get_input_id(product: u16) -> InputId {
	InputId::new(
		INPUT_ID_BUS_TYPE,
		INPUT_ID_VENDOR,
		product,
		INPUT_ID_VERSION,
	)
}

// Instead of using a function, I'd *like* to just use a constant: `pub const
// INPUT_ID_VIRTUAL_CONTROLLER: InputId`. The issue though is that the
// `InputId::new` function isn't const -_-
pub fn get_input_id_virtual_controller() -> InputId {
	get_input_id(INPUT_ID_PRODUCT_VIRTUAL_CONTROLLER)
}
