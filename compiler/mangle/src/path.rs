use std::fmt::Display;

#[derive(Clone, Hash)]
pub struct Path {
    path_components: Vec<String>,
}

impl Path {
    pub fn new(root: &str) -> Path { Path { path_components: vec![root.to_string()], } }

    pub fn append(mut self, child: &str) -> Path {
        self.path_components.push(child.to_string());
        self
    }

    pub fn into_components(self) -> Vec<String> {
        self.path_components
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for component in &self.path_components {
            let len = component.len();
            write!(f, "{len}{component}")?;
        }

        Ok(())
    }
}
