
use std::env;

use  crate::locale;

#[derive(Debug)]
pub struct Options {
	pub files_only: bool,
	pub dirs_only: bool,
	pub recursive: bool,
	pub include_dir: bool,
	pub ignore_case: bool,
	pub simulate: bool,
	pub verbose: bool,
	pub locale: locale::LangStrings,
	pub pattern: Option<String>,
	pub replacement: Option<String>,
	pub directories: Vec<String>,
}

impl Options {
	pub fn parse_args(prg_name: &str, version: &str) -> Self {
		let args: Vec<String> = env::args().skip(1).collect();
		let mut opts = Options {
			files_only: false,
			dirs_only: false,
			recursive: false,
			include_dir: false,
			ignore_case: false,
			simulate: false,
			verbose: false,
			locale: locale::set_lang_vec(),
			pattern: None,
			replacement: None,
			directories: Vec::new(),
		};

		if args.len() == 0 {
			eprintln!("{}", opts.locale.manque_args);
			help(prg_name, opts.locale.options, 1);	// forme longue
		}

		for arg in args.iter() {
			if arg == "-ver" {
				versions(prg_name, version, opts.locale.ver);	// version courte
			}
			else if arg.starts_with("--") {
				match arg.as_str() {
					"--recursive" => opts.recursive = true,
					"--include" => opts.include_dir = true,
					"--ignoreCase" => opts.ignore_case = true,
					"--simulate" => opts.simulate = true,
					"--verbose" => opts.verbose = true,
					"--version" => versions(prg_name, version, opts.locale.ver_desc), // version longue
					"--help" => help(prg_name, opts.locale.options, 0),	// forme longue
					_ => {
//						eprintln!("Erreur : option longue invalide '{}'", arg);
						eprintln!("{} '{}'", opts.locale.err_opt_longue, arg);
						std::process::exit(1);
					}
				}
			}
			else if arg.starts_with('-') && arg.len() > 2 {
			// Option combinée courte : -nI → -n et -I
				for ch in arg.chars().skip(1) {
					match ch {
						'f' => opts.files_only = true,
						'd' => opts.dirs_only = true,
						'r' => opts.recursive = true,
						'i' => opts.include_dir = true,
						'I' => opts.ignore_case = true,
						'n' => opts.simulate = true,
						'v' => opts.verbose = true,
						'h' => help(prg_name, opts.locale.usage, 0),		// forme courte
						// pas de version dans combinaison courte
						_ => {
//							eprintln!("Erreur : option invalide dans combinaison '-{}'", ch);
							eprintln!("{} '-{}'", opts.locale.err_opt_comb, ch);
							std::process::exit(1);
						}
					}
				}
			}
			else if arg.starts_with('-') {
			// Options courtes simples
				match arg.as_str() {
					"-f" => opts.files_only = true,
					"-d" => opts.dirs_only = true,
					"-r" => opts.recursive = true,
					"-i" => opts.include_dir = true,
					"-I" => opts.ignore_case = true,
					"-n" => opts.simulate = true,
					"-v" => opts.verbose = true,
					"-h" => help(prg_name, opts.locale.usage, 0),		// forme courte
					_ => {
//						eprintln!("Erreur : option invalide '{}'", arg);
						eprintln!("{} '{}'", opts.locale.err_opt_inv, arg);
						std::process::exit(1);
					}
				}
			}
			else {
			// Positionnels : motif, remplacement, dossier(s)
				if opts.pattern.is_none() {
					opts.pattern = Some(arg.to_string());
				}
				else if opts.replacement.is_none() {
					opts.replacement = Some(arg.to_string());
				}
				else {
					opts.directories.push(arg.to_string());
				}
			}
		}
		opts
	}
}

fn help(prg_name: &str, loc_string: &str, ecode: i32) {
	println!("usage: {prg_name} {loc_string}");
	std::process::exit(ecode);
}

fn versions(prg_name: &str, version: &str, loc_string: &str) {
	println!("{prg_name}{loc_string} {version}");
	std::process::exit(0);
}
