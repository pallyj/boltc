use serde::{Serialize, Deserialize};
use std::collections::HashMap;

mod function;
mod typ;
mod typealias;

pub use typ::*;
pub use function::*;
pub use typealias::*;

#[derive(Serialize, Deserialize)]
pub struct Header {
    /// The headerfile version
    header_version: String,

    /// The compiler used to make the header
    compiler: String,

    /// The version of the compiler used to make the header
    compiler_version: String,

    /// The name of the library
    library: String,

    /// Metadata for the header
    /// 
    /// Common metadata include
    /// `author` - 
    /// `version` - 
    /// `description` -
    /// 
    metadata: HashMap<String, String>,

    /// The files contained in this header
    /// 
    /// Used for errors
    files: HashMap<u64, String>,

    /// Libraries this header depends on
    depends_on: Vec<Dependency>,

    /// Type aliases defined in the header
    aliases: HashMap<String, TypeAlias>,

    /// Functions defined in the header
    functions: HashMap<String, Function>,
}

#[derive(Serialize, Deserialize)]
pub struct Dependency {
    library_name: String,
    version: String,
}

#[derive(Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Internal,
    Fileprivate,
    Private
}