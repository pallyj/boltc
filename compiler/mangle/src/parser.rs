use std::iter::Peekable;

use crate::symbol::SymbolKind;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub (crate) enum SymbolKindPrefix {
	Library(usize),
	Protocol(usize),
	Class(usize),
	Struct(usize),
	Enum(usize),
	Function(usize),
	Initializer(usize),
	Operator(usize),
	Variable(usize),

	Intrinsic(usize),
	Generic(usize),
}

impl SymbolKindPrefix {
	pub fn parse_list<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<Vec<SymbolKindPrefix>> {
		let len = Self::parse_length(iter)?;

		let mut prefixes = vec![];

		for _ in 0..len {
			prefixes.push(Self::parse(iter)?);
		}

		Some(prefixes)
	}

	pub fn parse<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<SymbolKindPrefix> {
		if let Some(prefix_ident) = iter.next() {
			let n = Self::parse_length(iter)?;

			Some(match prefix_ident {
				'L' => Self::Library(n),
				'P' => Self::Protocol(n),
				'C' => Self::Class(n),
				'S' => Self::Struct(n),
				'E' => Self::Enum(n),
				'F' => Self::Function(n),
				'I' => Self::Initializer(n),
				'O' => Self::Operator(n),
				'V' => Self::Variable(n),
				'i' => Self::Intrinsic(n),
				'g' => Self::Generic(n),
				_ => return None,
			})
		} else {
			return None;
		}
	}

	fn parse_length<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> Option<usize> {
		let mut n: usize = 0;

		while let Some(c) = iter.next_if(|c| c.is_digit(10)) {
			let digit = c.to_digit(10).unwrap() as usize;

			n = n * 10 + digit;

			if n > (u16::MAX as usize) {
				return None;
			}
		}

		Some(n)
	}

	pub fn len(&self) -> usize {
		match self {
			Self::Library(n) => *n,
			Self::Protocol(n) => *n,
			Self::Class(n) => *n,
			Self::Struct(n) => *n,
			Self::Enum(n) => *n,
			Self::Function(n) => *n,
			Self::Initializer(n) => *n,
			Self::Operator(n) => *n,
			Self::Variable(n) => *n,
			Self::Intrinsic(n) => *n,
			Self::Generic(n) => *n,
		}
	}

	pub fn read_from<I: Iterator<Item = char>>(self, iter: &mut Peekable<I>) -> Option<SymbolKind> {
		let len = self.len();

		let ident = iter
			.take(len)
			.collect::<String>();

		Some(match self {
			Self::Library(_) => SymbolKind::Library(ident),
			Self::Protocol(_) => SymbolKind::Protocol(ident),
			Self::Class(_) => SymbolKind::Class(ident),
			Self::Struct(_) => SymbolKind::Struct(ident),
			Self::Enum(_) => SymbolKind::Enum(ident),
			Self::Function(_) => SymbolKind::Function(ident),
			Self::Initializer(_) => SymbolKind::Initializer(ident),
			Self::Operator(_) => SymbolKind::Operator(ident),
			Self::Variable(_) => SymbolKind::Variable(ident),
			Self::Intrinsic(_) => SymbolKind::Intrinsic(ident),
			Self::Generic(_) => SymbolKind::Generic(ident),
		})
	}
}