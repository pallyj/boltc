use prelude::{BoltMessage, MessageLevel};

#[derive(Debug)]
pub enum ProjectError {
	DirectoryNotFound(String),
	ConfigNotFound(String),
	CfgError(serde_yaml::Error)
}

impl BoltMessage for ProjectError {
    fn code(&self) -> String {
        match self {
			Self::DirectoryNotFound(_) => "P001",
			Self::ConfigNotFound(_) => "P002",
			Self::CfgError(_) => "P003",
		}.to_string()
    }

    fn suggestion(&self) -> Option<String> {
        None
    }

    fn description(&self) -> String {
        match self {
			Self::DirectoryNotFound(dir) => format!("Directory '{}' not found", dir),
			Self::ConfigNotFound(cfg) => format!("Config '{}' not found", cfg),
			Self::CfgError(err) => err.to_string()
		}
    }

    fn level(&self) -> prelude::MessageLevel {
        MessageLevel::Error
    }
}