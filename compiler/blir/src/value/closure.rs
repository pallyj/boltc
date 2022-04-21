use crate::{code::CodeBlock, typ::Type};

#[derive(Clone)]
pub struct Closure {
    pub params: Vec<ClosureParam>,

    pub code: CodeBlock,
}

#[derive(Clone)]
pub struct ClosureParam {
    pub name: String,

    pub typ: Type,
}
