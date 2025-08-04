// main.rs

mod parse;
mod action;
mod locale;

use std::env;
use std::process;
use std::path::PathBuf;
use std::path::Path;
use regex::Regex;

const PRG_NAME: &str = "mren";
const VERSION: &str = "2025-08-04";

fn main() {
	let mut opts = parse::Options::parse_args(PRG_NAME, VERSION);

	if opts.files_only && opts.dirs_only {
		eprintln!("{}", opts.locale.err_mutuel);
		std::process::exit(1);
	}
	if opts.simulate { opts.verbose = true };

	let app_base_path: PathBuf = env::current_dir().unwrap_or_else(|_| process::exit(1));

	// Vérifie le motif et le remplacement
	let pattern = match &opts.pattern {
		Some(p) => p,
		None => {
			eprintln!("{}", opts.locale.err_regex_manque);
			return;
		}
	};

	let replacement = match &opts.replacement {
		Some(r) => r,
		None => {
			eprintln!("{}", opts.locale.err_rempl_manque);
			return;
		}
	};
	// Détermine les répertoires à explorer
	let dirs = if opts.directories.is_empty() { vec![".".to_string()] }
				else { opts.directories.clone() };

	// Détermine re en rapport à l'option ignore_case
	let re = if opts.ignore_case { Regex::new(&format!("(?i){}", pattern)).unwrap() }
				else { Regex::new(&pattern).unwrap() };

	// Lance le traitement pour chaque répertoire
	for (i, dir) in dirs.iter().enumerate() {
		let path = Path::new(&dir);
		if !path.is_dir() {
			eprintln!("\x1b[1;31m\x1b[40m {} \x1b[0m {}", path.display(), opts.locale.err_dir_invalide);
			continue;
		}
		// À partir d'ici nous avons un répertoire valide.

		if let Err(e) = env::set_current_dir(&path) {
			eprintln!("{} {:?} : {}", opts.locale.err_chdir, path, e);
			let _ = env::set_current_dir(&app_base_path);
			continue;
		}

		let mut abs_loop_dir: PathBuf = env::current_dir().unwrap_or_else(|_| process::exit(1));

		if opts.verbose {
			if i != 0 {
				println!("- - - - - - - - - - - - - - - - - - - - - - - - -")
			}
			println!("{} «\x1b[1;34m{}\x1b[0m»", opts.locale.process_dir, abs_loop_dir.display());
		}

		if opts.include_dir && !opts.files_only {
			let Some(base_path_dir) = abs_loop_dir.file_name().and_then(|n| n.to_str()) else {
				continue;
			};
			let abs_parent_dir = abs_loop_dir.parent().unwrap_or_else(|| Path::new(""));
			let new_base_path = re.replace(base_path_dir, replacement).to_string();

			if base_path_dir != new_base_path {
				if opts.verbose { println!("{}", opts.locale.ren_src_dir); }

				if let Err(e) = env::set_current_dir(abs_parent_dir) {
					eprintln!("{} {:?} : {}", opts.locale.err_chdir, abs_parent_dir, e);
					let _ = env::set_current_dir(&app_base_path);
					continue;
				}

				let (fait, verbatim) = action::renomme(base_path_dir, &new_base_path, "--", &opts);
				for line in verbatim { println!("{}", line); }

				if fait {
					abs_loop_dir = abs_parent_dir.join(&new_base_path);
					if opts.verbose { println!("{} {}", opts.locale.nouveau_path, abs_loop_dir.display()); }
				}
				if let Err(e) = env::set_current_dir(&abs_loop_dir) {
					eprintln!("{} {:?} : {}", opts.locale.err_chdir, abs_loop_dir, e);
					let _ = env::set_current_dir(&app_base_path);
					println!("- - - - -");
					continue;
				}
				if opts.verbose { println!("- - - - -"); }
			}
		}
		let (found, output_lines) = action::scan_dir(&replacement, &re, &opts, 0);

		if found { for line in output_lines { println!("{}", line); } }
		else { println!("{}", opts.locale.no_match); }

		let _ = env::set_current_dir(&app_base_path);	// On rammène app_base_path pour la boucle
	}
}
