use std::{path::PathBuf, process::Command, io::Write};

use etcetera::app_strategy::{AppStrategyArgs, AppStrategy};
use fs_extra::dir::CopyOptions;
use tera::Context;

const PATH_TO_STD: &str = "lib/0.6/";
const PATH_TO_STD_SOURCE: &str = "src/0.6";

#[cfg(target_os = "windows")]
type AppStrat = etcetera::app_strategy::Windows;

#[cfg(target_os = "macos")]
type AppStrat = etcetera::app_strategy::Apple;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
type AppStrat = etcetera::app_strategy::Unix;

pub struct StandardLib {
	strategy: AppStrat,
}

impl StandardLib {
	pub fn is_installed(&self) -> bool {
		self.strategy
			.in_config_dir(PATH_TO_STD)
			.exists()
	}

	pub fn compile(&self) {
		let path_to_std = self.strategy.in_config_dir(PATH_TO_STD_SOURCE);
		let path_to_lib = self.strategy.in_config_dir(PATH_TO_STD);

		let path_to_std_str = path_to_std.as_os_str().to_string_lossy();
		let path_to_lib = path_to_lib.as_os_str().to_string_lossy();

		Command::new("clang")
			.args([&format!("{path_to_std_str}/std/print.c"),
				   "-c",
				   "-o",
				   &format!("{path_to_lib}/print.o")
				   ])
			.status()
			.unwrap();

		Command::new("ar")
			.args(["-crs", &format!("{path_to_lib}libstd.a"),
				   &format!("{path_to_lib}print.o")])
			.spawn()
			.unwrap();

	}

	pub fn get_source_path(&self) -> PathBuf {
		self.strategy
			.in_config_dir(PATH_TO_STD_SOURCE)
	}

	pub fn get_lib_path(&self) -> PathBuf {
		self.strategy
			.in_config_dir(PATH_TO_STD)
	}

	pub fn install(&self) {
		let _ = fs_extra::dir::remove(self.strategy.config_dir());
		std::fs::create_dir_all(self.strategy.in_config_dir(PATH_TO_STD_SOURCE)).unwrap();
		std::fs::create_dir_all(self.strategy.in_config_dir(PATH_TO_STD)).unwrap();

		let template_engine = tera::Tera::new("runtime/**/*.bolt.tera").unwrap();
		let std_source = self.strategy.in_config_dir(PATH_TO_STD_SOURCE);

		fs_extra::dir::create_all(std_source.join("int"), true).unwrap();
		fs_extra::dir::create_all(std_source.join("float"), true).unwrap();
		fs_extra::dir::create_all(std_source.join("bool"), true).unwrap();
		fs_extra::dir::create_all(std_source.join("string"), true).unwrap();
		fs_extra::dir::create_all(std_source.join("std"), true).unwrap();
		fs_extra::dir::create_all(std_source.join("test"), true).unwrap();

		for int in INTS {
			let file = std::fs::File::create(std_source.join(format!("int/{}.bolt", int.name))).unwrap();

			template_engine.render_to("int/Int.bolt.tera", &int.context(), file).unwrap();
		}

		let bool_file = std::fs::File::create(std_source.join("bool/Bool.bolt")).unwrap();
		template_engine.render_to("bool/Bool.bolt.tera", &Context::new(), bool_file).unwrap();

		for float in FLOATS {
			let file = std::fs::File::create(std_source.join(format!("float/{}.bolt", float.name))).unwrap();

			template_engine.render_to("float/Float.bolt.tera", &float.context(), file).unwrap();
		}

		let char_file = std::fs::File::create(std_source.join("string/Char.bolt")).unwrap();
		template_engine.render_to("string/Char.bolt.tera", &Context::new(), char_file).unwrap();

		let string_file = std::fs::File::create(std_source.join("string/String.bolt")).unwrap();
		template_engine.render_to("string/String.bolt.tera", &Context::new(), string_file).unwrap();

		let files_to_copy = [ "std/print.c", "std/print.bolt", "test/testing.bolt" ];

		for file in files_to_copy {
			std::fs::copy(format!("runtime/{file}"), std_source.join(file)).unwrap();
		}

		self.compile();
	}
}

