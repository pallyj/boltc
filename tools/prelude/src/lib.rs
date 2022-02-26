mod error;
pub mod file_error;
mod source;
mod char_exts;
mod char_iter;
mod parser;
mod lexer;
mod token;
mod error_ctx;

pub use error::*;
pub use source::*;
pub use char_exts::*;
pub use char_iter::*;
pub use parser::*;
pub use lexer::*;
pub use token::*;
pub use error_ctx::*;

use std::{sync::Arc, fs::File, io::Read, collections::BTreeSet, path::Path};
use file_error::FileError;

pub struct SourceFile {
    file_name: String,

    code: Vec<char>,

    line_breaks: BTreeSet<usize>,
}

impl SourceFile {
    /// Opens a source file at the destination
    pub fn open_file(path: &Path) -> Result<Arc<SourceFile>, FileError> {
        let file_path_str = path.as_os_str().to_str().unwrap().to_string();

        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(FileError::FileNotFound(file_path_str));
            }
        };

        // Reads the code from the file
        let mut code = String::new();
        match file.read_to_string(&mut code) {
            Err(_) => {
                return Err(FileError::CantAccess(file_path_str));
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
            file_name: file_path_str,

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


        let slice = self.code[(line_start)..line_end].iter().collect::<String>();

        Some(slice)
    }

    pub fn index_of_line(&self, idx: usize, len: usize) -> Option<usize> {
        if (idx + len) >= self.code.len() {
            return None;
        }

        let line_start = self.line_breaks.range(..idx)
            .next_back()
            .map(|idx| *idx)
            .unwrap_or(0);

        Some(idx - line_start)
    }

    pub fn anon(self: &Arc<Self>) -> Source {
        Source { file: self.clone(), char_index: 0, len: 0 }
    }
}