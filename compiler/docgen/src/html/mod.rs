use std::fs::File;
use std::io::Write;
use std::path::Path;

use itertools::Itertools;
use maud::{html, Markup, DOCTYPE};

use crate::enum_::{Enum, EnumVariant};
use crate::{Bundle, Library};
use crate::func::{Function, FunctionParameter};
use crate::struct_::{Struct, StructField};
use crate::ty::Type;

pub fn render_docs(bundle: Bundle) {
	let root = std::env::current_dir().unwrap().join("bin/docs");

	std::fs::create_dir_all(&root).unwrap();

	for (_, library) in &bundle.libraries {
		let lib_dir = root.join(&library.name);

		std::fs::create_dir_all(&lib_dir).unwrap();

		let html = create_file(library.into_html(&library.name), &library.name, "..", &library.name).into_string();
		let mut file = File::create(lib_dir.join("index.html")).unwrap();
		file.write_all(html.as_bytes());

		for structure in &library.structs {
			save_struct(structure, &lib_dir, Path::new(".."), &library.name);
		}
	}
}

fn save_struct(structure: &Struct, path: &Path, root: &Path, rel_path: &str) {
	let newroot = root.join("..");

	let n_rel_path = format!("{rel_path}/{}", structure.name);

	let html = create_file(structure.into_html(&n_rel_path), &structure.name, newroot.as_os_str().to_str().unwrap(), &n_rel_path).into_string();
	let dir = path.join(&structure.name);
	std::fs::create_dir_all(&dir);
	let mut file = File::create(dir.join("index.html")).unwrap();
	file.write_all(html.as_bytes());

	for substruct in &structure.substructs {
		save_struct(substruct, &path.join(&structure.name), &newroot, &n_rel_path);
	}

	for subenum in &structure.subenums {
		save_enum(subenum, &path.join(&structure.name), &newroot, &n_rel_path);
	}
}

fn save_enum(enumeration: &Enum, path: &Path, root: &Path, rel_path: &str) {
	let newroot = root.join("..");

	let n_rel_path = format!("{rel_path}/{}", enumeration.name);

	let html = create_file(enumeration.into_html(&n_rel_path), &enumeration.name, newroot.as_os_str().to_str().unwrap(), &n_rel_path).into_string();
	let dir = path.join(&enumeration.name);
	std::fs::create_dir_all(&dir);
	let mut file = File::create(dir.join("index.html")).unwrap();
	file.write_all(html.as_bytes());

	for substruct in &enumeration.substructs {
		save_struct(substruct, &path.join(&enumeration.name), &newroot, &n_rel_path);
	}

	for subenum in &enumeration.subenums {
		save_enum(subenum, &path.join(&enumeration.name), &newroot, &n_rel_path);
	}
}

pub trait IntoHtml {
	fn into_html(&self, path: &str) -> Markup;
}

