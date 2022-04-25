mod args;

use std::process::Command;

use args::Args;
use blir::{BlirContext, Library};
use clap::StructOpt;
use codegen::config::{BuildConfig, BuildOutput, BuildProfile};
use errors::{debugger::Debugger, fileinterner::FileInterner};
use lower_ast::AstLowerer;
use lower_blir::BlirLowerer;
use parser::{operators::OperatorFactory, parser::parse};

fn main() {
    let args = Args::parse();

    let mut project = Project::new(&args.lib);

    // Add standard library
    let lang = ["std/print.bolt",
                "bool/Bool.bolt",
                "float/Half.bolt",
                "float/Float.bolt",
                "float/Double.bolt",
                "int/Int.bolt",
                "int/UInt.bolt",
                "int/Int8.bolt",
                "int/Int16.bolt",
                "int/Int32.bolt",
                "int/Int64.bolt",
                "int/UInt8.bolt",
                "int/UInt16.bolt",
                "int/UInt32.bolt",
                "int/UInt64.bolt",
                "string/StringSlice.bolt"];

    for file in lang {
        project.open_file(&format!("lang/{file}"));
    }

    project.open_file(&args.file);

    let compiled = project.compile();

    if !compiled.0 {
        return;
    }

    if let Some(entry_point) = compiled.1 {
        // Link with the c standard library
        Command::new("clang").args(["bin/print.o",
                                    &format!("bin/lib{}.o", args.lib),
                                    "-e",
                                    &format!("_{}", entry_point),
                                    "-o",
                                    &format!("bin/{}", args.lib)])
                             .output()
                             .unwrap();

        Command::new(&format!("bin/{}", args.lib)).spawn().unwrap();
    }
}

pub struct Project {
    library:  Option<Library>,
    interner: FileInterner,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project { library:  Some(Library::new(name)),
                  interner: FileInterner::new(), }
    }

    pub fn open_file(&mut self, file: &str) { self.interner.open_file(file); }

    pub fn compile(&mut self) -> (bool, Option<String>) {
        let mut debugger = Debugger::new(&self.interner);

        let mut operator_factory = OperatorFactory::new();
        operator_factory.register_intrinsics();

        for file in self.interner.iter() {
            let parse = parse(file.1.text(), &mut debugger, file.0, &operator_factory);

            if debugger.has_errors() {
                continue;
            }

            AstLowerer::new(parse).lower_file(self.library.as_mut().unwrap());
        }

        if debugger.has_errors() {
            return (false, None);
        }

        let mut context = BlirContext::new();

        let attribute_factory = blir::attributes::default_attributes();

        blir_passes::TypeResolvePass::new(&attribute_factory,
                                          &operator_factory,
                                          &mut context,
                                          &mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        // println!("{:?}", self.library);

        blir_passes::TypeInferPass::new(&mut context, &mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        blir_passes::ClosureResolvePass::new(&attribute_factory,
                                             &operator_factory,
                                             &mut context,
                                             &mut debugger).run_pass(self.library.as_mut().unwrap());

        // println!("{:?}", self.library.as_ref().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        blir_passes::TypeCheckPass::new(&mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        let mut lowerer = BlirLowerer::new(self.library.take().unwrap());
        lowerer.lower();

        let library = lowerer.finish();

        //println!("{library}");

        let config = BuildConfig::new(BuildProfile::Release, BuildOutput::Object, None);

        codegen::compile(library, config);

        (!debugger.has_errors(), context.entry_point)
    }
}

// fn print_error(e: &(dyn BoltMessage), source: Source) {
// if e.level() == MessageLevel::Warning {
// println!("{}:", "warning".yellow().bold())
// } else {
// println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
// }
// println!(" {} {}:{}:{}", "-->".blue().bold(), source.file_name(), source.line(), source.col());
// println!("  {}", "|".blue().bold());
//
// for line in source.line_slice().split('\n') {
// println!("  {} {}", "|".blue().bold(), line);
// }
//
// let offset = source.index_of_line();
//
// e.suggestion()
// .map(|suggestion| {
// println!("  {}{}{} {}", "|".blue().bold(), " ".repeat(offset), "^".repeat(source.len()).red().bold(), suggestion.red().bold() );
// });
//
// println!("  {}", "|".blue().bold());
// println!();
// }
//
// fn print_anon_error(e: &(dyn BoltMessage)) {
// if e.level() == MessageLevel::Warning {
// println!("{}:", "warning".yellow().bold())
// } else {
// println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
// }
//
// println!();
// }
