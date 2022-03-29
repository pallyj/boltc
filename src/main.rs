mod args;

use std::{process::Command, fs::File, io::Read};

use args::Args;
use blir::Library;
use clap::StructOpt;
use codegen::config::{BuildConfig, BuildProfile, BuildOutput};
use errors::debugger::Debugger;
use lower_ast::AstLowerer;
use lower_blir::BlirLowerer;
use parser::parser::parse;

fn main() {
    let args = Args::parse();

    let mut project = Project::new(&args.lib);

    // Add standard library
    project.open_file("std/print.bolt");

    project.open_file(&args.file);
    project.compile();

    // Link with the c standard library
    Command::new("clang")
        .args([ "bin/print.o", &format!("bin/lib{}.o", args.lib), "-e", &format!("_2L{}F4{}main", args.lib.len(), args.lib), "-o", &format!("bin/{}", args.lib) ])
        .output()
        .unwrap();
    
    Command::new(&format!("bin/{}", args.lib))
        .spawn()
        .unwrap();
}

pub struct Project {
    file_text: Vec<String>,
    library: Option<Library>,
    debugger: Debugger,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            file_text: vec![],
            library: Some(Library::new(name)),
            debugger: Debugger::new(),
        }
    }

    pub fn open_file(&mut self, file: &str) {
        let mut file = File::open(file).unwrap();

        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();
        let idx = self.file_text.len();
        self.file_text.push(code);
    
        let parse = parse(&self.file_text[idx], &mut self.debugger);

        AstLowerer::new(parse).lower_file(self.library.as_mut().unwrap());
    }

    pub fn compile(&mut self) {
        blir_passes::type_resolve::run_pass(self.library.as_mut().unwrap());
        blir_passes::type_infer::run_pass(self.library.as_mut().unwrap());
        blir_passes::type_check::run_pass(self.library.as_mut().unwrap());

        let mut lowerer = BlirLowerer::new(self.library.take().unwrap());
        lowerer.lower();
    
        let library = lowerer.finish();

        let config = BuildConfig::new(BuildProfile::Debug, BuildOutput::Object, None);
        
        codegen::compile(library, config);
    }
}

/*
fn print_error(e: &(dyn BoltMessage), source: Source) {
    if e.level() == MessageLevel::Warning {
        println!("{}:", "warning".yellow().bold())
    } else {
        println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
    }
    println!(" {} {}:{}:{}", "-->".blue().bold(), source.file_name(), source.line(), source.col());
    println!("  {}", "|".blue().bold());

    for line in source.line_slice().split('\n') {
        println!("  {} {}", "|".blue().bold(), line);
    }

    let offset = source.index_of_line();

    e.suggestion()
     .map(|suggestion| {
        println!("  {}{}{} {}", "|".blue().bold(), " ".repeat(offset), "^".repeat(source.len()).red().bold(), suggestion.red().bold() );
     });

    println!("  {}", "|".blue().bold());
    println!();
}

fn print_anon_error(e: &(dyn BoltMessage)) {
    if e.level() == MessageLevel::Warning {
        println!("{}:", "warning".yellow().bold())
    } else {
        println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
    }

    println!();
}*/