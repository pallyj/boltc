#![feature(let_else)]

mod args;
mod cstd;
mod extension_host;
mod testing;

use std::process::Command;

use args::{Args};
use blir::{BlirContext, Library};
use blir_lowering::BlirLowerer;
use clap::StructOpt;
//use codegen::config::{BuildConfig, BuildOutput, BuildProfile};
use colored::Colorize;
use errors::{debugger::Debugger, fileinterner::FileInterner, error::ErrorCode};
use extension_host::ExtensionHost;
use lower_ast::AstLowerer;
//use lower_blir::BlirLowerer;
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

    if !args.validate() { return; }

    let standard = cstd::StandardLib::default();

    if args.files.first().map(|first| first == "run-tests").unwrap_or(false) {
        testing::run_tests();
        return;
    }

    if args.files.first().map(|first| first == "install").unwrap_or(false) {
        standard.install();
        return;
    }

    if !standard.is_installed() {
        standard.install();
    }

    for feature in &args.feature {
        feature_gate::enable_feature(feature);
    }

    let lib_name = if let Some(lib) = args.lib.clone() {
        lib
    } else {
        println!("{} no project name specified", "error:".red().bold());
        return
    };

    let mut project = Project::new(&lib_name, args.extensions.clone());

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

    let compiled = project.compile(&args);

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

    pub fn compile(&mut self, args: &Args) -> (bool, Option<String>) {
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

        let mut project = mir::Project::new("test");

        BlirLowerer::new(&mut project, vec![ self.library.take().unwrap() ]).lower();

        //println!("{project}");

        let entry = context.entry_point;

        project.execute().run_function(&entry.unwrap(), vec![]);

        //let post_type_check = std::time::Instant::now();

        /*let mut lowerer = BlirLowerer::new(self.library.take().unwrap(), &mut debugger);
        lowerer.lower();

        let library = lowerer.finish();

        //let post_lower_blir = std::time::Instant::now();

        //println!("{library}");

        let build_profile = match args.optimization_level {
            0 => BuildProfile::Debug,
            1 => BuildProfile::Less,
            2 => BuildProfile::Normal,
            3 => BuildProfile::Aggressive,
            _ => unreachable!(),
        };

        let output = match args.emit {
            Emit::Asm => BuildOutput::ASM,
            Emit::Llvm => BuildOutput::LLVM,
            Emit::Object => BuildOutput::Object,
        };

        let config = BuildConfig::new(build_profile, output, None);

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

        (!debugger.has_errors(), context.entry_point)*/
        (false, None)
    }

    pub fn compile_test(&mut self) -> Result<(String, mir::Project), ()> {
        let mut debugger = Debugger::new(&self.interner);

        let mut host = ExtensionHost::new();

        for extension in &self.extensions {
            if let Err(_) = unsafe { host.load_extension(extension) } {
                debugger.throw(ErrorCode::ExtensionLoadFailed(extension.clone()), vec![]);
            }
        }

        for file in self.interner.iter() {
            let parse = parse(file.1.text(), &mut debugger, file.0, &host.operator_factory);

            if debugger.has_errors() {
                continue;
            }

            AstLowerer::new(parse, &mut debugger, &host.operator_factory).lower_file(self.library.as_mut().unwrap());
        }

        if debugger.has_errors() { return Err(()) }

        let mut context = BlirContext::new();
        blir_passes::TypeResolvePass::new(&host.attribute_factory,
                                          &host.operator_factory,
                                          &mut context,
                                          &mut debugger).run_pass(self.library.as_mut().unwrap());

        if debugger.has_errors() { return Err(()) }
        blir_passes::TypeInferPass::new(&mut context, &mut debugger).run_pass(self.library.as_mut().unwrap());
        if debugger.has_errors() { return Err(()) }


        blir_passes::ClosureResolvePass::new(&host.attribute_factory,
                                             &host.operator_factory,
                                             &mut context,
                                             &mut debugger).run_pass(self.library.as_mut().unwrap());


        if debugger.has_errors() { return Err(()) }
        blir_passes::TypeCheckPass::new(&mut debugger).run_pass(self.library.as_mut().unwrap());
        if debugger.has_errors() { return Err(()) }

        let mut project = mir::Project::new("test");
        BlirLowerer::new(&mut project, vec![ self.library.take().unwrap() ]).lower();

        return Ok((context.entry_point.unwrap(), project))
    }
}