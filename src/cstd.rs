use std::{path::PathBuf, process::Command};

use etcetera::app_strategy::{AppStrategyArgs, AppStrategy};
use fs_extra::dir::CopyOptions;

const PATH_TO_STD: &str = "lib/0.5/";
const PATH_TO_STD_SOURCE: &str = "src/0.5";

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

		Command::new("clang")
			.args([&format!("{path_to_std_str}/string/str.c"),
				   "-c",
				   "-o",
				   &format!("{path_to_lib}/str.o")
				   ])
			.status()
			.unwrap();

		Command::new("ar")
			.args(["-crs", &format!("{path_to_lib}libstd.a"),
				   &format!("{path_to_lib}str.o"),
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

		let std_path = "lang/";

		let std_items = fs_extra::dir::get_dir_content(std_path).unwrap();

		let mut items = std_items.directories;

		items.extend_from_slice(&std_items.files);

		fs_extra::copy_items(&items, self.strategy.in_config_dir(PATH_TO_STD_SOURCE), &CopyOptions::default()).unwrap();

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