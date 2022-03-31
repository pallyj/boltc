use drop_bomb::DropBomb;

use crate::{lexer::SyntaxKind, parser::event::Event};

use super::Parser;

pub struct Marker {
	pos: usize,
	bomb: DropBomb,
}

impl Marker {
	pub (super) fn new(pos: usize) -> Marker {
		Marker {
			pos,
			bomb: DropBomb::new("Compiler Error: Marker wasn't completed"),
		}
	}

	pub (super) fn complete(mut self, parser: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
		self.bomb.defuse();

		let event_at_pos = &mut parser.events[self.pos];
		assert_eq!(*event_at_pos, Event::Placeholder);

		*event_at_pos = Event::StartNode { kind, forward_parent: None };
		parser.events.push(Event::FinishNode);

		CompletedMarker { pos: self.pos }
	}
}

pub struct CompletedMarker {
	pos: usize
}

impl CompletedMarker {
	pub (super) fn precede(self, parser: &mut Parser) -> Marker {
		let new_marker = parser.start();

		let Event::StartNode { ref mut forward_parent, .. } = &mut parser.events[self.pos]
		else { unreachable!() };

        *forward_parent = Some(new_marker.pos - self.pos);

		new_marker

	}
}