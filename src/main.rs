use clap::Args;
use colored::Colorize;
use parser::{Lexer, Preprocessor};
use prelude::{SourceFile, Parser, Try, BoltMessage, MessageLevel, Source, GenericLexer};
use syntax::{TypeDecl, Decl};
use parser::Parse;

use clap::Parser as ClapParser;

mod args;

// TODO: Better errors
// TODO: Statements
// TODO: Expressions

fn main() {
    let args = args::Args::parse();

    let file = SourceFile::open_file(&args.file).unwrap();

    let mut lexer = Lexer::new(file.iter());
    let start = std::time::Instant::now();

    lexer.lex();

    let (file, tokens, ws) = GenericLexer::into(lexer);

    let mut preprocessor = Preprocessor::new(file, ws);

    preprocessor.process(tokens.into_iter());

    let mut parser = Parser::new(preprocessor);

    loop {
        match Decl::parse(&mut parser) {
            Try::Some(t) => {
                println!("{}", t.value());
            }
            Try::Err(e) => {
                let (err, source) = e.unwrap();
                println!("{err:?}");
                print_error(&err, source);
                break;
            }
            _ => {
                break
            }
        }
    }

    let interval = ((std::time::Instant::now() - start).as_nanos() as f64) / (1000000.);

    print!( "Took {} ms", interval );
}

fn print_error(e: &(dyn BoltMessage), source: Source) {
    if e.level() == MessageLevel::Warning {
        println!("{}:", "warning".yellow().bold())
    } else {
        println!("{}: {}", format!("error[{}]",  e.code()).red().bold(), e.description().bold());
    }

    println!("{}", source.line_slice());
}