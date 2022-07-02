#![feature(let_else)]
#![feature(let_chains)]

use std::collections::HashMap;

use blir::{intrinsics::Intrinsics, scope::ScopeRef};
use errors::{Span, DiagnosticReporter};
use parser::{ast::{file::FileItem, Parse, Root},
             operators::OperatorFactory};
use rowan::TextRange;

mod code;
mod typ;
mod value;
mod pattern;
pub (crate) mod err;

mod attributes;
#[cfg(test)]
mod tests;

pub struct AstLowerer<'a, 'b> {
    file:    u32,
    parse:   Root,
    comments:Vec<String>,
    factory: &'a OperatorFactory,
    reporter:&'a mut DiagnosticReporter<'b>,
    scopes:  &'a HashMap<String, ScopeRef>,
}

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub fn new(
        parse: Parse,
        reporter: &'a mut DiagnosticReporter<'b>,
        factory: &'a OperatorFactory,
        scopes: &'a HashMap<String, ScopeRef>) -> Self
    {
        AstLowerer { file: parse.file as u32,
                     parse: Root::cast(parse.root).unwrap(),
                     comments: parse.comments,
                     factory,
                     scopes,
                     reporter }
    }

    fn span(&self, range: TextRange) -> Span { Span::new(range, self.file) }

    pub fn lower_file(mut self, library: &mut blir::Library) {
        self.comments.reverse();
        let intrinsics = Intrinsics::new();

        intrinsics.populate();

        let parent = library.new_file();

        if let Some(runtime_scope) = self.scopes.get("runtime") {
            parent.import(runtime_scope.clone());
        }

        for file_item in self.parse.items().into_iter() {
            match file_item {
                FileItem::ImportDef(import_def) => {
                    self.comments.pop().unwrap();

                    if import_def.import_library() == "intrinsics" {
                        parent.import(intrinsics.scope());
                    } else if import_def.import_library() == "runtime" {}
                    else if let Some(import_scope) = self.scopes.get(&import_def.import_library()) {
                        parent.import(import_scope.clone())
                    } else {
                        println!("error: library {} doesn't exist", import_def.import_library());
                        // todo: throw an error
                    }
                }

                FileItem::LetDef(let_def) => {
                    library.add_constant(self.lower_struct_static_let(let_def));
                }

                FileItem::FuncDef(func_def) => {
                    if func_def.code().is_some() {
                        let lowered_function = self.lower_func(func_def, &parent, library.path());

                        library.add_function(lowered_function);
                    } else {
                        let lowered_function = self.lower_extern_func(func_def, &parent);

                        library.add_extern_function(lowered_function);
                    }
                }

                FileItem::StructDef(struct_def) => {
                    let lowered_struct = self.lower_struct(struct_def, &parent, library.path());

                    library.add_struct(lowered_struct);
                }

                FileItem::EnumDef(enum_def) => {
                    let lowered_enum = self.lower_enum(enum_def, &parent, library.path());

                    library.add_enum(lowered_enum);
                }

                FileItem::TypeAlias(type_alias) => {
                    let visibility = self.lower_visibility(type_alias.visibility());
                    let name = type_alias.name();
                    let aliased = self.lower_type(type_alias.aliased_type());

                    library.add_type(name, visibility, aliased.kind);
                }

                FileItem::NoOp(_) => {}

                FileItem::Error => panic!(),
            }
        }
    }
}
