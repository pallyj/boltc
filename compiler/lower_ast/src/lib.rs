#![feature(let_else)]

use blir::intrinsics::Intrinsics;
use errors::{Span, debugger::Debugger};
use parser::{ast::{file::FileItem, Parse, Root},
             operators::OperatorFactory};
use rowan::TextRange;

mod code;
mod typ;
mod value;
mod pattern;

mod attributes;
#[cfg(test)]
mod tests;

pub struct AstLowerer<'a, 'b> {
    file:    u32,
    parse:   Root,
    factory: &'a OperatorFactory,
    debugger:&'a mut Debugger<'b>
}

impl<'a, 'b> AstLowerer<'a, 'b> {
    pub fn new(parse: Parse, debugger: &'a mut Debugger<'b>, factory: &'a OperatorFactory) -> Self {
        AstLowerer { file: parse.file as u32,
                     parse: Root::cast(parse.root).unwrap(),
                     factory,
                    debugger }
    }

    fn span(&self, range: TextRange) -> Span { Span::new(range, self.file) }

    pub fn lower_file(mut self, library: &mut blir::Library) {
        let intrinsics = Intrinsics::new();

        intrinsics.populate();

        let parent = library.new_file();

        for file_item in self.parse.items().into_iter() {
            match file_item {
                FileItem::ImportDef(_import_def) => {
                    if _import_def.import_library() == "intrinsics" {
                        parent.import(intrinsics.scope());
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
