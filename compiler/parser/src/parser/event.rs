use crate::lexer::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event<'a> {
    StartNode {
        kind:           SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken {
        kind: SyntaxKind,
        text: &'a str,
    },
    FinishNode,
    Error(String),
    Placeholder,
}