impl IntoHtml for Struct {
    fn into_html(&self, path: &str) -> Markup {
		let initializers = self.methods
			.iter()
			.filter(|method| method.name == "init")
			.map(render_init)
			.collect_vec();

		let operators = self.methods
			.iter()
			.filter(|method| method.is_operator)
			.map(render_function)
			.collect_vec();

		let methods = self.methods
			.iter()
			.filter(|method| method.name != "init" && !method.is_operator)
			.map(render_function)
			.collect_vec();
		
		let fields = self.fields
			.iter()
			.map(render_field)
			.collect_vec();

        html! {
			h1 {
				span .keyword {
					"struct "
				}(self.name) }
			(meta_to_html(&self.meta))
			@if !initializers.is_empty() {
				h2 { "Initializers" }
			}
			@for init in initializers {
				(init)
			}
			@if !fields.is_empty() {
				h2 { "Fields" }
			}
			@for field in fields {
				(field)
			}
			@if !methods.is_empty() {
				h2 { "Methods" }
			}
			@for method in methods {
				(method)
			}
			@if !operators.is_empty() {
				h2 { "Operators" }
			}
			@for method in operators {
				(method)
			}
			@if self.substructs.len() > 0 {
				h2 { "Structs"  }
			}
			@for struct_ref in &self.substructs {
				a .struct_link href={(path)"/"(struct_ref.name)"/"} {
					(struct_ref.name)
				}
				@if let Some(first_line) = struct_ref.meta.trim_start().lines().next() {
					span .desc {
						" - " (first_line)
					}
				}
				br;
			}
			@if !self.subenums.is_empty() {
				h2 { "Enums" }
			}
			@for enum_ref in &self.subenums {
				a .struct_link href={(path)"/"(enum_ref.name)"/"} {
					(enum_ref.name)
				}
				@if let Some(first_line) = enum_ref.meta.trim_start().lines().next() {
					span .desc {
						" - " (first_line)
					}
				}
				br;
			}
		}
	}
}
impl IntoHtml for Enum {
    fn into_html(&self, path: &str) -> Markup {
		let operators = self.methods
			.iter()
			.filter(|method| method.is_operator)
			.map(render_function)
			.collect_vec();

		let methods = self.methods
			.iter()
			.filter(|method| method.name != "init" && !method.is_operator)
			.map(render_function)
			.collect_vec();
		
		let variants = self.variants
			.iter()
			.map(render_variant)
			.collect_vec();

        html! {
			h1 {
				span .keyword {
					"enum "
				}(self.name) }
			(meta_to_html(&self.meta))
			@if !variants.is_empty() {
				h2 { "Variants" }
			}
			@for variant in variants {
				(variant)
			}
			@if !methods.is_empty() {
				h2 { "Methods" }
			}
			@for method in methods {
				(method)
			}
			@if !operators.is_empty() {
				h2 { "Operators" }
			}
			@for method in operators {
				(method)
			}
			@if self.substructs.len() > 0 {
				h2 { "Structs"  }
			}
			@for struct_ref in &self.substructs {
				a .struct_link href={(path)"/"(struct_ref.name)"/"} {
					(struct_ref.name)
				}
				@if let Some(first_line) = struct_ref.meta.trim_start().lines().next() {
					span .desc {
						" - " (first_line)
					}
				}
				br;
			}
			@if !self.subenums.is_empty() {
				h2 { "Enums" }
			}
			@for enum_ref in &self.subenums {
				a .struct_link href={(path)"/"(enum_ref.name)"/"} {
					(enum_ref.name)
				}
				@if let Some(first_line) = enum_ref.meta.trim_start().lines().next() {
					span .desc {
						" - " (first_line)
					}
				}
				br;
			}
		}
	}
}
impl IntoHtml for Library {
    fn into_html(&self, path: &str) -> Markup {
		let funcs = self.functions
			.iter()
			.map(render_function)
			.collect_vec();

        html! {
			h1 { (self.name) }

			@if !self.structs.is_empty() {
				h2 { "Structs"  }
			}
			@for struct_ref in &self.structs {
				a .struct_link href={(path)"/"(struct_ref.name)"/"} {
					(struct_ref.name)
				}
				@if let Some(first_line) = struct_ref.meta.trim_start().lines().next() {
					span .desc {
						" - " (first_line)
					}
				}
				br;
			}
			@if !self.enums.is_empty() {
				h2 { "Enums" }
			}
			@for enum_ref in &self.enums {
				a .struct_link href={(path)"/"(enum_ref.name)"/"} {
					(enum_ref.name)
				}
				@if let Some(first_line) = enum_ref.meta.trim_start().lines().next() {
					span .desc {
						" - " (first_line)
					}
				}
				br;
			}
			@if !funcs.is_empty() {
				h2 { "Functions" }
			}
			@for func in funcs {
				(func)
			}
		}
    }
}

fn create_file(markup: Markup, title: &str, root: &str, steps: &str) -> Markup {
	const STEP: usize = 0;
	let splits = steps.split("/").collect_vec();
	html! {
		(DOCTYPE)
		html {
			head {
				link rel="stylesheet" href={(root)"/style.css"};
				title { (title) }
				base href={(root)};
			}
			body {
				div .sidebar {
					div .breadcrumbs {
						@for i in 0..(splits.len() - 1) {
							@let step = splits[0..=i].join("/");
							a .backtick href=(step) style={"margin-left: "(format!("{}", STEP * i))"px;"} {
								(splits[i])
							}
						}
						span .selftick style={"margin-left: "(format!("{}", STEP * splits.len()))"px;"} { (splits.last().unwrap()) }
					}
				}
				div .docs {
					(markup)
				}

				div .header {

				}
			}
		}
	}
}

fn render_function(func: &Function) -> Markup {
	let params = func.params.iter().map(render_param).intersperse_with(|| html!(", "));
	html! {
		div .block .func_block {
			div .def .func_def {
				span .keyword { (func.visibility.to_string())" " }
				@if func.reciever.is_none() && !func.is_operator {
					span .keyword { "static " }
				}
				@if func.is_mutating {
					span .keyword { "mutating " }
				}
				@if func.is_operator {
					span .keyword { "operator " }
				}
				span .keyword .func_keyword { "func " }
				span .ident { (func.name) }
				"("
				@for param in params {
					(param)
				}
				@if let Type::Void = &func.return_type {
					")"
				} @else {
					") -> "
					(render_type(&func.return_type))
				}
			}

			(meta_to_html(&func.meta))
		}
	}
}

fn render_field(field: &StructField) -> Markup {
	html!(
		div .block .var_block {
			div .def .var_def {
				span .keyword { (field.visibility.to_string())" " }
				@if field.is_variable {
					span .keyword { "var " }
				} @else {
					span .keyword { "let " }
				}
				span .ident { (field.field_name) }
				": "
				(render_type(&field.field_type))
			}
			(meta_to_html(&field.meta))
		}
	)
}

