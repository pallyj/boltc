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

    #[clap(short = 'o')]
    pub output_file: Option<String>,

    #[clap(long, arg_enum, default_value = "object")]
    pub emit: Emit,

    #[clap(long)]
    pub feature: Vec<String>
}

impl Args {
    pub fn validate(&mut self) -> bool {
        let mut is_valid = true;

        if self.optimization_level > 3 {
            eprintln!("{} optimization level {} is too high, use 0, 1, 2 or 3", "error:".red().bold(), self.optimization_level);
            is_valid = false;
        }



        if self.output_file.is_none() && self.lib.is_some() {
            self.output_file = Some(match self.emit {
                Emit::Asm => format!("{}.asm", self.lib.as_ref().unwrap()),
                Emit::Llvm => format!("{}.ll", self.lib.as_ref().unwrap()),
                Emit::Object => format!("{}", self.lib.as_ref().unwrap()),
            })
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