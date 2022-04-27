use std::fmt::Display;

use crate::{MangledType, Path};

pub struct MangledStruct<'a>(pub &'a Path);

impl<'a> Display for MangledStruct<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}S", self.0) }
}

pub struct MangledEnum<'a>(pub &'a Path);

impl<'a> Display for MangledEnum<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}V", self.0) }
}

pub struct MangledFunction<'s> {
    pub path:   &'s Path,
    pub args:   Vec<MangledType>,
    pub labels: Vec<Option<&'s str>>,
}

impl<'a> Display for MangledFunction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}F", self.path)?;

        for arg in &self.args {
            write!(f, "{arg}")?;
        }

        write!(f, "E")?;

        for label in &self.labels {
            if let Some(label) = label {
                let len = label.len();
                write!(f, "{len}{label}")?;
            } else {
                write!(f, "0")?;
            }
        }

        Ok(())
    }
}
