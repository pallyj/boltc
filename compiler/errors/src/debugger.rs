use colored::Colorize;

use crate::{error::{Error, ErrorCode},
            fileinterner::FileInterner,
            Span};

pub struct Debugger<'a> {
    errors:        Vec<Error>,
    parser_errors: usize,
    interner:      &'a FileInterner,
}

impl<'a> Debugger<'a> {
    pub fn new(interner: &'a FileInterner) -> Self {
        Debugger { errors: vec![],
                   parser_errors: 0,
                   interner }
    }

    pub fn throw_parse(&mut self, error: String, span: (usize, (usize, usize))) {
        eprintln!("{}", format!("{}: {error}", "  error".red()).bold());

        let line_info = self.interner.get_line_info(span.0, span.1 .0);

        eprintln!("    {} {}:{}:{}",
                 "-->".bold().blue(),
                 line_info.filename,
                 line_info.line,
                 line_info.col + 1);

        eprintln!("     {} ", "|".bold().blue());
        eprintln!("{:>4} {} {}",
                 line_info.line.to_string().bold().blue(),
                 "|".bold().blue(),
                 line_info.text.replace("\t", "    ").trim_end());

        let ntabs = (&line_info.text[0..line_info.col]).rmatches("\t").count();

        let width = line_info.col + (3 * ntabs);

        let sep = (0..(span.1 .1 - span.1 .0)).map(|_| '^')
                                              .collect::<String>()
                                              .red()
                                              .bold();

        eprintln!("     {:width$}  {sep}", "|".bold().blue());

        eprintln!("     {} ", "|".bold().blue());

        self.parser_errors += 1;
    }

    pub fn throw(&mut self, code: ErrorCode, spans: Vec<Span>) {
        let description = code.description();
        eprintln!("  {}",
                 format!("[{}] {}", code.error_code().red(), description).bold());

        for span in spans.iter() {
            let (mut start, end): (u32, u32) = unsafe { std::mem::transmute(span.range) };

            let mut line_info = self.interner
                                    .get_line_info(span.file as usize, start as usize);

            eprintln!("    {} {}:{}:{}",
                     "-->".bold().blue(),
                     line_info.filename,
                     line_info.line,
                     line_info.col + 1);

            eprintln!("     {} ", "|".bold().blue());
            while start < end {
                eprintln!("{:>4} {} {}",
                         line_info.line.to_string().bold().blue(),
                         "|".bold().blue(),
                         line_info.text.replace("\t", "    ").trim_end());
    
                let ntabs = (&line_info.text[0..line_info.col]).rmatches("\t").count();
    
                let width = line_info.col + (3 * ntabs);

                let nwidth = u32::min(end - start, (line_info.text.len() - line_info.col) as u32);
    
                let sep = (0..(nwidth)).map(|_| '^')
                                            .collect::<String>()
                                            .red()
                                            .bold();
                eprintln!("     {} {space:width$}{sep}", "|".bold().blue(), space = "");

                start += line_info.text.len() as u32;
                let old_line = line_info.line;
                line_info = self.interner
                                .get_line_info(span.file as usize, start as usize + 1);
                if old_line == line_info.line {
                    break;
                }
            }
            eprintln!();
        }

        self.errors.push(Error::new(code, spans));
    }

    pub fn throw_single(&mut self, code: ErrorCode, span: &Option<Span>) { self.throw(code, span.clone().into_iter().collect()); }

    pub fn has_errors(&self) -> bool { self.errors.len() > 0 || self.parser_errors > 0 }
}
