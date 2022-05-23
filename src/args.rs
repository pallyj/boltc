use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// A specific file to run boltcc on
    pub file: String,

    /// Library name
    #[clap(long)]
    pub lib: Option<String>,

    #[clap(short('x'))]
    pub extensions: Vec<String>,
}
