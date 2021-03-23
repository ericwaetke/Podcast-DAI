use std::process::Command;
use std::str;

fn main() {
	// https://doc.rust-lang.org/std/process/struct.Command.html
	let output = Command::new("cmd")
				.args(&["/C", "echo hello"])
				.output()
				.expect("failed to execute process");
	// else {
	// 	Command::new("sh")
	// 			.arg("-c")
	// 			.arg("echo hello")
	// 			.output()
	// 			.expect("failed to execute process")
	// };
	
	let hello = str::from_utf8(&output.stdout).unwrap();

	println!("{:?}", hello);
}