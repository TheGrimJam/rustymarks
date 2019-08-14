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

fn convert_to_word_vector(text: std::string::String, state_size : i32) -> Vec<String> {
	// Time testing: 
	let start = Instant::now();
	// Extra cleaning needed here for punctuation + other weirdnesses
	let space_match_re = Regex::new(r"[\n\r\s]+").unwrap();
	let words: Vec<String>  = space_match_re.split(&text).map(|s| s.to_string()).collect();
	let mut state_sized_words : Vec<String> = vec![];
	if state_size > 1 {
		let mut state_length : i32 = 0;
		let mut state_string : String = "".to_string();
		for word in &words {
			state_string = format!("{} {}", state_string, word.to_string());
			state_length += 1;
			if state_length == state_size {
				state_sized_words.push(state_string);
				state_string = "".to_string();
				state_length = 0;
			}
		}	
	}

	// Time testing: 
	let duration = start.elapsed();
	println!("Time ( convert_to_word_vector ): {:?}", duration);
	state_sized_words
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

fn make_sentence(model: HashMap<String, HashMap<String, i32>>, start_word: String) -> String{
	// Baby version of this is optional length field, generate those words, return as string.AsMut

	// Better version may be take the average sentence length from corpus and feed that in here. May require an adjustment
	// of the data structure we save to file. 
	// Random word selection. Used later.
	let mut current_word : String = start_word.to_owned();
	let mut output = vec![];
	for n in 1..15 {
		output.push(current_word.to_owned());
		current_word = weighted_random(model[&current_word].to_owned());
	}
	output.join(" ")
}

fn join_models(modelVector : Vec<Model>) -> Model {
	// Join two model files ( i.e the nested hashmaps )
	let mut joined_model = Model { map : HashMap::new(), };
	for (index, model) in modelVector.iter().enumerate() {
		if index == 0 { joined_model = model.clone() } 
		else {
			joined_model.map.extend(model.map.clone());
		};
	}
	joined_model
}

#[derive(Clone, Debug)] 
struct Model {
	map : HashMap<String, HashMap<String, i32>>
}


fn main() {

	// Time testing: 
	let start = Instant::now();

	let state_size : i32 = 5;
	let content = return_target_file_contents();
	let processed_content = convert_to_word_vector(content, state_size);
	let mut model = Model { map : HashMap::new(), };
	let mut i : usize = 0;

	let random_word = processed_content.choose(&mut rand::thread_rng()).unwrap();
	println!("-------------------- \n Random start word: {:?}", random_word);

	// This should likely be a function of it's own - content_processing
	for word in &processed_content {
		let next_word_index = i + 1; // Final word will error.
		let next_word = processed_content.get(next_word_index).unwrap().to_owned();
		println!("{}", word.clone());


		let mut inner_map = if model.map.contains_key(word) {
			let map : HashMap<String, i32> = model.map.get(word).unwrap().to_owned();
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
		model.map.insert(word.to_string(), inner_map);

		if i < processed_content.len() - (2*state_size as usize) {
			i += 1;
		}
	}
	
	// Should test this with another text source ot see if the output differs 
	// Test model joining
	model = join_models(vec![model.clone(), model]);

	// Test make sentence
	let mut output = vec![];
	for i in 1..10 {
		output.push(make_sentence(model.map.to_owned(), processed_content.choose(&mut rand::thread_rng()).unwrap().to_string()));
	}
	println!("Test make sentence: {:?}", output.join(". "));
	// Test JSON serializer 
	let output_model = model.map.to_owned();
	let serialized_data = serde_json::to_string(&output_model).unwrap();
	let serialized_data_bytes : &[u8] = serialized_data.as_bytes();

	// Saving to a .mdl file
	let args : Vec<String> = env::args().collect();
	let mut f = File::create(format!("{}{}", &args[1], ".mdl")).expect("Unable to create file");
    f.write_all(serialized_data_bytes).expect("Unable to write data");


		
    let duration = start.elapsed();
    println!("Total runtime : {:?}", duration);
}
