use std::{sync::atomic::{AtomicU64, Ordering}, cell::RefCell};

use colored::{Colorize, ColoredString};

use crate::{Span, fileinterner::{FileInterner, LineInfo}};

///
/// How serious a diagnostic is
/// 
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiagnosticLevel {
	///
	/// Halts compilation because of an irrecoverable error
	/// 
	Error,

	///
	/// Warns the developer about potentially dangerous code
	/// 
	Warning,

	///
	/// Provides info about a section of code
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

impl CodeLocation {
	pub fn new(span: Span, suggestion: Option<String>) -> CodeLocation {
		Self { suggestion, span }
	}

	///
	/// A suggestion to fix the code at this location
	/// 
	pub fn suggestion(&self) -> &Option<String> {
		&self.suggestion
	}

	///
	/// Where the error occurred
	/// 
	pub fn span(&self) -> Span {
		self.span
	}
}

///
/// Represents a diagnostic message
/// 
pub struct Diagnostic {
	level: DiagnosticLevel,
	error_code: &'static str,
	error_message: String,
	locations: Vec<CodeLocation>
}

impl Diagnostic {
	pub fn new(level: DiagnosticLevel,
			   error_code: &'static str,
			   error_message: String,
			   locations: Vec<CodeLocation>) -> Diagnostic
	{
		Diagnostic {
			level,
			error_code,
			error_message,
			locations }
	}

	/// 
	/// How severe the diagnostic is
	/// 
	/// `Error` - A fatal error
	/// `Warning` - An annoyance
	/// `Info` - A hint
	/// 
	pub fn level(&self) -> DiagnosticLevel {
		self.level
	}

	///
	/// The message thrown with the error
	/// 
	pub fn message(&self) -> &String {
		&self.error_message
	}

	///
	/// The locations wheter the error took place
	/// 
	pub fn locations(&self) -> &Vec<CodeLocation> {
		&self.locations
	}
}

///
/// An error type that can be turned into a diagnostic
/// 
pub trait IntoDiagnostic {
	///
	/// Converts an error into a diagnostic
	/// 
	fn into_diagnostic(self) -> Diagnostic;
}

///
/// Reports diagnostics
/// 
pub struct DiagnosticReporter<'a> {
	interner: 		 &'a FileInterner,
	n_errors_thrown: AtomicU64,
	throws:   		 bool,
	messages: 		 RefCell<Vec<Diagnostic>>
}

impl<'a> DiagnosticReporter<'a> {
	///
	/// Creates a global diagnostic reporter for a project
	/// 
	pub fn new(interner: &'a FileInterner) -> Self {
		Self { interner,
			   n_errors_thrown: AtomicU64::new(0),
			   throws: true,
			   messages: RefCell::new(Vec::new()) }
	}

	///
	/// Creates a diagnostic reporter for a project. This reporter can store errors.
	/// 
	pub fn new_stores(interner: &'a FileInterner) -> Self {
		Self { interner,
			   n_errors_thrown: AtomicU64::new(0),
			   throws: false,
			   messages: RefCell::new(Vec::new()) }
	}

	///
	/// Throws an error
	/// 
	pub fn throw_diagnostic<T: IntoDiagnostic>(&self, diagnostic: T) {
		use DiagnosticLevel::*;

		let diagnostic = IntoDiagnostic::into_diagnostic(diagnostic);

		if self.throws {
			match diagnostic.level {
				Error => {
					self.n_errors_thrown.fetch_add(1, Ordering::Relaxed);
					eprintln!("{}: {}", diagnostic.error_code.red().bold(), diagnostic.error_message.bold())
				}
				Warning => eprintln!("{}: {}", "warning".yellow().bold(), diagnostic.error_message.bold()),
				Info => eprintln!("{}: {}", diagnostic.error_code.blue().bold(), diagnostic.error_message.bold()),
			}

			let mut last_file = u32::MAX;

			for loc in diagnostic.locations {
				self.print_span(&loc, diagnostic.level, last_file == loc.span.file);

				last_file = loc.span.file;
			}

			eprintln!();
		} else {
			if let Error = diagnostic.level {
				self.n_errors_thrown.fetch_add(1, Ordering::Relaxed);
			}

			self.messages.borrow_mut().push(diagnostic);
		}
	}

	///
	/// Prints a span of a diagnostic
	/// 
	pub (crate) fn print_span(&self, loc: &CodeLocation, level: DiagnosticLevel, combine_with_last: bool) {
		let (start, end): (u32, u32) = unsafe { std::mem::transmute(loc.span.range) };

		let line_info = self.interner.get_line_info(loc.span.file as usize, start as usize);

		if loc.span.file == 0
		   && start == 0
		   && end == 0
		{
			return;
		}

		// --> filename:line:col

		if !combine_with_last {
			eprintln!("    {} {}:{}:{}",
					"-->".bold().blue(),
					line_info.filename,
					line_info.line,
					line_info.col + 1);
		}

		eprintln!("     {}", "|".bold().blue());
				
		// num | line_text
		eprintln!("{:>4} {} {}",
				 line_info.line.to_string().bold().blue(),
				 "|".bold().blue(),
				 line_info.text.replace("\t", "    ").trim_end());

		let selection_len = end - start;
		let line_remaining_len = line_info.text.trim_end().len() - line_info.col + 1;
		let visible_selection_len = line_remaining_len.min(selection_len as usize);

		let selector = std::iter::repeat("^")
			.take(visible_selection_len)
			.collect::<String>();

		let leading_tabs = line_info.text[0..line_info.col].rmatches("\t").count();
		let leading_offset = line_info.col + 3 * leading_tabs;

		let colorify: fn(ColoredString) -> ColoredString = match level {
			DiagnosticLevel::Error => Colorize::red,
			DiagnosticLevel::Warning => Colorize::yellow,
			DiagnosticLevel::Info => Colorize::blue,
		};


		print!("     {} {space:width$}{}",
			   "|".bold().blue(),
			   colorify(selector.bold()),
			   space = "",
			   width = leading_offset);

		if let Some(suggestion) = &loc.suggestion {
			eprintln!(" {}",  colorify(suggestion.bold()));
		} else {
			eprintln!()
		}
	}
	
	pub fn errors(&self) -> Result<(), ()> {
		if self.n_errors_thrown.load(Ordering::Relaxed) > 0 {
			Err(())
		} else {
			Ok(())
		}
	}

	pub fn diagnostics(self) -> Vec<Diagnostic> {
		self.messages.into_inner()
	}

	pub fn lookup(&self, span: Span) -> LineInfo {
		self.interner.get_line_info(span.file as usize, span.range.start().into())
	}
}