#![feature(let_else)]

mod args;
mod cstd;
mod extension_host;
mod testing;
mod link;

use std::{collections::HashMap, mem};

use args::{Args, Emit};
use blir::{BlirContext, Library};
use blir_lowering::BlirLowerer;
use blir_passes::{MacroExpansionPass, TypeCheckPass, ClosureResolvePass, TypeInferPass, TypeResolvePass};
use clap::StructOpt;
//use codegen::config::{BuildConfig, BuildOutput, BuildProfile};
use colored::Colorize;
use cstd::StandardLib;
use errors::{fileinterner::FileInterner, DiagnosticReporter};
use extension_host::{ExtensionHost, ExtensionError};
use lower_ast::AstLowerer;
use mir::exc::ExecutionEngine;
use mir_lowering::{MirLowerer, BuildConfig, BuildOutput};
//use lower_blir::BlirLowerer;
use parser::{parser::parse};


fn main() {
    let args = Args::parse();

    if args.lib.is_none() {
        eprintln!("{} {}", "error:".red().bold(), "missing project name".bold());
        return
    };

    let _ =run(args);
}

fn run(mut args: Args) -> Result<(), ()>
{
    feature_gate::enable_features(&args.feature);

    let mut project = Project::new(args.lib.as_ref().unwrap());
    project.add_runtime();

    match args.files.first()
    {
        Some(s) if s.as_str() == "run-tests" => {
            testing::run_tests();
        }
        Some(s) if s.as_str() == "install" => {
            let standard = cstd::StandardLib::default();

            standard.install();
        }
        Some(s) if s.as_str() == "doc" => {
            if !args.validate() { return Err(()); }
            if !StandardLib::default().is_installed() { StandardLib::default().install() }
            project.open_files(&args.files[1..]); // open files
            project.compile_to_blir()?; // compile to blir
            project.run_passes()?; // run passes
            project.document(); // and then create documentation
        }
        Some(s) if s.as_str() == "emu" => {
            if !args.validate() { return Err(()); }
            if !StandardLib::default().is_installed() { StandardLib::default().install() }
            project.open_files(&args.files[1..]); // open files
            project.compile_to_blir()?; // compile to blir
            project.run_passes()?; // run passes
            project.compile_to_mir()?; // compile to mir
            project.emulate(); // run in emulator
        }
        _ => {
            if !args.validate() { return Err(()); }
            if !StandardLib::default().is_installed() { StandardLib::default().install() }
            project.open_files(&args.files); // open files
            project.compile_to_blir()?; // compile to blir
            project.run_passes()?; // run passes
            project.compile_to_mir()?; // compile to mir
            project.compile_to_llvm(&args); // lower to llvm
            // link the executable
        }
    }

    Ok(())
}

pub struct Project
{
    project_name: String,
    interner: FileInterner,
    extensions: Vec<String>,
    file_access_errors: Vec<String>,
    project_state: ProjectState,
    host: ExtensionHost,
    context: BlirContext,
}

impl Project
{
    pub fn new(project_name: &str) -> Project
    {
        Project
        {
            project_name: project_name.into(),
            interner: FileInterner::new(),
            extensions: Vec::new(),
            file_access_errors: Vec::new(),
            project_state: ProjectState::Initialize,
            host: ExtensionHost::new(),
            context: BlirContext::new(),
        }
    }

    pub fn add_runtime(&mut self)
    {
        let runtime = ["std/print.bolt",
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
                       "string/String.bolt",
                       "string/Char.bolt"];

        let standard = cstd::StandardLib::default();

        let lib_path = standard.get_source_path();
        let lib_path_str = lib_path.as_os_str().to_string_lossy();

        for file in runtime {
            self.open_file(&format!("{}/{file}", lib_path_str), "runtime");
        }
    }

    pub fn add_test_runtime(&mut self)
    {
        let runtime = ["test/testing.bolt"];

        let standard = cstd::StandardLib::default();

        let lib_path = standard.get_source_path();
        let lib_path_str = lib_path.as_os_str().to_string_lossy();

        for file in runtime {
            self.open_file(&format!("{}/{file}", lib_path_str), "runtime");
        }
    }

    pub fn open_files(&mut self, files: &[String])
    {
        let project_name = self.project_name.clone();
        for file in files
        {
            self.open_file(file, &project_name);
        }
    }

    fn open_file(&mut self, file: &str, project: &str)
    {
        // check if the file exists
        if !std::path::Path::new(file).exists()
        {
            self.file_access_errors.push(file.to_string());
            return
        }

        self.interner.open_file(file, project);
    }

