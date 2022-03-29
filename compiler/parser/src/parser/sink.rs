use rowan::{GreenNodeBuilder, GreenNode, Language};

use crate::{ast::BoltLanguage, lexer::{Token, SyntaxKind}};

use super::event::Event;

pub (super) struct Sink<'input, 'l> {
	builder: GreenNodeBuilder<'static>,
    lexemes: &'l [Token<'input>],
    cursor: usize,
	events: Vec<Event<'input>>
}

impl<'input, 'l> Sink<'input, 'l> {
	pub(super) fn new(events: Vec<Event<'input>>, lexemes: &'l [Token<'input>]) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            cursor: 0,
            events,
        }
    }

	pub(super) fn finish(mut self) -> GreenNode {
		let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, at } = event {
                reordered_events.remove(idx);
                reordered_events.insert(*at, Event::StartNode { kind: *kind });
            }
        }

        self.eat_trivia();

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(BoltLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt { kind, .. } => self
                    .builder
                    .start_node(BoltLanguage::kind_to_raw(kind)),
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
            }

            self.eat_trivia();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(BoltLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if !lexeme.kind.is_trivia() {
                break;
            }

            self.token(lexeme.kind, lexeme.source.into());
        }
    }
}