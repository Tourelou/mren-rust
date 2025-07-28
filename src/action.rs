// action.rs

// use std::env;
use std::fs;
// use std::process;
use regex::Regex;
// use std::path::Path;
// use std::path::PathBuf;
use crate::parse::Options;

pub fn scan_dir(repl: &String, regex: &Regex, opts: &Options, iteration: usize) {

	if opts.recursive { println!("Recherche récursive dans le répertoire courant."); }
	else { println!("Recherche non récursive dans le répertoire courant."); }


	println!("Itération: {}", iteration);
	let prefix = format!("{}▶︎ ", "  ".repeat(iteration));
	println!("{}", prefix);

	let entries = match fs::read_dir(".") {
		Ok(e) => e,
		Err(e) => {
			eprintln!("Erreur de lecture du répertoire: {}", e);
			return;
		}
	};

	for entry in entries {
		if let Ok(entry) = entry {
			let file_name = entry.file_name();
			let file_name_str = match file_name.to_str() {
				Some(s) => s,
				None => continue,
			};

			if regex.is_match(file_name_str) {
				let nouveau_nom = regex.replace_all(file_name_str, repl).to_string();
				renomme(file_name_str, &nouveau_nom, &prefix, &opts);
			}
		}
	}
}

pub fn renomme(nom: &str, nouveau_nom: &str, indent: &str, opts: &Options) -> bool {
	if opts.simulate {
		println!("{indent} {nom} \x1b[92m\x1b[40m ==> Deviendrait ==> \x1b[0m {nouveau_nom}");
		return false;
	}
	else {
		// renomme ici
		match fs::rename(nom, nouveau_nom) {
			Ok(_) => {
				if opts.verbose {
					println!("{indent} {nom} \x1b[91m\x1b[40m ==> est devenu ==> \x1b[0m {nouveau_nom}");
				}
				return true;
			}
			Err(e) => {
				eprintln!("Erreur lors du renommage de {}: {}", nom, e);
				return false;
			}
		}
	}
}
