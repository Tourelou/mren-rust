// parse.rs

use std::env;

#[derive(Debug)]
pub struct Options {
	pub files_only: bool,
	pub dirs_only: bool,
	pub recursive: bool,
	pub include_dir: bool,
	pub ignore_case: bool,
	pub simulate: bool,
	pub verbose: bool,
	pub version: u8,
	pub help: u8,
	pub pattern: Option<String>,
	pub replacement: Option<String>,
	pub directories: Vec<String>,
}

impl Options {
	pub fn parse_args() -> Self {
		let args: Vec<String> = env::args().skip(1).collect();
		let mut opts = Options {
			files_only: false,
			dirs_only: false,
			recursive: false,
			include_dir: false,
			ignore_case: false,
			simulate: false,
			verbose: false,
			version: 0,
			help: 0,
			pattern: None,
			replacement: None,
			directories: Vec::new(),
		};

		let mut parsing_flags = true;

		for arg in args {
			if parsing_flags && arg.starts_with("--") {
				match arg.as_str() {
					"--recursive" => opts.recursive = true,
					"--include" => opts.include_dir = true,
					"--ignoreCase" => opts.ignore_case = true,
					"--simulate" => opts.simulate = true,
					"--verbose" => opts.verbose = true,
					"--version" => opts.version = 2,
					"--help" => opts.help = 2,
					_ => parsing_flags = false,
				}
			}
			else if parsing_flags && arg == "-ver" {
				opts.version = 1
			}
			else if parsing_flags && arg.starts_with('-') && arg.len() > 1 {
				for c in arg.chars().skip(1) {
					match c {
						'f' => opts.files_only = true,
						'd' => opts.dirs_only = true,
						'r' => opts.recursive = true,
						'i' => opts.include_dir = true,
						'I' => opts.ignore_case = true,
						'n' => opts.simulate = true,
						'v' => opts.verbose = true,
						'h' => opts.help = 1,
						_ => parsing_flags = false,
					}
				}
			}
			else if opts.pattern.is_none() {
				opts.pattern = Some(arg);
			}
			else if opts.replacement.is_none() {
				opts.replacement = Some(arg);
			}
			else {
				opts.directories.push(arg);
			}
		}
		opts
	}
}