impl Default for StandardLib {
    fn default() -> Self {
        let strategy = AppStrat::new(AppStrategyArgs {
			top_level_domain: "com".to_string(),
			author: "bolt".to_string(),
			app_name: "boltc".to_string(),
		}).unwrap();

		Self { strategy }
    }
}

const FLOATS: &[FloatModel] = &[
	FloatModel {
		name: "Half",
		description: r#"A 16-bit floating point type
///		, represented by an IEEE 754-2008 binary16 floating point value.
///
///		The `Half` type can represent a range of numbers, like 0, -1, or 3.14.
///		Unlike an `Int16`, a `Half` can express non-integer numbers. A `Half` is
/// 	the fastest floating-point type, and should be used when precision is not
/// 	an object and speed is paramount.
///
///		A `Half` can represent values in the range of ±65504, but the large range
///		comes at a loss of precision. A `Half` has 10 bits of precision, giving it
///		a precision of 1/1024. Whole numbers bigger than 65504 cannot be represented
///		by a `Half`.
///
///		In addition, a `Half` has some special values:
///
///		- -0.0: Floating point numbers have both a positive and a negative zero. A
///		negative zero is equal to a positive zero, but if they are multiplied together,
///		the result is negative.
///		- ∞ and -∞: The result of dividing by zero.
///		- NaN: The result of `sqrt(-1.0)`. NaN doesn't equal any float, can't be compared
///		to any float, and any calculation with NaN as an operand is poisoned and becomes NaN.
///		"#,
		bits: 16,
		is_default: false,
	},
	FloatModel {
		name: "Float",
		description: r#"A 32-bit floating point type
///		, represented by an IEEE 754-2008 binary32 floating point value.
/// 	
/// 	The `Float` type can represent a range of numbers, like 0, -1, 3.14, 101.56829315,
/// 	and -∞.  Unlike an integer, a `Float` can express non-integer numbers. `Float` is a
/// 	balance between the speed of a `Half` and the precision of a `Double`, and should be used
/// 	in most cases.
/// 	
/// 	A `Float` can represent values in the range ±3.4028235 x 10^38, a much large range
/// 	than an integer can represent. This large range of representation comes with a sacrifice
/// 	of some precision. A `Float` has 24 bits of precision, meaning it can represent any integer
/// 	up to 10^7, and the smallest fraction it can represent is 1/16777216.
/// 
///		In addition, a `Float` has some special values:
///
///		- -0.0: Floating point numbers have both a positive and a negative zero. A
///		negative zero is equal to a positive zero, but if they are multiplied together,
///		the result is negative.
///		- ∞ and -∞: The result of dividing by zero.
///		- NaN: The result of `sqrt(-1.0)`. NaN doesn't equal any float, can't be compared
///		to any float, and any calculation with NaN as an operand is poisoned and becomes NaN.
///		"#,
		bits: 32,
		is_default: false,
	},
	FloatModel {
		name: "Double",
		description: r#"A 64-bit floating point type
/// 	, represented by an IEEE 754-2008 binary64 floating point value.
/// 
/// 	The `Double` type can represent a range of numbers, like 0, -1, 3.14, 101.56829315,
/// 	and -∞.  Unlike an integer, a `Double` can express non-integer numbers. `Double` is the
/// 	most precise floating-point type, but is also the slowest, and is mostly suitable
/// 	for scientific calculations.
/// 
/// 	A `Double` can represent values in the range of ±2^1024, a practically infinite
/// 	range. A `Double` doesn't sacrifice much precision, having 53 bits of precision, or about
/// 	15 - 17 significant figures. This is enough detail to represent any number between
/// 	±9,007,199,254,740,992 as an integer.
/// 
///		In addition, a `Double` has some special values:
///
///		- -0.0: Floating point numbers have both a positive and a negative zero. A
///		negative zero is equal to a positive zero, but if they are multiplied together,
///		the result is negative.
///		- ∞ and -∞: The result of dividing by zero.
///		- NaN: The result of `sqrt(-1.0)`. NaN doesn't equal any float, can't be compared
///		to any float, and any calculation with NaN as an operand is poisoned and becomes NaN.
///		"#,
		bits: 64,
		is_default: false,
	}
];

