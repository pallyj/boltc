#![feature(let_else)]

use std::{marker::PhantomData, path::Path};

use inkwell::{passes::{PassManagerBuilder, PassManager}, OptimizationLevel, module::Module, targets::{Target, TargetMachine, FileType, CodeModel, RelocMode}};
use tempfile::{NamedTempFile, TempPath};

mod ty;
mod code;

pub struct MirLowerer
{
    project: mir::Project
}

impl MirLowerer
{
    pub fn new(project: mir::Project) -> Self
    {
        Self { project }
    }

    pub fn lower_project(self, config: BuildConfig) -> Option<TempPath>
    {
        let context = inkwell::context::Context::create();

        let context = MirLowerContext::new(self.project, &context);
        context.lower_project();


        let pass_manager_builder = PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(OptimizationLevel::Aggressive);

        let pass_manager: PassManager<Module> = PassManager::create(());
        pass_manager.add_always_inliner_pass();
        pass_manager.add_loop_unroll_pass();
        pass_manager_builder.populate_module_pass_manager(&pass_manager);

        pass_manager.run_on(&context.module);

        if let BuildOutput::Llvm = config.build_output
        {
            context.module.print_to_file(&config.output_file).unwrap();
            return None
        }

        let optimization_level = match config.optimization_level
        {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Less,
            2 => OptimizationLevel::Default,
            3 => OptimizationLevel::Aggressive,
            _ => unreachable!(),
        };

        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();
        let target_machine = target.create_target_machine(&target_triple,
                                                      TargetMachine::get_host_cpu_name().to_str().unwrap(),
                                                      "+avx2",
                                                      optimization_level,
                                                      RelocMode::Static,
                                                      CodeModel::Default).unwrap();

        match config.build_output {
            BuildOutput::Assembly => {
                target_machine.write_to_file(&context.module, FileType::Assembly, Path::new(&config.output_file)).unwrap();
            }
            BuildOutput::Object => {
                let temp_file = NamedTempFile::new().unwrap();

                let path = temp_file.into_temp_path();

                target_machine.write_to_file(&context.module, FileType::Object, &path).unwrap();

                return Some(path);
            }
            _ => unreachable!()
        };
        
        return None;
    }
}


struct MirLowerContext<'a, 'ctx>
{
    context: &'ctx inkwell::context::Context,
    module:  inkwell::module::Module<'ctx>,
    builder: inkwell::builder::Builder<'ctx>,
    layout:  &'ctx inkwell::targets::TargetData,

    phantom: PhantomData<&'a ()>,

    project: mir::Project,
}

impl<'a, 'ctx> MirLowerContext<'a, 'ctx>
{
    pub fn new(project: mir::Project, context: &'ctx inkwell::context::Context) -> Self
    {
        let module = context.create_module(project.name());
        let builder = context.create_builder();
        let exe = Box::leak(Box::new(module.create_execution_engine().unwrap()));
        let target_data = exe.get_target_data();

        Self {
            context,
            module,
            builder,
            layout: &target_data,

            phantom: PhantomData,

            project
        }
    }

    pub fn lower_project(&self)
    {
        // Create global definitions
        for global in self.project.globals() {
            self.create_global(global)
        }

        // Create struct definitions
        for structure in self.project.structs() {
            self.create_struct(structure)
        }

        // Add fields to structs
        for structure in self.project.structs() {
            self.fill_struct_fields(structure);
        }

        // Create enum definitions
        for enumeration in self.project.enums() {
            self.create_enum(enumeration);
        }

        // Create enum definitions
        for enumeration in self.project.enums() {
            self.fill_enum_variants(enumeration);
        }

        // Create extern function definitions
        for function in self.project.extern_functions() {
            self.create_extern_function(function);
        }

        // Create function definitions
        for function in self.project.functions() {
            self.create_function(function);
        }

        // Lower function code
        for function in self.project.functions() {
            self.lower_function_code(function);
        }
    }

    #[allow(dead_code)]
    pub fn display(&self)
    {
        self.module.print_to_stderr();
    }
}

pub struct BuildConfig
{
    output_file: String,
    build_output: BuildOutput,
    optimization_level: u32,
}

impl BuildConfig
{
    pub fn new(output_file: &str, build_output: BuildOutput, optimization_level: u32) -> Self
    {
        Self {
            output_file: output_file.to_string(),
            build_output,
            optimization_level
        }
    }
}

pub enum BuildOutput
{
    Llvm,
    Assembly,
    Object
}