fn render_variant(variant: &EnumVariant) -> Markup {
	let associated_types = variant.associated_types
		.iter()
		.map(|(label, ty)| html! {
			@if let Some(label) = label {
				(label) ": "
			}
			(render_type(ty))
		})
		.intersperse_with(|| html!(", "))
		.collect_vec();

	html! {
		div .block .var_block {
			div .def .var_def {
				span .keyword { "case " }
				span .ident { (variant.name) }
				@if !associated_types.is_empty() {
				"("
				@for block in associated_types {
					(block)
				}
				")"
			}
			}
			(meta_to_html(&variant.meta))
		}
	}
}

fn render_init(func: &Function) -> Markup {
	let params = func.params.iter().map(render_param).intersperse_with(|| html!(", "));
	html! {
		div .block .func_block {
			div .def .func_def {
				span .keyword { (func.visibility.to_string())" " }
				span .keyword .init_keyword { "init" }
				"("
				@for param in params {
					(param)
				}
				")"
			}

			div .meta {
				(meta_to_html(&func.meta))
			}
		}
	}
}

fn render_param(param: &FunctionParameter) -> Markup {
	html! {
		span {
			@if param.is_shared {
				"shared "
			}
			@if param.label != "_" {
				b { (param.label) }
				": "
			}
			
			(render_type(&param.param_type))
		}
	}
}

fn render_type(ty: &Type) -> Markup {
	use Type::*;

	match ty {
		Void => html!("()"),
		Divergent => html!("!"),
		Function { parameters, return_type } => html! {
			span .keyword { "func" }
			" ("
			@for ty in parameters {
				(render_type(ty))
			}
			") -> "
			(render_type(return_type))
		},
		Tuple(types) => html! {
			"("
			@for (i, ty) in types.iter().enumerate() {
				@if let Some(label) = &ty.0 {
					(label)": "
				}
				(render_type(&ty.1))
				@if i != (types.len() - 1) {
					", "
				}
			}
			")"
		},
		Integer(bits) => html!((format!("i{bits}"))),
		Float(bits) => html!((format!("f{bits}"))),
		RawPointer(pointer) => html!( span .raw_pointer_type { "RawPointer<"(render_type(pointer))">" } ),
		Temp(name) => html!( span .named_type { (name) } ),
		Path(name, path) => html! {
			a .named_type href={(path.join("/"))} {
				(name)
			}
		}
	}
}

pub fn meta_to_html(meta: &str) -> Markup {
	let markdown = markdown::tokenize(meta);

	html! {
		div .meta {
			@for block in markdown {
				(render_block(&block))
			}
		}
	}
}

fn render_block(block: &markdown::Block) -> Markup {
	use markdown::Block::*;

	match block {
			Header(spans, level) => {
				let spans = spans.iter().map(render_span);

				match level {
					1 => html! { h3 {
						@for span in spans {
							(span)
						}
					}},
					2 => html! { h4 {
						@for span in spans {
							(span)
						}
					}},
					3 => html! { h5 {
						@for span in spans {
							(span)
						}
					}},
					4 => html! { h6 {
						@for span in spans {
							(span)
						}
					}},
					5 => html! { h6 {
						@for span in spans {
							(span)
						}
					}},
					6 => html! { h6 {
						@for span in spans {
							(span)
						}
					}},
					_ => unreachable!()
				}
			}
			Paragraph(spans) => html! {
				p {
					@for span in spans {
						(render_span(span))
					}
				}
			},
			Blockquote(blocks) => html! {
				p {
					@for block in blocks {
						(render_block(block))
					}
				}
			},
			CodeBlock(lang, code) => html! {
				pre {
					code {
						(code)
					}
				}
			},
			OrderedList(list_items, list_type) => html! {
				ol {
					@for li in list_items {
						(list_item_to_html(li))
					}
				}
			},
			UnorderedList(list_items) => html! {
				ul {
					@for li in list_items {
						(list_item_to_html(li))
					}
				}
			},
			Raw(string) => html!( (string) ),
			Hr => html! { hr; },
		}
}

fn list_item_to_html(list_item: &markdown::ListItem) -> Markup {
	use markdown::ListItem::*;
	
	match list_item {
		Simple(spans) => html! {
			li {
				@for span in spans {
					(render_span(span))
				}
			}
		},
		Paragraph(blocks) => html! {
			li {
				@for block in blocks {
					(render_block(block))
				}
			}
		},
	}
}

fn render_span(span: &markdown::Span) -> Markup {
	use markdown::Span::*;

	match span {
		Break => html! ( br; ),
		Text(text) => html! {
			span {
				(text)
			}
		},
		Code(code) => html! {
			code {
				(code)
			}
		},
		Link(link, desc, idk) => html! {
			span {
				(desc)
			}
		}, // todo: add the linke
		Image(path, desc, idk) => html! {
			span {
				(desc)
			}
		}, // todo: add the img
		Emphasis(italics) => html! {
			i {
				@for span in italics {
					(render_span(span))
				}
			}
		},
		Strong(bolds) => html! {
			b {
				@for span in bolds {
					(render_span(span))
				}
			}
		},
	}
}