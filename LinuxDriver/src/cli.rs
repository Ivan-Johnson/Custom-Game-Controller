use argh::FromArgs;

use crate::read::read_main;
use crate::VirtualController;

/// A driver for virtual controllers.
#[derive(FromArgs)]
pub struct MyParsedArgs {
	#[argh(subcommand)]
	subcommand: SubcommandCLI,
}

impl MyParsedArgs {
	pub fn main(self) -> ! {
		self.subcommand.main()
	}
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum SubcommandCLI {
	StartDaemon(StartDaemonConfig),
	Read(ReadConfig),
	Scratch(ScratchConfig),
}

impl SubcommandCLI {
	pub fn main(self) -> ! {
		match self {
			SubcommandCLI::StartDaemon(conf) => conf.main(),
			SubcommandCLI::Read(conf) => conf.main(),
			SubcommandCLI::Scratch(conf) => conf.main(),
		}
	}
}

/// "read" data from the given event file
///
/// Basically, this just prints any events from the given device.
///
/// Example output:
///
/// ```
/// axis ABS_X 829
/// axis ABS_Y -4540
/// sync
/// axis ABS_Y -3613
/// sync
/// key BTN_EAST down
/// sync
/// key BTN_EAST up
/// sync
/// ```
#[derive(FromArgs)]
#[argh(subcommand, name = "daemon")]
struct StartDaemonConfig {}

impl StartDaemonConfig {
	pub fn main(self) -> ! {
		let mut merged = VirtualController::new();
		merged.poll_loop()
	}
}

/// Start a new virtual controller that mirrors the given controller.
#[derive(FromArgs)]
#[argh(subcommand, name = "read")]
struct ReadConfig {
	#[argh(positional)]
	node: String,
}

impl ReadConfig {
	pub fn main(self) -> ! {
		read_main(&self.node)
	}
}

/// A placeholder subcommand for testing code changes locally.
///
/// For example, the motivating usecase was to run some example code from the
/// `evdev` crate: I pasted that example code into `ScratchConfig::main`, tested
/// if it worked, then reverted it back to the original code when I was done.
#[derive(FromArgs)]
#[argh(subcommand, name = "scratch")]
struct ScratchConfig {}

impl ScratchConfig {
	pub fn main(self) -> ! {
		println!("Hello, World!");
		std::process::exit(0)
	}
}

pub fn parse_args() -> MyParsedArgs {
	let args: MyParsedArgs = argh::from_env();
	args
}
