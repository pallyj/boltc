use std::{collections::HashMap, fmt::Debug};

pub struct Metadata {
	metadata: HashMap<String, String>
}

impl Metadata {
	/// Creates a new Metadata object
	pub fn new() -> Self {
		let mut metadata = HashMap::new();

		metadata.insert("doc".to_string(), "".to_string());

		Metadata {
			metadata
		}
	}

	/// Adds a doc comment to the metadata
	pub fn doc_comment(&mut self, doc: &str) {
		self.metadata
			.get_mut("doc")
			.unwrap()
			.push_str(doc);
	}

	/// 
	/// Adds metadata to the library
	/// 
	/// Common metadata keys are 
	/// 
	/// `description`
	/// `authors`
	/// `version`
	/// `comment`
	/// 
	pub fn insert(&mut self, key: String, value: String) {
		self.metadata
			.insert(key, value);
	}
}

impl Debug for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for field in self.metadata.iter() {
			writeln!(f, "{}: {}", field.0, field.1)?;
		}
		Ok(())
    }
}