pub mod error;
pub mod fileinterner;
mod diag;
#[cfg(test)]
mod tests;

pub use diag::*;
use rowan::{TextRange, TextSize};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Span {
    range: TextRange,
    file:  u32,
}

impl Span {
    pub fn empty() -> Span {
        Span { range: TextRange::empty(TextSize::from(0)),
               file:  0, }
    }

    pub fn new(range: TextRange, file: u32) -> Span { Span { range, file } }

    pub fn range(&self) -> TextRange { self.range }

    pub fn file_key(&self) -> u32 { self.file }
}

impl Default for Span {
    fn default() -> Self {
        Self::empty()
    }
}

// pub struct Debugger<T> {
// errors: Vec<T>,
// warnings: Vec<T>,
// }

// pub struct Record {
// x: m,
// y:
// }
//
// type Record = [String + Value];
//
// type Value =
// String |
// Int |
// Record;
//
// func print(rec: Record) =
// rec.forEach -> match {
// String str => println(str),
// Int i => println(i),
// Record r => print(r)
// }
//
// public class Record {
// var items: [(String, Value)]
//
// TODO: Use match as a function
// TODO: Enum variant binding can pipe to a function
// }
//
// let printJson = parseJson -> printRecord;
//
// func parseJson(s: String): Record
//
// func printRecord(r: Record) =
// r.items.forEach -> match {
// .string -> println,
// .int -> println,
// .record -> printRecord
// }
//
// public enum Value {
// string(String),
// int(Int),
// record(Record)
// }
//

// Function currying and combination
//
// func mul(a: Int, b: Int) = a * b
// func add(a: Int, b: Int) = a + b
//
// func double = mul(2, _)
// func addOne = add(1, _)
// func doublePlus1 = double -> addOne
//
// Named currying:
//
// func square = mul($1, $1)
//