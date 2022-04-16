use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Internal,
    Fileprivate,
    Private,
    Local,
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
