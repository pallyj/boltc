#![feature(path_file_prefix)]

fn main() {
	let mut parser = Parser::new(r#"
import intrinsics

struct Int64 {
    var repr: i64

    private func unit(): i64 {
        self.repr
    }
}

func factorial(n: i64): i64 {
    if integer64CmpEq(n, 0) {
        1
    } else {
        integer64Mul(n, factorial(integer64Sub(n, 1)))
    }
}

func main(a: i64) {
    printi(factorial(5));
}

func printi(n: i64)
"#);
/*

func factorial(n: i64): i64 {
    if integer64CmpEq( n, 0 ) {
        1
    } else {
        integer64Mul( n, factorial(integer64Sub(n, 1)) )
    }
}
	"#);*/

	parser.operator_factory().register_intrinsics();

	let mut lib = Library::new("");

	AstLowerer::new(parser.parse_file())
		.lower_file(&mut lib);

    blir_passes::type_resolve::run_pass(&mut lib);
    blir_passes::type_infer::run_pass(&mut lib);
    blir_passes::type_check::run_pass(&mut lib);

    let mut lowerer = BlirLowerer::new(lib);

    lowerer.lower();

    let library = lowerer.finish();

    let config = BuildConfig::new(BuildProfile::Debug, BuildOutput::Object, None);

    codegen::compile(library, config);

    Command::new("clang")
        .args([ "test/test.o", "output.o", "-e", "_main" ])
        .output()
        .unwrap();
    
    Command::new("./a.out")
        .spawn()
        .unwrap();
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

use std::process::Command;

use blir::Library;
use codegen::config::{BuildConfig, BuildProfile, BuildOutput};
use lower_ast::AstLowerer;
use lower_blir::BlirLowerer;
use parser::parser::Parser;