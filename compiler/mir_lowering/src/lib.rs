#![feature(let_else)]

use std::collections::HashMap;

use itertools::Itertools;
use mir::ty::StructId;

mod ty;


pub struct MirLowerer
{
    context: &'static mut inkwell::context::Context,
    module:  inkwell::module::Module<'static>,
    builder: inkwell::builder::Builder<'static>,

    project: mir::Project,
}

impl MirLowerer
{
    pub fn new(project: mir::Project) -> Self
    {
        let context = Box::new(inkwell::context::Context::create());
        let context = Box::into_raw(context);

        unsafe {
            let module = (&*context).create_module(project.name());
            let builder = (&*context).create_builder();

            Self {
                context: &mut *context,
                module,
                builder,
                project
            }
        }
    }

    pub fn lower_project(&mut self)
    {
        // Create struct definitions
        for structure in &self.project.structs().clone() {
            self.create_struct(structure)
        }

        // Add fields to structs
        for structure in &self.project.structs().clone() {
            self.fill_struct_fields(structure);
        }
    }

    pub fn display(&self)
    {
        self.module.print_to_stderr();
    }
}

impl Drop for MirLowerer {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.context);
        }
    }
}