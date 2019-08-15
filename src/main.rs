use std::fs::File;
use std::io::Write;
use std::time::Instant;
use serde_json;
use hashbrown::HashMap;
use rand::seq::SliceRandom;
use std::env;
mod models;
mod generator;
mod import;


fn populate_model(model : &models::Model, word_list : &Vec<String>){
	let mut i : usize = 0;
	for word in word_list {
		let next_word_index = i + 1;
		let next_word = word_list.get(next_word_index).unwrap().to_owned();
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

		if i < word_list.len() - (2*state_size as usize) {
			i += 1;
		}
	}
}
fn main() {
	// The messy scripty part. Should be divided up.
	// Time testing: 
	let start = Instant::now();
	
	let state_size : i32 = 5;

	let mut model = models::Model { map : HashMap::new(), };
	
	let content = import::from_file();
	let processed_content = generator::convert_to_word_vector(content, state_size);

	let random_word = processed_content.choose(&mut rand::thread_rng()).unwrap();
	println!("-------------------- \n Random start word: {:?}", random_word);

	populate_model(&model, &processed_content);
	
	// Should test this with another text source ot see if the output differs 
	// Test model joining
	model = models::join_models(vec![model.clone(), model]);

	// Test make sentence
	let mut output = vec![];
	for i in 1..10 {
		let output_string = generator::make_sentence(model.map.to_owned(), processed_content.choose(&mut rand::thread_rng()).unwrap().to_string());
		output.push(output_string);
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
