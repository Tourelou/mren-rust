// parse.rs

use lexopt::{Arg, Parser};
use  crate::locale;

pub struct AppOptions {
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

fn help(prg_name: &str, loc_string: &str, ecode: i32) {
	println!("usage: {prg_name} {loc_string}");
	std::process::exit(ecode);
}

fn versions(prg_name: &str, version: &str, loc_string: &str) {
	println!("{prg_name}{loc_string} {version}");
	std::process::exit(0);
}

pub fn parse_args(prg_name: &str,prg_version: &str) -> Result<AppOptions, String> {
	let mut opts = AppOptions {
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
	let mut parser = Parser::from_env();

	while let Some(arg) = parser.next().map_err(|_| opts.locale.err_opt_format.to_string())? {
		match arg {
			Arg::Short('h') => help(prg_name, opts.locale.usage, 0),
			Arg::Long("help") => help(prg_name, opts.locale.options, 0),
			Arg::Short('V') => versions(prg_name, prg_version, opts.locale.ver),
			Arg::Long("version") => versions(prg_name, prg_version, opts.locale.ver_desc),

			Arg::Short('f') => opts.files_only = true,
			Arg::Short('d') => opts.dirs_only = true,
			Arg::Short('r') | Arg::Long("recursive") => opts.recursive = true,
			Arg::Short('i') | Arg::Long("include") => opts.include_dir = true,
			Arg::Short('I') | Arg::Long("ignoreCase") => opts.ignore_case = true,
			Arg::Short('n') | Arg::Long("simulate") => opts.simulate = true,
			Arg::Short('v') | Arg::Long("verbose") => opts.verbose = true,

			Arg::Value(val) => { 
				if opts.pattern == None { opts.pattern = val.into_string().ok();}
				else if opts.replacement == None { opts.replacement = val.into_string().ok(); }
				else { opts.directories.push(val.to_string_lossy().into_owned()); }},

			inconnu => {
				let nom = match inconnu {
					Arg::Short(c) => format!("-{}", c),
					Arg::Long(s) => format!("--{}", s),
					_ => "inconnue".to_string(),
				};
				return Err(format!("{}", opts.locale.err_opt_inv.replace("{1}", &nom)));
			}
		}
	}
	Ok(opts)
}
