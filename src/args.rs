use clap::{Parser, ArgEnum};
use colored::Colorize;

#[derive(Parser)]
pub struct Args {
    /// A specific file to run boltc on
    pub files: Vec<String>,

    /// Library name
    #[clap(long = "project-name")]
    pub lib: Option<String>,

    /// Extension binaries. Pass each one with the -x flag
    #[clap(short('x'))]
    pub extensions: Vec<String>,

    /// The optimization level to run the compiler with.
    #[clap(short = 'O', default_value = "0")]
    pub optimization_level: usize,

    /*
    #[clap(short = 'o')]
    pub output_file: Option<String>,*/

    #[clap(long, arg_enum, default_value = "object")]
    pub emit: Emit,

    #[clap(long)]
    pub feature: Vec<String>
}

impl Args {
    pub fn validate(&self) -> bool {
        let mut is_valid = true;

        if self.optimization_level > 3 {
            println!("{} optimization level {} is too high, use 0, 1, 2 or 3", "error:".red().bold(), self.optimization_level);
            is_valid = false;
        }

        return is_valid
    }
}

/// (object, llvm, asm)
#[derive(Clone, ArgEnum)]
pub enum Emit {
    Object,
    Llvm,
    Asm
}