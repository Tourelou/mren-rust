// action.rs

use std::fs;
use regex::Regex;
use std::path::Path;
use crate::parse::Options;

pub fn scan_dir(repl: &String, regex: &Regex, opts: &Options, iteration: usize) -> (bool, Vec<String>) {
	let mut found = false;
	let mut lines = Vec::new();
	if iteration > 100 {
		lines.push("Profondeur de récursion trop grande (>100)".to_string());
		return (false, lines);
	}

	let prefix = format!("{}", "  ".repeat(iteration));

	let entries = match fs::read_dir(".") {
		Ok(e) => e,
		Err(e) => {
			lines.push(format!("{}Erreur de lecture du répertoire: {}", prefix, e));
			return (false, lines);
		}
	};

	let mut files = Vec::new();
	let mut dirs = Vec::new();

	// Séparer fichiers et répertoires
	for entry in entries {
		match entry {
			Ok(e) => {
				let path = e.path();
				if path.is_file() {
					files.push(e);
				} else if path.is_dir() {
					dirs.push(e);
				}
			}
			Err(e) => {
				lines.push(format!("{}Entrée illisible: {}", prefix, e));
			}
		}
	}

	// 🔹 Renommer les fichiers
	for entry in files {
		let file_name = entry.file_name();
		let file_name_str = match file_name.to_str() {
			Some(s) => s,
			None => continue,
		};

		if regex.is_match(file_name_str) {
			let new_file_name = regex.replace_all(file_name_str, repl).to_string();
			let (_, sub_lines) = renomme(file_name_str, &new_file_name, &prefix, opts);
			lines.extend(sub_lines);
			found = true;
		}
	}

	// 🔹 Renommer les répertoires et explorer récursivement
	for entry in dirs {
		let dir_name = entry.file_name();
		let dir_name_str = match dir_name.to_str() {
			Some(s) => s,
			None => continue,
		};

		let mut new_dir_name = dir_name_str.to_string();

		if regex.is_match(dir_name_str) {
			new_dir_name = regex.replace_all(dir_name_str, repl).to_string();
			let (_, sub_lines) = renomme(dir_name_str, &new_dir_name, &prefix, opts);
			lines.extend(sub_lines);
			found = true;
		}

		let path = Path::new(&new_dir_name);
		if opts.recursive && path.is_dir() {
			if let Err(e) = std::env::set_current_dir(&path) {
				lines.push(format!("{}Impossible d'entrer dans le dossier {:?}: {}", prefix, path, e));
				continue;
			}

			let (sub_found, sub_lines) = scan_dir(repl, regex, opts, iteration + 1);

			if sub_found {
				lines.push(format!("{}└ \x1b[1;34m{}\x1b[0m ┐", prefix, path.display()));
				lines.extend(sub_lines);
				found = true;
				lines.push(format!("{}┌ \x1b[1;34m{}\x1b[0m ┘", prefix, path.display()));
			}

			if let Err(e) = std::env::set_current_dir("..") {
				lines.push(format!("{}Erreur en revenant au dossier parent: {}", prefix, e));
				return (found, lines);
			}
		}
	}
	(found, lines)
}

pub fn renomme(nom: &str, nouveau_nom: &str, indent: &str, opts: &Options) -> (bool, Vec<String>) {
	let mut lines = Vec::new();
	if opts.simulate {
		lines.push(format!("{indent}▶︎ {nom} \x1b[92m\x1b[40m ==> Deviendrait ==> \x1b[0m {nouveau_nom}"));
		return (false, lines);
	}
	else {
		// renomme ici
		match fs::rename(nom, nouveau_nom) {
			Ok(_) => {
				if opts.verbose {
					lines.push(format!("{indent}▶︎ {nom} \x1b[91m\x1b[40m ==> est devenu ==> \x1b[0m {nouveau_nom}"));
				}
				return (true, lines);
			}
			Err(e) => {
				lines.push(format!("Erreur lors du renommage de {}: {}", nom, e));
				return (false, lines);
			}
		}
	}
}
