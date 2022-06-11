use errors::debugger::Debugger;
use rowan::{GreenNode, GreenNodeBuilder, Language};

use super::event::Event;
use crate::{ast::BoltLanguage,
            lexer::{SyntaxKind, Token}};

pub(super) struct Sink<'input, 'l> {
    builder:     GreenNodeBuilder<'static>,
    lexemes:     &'l [Token<'input>],
    cursor:      usize,
    events:      Vec<Event<'input>>,
    text_cursor: usize,
    file:        usize,
}

impl<'input, 'l> Sink<'input, 'l> {
    pub(super) fn new(events: Vec<Event<'input>>, lexemes: &'l [Token<'input>], file: usize) -> Self {
        Self { builder: GreenNodeBuilder::new(),
               lexemes,
               cursor: 0,
               events,
               text_cursor: 0,
               file }
    }

    pub(super) fn finish(mut self, debugger: &mut Debugger) -> GreenNode {
        for idx in 0..self.events.len() {
            // self.eat_trivia();
            let event = std::mem::replace(&mut self.events[idx], Event::Placeholder);
            match event {
                Event::StartNode { kind, forward_parent } => {
                    let mut kinds = vec![kind];

                    let mut idx = idx;
                    let mut forward_parent = forward_parent;

                    // Walk through the forward parent of the forward parent, and the forward parent
                    // of that, and of that, etc. until we reach a StartNode event without a forward
                    // parent.
                    while let Some(fp) = forward_parent {
                        idx += fp;

                        forward_parent = if let Event::StartNode { kind, forward_parent } = std::mem::replace(&mut self.events[idx], Event::Placeholder) {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    if kinds.first().map(|first| *first != SyntaxKind::Root).unwrap_or(true) {
                        self.eat_trivia();
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(BoltLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken { kind, text } => {
                    self.eat_trivia();
                    self.token(kind, text)
                }
                Event::FinishNode => {
                    self.builder.finish_node();
                    self.eat_trivia();
                }
                Event::Error(error) => {
                    self.eat_trivia();
                    let description = format!("{error}, found {}", token_specific(self.peek()));
                    let span = self.next_span();
                    debugger.throw_parse(description, (self.file, span));
                }
                Event::Placeholder => {}
            }
        }

        self.builder.finish()
    }

    fn next_span(&self) -> (usize, usize) {
        let sz = self.lexemes
                     .get(self.cursor)
                     .map(|lexeme| lexeme.source.len())
                     .unwrap_or(0);

        (self.text_cursor, self.text_cursor + sz)
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(BoltLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
        self.text_cursor += text.len();
    }

    fn peek(&self) -> Option<Token> { self.lexemes.get(self.cursor).cloned() }

    fn eat_trivia(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if !lexeme.kind.is_trivia() {
                break;
            }

            self.token(lexeme.kind, lexeme.source);
        }
    }
}

fn token_specific(token: Option<Token>) -> String {
    let Some(token) = token else {
		return "<eof>".to_string();
	};

    match &token.kind {
        SyntaxKind::StructKw |
        SyntaxKind::EnumKw |
        SyntaxKind::CaseKw |
        SyntaxKind::ImportKw |
        SyntaxKind::FuncKw |
        SyntaxKind::InitKw |
        SyntaxKind::LetKw |
        SyntaxKind::VarKw |
        SyntaxKind::IfKw |
        SyntaxKind::ElseKw |
        SyntaxKind::MatchKw |
        SyntaxKind::ReturnKw |
        SyntaxKind::StaticKw |
        SyntaxKind::PublicKw |
        SyntaxKind::InternalKw |
        SyntaxKind::FilePrivateKw |
        SyntaxKind::PrivateKw |
        SyntaxKind::UnderscoreKw |
        SyntaxKind::OperatorKw |
        SyntaxKind::TypeAliasKw |
        SyntaxKind::MutatingKw |
        SyntaxKind::SharedKw |
        SyntaxKind::BreakKw |
        SyntaxKind::ContinueKw |
        SyntaxKind::WhileKw |
        SyntaxKind::RepeatKw |
        SyntaxKind::GuardKw => format!("keyword `{}`", token.source),

        SyntaxKind::Comment => "comment".to_string(),
        SyntaxKind::Whitespace => "whitespace".to_string(),
        SyntaxKind::Error => "error".to_string(),

        _ => format!("`{}`", token.source),
    }
}
