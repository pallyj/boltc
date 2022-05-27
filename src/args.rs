use clap::{Parser, ArgEnum};

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

    #[clap(short = 'o', default_value = "a.out")]
    pub output_file: String,

    #[clap(long, arg_enum, default_value = "object")]
    pub emit: Emit
}

/// (object, llvm, asm)
#[derive(Clone, ArgEnum)]
pub enum Emit {
    Object,
    Llvm,
    Asm
}