mod attribute;
mod event;
mod expr;
mod file;
mod func;
mod marker;
mod sink;
mod smt;
mod struct_;
mod typ;
mod var;
mod enum_;
mod pattern;
mod alias;

use errors::DiagnosticReporter;

use self::{event::Event,
           marker::{CompletedMarker, Marker},
           sink::Sink};
use crate::{ast::{Parse, SyntaxNode},
            lexer::{Lexer, SyntaxKind, Token},
            operators::OperatorFactory};

pub struct Parser<'input, 'l> {
    lexemes:   &'l [Token<'input>],
    operators: &'input OperatorFactory,
    events:    Vec<Event<'input>>,
    cursor:    usize,
}

impl<'input, 'l> Parser<'input, 'l> {
    pub fn new(lexemes: &'l [Token<'input>], operator_factory: &'input OperatorFactory) -> Self {
        Self { lexemes,
               events: Vec::new(),
               operators: operator_factory,
               cursor: 0 }
    }

    pub fn check(&mut self, token: SyntaxKind) -> bool {
        self.peek()
            .map(|next_token| next_token == token)
            .unwrap_or(false)
    }

    pub fn check_ahead(&mut self, n: usize, token: SyntaxKind) -> bool {
        self.peek_ahead(n)
            .map(|next_token| next_token == token)
            .unwrap_or(false)
    }

    pub fn eat(&mut self, token: SyntaxKind) -> bool {
        if !self.check(token) {
            return false;
        }

        self.bump();
        true
    }

    pub fn bump(&mut self) {
        self.eat_trivia();
        if let Some(next) = self.lexemes.get(self.cursor) {
            self.events.push(Event::AddToken { kind: next.kind,
                                               text: next.source, });
            self.cursor += 1;
        }
    }

    fn parse_delim(&mut self, node: SyntaxKind, bra: SyntaxKind, ket: SyntaxKind, mut f: impl FnMut(&mut Self)) {
        let marker = self.start();
        if !self.eat(bra) {
            // Throw an error
            // Recover from this
            self.bump();
        } else {
            while !self.eat(ket) {
                f(self);
            }
        }
        marker.complete(self, node);
    }

    fn parse_delim_end(&mut self, node: SyntaxKind, ket: SyntaxKind, mut f: impl FnMut(&mut Self)) {
        let marker = self.start();
        while !self.eat(ket) {
            f(self);
        }
        marker.complete(self, node);
    }

    fn parse_delim_separated(&mut self, node: SyntaxKind, bra: SyntaxKind, ket: SyntaxKind, sep: SyntaxKind, mut f: impl FnMut(&mut Self)) -> usize {
        let marker = self.start();
        let mut n = 0;

        if !self.eat(bra) {
            // Throw an error
            // Recover from this
            self.error(&format!("token {bra:?} not found"));
        } else {
            while !self.eat(ket) {
                n += 1;
                f(self);

                if !self.eat(sep) {
                    // End of list
                    if !self.eat(ket) {
                        // Recover from missing separator
                    }
                    break;
                }
            }
        }

        marker.complete(self, node);
        n
    }

    pub fn error(&mut self, error: &str) {
        let event = Event::Error(error.to_string());

        self.events.push(event);

        // Do error recovery
    }

    pub fn error_recover(&mut self, error: &str, recovery_set: &[SyntaxKind]) -> CompletedMarker {
        self.error(error);
        let err = self.start();
        if !self.peek()
                .map(|peeked_token| recovery_set.contains(&peeked_token))
                .unwrap_or(false)
        {
            self.bump();
        }
        err.complete(self, SyntaxKind::Error)
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    pub fn node<F: FnOnce(&mut Self)>(&mut self, kind: SyntaxKind, f: F) {
        let node = self.start();

        f(self);

        node.complete(self, kind);
    }

    pub fn name(&mut self, recovery_set: &[SyntaxKind]) {
        if self.check(SyntaxKind::Ident) {
            let func_name = self.start();
            self.eat(SyntaxKind::Ident);
            func_name.complete(self, SyntaxKind::FuncName);
        } else {
            self.error_recover("expected name", recovery_set);
        }
    }

    // fn parse_delim_separated_at(
    // &mut self,
    // node: SyntaxKind,
    // bra: SyntaxKind,
    // ket: SyntaxKind,
    // sep: SyntaxKind,
    // chk: Checkpoint,
    // mut f: impl FnMut(&mut Self))
    // {
    // self.start_node_at(node, chk);
    // if !self.eat(bra) {
    // Throw an error
    // Recover from this
    // self.bump();
    // self.finish_node();
    // return
    // }
    //
    // while !self.eat(ket) {
    // f(self);
    //
    // if !self.eat(sep) {
    // End of list
    // if !self.eat(ket) {
    // Recover from missing separator
    // }
    // break
    // }
    // }
    // self.finish_node();
    // }

    fn eat_trivia(&mut self) {
        while self.peek_raw()
                  .map(|peek| peek.is_trivia())
                  .unwrap_or(false)
        {
            self.cursor += 1;
        }
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.eat_trivia();
        self.peek_raw()
    }

    fn peek_ahead(&mut self, n: usize) -> Option<SyntaxKind> {
        let mut i = 0;
        let mut raw_index = self.cursor;

        while let Some(lexeme) = self.lexemes.get(raw_index) {
            raw_index += 1;

            if lexeme.kind.is_trivia() {
                continue;
            }

            if i == n {
                return Some(lexeme.kind);
            } else if i > n {
                return None;
            }

            i += 1;
        }

        None
    }

    fn peek_raw(&self) -> Option<SyntaxKind> { self.lexemes.get(self.cursor).map(|token| token.kind) }

    pub fn parse_paren_comma_seq(&mut self, f: impl FnMut(&mut Self)) -> usize {
        self.parse_delim_separated(SyntaxKind::CommaSeparatedList,
                                   SyntaxKind::OpenParen,
                                   SyntaxKind::CloseParen,
                                   SyntaxKind::Comma,
                                   f)
    }

    pub fn parse_visibility(&mut self) {
        let marker = self.start();

        if self.eat(SyntaxKind::PublicKw) || self.eat(SyntaxKind::InternalKw) || self.eat(SyntaxKind::FilePrivateKw) || self.eat(SyntaxKind::PrivateKw) {}

        marker.complete(self, SyntaxKind::Visibility);
    }
}

pub fn parse<'input>(input: &'input str, debugger: &'input DiagnosticReporter, file: usize, operator_factory: &OperatorFactory) -> Parse {
    let lexemes: Vec<_> = Lexer::new(input).collect();

    let parser = Parser::new(&lexemes, operator_factory);

    let events = parser.parse_file();
    let sink = Sink::new(events, &lexemes, file);

    let (root, comments) = sink.finish(debugger);

    Parse { file,
            root: SyntaxNode::new_root(root),
            comments }
}

pub fn test<'input, F>(input: &'input str, debugger: &'input DiagnosticReporter, file: usize, operator_factory: &OperatorFactory, test: F) -> Parse
    where F: Fn(&mut Parser)
{
    let lexemes: Vec<_> = Lexer::new(input).collect();

    let parser = Parser::new(&lexemes, operator_factory);

    let events = parser.parse_test(test);

    let sink = Sink::new(events, &lexemes, file);

    let (root, comments) = sink.finish(debugger);

    Parse { file,
            root: SyntaxNode::new_root(root),
            comments }
}
