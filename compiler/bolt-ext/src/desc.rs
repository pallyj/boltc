use crate::host::ExtensionHost;

pub static HOST_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// Not used directly
/// Use `declare_extension!` to create an `ExtensionDescription` for your extension
/// 
/// # Safety
/// 
/// `declare_extension!` is the ONLY way to safely create an extension. Using this struct without it causes undefined behavior.
/// 
pub struct ExtensionDescription {
	pub rustc_version: &'static str,
	pub host_version: &'static str,
	pub register_fn: extern "C" fn(&mut dyn ExtensionHost)
}

impl ExtensionDescription {
	pub fn rustc_version(&self) -> &'static str {
		&self.rustc_version
	}

	pub fn host_version(&self) -> &'static str {
		&self.host_version
	}

	pub fn register(&self, ext_host: &mut dyn ExtensionHost) {
		(self.register_fn)(ext_host)
	}
}

#[macro_export]
macro_rules! declare_extension {
	($register:expr) => {
		#[doc(hidden)]
		#[no_mangle]
		pub static extension_declaration: $crate::ExtensionDescription = $crate::ExtensionDescription {
			rustc_version: $crate::RUSTC_VERSION,
			host_version: $crate::HOST_VERSION,
			register_fn: $register
		};
	};
}