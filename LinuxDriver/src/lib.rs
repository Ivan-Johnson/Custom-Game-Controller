#![forbid(unsafe_code)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]
#![deny(clippy::large_stack_frames)]
pub mod cli;
mod constants;
mod my_event_summary;
mod read;
mod virtual_controller;

pub use virtual_controller::VirtualController;
