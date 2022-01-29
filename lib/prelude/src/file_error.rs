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