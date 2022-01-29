mod error;
pub mod file_error;
mod source;
mod char_exts;
mod char_iter;
mod parser;
mod lexer;
mod token;

pub use error::*;
pub use source::*;
pub use char_exts::*;
pub use char_iter::*;
pub use parser::*;
pub use lexer::*;
pub use token::*;

use std::{sync::Arc, fs::File, io::Read, collections::BTreeSet};
use file_error::FileError;

pub struct SourceFile {
    file_name: String,

    code: Vec<char>,

    line_breaks: BTreeSet<usize>,
}

impl SourceFile {
    /// Opens a source file at the destination
    pub fn open_file(path: &str) -> Result<Arc<SourceFile>, FileError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(FileError::FileNotFound(path.to_string()))
            }
        };

        // Reads the code from the file
        let mut code = String::new();
        match file.read_to_string(&mut code) {
            Err(_) => {
                return Err(FileError::CantAccess(path.to_string()));
            }
            _ => {}
        }

        let code: Vec<char> = code.chars().collect();

        // Scan for lines (and whitespace?)
        let mut line_breaks = BTreeSet::new();

        for (i, c) in code.iter().enumerate() {
            if c.is_newline() {
                line_breaks.insert(i);
            }
        }
        
        Ok(Arc::new(SourceFile {
            file_name: path.to_string(),

            code,

            line_breaks
        }))
    }

    pub fn iter<'a>(self: &'a Arc<Self>) -> CharIter<'a> {
        CharIter {
            file: self.clone(),
            buffer: &self.code,
            idx: 0
        }
    }

    /// The name of the input file
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Gets the line number of the character at `idx`
    pub fn line(&self, idx: usize) -> usize {
        self.line_breaks.range(..idx).count() + 1
    }

    /// Gets the column number of the character at `idx`
    pub fn col(&self, idx: usize) -> usize {
        let line_start = self.line_breaks.range(..idx)
            .next_back()
            .map(|idx| *idx)
            .unwrap_or(0);

        (idx - line_start) + 1
    }

    /// Returns a slice of the file starting at `idx` extending `len` chars
    pub fn slice(&self, idx: usize, len: usize) -> Option<String> {
        if (idx + len) >= self.code.len() {
            return None;
        }

        let slice = self.code[idx..idx + len].iter().collect::<String>();

        Some(slice)
    }

    /// Returns a slice of the file starting at the line before `idx`,
    /// ending at the line after `idx` + `len`
    pub fn line_slice(&self, idx: usize, len: usize) -> Option<String> {
        if (idx + len) >= self.code.len() {
            return None;
        }

        let line_start = self.line_breaks.range(..idx)
            .next_back()
            .map(|idx| *idx)
            .unwrap_or(0);

        let line_end = self.line_breaks.range(idx..)
            .next()
            .map(|idx| *idx)
            .unwrap_or(self.code.len());


        let slice = self.code[(line_start + 1)..line_end].iter().collect::<String>();

        Some(slice)
    }

    pub fn anon(self: &Arc<Self>) -> Source {
        Source { file: self.clone(), char_index: 0, len: 0 }
    }
}