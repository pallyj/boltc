use std::{collections::BTreeSet, io::Read};

pub struct FileInterner {
    files: Vec<File>,
}

impl FileInterner {
    pub fn new() -> FileInterner { FileInterner { files: vec![] } }

    pub fn get_line_info<'a>(&'a self, file: usize, pos: usize) -> LineInfo<'a> { self.files[file].get_line_info(pos) }

    pub fn open_file(&mut self, path: &str) {
        let mut file = std::fs::File::open(path).unwrap();

        let mut code = String::new();
        file.read_to_string(&mut code).unwrap();

        let file = File::new(path.to_string(), code);

        self.files.push(file);
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &File)> { self.files.iter().enumerate() }
}

pub struct File {
    text:        String,
    file_name:   String,
    line_breaks: BTreeSet<usize>,
}

impl File {
    pub fn new(file_name: String, text: String) -> File {
        let mut line_breaks = BTreeSet::new();

        line_breaks.insert(0);

        for (i, c) in text.char_indices() {
            if c == '\n' || c == '\r' {
                line_breaks.insert(i + 1);
            }
        }

        line_breaks.insert(text.len());

        File { text,
               line_breaks,
               file_name }
    }

    pub fn get_line_info<'a>(&'a self, n: usize) -> LineInfo {
        let last_break = self.line_breaks
                             .range(..=n)
                             .next_back()
                             .cloned()
                             .unwrap_or(0);

        let next_break = self.line_breaks
                             .range(n..)
                             .next()
                             .cloned()
                             .unwrap_or_else(|| self.text.len());

        // let next_break = if next_break == 0 { 0 } else { next_break - 1 };

        let line = self.line_breaks.range(..n).count();

        // if next_break

        let text = &self.text[last_break..next_break];

        let col = n - last_break;

        let filename = &self.file_name;

        LineInfo { line,
                   col,
                   text,
                   filename }
    }

    pub fn text(&self) -> &str { &self.text }
}

pub struct LineInfo<'a> {
    pub line:     usize,
    pub col:      usize,
    pub text:     &'a str,
    pub filename: &'a str,
}
