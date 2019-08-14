use hashbrown::HashMap;

#[derive(Clone, Debug)] 
pub struct Model {
	pub map : HashMap<String, HashMap<String, i32>>
}

pub fn join_models(modelVector : Vec<Model>) -> Model {
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
