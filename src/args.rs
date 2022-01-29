use clap::Parser;

#[derive(Parser)]
pub struct Args {
	/// A specific file to run boltcc on
	pub file: String,

	/// Generate .h files for each c file in the project
	#[clap(long)]
	pub generate_headers: bool,

	#[clap(long)]
	pub output: Option<String>,
}