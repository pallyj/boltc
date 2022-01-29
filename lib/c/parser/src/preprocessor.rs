use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::sync::Arc;

use prelude::{CharIter, CharExts, WithSource, HasSource, SourceFile, Source, GenericLexer};

use crate::{is_ident_body, Token, LexError};
use crate::lexer::*;

// TODO: Add recursion
// TODO: Add if equals
// TODO: Add else if
// TODO: Add spaces before macro
pub struct Preprocessor {
	macros: HashMap<String, Macro>,
	pub tokens: Vec<WithSource<Token>>,
	pub file: Arc<SourceFile>,
	whitespaces: HashSet<usize>,
}

impl Preprocessor {
	pub fn new(file: Arc<SourceFile>, whitespaces: HashSet<usize>) -> Preprocessor {
		Preprocessor {
			macros: HashMap::new(),
			tokens: vec![],
			file,
			whitespaces,
		}
	}
	pub fn process<T: Iterator<Item=WithSource<Token>>>(&mut self, mut tokens: T) {
		let mut peekable = tokens.peekable();

		while let Some(tok) = peekable.next() {
			match tok.value() {
				Token::Directive(dir) if dir.as_str() == "define" => {
					self.lex_define(&mut peekable);
				}
				Token::Directive(dir) if dir.as_str() == "ifdef" => {
					self.lex_ifdef(&mut peekable);
				}
				Token::Directive(dir) if dir.as_str() == "ifndef" => {
					self.lex_ifndef(&mut peekable);
				}
				Token::Directive(dir) if dir.as_str() == "elseif" || dir.as_str() == "else" => {
					while peekable.next().map(|t| t.value() != &Token::Directive("endif".to_string())).unwrap_or(false) {}
				}
				Token::Ident(name) if self.has_macro(name) => {
					self.expand_macro(&mut peekable, name, tok.source());
				}
				Token::Directive(dir) if dir.as_str() == "endif" => {}
				_ => { self.tokens.push(tok) }
			}
		}
	}

	pub fn lex_define<T: Iterator<Item=WithSource<Token>>>(&mut self, tokens: &mut Peekable<T>) {

		let name = match tokens.next().map(|t| t.unwrap().0) {
			Some(Token::Ident(n)) => n,
			_ => { /* Emit error */ return }
		};

		let mut pars: Vec<String> = vec![];

		if tokens.next_if(|t| t.value() == &Token::Symbol("(".to_string())).is_some() {
			while tokens.next_if(|t| t.value() == &Token::Symbol(")".to_string())).is_none() {
				let par_name = match tokens.next().map(|t| t.unwrap().0) {
					Some(Token::Ident(n)) => n,
					_ => { /* Emit error */ return }
				};

				pars.push(par_name);

				tokens.next_if(|t| t.value() == &Token::Symbol(",".to_string()));
			}
		}

		let mut token_col = vec![];

		while let Some(next) = tokens.next() {
			if next.value() == &Token::Symbol("\\".to_string()) {
				break;
			} if let Token::Ident(id) = next.value() {
				if let Some(idx) = pars.iter().position(|x| x == id) {
					token_col.push(Token::MacroReplace(idx));
				} else {
					token_col.push(next.value().clone());
				}
			} else {
				token_col.push(next.value().clone());
			}
		}

		self.macros.insert(name, Macro {
			params: pars,
			replace_seq: token_col,
		});
	}

	pub fn lex_ifdef<T: Iterator<Item=WithSource<Token>>>(&mut self, tokens: &mut Peekable<T>) {
		let name = match tokens.next().map(|t| t.unwrap().0) {
			Some(Token::Ident(n)) => n,
			_ => { /* Emit error */ return } // TODO: If it returns, it does nothing
		};

		self.lex_if(tokens, self.has_macro(&name))
	}

	pub fn lex_ifndef<T: Iterator<Item=WithSource<Token>>>(&mut self, tokens: &mut Peekable<T>) {
		let name = match tokens.next().map(|t| t.unwrap().0) {
			Some(Token::Ident(n)) => n,
			_ => { /* Emit error */ return } // TODO: If it returns, it does nothing
		};

		self.lex_if(tokens, !self.has_macro(&name))
	}

	pub fn expand_macro<T: Iterator<Item=WithSource<Token>>>(&mut self, tokens: &mut Peekable<T>, macro_name: &str, source: &Source) {
		let mac = &self.macros[macro_name];

		if mac.params.len() == 0 {
			self.tokens.extend(mac.replace_seq.iter().map(|t| t.clone().with_source(source.clone())));
			return;
		}

		if !tokens.next_if(|t| t.value() == &Token::Symbol("(".to_string())).is_some() {
			// Error
			println!("Next is not (");
			return;
		}

		let mut pars = vec![];

		'outer: while tokens.next_if(|t| t.value() == &Token::Symbol(")".to_string())).is_none() {
			let mut toks = vec![];

			while tokens.next_if(|t| t.value() == &Token::Symbol(",".to_string())).is_none() {
				if tokens.next_if(|t| t.value() == &Token::Symbol(")".to_string())).is_some() {
					pars.push(toks);
					break 'outer;
				} else if let Some(next) = tokens.next() {
					toks.push(next);
				} else {
					// Error
					println!("Ran out");
					return;
				}
			}

			pars.push(toks)
		}

