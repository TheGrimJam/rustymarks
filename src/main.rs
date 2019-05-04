use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use rand::Rng;
use regex::Regex;
use serde_json;
use hashbrown::HashMap;
use rand::seq::SliceRandom;

fn return_target_file_contents() -> std::string::String {


	// Time testing: 
	let start = Instant::now();

	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let filename = &args[1];
	println!("In file... {}", filename);
	let contents = fs::read_to_string(filename)
				   .expect("Nah sorry mate, file is fucked");

	// Time testing: 
	let duration = start.elapsed();
	println!("Time ( return_target_file_contents ): {:?}", duration);
	
	return contents;
}

fn convert_to_word_vector(text: std::string::String) -> Vec<String> {


	// Time testing: 
	let start = Instant::now();
	// Extra cleaning needed here for punctuation + other weirdnesses
	let space_match_re = Regex::new(r"[\n\r\s]+").unwrap();
	let words: Vec<String>  = space_match_re.split(&text).map(|s| s.to_string()).collect();


	// Time testing: 
	let duration = start.elapsed();
	println!("Time ( convert_to_word_vector ): {:?}", duration);
	words
}

fn weighted_random(pairs: HashMap<String, i32>) -> String {
	let sum : i32 = pairs.iter().fold(0, |acc, x| acc + x.1);
	let mut num = rand::thread_rng().gen_range(0, sum);
	for pair in pairs {
		num -= pair.1;
		if num <= 0 { return pair.0; }
	};
	"FAILED".to_string()
}

fn main() {

	// Time testing: 
	let start = Instant::now();

	let content = return_target_file_contents();
	let processed_content = convert_to_word_vector(content);
	let mut word_map : HashMap<String, HashMap<String, i32>> = HashMap::new();
	let mut i : usize = 0;


	// Random word selection. Used later.
	let random_word = processed_content.choose(&mut rand::thread_rng()).unwrap();
	println!("-------------------- \n Random start word: {:?}", random_word);

	for word in &processed_content {
		let next_word_index = i + 1; // Final word will error.
		let next_word = processed_content.get(next_word_index).unwrap().to_owned();



		let mut inner_map = if word_map.contains_key(word) {
			let map : HashMap<String, i32> = word_map.get(word).unwrap().to_owned();
			map
		} else { // Create the inner map then add the word
			let map: HashMap<String, i32> = HashMap::new();
			map
		};
		if inner_map.contains_key(&next_word) {
			let mut count = inner_map.get(&next_word).unwrap().to_owned();
 			count += 1;
			inner_map.insert(next_word, count);
		} else {
			inner_map.insert(next_word, 1);
		};
		word_map.insert(word.to_string(), inner_map);

		if i < processed_content.len()-2 {
			i += 1;
		}


	}

	
	// Wee lazy test to see if we're getting right-ish values
	println!("Test hashmap : {:?}", word_map["Then"]);
	println!("Test randomizer : {:?}", weighted_random(word_map["Then"].to_owned()));
	
	let mut current_word : String = random_word.to_owned();
	for n in 1..100 {
		print!("{} ", current_word);
		current_word = weighted_random(word_map[&current_word].to_owned());
	}
	// Test JSON serializer 
	let output_word_map = word_map.to_owned();
	let serialized_data = serde_json::to_string(&output_word_map).unwrap();
	let serialized_data_bytes : &[u8] = serialized_data.as_bytes();

	// Saving to a .mdl file
	let args : Vec<String> = env::args().collect();
	let mut f = File::create(format!("{}{}", &args[1], ".mdl")).expect("Unable to create file");
    f.write_all(serialized_data_bytes).expect("Unable to write data");


		
    let duration = start.elapsed();
    println!("Total runtime : {:?}", duration);
}
