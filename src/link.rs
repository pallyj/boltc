use std::{path::Path, fmt::format};

use crate::{args::Args, cstd};

pub fn with_args(
	obj_path: &Path,
	args: &Args,
	entry_point: &str)
{
	let standard = cstd::StandardLib::default();

	let output =
		std::process::Command::new("clang")
			.arg(obj_path.as_os_str().to_str().unwrap())
			.arg(&format!("-L{}", standard.get_lib_path().as_os_str().to_str().unwrap()))
			.arg("-lstd")
			.arg("-e")
			.arg(&format!("_{entry_point}"))
			.arg("-o")
			.arg(args.output_file.as_ref().unwrap())
			.output()
			.unwrap()
			.stderr;

	println!("{}", std::str::from_utf8(&output).unwrap());
}