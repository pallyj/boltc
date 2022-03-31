use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    // class, enum, case, protocol, extension, alias
    // guard, match
    // try, catch, throws, async, await
    // defer, throw
    // for, in, while, repeat
    // as
    // break, continue
    // get, set
    // mutating
    #[regex("struct")]
    StructKw,
    #[regex("import")]
    ImportKw,
    #[regex("func")]
    FuncKw,
    #[regex("init", priority = 3)]
    InitKw,

    #[regex("let", priority = 3)]
    LetKw,
    #[regex("var", priority = 3)]
    VarKw,

    #[regex("if", priority = 3)]
    IfKw,
    #[regex("else", priority = 3)]
    ElseKw,

    #[regex("return", priority = 3)]
    ReturnKw,

    #[regex("static", priority = 3)]
    StaticKw,

    #[regex("public", priority = 3)]
    PublicKw,
    #[regex("internal", priority = 3)]
    InternalKw,
    #[regex("fileprivate", priority = 3)]
    FilePrivateKw,
    #[regex("private", priority = 3)]
    PrivateKw,
    #[regex("_", priority = 3)]
    UnderscoreKw,


    #[regex("[a-zA-Z_$][a-zA-Z_$0-9]*", priority = 2)]
    Ident,

    #[regex("[+|-|*|/|%|<|>|&|\\||^|=|!|?|\\.]+")]
    Operator,

    #[regex("true")]
    LiteralTrue,

    #[regex("false")]
    LiteralFalse,

    #[regex("[0-9][0-9_]*")]
    LiteralDecInt,

    #[regex("0x[0-9A-Fa-f_]+")]
    LiteralHexInt,

    #[regex("0o[0-7_]+")]
    LiteralOctInt,

    #[regex("0b[0|1_]+")]
    LiteralBinInt,

    #[regex("[0-9][0-9_]*\\.[0-9_]*")]
    LiteralDecFloat,


    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token("`")]
    Backtick,

    #[token(".")]
    Period,

    #[token("=")]
    Equals,


    #[token("@")]
    At,

    #[regex("//.*")]
    #[regex(r#"/\*[^\*]*\*/"#)]
    Comment,

    #[regex(r"[ \n\r\f\t]")]
    Whitespace,


    #[error]
    Error,

    Root,
    NamedType,
    MemberType,
    UnitType,
    InferType,
    FuncType,

    FuncReturn,

    PrefixExpr,
    ParenthesizedExpr,
    IfExpr,
    UnitExpr,
    MemberExpr,
    FuncCallExpr,
    PostfixExpr,
    InfixExpr,
    NamedExpr,
    Literal,

    Condition,
    Positive,
    Negative,

    EvalSmt,
    ReturnSmt,
    LetSmt,
    BindType,
    AssignValue,
    NoOp,

    CodeBlock,

    FuncDef,
    InitDef,
    FuncPar,
    FuncName,

    VarDef,
    LetDef,

    StructDef,
    StructBody,

    Visibility,

    Import,
    

    CommaSeparatedList,


    _Invalid
}

impl SyntaxKind {
    pub (crate) fn is_trivia(self) -> bool {
        match self {
            SyntaxKind::Comment => true,
            SyntaxKind::Whitespace => true,
            _ => false
        }
    }
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Lexer {
            lexer: SyntaxKind::lexer(code)
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let source = self.lexer.slice();

        Some(Token {
            kind,
            source
        })
    }
}

#[derive(Clone)]
pub struct Token<'a> {
    pub kind: SyntaxKind,
    pub source: &'a str
}
