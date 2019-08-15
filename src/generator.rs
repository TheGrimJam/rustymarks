
/*
    Generate markov data from the model.
*/
use rand::Rng;
use hashbrown::HashMap;
use regex::Regex;

fn weighted_random(pairs: HashMap<String, i32>) -> String {
	let sum : i32 = pairs.iter().fold(0, |acc, x| acc + x.1);
	let mut num = rand::thread_rng().gen_range(0, sum);
	for pair in pairs {
		num -= pair.1;
		if num <= 0 { return pair.0; }
	};
	"FAILED".to_string()
}

pub fn make_sentence(model: HashMap<String, HashMap<String, i32>>, start_word: String) -> String{
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
    uppercase_first_letter(&output.join(" "))
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn convert_to_word_vector(text: std::string::String, state_size : i32) -> Vec<String> {
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
				let state_without_whitespace = state_string.trim().to_string();
				state_sized_words.push(state_without_whitespace);
				state_string = "".to_string();
				state_length = 0;
			}
		}	
	}
	state_sized_words
}