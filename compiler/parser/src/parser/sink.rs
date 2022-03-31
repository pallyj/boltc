use colored::Colorize;
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

        self.eat_trivia();

        for idx in 0..self.events.len() {
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

                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            std::mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(BoltLanguage::kind_to_raw(kind));
                    }
                }
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(error) => {
                    let description = error.to_string(self.peek());
                    println!("{}{} {}", "  error".red().bold(), ":".bold(), description.bold());
                }
                Event::Placeholder => {}
            }

            self.eat_trivia();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(BoltLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn peek(&self) -> Option<Token> {
        self.lexemes.get(self.cursor).cloned()
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