#![feature(let_else)]

use std::{collections::HashMap, marker::PhantomData};

use inkwell::{passes::{PassManagerBuilder, PassManager}, builder, OptimizationLevel, module::Module};
use itertools::Itertools;
use mir::ty::StructId;

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

    pub fn lower_project(mut self)
    {
        let context = inkwell::context::Context::create();

        let context = MirLowerContext::new(self.project, &context);
        context.lower_project();


        /*let pass_manager_builder = PassManagerBuilder::create();
        pass_manager_builder.set_optimization_level(OptimizationLevel::Aggressive);

        let pass_manager: PassManager<Module> = PassManager::create(());
        pass_manager.add_always_inliner_pass();
        pass_manager_builder.populate_module_pass_manager(&pass_manager);

        pass_manager.run_on(&context.module);*/

        //context.display();

        context.module.print_to_file("module.ll").unwrap()
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
        unsafe {
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
    }

    pub fn lower_project(&self)
    {
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

    pub fn display(&self)
    {
        self.module.print_to_stderr();
    }
}