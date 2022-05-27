mod args;
mod cstd;
mod extension_host;

use std::process::Command;

use args::Args;
use blir::{BlirContext, Library};
use clap::StructOpt;
use codegen::config::{BuildConfig, BuildOutput, BuildProfile};
use colored::Colorize;
use errors::{debugger::Debugger, fileinterner::FileInterner, error::ErrorCode};
use extension_host::ExtensionHost;
use lower_ast::AstLowerer;
use lower_blir::BlirLowerer;
use parser::{parser::parse};

/*

parse: 59ms
lower ast: 76ms
resolve: 26ms
check: 0ms
closures: 0ms
lower blir: 27ms
codegen: 307ms

*/
fn main() {
    let args = Args::parse();

    let standard = cstd::StandardLib::default();

    if args.files.first().map(|first| first == "install").unwrap_or(false) {
        standard.install();
        return;
    }

    if !standard.is_installed() {
        standard.install();
    }

    let lib_name = if let Some(lib) = args.lib {
        lib
    } else {
        println!("error: no lib specified");
        return
    };

    let mut project = Project::new(&lib_name, args.extensions);

    // Add standard library
    let lang = [//"test/test.bolt"
                "std/print.bolt",
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
                "string/StringSlice.bolt",
                "string/Char.bolt"];

    let lib_path = standard.get_source_path();
    let lib_path_str = lib_path.as_os_str().to_string_lossy();

    for file in lang {
        project.open_file(&format!("{}/{file}", lib_path_str));
    }

    for file in &args.files {
        project.open_file(file);
    }

    let compiled = project.compile();

    if !compiled.0 {
        return;
    }

    if let Some(entry_point) = compiled.1 {
        let lib = standard.get_lib_path();
        let lib_str = lib.as_os_str().to_string_lossy();
        // Link with the c standard library
        let stderr =
        Command::new("clang").args(["-L",
                                    &lib_str,
                                    "-lstd",
                                    &format!("bin/lib{}.o", lib_name),
                                    "-e",
                                    &format!("_{}", entry_point),
                                    "-o",
                                    &format!("bin/{}", lib_name)])
                             .output()
                             .unwrap()
                             .stderr;

        if stderr.is_empty() {
            Command::new(&format!("bin/{}", lib_name)).spawn().unwrap();
        } else {
            println!("{} {}", "error:".red().bold(), unsafe { String::from_utf8_unchecked(stderr) });
        }
    }
}

pub struct Project {
    library:  Option<Library>,
    interner: FileInterner,
    extensions: Vec<String>,
}

impl Project {
    pub fn new(name: &str, extensions: Vec<String>) -> Project {
        Project { library:  Some(Library::new(name)),
                  interner: FileInterner::new(),
                  extensions }
    }

    pub fn open_file(&mut self, file: &str) { self.interner.open_file(file); }

    pub fn compile(&mut self) -> (bool, Option<String>) {
        let mut debugger = Debugger::new(&self.interner);

        let mut host = ExtensionHost::new();

        for extension in &self.extensions {
            if let Err(_) = unsafe { host.load_extension(extension) } {
                debugger.throw(ErrorCode::ExtensionLoadFailed(extension.clone()), vec![]);
            }
        }

        //let mut parse_time = std::time::Duration::ZERO;
        //let mut lower_time = std::time::Duration::ZERO;

        for file in self.interner.iter() {
            //let start = std::time::Instant::now();
            let parse = parse(file.1.text(), &mut debugger, file.0, &host.operator_factory);
            //let post_parse = std::time::Instant::now();

            if debugger.has_errors() {
                continue;
            }

            AstLowerer::new(parse, &mut debugger, &host.operator_factory).lower_file(self.library.as_mut().unwrap());
            //let post_lower = std::time::Instant::now();

            //lower_time += post_lower - post_parse;
            //parse_time += post_parse - start;
        }

        //let post_parse = std::time::Instant::now();

        if debugger.has_errors() {
            return (false, None);
        }

        let mut context = BlirContext::new();

        blir_passes::TypeResolvePass::new(&host.attribute_factory,
                                          &host.operator_factory,
                                          &mut context,
                                          &mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        //let post_type_resolve = std::time::Instant::now();

        //println!("{:?}", self.library);

        blir_passes::TypeInferPass::new(&mut context, &mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        //let post_type_infer = std::time::Instant::now();

        blir_passes::ClosureResolvePass::new(&host.attribute_factory,
                                             &host.operator_factory,
                                             &mut context,
                                             &mut debugger).run_pass(self.library.as_mut().unwrap());

        //println!("{:?}", self.library.as_ref().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        //let post_closure_resolve = std::time::Instant::now();

        blir_passes::TypeCheckPass::new(&mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() {
            return (false, None);
        }

        //let post_type_check = std::time::Instant::now();

        let mut lowerer = BlirLowerer::new(self.library.take().unwrap(), &mut debugger);
        lowerer.lower();

        let library = lowerer.finish();

        //let post_lower_blir = std::time::Instant::now();

        //println!("{library}");

        let config = BuildConfig::new(BuildProfile::Release, BuildOutput::Object, None);

        codegen::compile(library, config);

        //let post_llvm = std::time::Instant::now();

        /*println!(r#"
    parse took {} ms
    blir took {} ms
    resolve took {} ms
    infer took {} ms
    check took {} ms
    closures took {} ms
    blirsssa took {} ms
    llvm took {} ms"#,
    parse_time.as_millis(),
    lower_time.as_millis(),
    (post_type_resolve - post_parse).as_millis(),
    (post_type_infer - post_type_resolve).as_millis(),
    (post_closure_resolve - post_type_infer).as_millis(),
    (post_type_check - post_closure_resolve).as_millis(),
    (post_lower_blir - post_closure_resolve).as_millis(),
    (post_llvm - post_lower_blir).as_millis());*/

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