		for t in self.macros[macro_name].replace_seq.iter() {
			if let Token::MacroReplace(idx) = t {
				// TODO: Check if idx is in range
				self.tokens.extend(pars[*idx].iter().map(|p| p.clone()));
			} else {
				self.tokens.push(t.clone().with_source(source.clone()));
			}
		}
	}

	pub fn lex_if<T: Iterator<Item=WithSource<Token>>>(&mut self, tokens: &mut Peekable<T>, success: bool) {
		if success {
			// No elsifs work
			// If we get elseif, eat tokens
		} else {
			loop {
				let next = tokens.next();

				let Some(next) = next else {
					break;
				};

				if next.value() == &Token::Directive("endif".to_string()) {
					return
				} else if next.value() == &Token::Directive("else".to_string()) {
					return
				} else if next.value() == &Token::Directive("elif".to_string()) {
					self.lex_elif(tokens);
					return
				}
			}
		}
	}

	pub fn lex_elif<T: Iterator<Item=WithSource<Token>>>(&mut self, tokens: &mut Peekable<T>) {
		/*if success {
			// No elsifs work
			// If we get elseif, eat tokens
		} else {
			loop {
				let next = tokens.next();

				let Some(next) = next else {
					break;
				};

				if next.value() == &Token::Directive("endif".to_string()) {
					return
				} else if next.value() == &Token::Directive("else".to_string()) {
					return
				} else if next.value() == &Token::Directive("elif".to_string()) {
					return // Idk
				}
			}
		}*/
	}

	fn lex(&mut self, iter: &mut CharIter<'_>) -> Option<Token> {
		// TODO: Add is at end
		while iter.peek().is_some() {
			if let Some(token) = self.lex_next(iter) {
				return Some(token)
			}
		}
		None
	}


	fn lex_next(&mut self, iter: &mut CharIter<'_>) -> Option<Token> {
		let c = iter.next()?;
	
		if is_ident_head(c) {
			Some(self.lex_ident( iter, c))
			// Definitions
		} else if c == '0' {
			Some(lex_non_decimal(iter))
		} else if c.is_decimal_digit() {
			Some(lex_number(iter, c))
		} else if c == '"' {
			Some(lex_string(iter))
		} else if c == '/' {
			match iter.peek() {
				Some('/') => { iter.advance(); iter.advance_while(|c| !c.is_newline()); None },
				Some('*') => { iter.advance(); lex_long_comment(iter) },
				_ => { Some(Token::Operator("/".to_string())) }
			}
		} else if SYMBOLS.contains(&c) {
			Some(Token::Symbol(c.to_string()))
		} else if OP_CHARS.contains(&c) {
			Some(lex_op(iter, c))
		}
		else {
			None
		}
	}

	fn lex_ident(&mut self, iter: &mut CharIter<'_>, head: char) -> Token {
		let ident = format!("{}{}", head, iter.advance_collect(is_ident_body));
	
		if KEYWORDS.contains(&ident.as_str()) {
			Token::Keyword(ident)
		} else if self.has_macro(ident.as_str()) {
			Token::Ident("".to_string())
		} else {
			Token::Ident(ident)
		}
	}

	pub fn has_macro(&self, macro_name: &str) -> bool {
		self.macros.contains_key(macro_name)
	}

	/*pub fn expand_define(&mut self, macro_name: &str, iter: &mut CharIter<'_>) -> Vec<Token> {
		if let Some(mac) = self.macros.get(macro_name) {
			mac.replace_seq
				.iter()
				.filter_map(|mt| -> Option<Token> {
					match mt {
						MacroToken::Normal(tok) => Some(tok.clone()),
						MacroToken::Variable(var) => None,
					}
				}).collect()
		} else {
			vec![Token::Ident(macro_name.to_string())]
		}
	}*/
}


pub struct Macro {
	params: Vec<String>,
	replace_seq: Vec<Token>,
}

impl GenericLexer for Preprocessor {
    type Token = Token;

    type Error = LexError;

    fn into(self) -> (Arc<SourceFile>, Vec<WithSource<Self::Token>>, HashSet<usize>) {
        (self.file, self.tokens, self.whitespaces)
    }

    fn errors(&self) -> Option<Vec<Self::Error>> {
        todo!()
    }
}