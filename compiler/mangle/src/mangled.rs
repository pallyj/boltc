use crate::parser::SymbolKindPrefix;

#[derive(Debug, Clone)]
pub enum MangleComponent {
    Library(String),
    Protocol(String),
    Class(String),
    Struct(String),
    Enum(String),
    Function(String),
    Initializer(String),
    Operator(String),
    Variable(String),

    Intrinsic(String),
    Generic(String),
}

impl MangleComponent {
    pub fn prefix(&self) -> String {
        match self {
            Self::Library(n) => format!("{}{}", 'L', n.len()),
            Self::Protocol(n) => format!("{}{}", 'P', n.len()),
            Self::Class(n) => format!("{}{}", 'C', n.len()),
            Self::Struct(n) => format!("{}{}", 'S', n.len()),
            Self::Enum(n) => format!("{}{}", 'E', n.len()),
            Self::Function(n) => format!("{}{}", 'F', n.len()),
            Self::Initializer(n) => format!("{}{}", 'I', n.len()),
            Self::Operator(n) => format!("{}{}", 'O', n.len()),
            Self::Variable(n) => format!("{}{}", 'V', n.len()),
            Self::Intrinsic(n) => format!("{}{}", 'i', n.len()),
            Self::Generic(n) => format!("{}{}", 'g', n.len()),
        }
    }

    pub fn name(&self) -> &String {
        match self {
            Self::Library(n) => n,
            Self::Protocol(n) => n,
            Self::Class(n) => n,
            Self::Struct(n) => n,
            Self::Enum(n) => n,
            Self::Function(n) => n,
            Self::Initializer(n) => n,
            Self::Operator(n) => n,
            Self::Variable(n) => n,
            Self::Intrinsic(n) => n,
            Self::Generic(n) => n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mangled {
    components: Vec<MangleComponent>,
}

impl Mangled {
    pub fn from_components(components: Vec<MangleComponent>) -> Mangled { Mangled { components } }

    pub fn new(component: MangleComponent) -> Mangled { Mangled { components: vec![component], } }

    pub fn append(mut self, component: MangleComponent) -> Self {
        self.components.push(component);
        self
    }

    pub fn mangle(&self) -> String {
        let count = self.components.len();

        let prefix = self.components
                         .iter()
                         .map(|com| com.prefix())
                         .collect::<Vec<_>>()
                         .join("");

        let string_table = self.components
                               .iter()
                               .map(|com| com.name().clone())
                               .collect::<Vec<_>>()
                               .join("");

        format!("{count}{prefix}{string_table}")
    }

    pub fn parse_list(from: &str) -> Option<Mangled> {
        let mut iter = from.chars().peekable();

        let prefix = SymbolKindPrefix::parse_list(&mut iter)?;

        let len = prefix.len();

        let components = prefix.iter()
                               .filter_map(|pre| pre.read_from(&mut iter))
                               .collect::<Vec<_>>();

        if len != components.len() {
            return None;
        }

        Some(Mangled::from_components(components))
    }
}