const INTS: &[IntModel] = &[
	IntModel {
		name: "Int8",
		description: "An 8-bit signed integer",
		bits: 8,
		signed: true,
		is_default: false,
		minimum: "0xff",
		maximum: "0x7f"
	},
	IntModel {
		name: "UInt8",
		description: "An 8-bit unsigned integer",
		bits: 8,
		signed: false,
		is_default: false,
		minimum: "0x0",
		maximum: "0xff"
	},
	IntModel {
		name: "Int16",
		description: "A 16-bit signed integer",
		bits: 16,
		signed: true,
		is_default: false,
		minimum: "0xffff",
		maximum: "0x7fff"
	},
	IntModel {
		name: "UInt16",
		description: "A 16-bit unsigned integer",
		bits: 16,
		signed: false,
		is_default: false,
		minimum: "0x0",
		maximum: "0xffff"
	},
	IntModel {
		name: "Int32",
		description: "A 32-bit signed integer",
		bits: 32,
		signed: true,
		is_default: false,
		minimum: "0xffff_ffff",
		maximum: "0x7fff_ffff"
	},
	IntModel {
		name: "UInt32",
		description: "A 32-bit unsigned integer",
		bits: 32,
		signed: false,
		is_default: false,
		minimum: "0x0",
		maximum: "0xffff_ffff"
	},
	IntModel {
		name: "Int64",
		description: "A 64-bit signed integer",
		bits: 64,
		signed: true,
		is_default: false,
		minimum: "0xffff_ffff_ffff_ffff",
		maximum: "0x7fff_ffff_ffff_ffff"
	},
	IntModel {
		name: "UInt64",
		description: "A 64-bit unsigned integer",
		bits: 64,
		signed: false,
		is_default: false,
		minimum: "0x0",
		maximum: "0xffff_ffff_ffff_ffff"
	},
	IntModel {
		name: "Int",
		description: "A platform-sized signed integer",
		bits: 64,
		signed: true,
		is_default: true,
		minimum: "0xffff_ffff_ffff_ffff",
		maximum: "0x7fff_ffff_ffff_ffff"
	},
	IntModel {
		name: "UInt",
		description: "A platform-sized unsigned integer",
		bits: 64,
		signed: false,
		is_default: false,
		minimum: "0x0",
		maximum: "0xffff_ffff_ffff_ffff"
	},
];

pub struct IntModel {
	name: &'static str,
	description: &'static str,
	bits: u32,
	signed: bool,
	is_default: bool,
	minimum: &'static str,
	maximum: &'static str
}

impl IntModel {
	pub fn context(&self) -> tera::Context {
		let mut ctx = tera::Context::new();

		ctx.insert("name", self.name);
		ctx.insert("description", self.description);
		ctx.insert("bits", &self.bits);
		ctx.insert("signed", &self.signed);
		ctx.insert("is_default", &self.is_default);
		ctx.insert("minimum", self.minimum);
		ctx.insert("maximum", self.maximum);

		return ctx
	}
}

pub struct FloatModel {
	name: &'static str,
	description: &'static str,
	bits: u32,
	is_default: bool,
}

impl FloatModel {
	pub fn context(&self) -> tera::Context {
		let mut ctx = tera::Context::new();

		ctx.insert("name", self.name);
		ctx.insert("description", self.description);
		ctx.insert("bits", &self.bits);
		ctx.insert("is_default", &self.is_default);

		return ctx
	}
}