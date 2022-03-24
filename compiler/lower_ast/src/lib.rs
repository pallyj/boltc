use blir::intrinsics::Intrinsics;
use errors::Span;
use parser::ast::{Parse, Root, file::FileItem};
use rowan::TextRange;

mod typ;
mod value;
mod code;

#[cfg(test)]
mod tests;

pub struct AstLowerer {
	file: u32,
	parse: Root
}

impl AstLowerer {
	pub fn new(parse: Parse) -> AstLowerer {
		AstLowerer {
			file: 0,
			parse: Root::cast(parse.root).unwrap()
		}
	}

	fn span(&self, range: TextRange) -> Span {
		Span::new(range, self.file)
	}

	pub fn lower_file(self, library: &mut blir::Library) {
		let parent = library.scope().clone();
		let intrinsics = Intrinsics::new();

		intrinsics.populate();

		for file_item in self.parse
			.items()
			.into_iter()
		{
			match file_item {
				FileItem::ImportDef(_import_def) => {
					// Do nothing for now
					match _import_def.import_library().as_str() {
						"intrinsics" => {
							parent.import(intrinsics.scope());
						},
						_ => {}
					}
				}

				FileItem::FuncDef(func_def) => {
					if func_def.code().is_some() {
						let lowered_function = self.lower_func(func_def, &parent);

						library.add_function(lowered_function);
					} else {
						let lowered_function = self.lower_extern_func(func_def);

						library.add_extern_function(lowered_function);
					}
				}

				FileItem::StructDef(struct_def) => {
					let lowered_struct = self.lower_struct(struct_def, &parent);

					library.add_struct(lowered_struct);
				}

				FileItem::Error => panic!(),
			}
		}
	}
}