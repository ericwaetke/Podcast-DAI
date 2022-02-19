use std::process::Command;


pub fn ffmpeg_installation_valid() -> bool {
	let ffmpeg_check = Command::new("cmd")
		.args(&["/C", "ffmpeg -version"])
		.output()
		.expect("failed to execute process");

	if ffmpeg_check.status.success() {
		true
	} else {
		false
	}
}