#![feature(path_file_prefix)]

use blirssa::{Builder, typ::{Type, StructField}, value::BinaryIntrinsicFn, Library};
use lower_blirssa::lower_blirssa_library;

fn main() {
    let mut builder = Builder::new();
    let mut library = Library::new("helloWorld".to_string());

    library.add_struct("Int64", false, false);
    let int = library.get_struct("Int64").unwrap();
    int.add_field(StructField::new("repr".to_string(), Type::Integer { bits: 64 }));
    let int_ty = int.typ();

	library.add_function("add", Type::Integer { bits: 64 }.func_type(vec![ int_ty.clone().pointer(), int_ty.pointer() ]));
    let func = library.get_function("add").unwrap();

	let block = func.append_block("start");

	builder.position_at_end(&block);

    let a = func.arg(0);
	let b = func.arg(1);

    let a = builder.build_deref_struct_field(a, "repr");
    let b = builder.build_deref_struct_field(b, "repr");

	let c = builder.build_binary_intrinsic(BinaryIntrinsicFn::IntegerAdd, a, b);

	builder.build_return(Some(c));

	println!("{func}");

    lower_blirssa_library(library).unwrap();
}

/*
fn print_error(e: &(dyn BoltMessage), source: Source) {
    if e.level() == MessageLevel::Warning {
        println!("{}:", "warning".yellow().bold())
    } else {
        println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
    }
    println!(" {} {}:{}:{}", "-->".blue().bold(), source.file_name(), source.line(), source.col());
    println!("  {}", "|".blue().bold());

    for line in source.line_slice().split('\n') {
        println!("  {} {}", "|".blue().bold(), line);
    }

    let offset = source.index_of_line();

    e.suggestion()
     .map(|suggestion| {
        println!("  {}{}{} {}", "|".blue().bold(), " ".repeat(offset), "^".repeat(source.len()).red().bold(), suggestion.red().bold() );
     });

    println!("  {}", "|".blue().bold());
    println!();
}

fn print_anon_error(e: &(dyn BoltMessage)) {
    if e.level() == MessageLevel::Warning {
        println!("{}:", "warning".yellow().bold())
    } else {
        println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
    }

    println!();
}*/