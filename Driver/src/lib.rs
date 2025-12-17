#![forbid(unsafe_code)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unreachable_code))]
#![deny(clippy::large_stack_frames)]
pub mod cli;
mod constants;
mod merged_controller;
mod my_event_summary;
mod read;

pub use merged_controller::MergedController;
