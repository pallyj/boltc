use std::path::Path;

use prelude::{SourceFile, GenericLexer, Try};
use tokens::lexer::Lexer;

use crate::tokens::{parse::{Parser, Parse}, ast::Ast};

mod tokens;

fn main() {
    let sf = SourceFile::open_file(Path::new("/Users/pallyj/Documents/Coding/Bolt/examples/grammar/bolt/example.tok.bpg")).unwrap();

    let mut lexer = Lexer::new(sf.iter());

    lexer.lex();

    let mut parser = Parser::new(lexer);

    match Ast::parse(&mut parser) {
        Try::Some(ast) => {
            println!("{ast:?}");
        }
        Try::None(err) | Try::Err(err) => {
            println!("{:?}", err.source());
            println!("{err:?}");
        }
    }

}