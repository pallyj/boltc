#![feature(path_file_prefix)]

use std::path::Path;

use args::Args;
use blir::{Walker, Library};
use bolt_parser::{Parse as BoltParse, asttree::AstTree, Context, Parser};
use codegen::codegen_library;
use colored::Colorize;
use lower_ast::lower_file;
use prelude::{BoltMessage, MessageLevel, Source, SourceFile};
use project::Project;

use clap::Parser as ClapParser;

mod args;

// TODO: Better errors
// TODO: Statements
// TODO: Expressions

macro_rules! handle_error {
    ($val:expr) => {
        match $val {
            Ok(e) => e,
            Err(err) => {
                print_anon_error(&err);
                return;
            }
        }
    };
}

fn main() {
    let args = Args::parse();

    let file_path = Path::new(&args.file);

    let library_name = file_path.file_prefix().unwrap().to_str().unwrap().to_string();

    let source_file = match SourceFile::open_file(file_path) {
        Ok(file) => file,
        Err(e) => {
            print_anon_error(&e);
            return;
        },
    };

    let mut lexer = bolt_parser::Lexer::new(source_file.iter());
    lexer.lex();

    // Handle lexer errors

    let ctx = Context::new();

    let mut parser = Parser::new(lexer);
    let ast_file = AstTree::parse(&mut parser, &ctx);

    for msg in parser.messages() {
        let (msg, s) = msg.clone().unwrap();

        println!("{msg:?} at {s:?}");
    }

    let library = Library::new(library_name);

    lower_file(ast_file.into_declarations(), library.clone());

    let sym_resolver = passes::SymbolResolver::new();
    sym_resolver.walk_library(&library);

    let mangler = passes::ManglePass::new();
    mangler.walk_library(&library);

    let type_inferer = type_infer::InferWalker::new();
    type_inferer.walk_library(&library);

    type_inferer.context().solve();

    let replacer = type_infer::ReplacementWalker::new(type_inferer);
    replacer.walk_library(&library);

    let literal_replacer = passes::LiteralReplace {};
    literal_replacer.walk_library(&library);

    //println!("{library}");

    codegen_library(&library);
}

/*fn main() {
    let args = Args::parse();

    let mut project = handle_error!(Project::new(args.file.clone()));
        
    handle_error!(project.read_config());

    match project.search() {
        Ok(_) => {},
        Err(err) => {
            print_anon_error(&*err);
            return;
        }
    }

    let config = project.config();
    let source_files = project.source_files();

    let ast_files = source_files
        .iter()
        .map(|source_file| {
            let mut lexer = bolt_parser::Lexer::new(source_file.iter());

            lexer.lex();

            // Handle lexer errors

            let ctx = Context::new();

            let mut parser = Parser::new(lexer);

            let file = AstTree::parse(&mut parser, &ctx);

            for msg in parser.messages() {
                let (msg, s) = msg.clone().unwrap();

                println!("{msg:?} at {s:?}");
            }

            file
        }).collect::<Vec<_>>();

        
    let library = Library::new("lang".to_string());
    for ast_file in ast_files {
        lower_file(ast_file.into_declarations(), library.clone());
    }

    let sym_resolver = passes::SymbolResolver::new();
    sym_resolver.walk_library(&library);

    let type_inferer = type_infer::InferWalker::new();
    type_inferer.walk_library(&library);

    type_inferer.context().collect();
    type_inferer.context().solve();

    let replacer = type_infer::ReplacementWalker::new(type_inferer);
    replacer.walk_library(&library);

    //println!("{}", library);

    codegen_library(&library);
}*/

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
}