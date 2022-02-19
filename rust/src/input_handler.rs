// use dialoguer::Input;

// pub fn midroll_time(){
//     let input : String = Input::new()
//     .with_prompt("Tea or coffee?")
//     .with_initial_text("Yes")
//     .default("No".into())
//     .interact_text()?;
// }
use dirs;

use std::num::ParseIntError;
use std::io;
use std::fs;
use std::path::Path;

pub fn clear_console() {
	print! ("\x1B[2J\x1B[1;1H");
}

pub fn midroll_time(){
	println!("Please enter the time you want your Midroll Ad to be.");
	println!("");
	println!("Please use this format: Minute:Second:Milisecond");
	println!("For example: 12:02.973");

	let mut input = String::new();

	io::stdin().read_line(&mut input)
		.ok()
		.expect("Couldn't read line");


	// Check if every time variable is entered
	if input.is_empty() || input == "\r\n"{
		clear_console();
		println!("Please enter a valid time.");

		midroll_time();
	} else {
		match calculate_time_from_input(input) {
			Ok(value) => {
				// Input was valid, I want to save it

				// Check if the file already exists
				match dirs::data_dir() {
					Some (mut dir) => {
						write_to_file(data);
					},
					None => {
						println!("The Data Directory doesn't exist on your system.");
					}
				}

			},
			Err(err) => {
				clear_console();
				println!("Please enter a valid time.");

				midroll_time();
			}
		}
	}
}

fn calculate_time_from_input(input: String) -> Result<u32, ParseIntError> {
	// Parsing the String as an Integer
	let minute = input.split(":").collect::<Vec<&str>>()[0].parse::<u16>()?;

	let minutes_in_seconds = minute * 60;

	let second = input.split(":").collect::<Vec<&str>>()[1].split(".").collect::<Vec<&str>>()[0].parse::<u16>()?;

	let mut millisecond_string = String::from(input.split(":").collect::<Vec<&str>>()[1].split(".").collect::<Vec<&str>>()[1]);
	
	// Remove the new line and return flags from the String
	let len = millisecond_string.trim_end_matches(&['\r', '\n'][..]).len();
	millisecond_string.truncate(len);

	// Check if the Milisecond String is 3 Characters, if not, make it 3 Characters
	match millisecond_string.len() {
		1 => millisecond_string.push_str("00"),
		2 => millisecond_string.push('0'),
		_ => {}
	}

	let mut final_string = (minutes_in_seconds + second).to_string();
	final_string.push_str(&millisecond_string);

	final_string.parse::<u32>()
}

fn write_to_file(data) {
	dir.push("PodcastDAI");
						
	if let Err(err) = fs::create_dir(&dir) {
		println!("Error");
	}

	dir.push("midroll_times.json");

	if dir.is_file() {
		// File exists
		dbg!("file exists");
	} else {
		// File doesnt exist
		dbg!(&dir);
		fs::write(&dir, "test").expect("Unable to write file");
	}
}