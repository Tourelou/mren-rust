// main.rs
mod parse;
mod action;

use std::env;
use std::process;
use std::path::PathBuf;
use std::path::Path;
// use std::fs;
use regex::Regex;

use parse::Options;
use action::scan_dir;
use action::renomme;

const PRG_NAME: &str = "mren";
const VERSION: &str = "2025-07-31";

fn main() {
	let mut opts = Options::parse_args(PRG_NAME, VERSION);

	if opts.files_only && opts.dirs_only {
		eprintln!("-f et -d sont mutuellement exclusif. C'est un ou c'est l'autre.");
		std::process::exit(1);
	}
	if opts.simulate { opts.verbose = true };

	let app_base_path: PathBuf = env::current_dir().unwrap_or_else(|_| process::exit(1));

	// Vérifie le motif et le remplacement
	let pattern = match &opts.pattern {
		Some(p) => p,
		None => {
			eprintln!("Erreur : motif regex manquant.");
			return;
		}
	};

	let replacement = match &opts.replacement {
		Some(r) => r,
		None => {
			eprintln!("Erreur : chaîne de remplacement manquante.");
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
			eprintln!("\x1b[1;31m\x1b[40m {} \x1b[0m n'est pas un répertoire valide.", path.display());
			continue;
		}
		// À partir d'ici nous avons un répertoire valide.

		if let Err(e) = env::set_current_dir(&path) {
			eprintln!("Erreur changement vers {:?} : {}", path, e);
			let _ = env::set_current_dir(&app_base_path);
			continue;
		}

		let mut abs_loop_dir: PathBuf = env::current_dir().unwrap_or_else(|_| process::exit(1));

		if opts.verbose {
			if i != 0 {
				println!("- - - - - - - - - - - - -")
			}
			println!("Traitement du répertoire «\x1b[1;34m{}\x1b[0m»", abs_loop_dir.display());
		}

		if opts.include_dir && !opts.files_only {
			let Some(base_path_dir) = abs_loop_dir.file_name().and_then(|n| n.to_str()) else {
				continue;
			};
			let abs_parent_dir = abs_loop_dir.parent().unwrap_or_else(|| Path::new(""));
			let new_base_path = re.replace(base_path_dir, replacement).to_string();

			if base_path_dir != new_base_path {
				println!("- - - - -\nRenommage du répertoire source");

				if let Err(e) = env::set_current_dir(abs_parent_dir) {
					eprintln!("Erreur changement vers {:?} : {}", abs_parent_dir, e);
					let _ = env::set_current_dir(&app_base_path);
					continue;
				}

				let (fait, verbatim) = renomme(base_path_dir, &new_base_path, "--", &opts);
				for line in verbatim { println!("{}", line); }

				if fait {
					abs_loop_dir = abs_parent_dir.join(&new_base_path);
					println!("Nouveau chemin absolu : {}", abs_loop_dir.display());
				}
				if let Err(e) = env::set_current_dir(&abs_loop_dir) {
					eprintln!("Erreur changement vers {:?} : {}", abs_loop_dir, e);
					let _ = env::set_current_dir(&app_base_path);
					println!("- - - - -");
					continue;
				}
				println!("- - - - -");

			}
			println!("- - - - - - - - - - - - - - - - - - - - - - - - - - - -");
		}
		let (found, output_lines) = scan_dir(&replacement, &re, &opts, 0);

		if found { for line in output_lines { println!("{}", line); } }
		else { println!("Pas de correspondance dans ce dossier"); }

		let _ = env::set_current_dir(&app_base_path);	// On rammène app_base_path pour la boucle
	}
}
