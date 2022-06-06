use colored::Colorize;

use crate::{Span, fileinterner::FileInterner};

///
/// 
/// 
pub enum Level {
	///
	/// Halts compilation
	/// 
	Error,

	///
	/// 
	/// 
	Warning,

	///
	/// 
	/// 
	Info
}

///
/// A location that causes an error, with a suggestion to fix that location
/// 
pub struct CodeLocation {
	pub (crate) suggestion: Option<String>,
	pub (crate) span: Span
}

///
/// 
/// 
pub struct Diagnostic {
	level: Level,
	error_code: String,
	error_message: String
}

///
/// 
/// 
pub struct DiagnosticReporter<'a> {
	interner: &'a FileInterner,
}

impl<'a> DiagnosticReporter<'a> {
	pub fn new(interner: &'a FileInterner) -> Self {
		Self { interner }
	}

	pub fn throw_diagnostic<T>() {

	}

	///
	/// 
	/// 
	pub (crate) fn print_span(&self, loc: &CodeLocation) {
		let (start, end): (u32, u32) = unsafe { std::mem::transmute(loc.span.range) };

		let line_info = self.interner.get_line_info(loc.span.file as usize, start as usize);

		// --> filename:line:col
		println!("    {} {}:{}:{}",
				 "-->".bold().blue(),
				 line_info.filename,
				 line_info.line,
				 line_info.col + 1);
		
		// num | line_text
		println!("{:>4} {} {}",
				 line_info.line.to_string().bold().blue(),
				 "|".bold().blue(),
				 line_info.text.replace("\t", "    ").trim_end());

		let selection_len = end - start;
		let line_remaining_len = line_info.text.trim_end().len() - line_info.col;
		let visible_selection_len = line_remaining_len.min(selection_len as usize);

		let selector = std::iter::repeat("^")
			.take(visible_selection_len)
			.collect::<String>();

		let leading_tabs = line_info.text[0..line_info.col].rmatches("\t").count();
		let leading_offset = line_info.col + 3 * leading_tabs;

		print!("     {}{space:width$}{}",
			   "|".bold().blue(),
			   selector.bold().red(),
			   space = "",
			   width = leading_offset);

		if let Some(suggestion) = &loc.suggestion {
			println!(" {}",  suggestion.bold().red());
		} else {
			println!()
		}
	}
	
}