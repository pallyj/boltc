use crate::{error::{Error, ErrorCode}, Span, fileinterner::FileInterner};
use colored::Colorize;

pub struct Debugger<'a> {
	errors: Vec<Error>,
	parser_errors: usize,
	interner: &'a FileInterner,
	
}

impl<'a> Debugger<'a> {
	pub fn new(interner: &'a FileInterner) -> Self {
		Debugger { errors: vec![], parser_errors: 0, interner }
	}

	pub fn throw_parse(&mut self, error: String, span: (usize, (usize, usize))) {
		println!("{}", format!("{}: {error}", "  error".red()).bold());

		let line_info = self.interner.get_line_info(span.0, span.1.0);

		println!("    {} {}:{}:{}", "-->".bold().blue(), line_info.filename, line_info.line, line_info.col + 1);

		println!("     {} ", "|".bold().blue());
		println!("{:>4} {} {}", line_info.line.to_string().bold().blue(), "|".bold().blue(), line_info.text);

		let sep = (0..(span.1.1 - span.1.0)).map(|_| '^').collect::<String>().red().bold();

		println!("    {} {:width$}{sep}", "", "|".bold().blue(), width = line_info.col + 2);

		println!("     {} ", "|".bold().blue());

		self.parser_errors += 1;
	}

	pub fn throw(&mut self, code: ErrorCode, spans: Vec<Span>) {
		println!("{}", code.error_code().red());

		self.errors.push(Error::new(code, spans));
	}

	pub fn has_errors(&self) -> bool {
		self.errors.len() > 0 || self.parser_errors > 0
	}
}