#![feature(let_else)]
//#[feature(let_chain)]

pub mod closure_resolve;
mod init_default;
pub mod type_check;
pub mod type_infer;
pub mod type_resolve;

pub use closure_resolve::ClosureResolvePass;
pub use type_check::TypeCheckPass;
pub use type_infer::TypeInferPass;
pub use type_resolve::TypeResolvePass;
