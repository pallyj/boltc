#![feature(let_else)]

use blir::intrinsics::Intrinsics;
use errors::Span;
use parser::{ast::{file::FileItem, Parse, Root},
             operators::OperatorFactory};
use rowan::TextRange;

mod code;
mod typ;
mod value;

mod attributes;
#[cfg(test)]
mod tests;

pub struct AstLowerer {
    file:    u32,
    parse:   Root,
    factory: OperatorFactory,
}

impl AstLowerer {
    pub fn new(parse: Parse) -> AstLowerer {
        let mut factory = OperatorFactory::new();

        factory.register_intrinsics();

        AstLowerer { file: parse.file as u32,
                     parse: Root::cast(parse.root).unwrap(),
                     factory }
    }

    fn span(&self, range: TextRange) -> Span { Span::new(range, self.file) }

    pub fn lower_file(self, library: &mut blir::Library) {
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

                FileItem::NoOp(_) => {}

                FileItem::Error => panic!(),
            }
        }
    }
}
