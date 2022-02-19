// use std::process::Command;
use human_panic::setup_panic;

mod ffmpeg_manager;
mod input_handler;

fn main() {
	setup_panic!();
	// https://doc.rust-lang.org/std/process/struct.Command.html
	
	if ffmpeg_manager::ffmpeg_installation_valid() {
		// FFmpeg Installation is valid, continue with regular flow
		println!("Installation valid, continueing");

		input_handler::midroll_time();

	} else {
		// FFmpeg is invalid
		println!("Your FFmpeg installation seems to be invalid. Please make
		sure you have it installed correctly!");

		/* Maybe trigger automatic installation flow, until then: "Press any
		button to close this window" */
	}
}
