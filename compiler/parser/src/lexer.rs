use logos::Logos;

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {

    // 0.7
    //
    // get set

    // 0.8
    //
    // protocol
    // extension
    // alias

    // 0.9
    //
    // class as

    // 0.11
    //
    // throws try catch throw
    // for in

    // 2.0
    //
    // async await
    // actor

    // defer
    #[regex("struct")]
    StructKw,
    #[regex("enum")]
    EnumKw,
    #[regex("case")]
    CaseKw,
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
    #[regex("match")]
    MatchKw,

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
    #[regex("operator", priority = 3)]
    OperatorKw,
    #[regex("typealias", priority = 3)]
    TypeAliasKw,

    #[regex("mutating", priority = 3)]
    MutatingKw,
    #[regex("shared", priority = 3)]
    SharedKw,
    #[regex("break", priority = 3)]
    BreakKw,
    #[regex("continue", priority = 3)]
    ContinueKw,
    #[regex("while", priority = 3)]
    WhileKw,
    #[regex("repeat", priority = 3)]
    RepeatKw,
    #[regex("guard", priority = 3)]
    GuardKw,

    #[regex("[a-zA-Z_$][a-zA-Z_$0-9]*", priority = 2)]
    Ident,

    #[regex("`[a-zA-Z_$][a-zA-Z_$0-9]*")]
    Scope,

    #[regex("[+|\\-|*|/|%|<|>|&|\\||^|=|!|?|\\.|~]+")]
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

    #[regex("[0-9][0-9_]*\\.[0-9_]+")]
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
    #[token("=>")]
    BigArrow,
    #[token("->")]
    Arrow,

    #[token("@")]
    At,

    #[regex("//.*")]
    #[regex(r#"/\*"#, lex_long_comment)]
    Comment,

    #[regex(r"[ \n\r\f\t]")]
    Whitespace,

    #[regex(r#"""#, lex_string)]
    StringLiteral,

    #[regex(r#"""""#, lex_long_string, priority = 2)]
    LongStringLiteral,

    #[error]
    Error,

    Root,
    NamedType,
    MemberType,
    UnitType,
    InferType,
    FuncType,
    GenericType,
    ArrayType,
    SliceType,

    FuncReturn,

    PrefixExpr,
    ParenthesizedExpr,
    IfExpr,
    MatchExpr,
    MatchBranch,
    UnitExpr,
    MemberExpr,
    FuncCallExpr,
    PostfixExpr,
    InfixExpr,
    NamedExpr,
    Literal,
    IndexExpr,

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

    EnumDef,
    EnumBody,
    CaseDef,
    CaseItem,

    Visibility,

    Import,

    CommaSeparatedList,

    Attribute,
    Attributes,

    FuncArg,

    Closure,
    TrailingClosure,

    ParenthesizedType,
    TupleType,
    Tuple,

    VariantLiteral,
    ArrayLiteral,

    BindPattern,
    WildcardPattern,
    VariantPattern,
    TuplePattern,
    LiteralPattern,
    VaryingPattern,

    TypeAlias,

    RepeatLoop,
    WhileLoop,
    WhileLetLoop,

    IfLet,
    Guard,
    GuardLet,

    BreakSmt,
    ContinueSmt,

    Docs,

    MapItem,
    ArrayItem,

    Macro,

    _Invalid,
}

fn lex_string(lexer: &mut logos::Lexer<SyntaxKind>) -> Result<(), ()> {
    let remaining = lexer.remainder();

    let mut look_behind = [' '; 3];

    for (i, next_c) in remaining.chars().enumerate() {
        look_behind[0] = look_behind[1];
        look_behind[1] = look_behind[2];
        look_behind[2] = next_c;

        if look_behind[2] == '\"' {
            if !(look_behind[1] == '\\' && look_behind[0] != '\\') {
                // The string is over
                lexer.bump(i + 1);
                return Ok(());
            }
        }
    }

    // Throw an error
    return Err(())
}

fn lex_long_string(lexer: &mut logos::Lexer<SyntaxKind>) -> Result<(), ()> {
    let remaining = lexer.remainder();

    let mut look_behind = [' '; 3];

    for (i, next_c) in remaining.chars().enumerate() {
        look_behind[0] = look_behind[1];
        look_behind[1] = look_behind[2];
        look_behind[2] = next_c;

        if look_behind == ['"', '"', '"'] {
            // The string is over
            lexer.bump(i + 1);
            return Ok(());
        }
    }



    // Throw an error
    return Err(())
}

fn lex_long_comment(lexer: &mut logos::Lexer<SyntaxKind>) -> Result<(), ()> {
    let remaining = lexer.remainder();

    let mut look_behind = [' '; 2];
    let mut levels = 0;

    for (i, next_c) in remaining.chars().enumerate() {
        look_behind[0] = look_behind[1];
        look_behind[1] = next_c;

        if look_behind == ['*', '/'] {
            // The string is over
            if levels <= 0 {
                lexer.bump(i + 1);
                return Ok(());
            }
            levels -= 1;
        } else if look_behind == ['/', '*'] {
            levels += 1
        }
    }

    // Throw an error
    return Err(())
}

impl SyntaxKind {
    pub(crate) fn is_trivia(self) -> bool { matches!(self, SyntaxKind::Comment | SyntaxKind::Whitespace) }
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self { Lexer { lexer: SyntaxKind::lexer(code), } }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?;
        let source = self.lexer.slice();

        Some(Token { kind, source })
    }
}

#[derive(Clone)]
pub struct Token<'a> {
    pub kind:   SyntaxKind,
    pub source: &'a str,
}
