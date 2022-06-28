use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    Public = 50,
    Internal = 40,
    Fileprivate = 30,
    Private = 20,
    Local = 10,
}

impl Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Internal => write!(f, "internal"),
            Self::Fileprivate => write!(f, "public"),
            Self::Private => write!(f, "private"),
            Self::Local => write!(f, "local"),
        }
    }
}
