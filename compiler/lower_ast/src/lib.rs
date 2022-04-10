#![feature(let_else)]

use blir::intrinsics::Intrinsics;
use errors::Span;
use parser::ast::{file::FileItem, Parse, Root};
use rowan::TextRange;

mod code;
mod typ;
mod value;

mod attributes;
#[cfg(test)]
mod tests;

pub struct AstLowerer {
    file:  u32,
    parse: Root,
}

impl AstLowerer {
    pub fn new(parse: Parse) -> AstLowerer {
        AstLowerer { file:  parse.file as u32,
                     parse: Root::cast(parse.root).unwrap(), }
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
