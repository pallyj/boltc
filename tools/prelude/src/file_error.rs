use crate::BoltMessage;

#[derive(Debug)]
pub enum FileError {
	/// Error code #F001
	/// 
	/// Couldn't find file {} in the current directory 
	FileNotFound(String),

	/// Error code #F002
	/// 
	/// Didn't have permission to access file {}
	CantAccess(String),

	/// Error code #F003
	/// 
	/// File {} is locked
	LockedFile(String),
}

impl BoltMessage for FileError {
    fn code(&self) -> String {
        match self {
			Self::FileNotFound(_) => "F001",
			Self::CantAccess(_) => "F002",
			Self::LockedFile(_) => "F003",
		}.to_string()
    }

    fn suggestion(&self) -> Option<String> {
        None
    }

    fn description(&self) -> String {
        match self {
			Self::FileNotFound(file) => format!("Couldn't find file {} in the current directory", file),
			Self::CantAccess(file) => format!("Didn't have permission to access file {}", file),
			Self::LockedFile(file) => format!("File {} is locked", file)
		}
    }

    fn level(&self) -> crate::MessageLevel {
        crate::MessageLevel::Error
    }
}