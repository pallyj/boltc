use std::{path::{Path}, io::Read, collections::HashMap};

use colored::Colorize;
use json::JsonValue;
use mir::exc::val::Value;

use crate::Project;

pub fn run_tests()  {
	use std::fs;

	let Ok(test_folder) = fs::read_dir("test") else {
		eprintln!("{} folder `test` doesn't exist", "error:".red().bold());
		return;
	};


	let subfolders =
		test_folder.flat_map(|ref entry_path| if entry_path.as_ref().unwrap().file_type().unwrap().is_dir() {
			Some(entry_path.as_ref().unwrap().path())
		} else {
			None
		} );

	for subfolder in subfolders {
		let test_path = subfolder.join("test.json");
		let Ok(mut test_manifest) = std::fs::File::open(&test_path) else {
			eprintln!("{} test manifest `{}` not found", "error:".red().bold(), test_path.as_os_str().to_str().unwrap());
			return
		};

		let mut test_json = String::new();
		test_manifest.read_to_string(&mut test_json).unwrap();

		let test = json::parse(&test_json).unwrap();

		run_test_manifest(&subfolder, test);
	}
}

pub fn run_test_manifest(path: &Path, test: JsonValue) {
	let Some(group_name) = test["name"].as_str() else {
		panic!()
	};

	eprintln!("     {} `{}`", "Running".green().bold(), group_name);

	for test in test["tests"].members() {
		run_test(path, &test);
	}
}

fn run_test(path: &Path, test: &JsonValue) {
	if let Some(test_name) = test.as_str() {
		eprintln!("test {test_name} ... {}", "ok".green());
	} else if test.is_object() {
		let test_name = test["name"].as_str().unwrap();
		let include = test["include"].as_str().unwrap();

		print!("test {test_name} ...");

		let mut project = super::Project::new("test");

		project.open_file(path.join(include).as_os_str().to_str().unwrap(), "test");
		for std_file in get_std(test["std"].as_str()) {
			project.open_file(std_file, "test");
		}

		if test.has_key("expect") {
			if let Ok((exc, entry_point)) = compile_test(&mut project) {
				for run in test["expect"].members() {
					let inputs = run["inputs"].members().map(|member| {
						let v = if let Some(n) = member.as_u64() {
							Value::Int(n)
						} else if let Some(n) = member.as_f64() {
							Value::Float(n)
						} else {
							Value::Undef
						};

						let mut hm = HashMap::new();

						hm.insert(String::from("repr"), v);

						Value::Struct(hm)
					}).collect();

					let output = exc.enter(&entry_point, inputs);

					if !switch_output(&run["output"], output) {
						break
					}
				}
			} else {
				if let Some(_err) = test["expected"]["error"].as_str() {
					eprintln!("{}", "ok".green().bold())
				}
			}
		}
	}
}

fn switch_output(expect: &JsonValue, found: Value) -> bool {
	match found {
		Value::Int(n) => {
			if Some(n) == expect.as_u64() {
				eprintln!("{}", "ok".green());
				return true;
			}
		}

		Value::Float(n) => {
			if Some(n) == expect.as_f64() {
				eprintln!("{}", "ok".green());
				return true
			}
		}

		Value::Struct(members) => {
			return switch_output(expect, members["repr"].clone())
		}

		_ => {}
	}

	eprintln!("{} {} != {:?}", "error".red(), expect, found);
	return false
}

fn get_std(std: Option<&str>) -> &[&str] {
	match std {
		Some("full") => &[
			"runtime/std/print.bolt",
			"runtime/bool/Bool.bolt",
			"runtime/float/Half.bolt",
			"runtime/float/Float.bolt",
			"runtime/float/Double.bolt",
			"runtime/int/Int.bolt",
			"runtime/int/UInt.bolt",
			"runtime/int/Int8.bolt",
			"runtime/int/Int16.bolt",
			"runtime/int/Int32.bolt",
			"runtime/int/Int64.bolt",
			"runtime/int/UInt8.bolt",
			"runtime/int/UInt16.bolt",
			"runtime/int/UInt32.bolt",
			"runtime/int/UInt64.bolt",
			"runtime/string/StringSlice.bolt",
			"runtime/string/Char.bolt"
		],
		Some("minimal") => &["runtime/test/test.bolt"],
		_ => &[],
	}
}

fn compile_test(project: &mut Project) -> Result<(mir::exc::ExecutionEngine, String), ()>
{
	project.compile_to_blir()?;
	project.run_passes()?;
	project.compile_to_mir()?;
	let entry_point = project.entry_point().cloned().unwrap();
	Ok((project.create_execution_engine(), entry_point))
}