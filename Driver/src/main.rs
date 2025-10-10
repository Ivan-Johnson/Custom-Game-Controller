#![forbid(unsafe_code)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]
#![deny(clippy::large_stack_frames)]
use driver::cli::parse_args;

pub fn main() {
	let args = parse_args();
	args.main();
}
