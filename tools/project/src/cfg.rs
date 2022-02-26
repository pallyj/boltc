use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCfg {
	name: String,
	description: String,
	version: String,
	author: String,

	target: Target
}

#[derive(Debug, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Target {
	#[serde(rename = "bin")]
	Binary,

	#[serde(rename = "lib")]
	Library,
}