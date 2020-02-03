/*
    Taking in corpus data, all methods should return a string.

    Currently locked into using an env var
*/
use std::fs;
use std::env;

pub fn from_file() -> std::string::String { // Microseconds. This seems fine as is.
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let filename = "/opt/rustymarks/tkill_test.txt";
	println!("In file... {}", filename);
	let contents = fs::read_to_string(filename)
				   .expect("Nah sorry mate, file is fucked");
	return contents;
}