    pub fn compile_to_blir(&mut self) -> Result<(), ()>
    {
        // Set up a debugger for compiling to blir
        let mut reporter = DiagnosticReporter::new(&self.interner);

        // Load extensions
        for extension in &self.extensions
        {
            let _ =
            unsafe { self.host.load_extension(extension) }
                .map_err(|_| {
                     reporter.throw_diagnostic(ExtensionError::LoadFailed(extension.clone()))
                });
        }

        // Separate the interner's files into libraries
        let (mut libraries, mut scopes) = (HashMap::new(), HashMap::new());

        for file in self.interner.iter()
        {
            let project = file.1.project();

            if libraries.contains_key(project)
            {
                continue
            }

            let library = Library::new(project);

            scopes.insert(project.to_string(), library.scope().clone());
            libraries.insert(project.to_string(), library);
        }

        // Parse each file
        for file in self.interner.iter()
        {
            let parsed = parse(file.1.text(),
                               &reporter,
                               file.0,
                               &self.host.operator_factory);

            reporter.errors()?;

            let library = libraries.get_mut(file.1.project()).unwrap();

            AstLowerer::new(
                parsed,
                &mut reporter,
                &self.host.operator_factory,
                &scopes)
                .lower_file(library);
        }

        self.project_state = ProjectState::Blir(libraries);

        Ok(())
    }

    pub fn run_passes(&mut self) -> Result<(), ()>
    {
        let ProjectState::Blir(mut libraries) = mem::take(&mut self.project_state) else { panic!() };
        let mut reporter = DiagnosticReporter::new(&self.interner);

        // Expand macros
        let mut macro_expansion_pass = MacroExpansionPass::new(&self.host.attribute_factory, &mut reporter);
        libraries.values_mut()
                 .for_each(|library| macro_expansion_pass.run_pass(library));
        reporter.errors()?;

        // Resolve types and values
        let mut resolve_pass = TypeResolvePass::new(&self.host.attribute_factory,
                                                &self.host.operator_factory,
                                                &mut self.context,
                                                &mut reporter);
        libraries.values_mut()
                 .for_each(|library| resolve_pass.run_pass(library));
        reporter.errors()?;

        // Infer types
        let mut infer_pass = TypeInferPass::new(&mut self.context, &mut reporter);
        libraries.values_mut()
                 .for_each(|library| infer_pass.run_pass(library));
        reporter.errors()?;

        // Resolve closures
        let mut closure_resolve_pass = ClosureResolvePass::new(&self.host.attribute_factory,
                                                           &self.host.operator_factory,
                                                           &mut self.context,
                                                           &mut reporter);
        libraries.values_mut()
                 .for_each(|library| closure_resolve_pass.run_pass(library));
        reporter.errors()?;

        // Check types
        let mut type_check_pass = TypeCheckPass::new(&mut reporter);
        libraries.values_mut()
                 .for_each(|library| type_check_pass.run_pass(library));
        reporter.errors()?;

        self.project_state = ProjectState::ProcessedBlir(libraries);

        Ok(())
    }

    pub fn compile_to_mir(&mut self) -> Result<(), ()>
    {
        let ProjectState::ProcessedBlir(libraries) = mem::take(&mut self.project_state) else { panic!() };
        let mut reporter = DiagnosticReporter::new(&self.interner);

        let mut project = mir::Project::new(&self.project_name);
        BlirLowerer::new(&mut project, &mut reporter, libraries.into_values().collect()).lower();

        reporter.errors()?;

        self.project_state = ProjectState::Mir(project);
        Ok(())
    }

    pub fn compile_to_llvm(&mut self, args: &Args)
    {
        let ProjectState::Mir(project) = mem::take(&mut self.project_state) else { panic!() };

        let mir_lowerer = MirLowerer::new(project);

        let build_output = match args.emit {
            Emit::Llvm => BuildOutput::Llvm,
            Emit::Asm => BuildOutput::Assembly,
            Emit::Object => BuildOutput::Object,
        };

        let build_config = BuildConfig::new(args.output_file.as_ref().unwrap(), build_output, args.optimization_level as u32);

        if let Some(temp_path) = mir_lowerer.lower_project(build_config)
        {
            if let Some(entry_point) = &self.context.entry_point {
                link::with_args(&temp_path, args, &entry_point)
            } else {
                eprintln!("{} {}", "error:".red().bold(), "no entry point provided".bold());
            }
        }
    }

    pub fn emulate(&mut self) -> mir::exc::val::Value
    {
        let ProjectState::Mir(project) = mem::take(&mut self.project_state) else { panic!() };

        let entry = self.context.entry_point.as_ref();
        project.execute().enter(&entry.unwrap(), vec![])
    }


    pub fn document(mut self) {
        let ProjectState::ProcessedBlir(libraries) = mem::take(&mut self.project_state) else { panic!() };

        let mut bundle = docgen::Bundle::new(self.project_name.clone());

        for library in libraries.into_values() {
            bundle.add_library(library);
        }
        bundle.hide_internals();
        bundle.save();
    }

    pub fn entry_point(&self) -> Option<&String>
    {
        self.context.entry_point.as_ref()
    }

    pub fn create_execution_engine<'a>(&'a mut self) -> ExecutionEngine<'a>
    {
        let ProjectState::Mir(project) = &mut self.project_state else { panic!() };

        project.execute()
    }
}

pub enum ProjectState {
    Initialize,
    Blir(HashMap<String, Library>),
    ProcessedBlir(HashMap<String, Library>),
    Mir(mir::Project),
    Llvm(mir_lowering::MirLowerer),
}

impl Default for ProjectState {
    fn default() -> Self {
        Self::Initialize
    }
}