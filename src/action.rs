// action.rs

use std::fs;
use regex_lite::Regex;
use crate::parse::AppOptions;

pub fn scan_dir(repl: &String, regex: &Regex, opts: &AppOptions, iteration: usize) -> (bool, Vec<String>) {
	let mut found = false;
	let mut lines = Vec::new();
	if iteration > 100 {
		lines.push(opts.locale.err_trop_recurs.to_string());
		return (false, lines);
	}

	let prefix = format!("{}", "  ".repeat(iteration));	// 2 espaces par itération

	let entries = match fs::read_dir(".") {
		Ok(e) => e,
		Err(e) => {
			lines.push(format!("{prefix}{} {e}", opts.locale.err_read_dir));
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
				if path.is_file() && ! opts.dirs_only {
					files.push(e);
				} else if path.is_dir() && ! path.is_symlink() {
					dirs.push(e);
				}
			}
			Err(e) => {
				lines.push(format!("{prefix}{} {e}", opts.locale.err_entry));
			}
		}
	}
	// Trier les fichiers par nom de fichier (insensible à la casse)
	if ! files.is_empty() {
		files.sort_by(|a, b| {
			let name_a = a.file_name().to_string_lossy().to_lowercase();
			let name_b = b.file_name().to_string_lossy().to_lowercase();
			name_a.cmp(&name_b)
		});
	}
	// Trier les répertoires aussi si tu veux
	if ! dirs.is_empty() {
		dirs.sort_by(|a, b| {
			let name_a = a.file_name().to_string_lossy().to_lowercase();
			let name_b = b.file_name().to_string_lossy().to_lowercase();
			name_a.cmp(&name_b)
		});
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

		let mut path = match dir_name.to_str() {
			Some(s) => s.to_string(),
			None => continue,
		};

		if ! opts.files_only {
			if regex.is_match(dir_name_str) {
				let new_dir_name = regex.replace_all(dir_name_str, repl).to_string();
				let (fait, sub_lines) = renomme(dir_name_str, &new_dir_name, &prefix, opts);
	
				if fait {
					path = new_dir_name;
				}
				lines.extend(sub_lines);
				found = true;
			}
		}

		if opts.recursive {
			if let Err(e) = std::env::set_current_dir(&path) {
				lines.push(format!("{prefix}{} {path}: {e}", opts.locale.err_chdir));
				continue;
			}

			let (sub_found, sub_lines) = scan_dir(repl, regex, opts, iteration + 1);

			if sub_found {
				lines.push(format!("{prefix}└ \x1b[1;34m{path}\x1b[0m ┐"));
				lines.extend(sub_lines);
				found = true;
				lines.push(format!("{prefix}┌ \x1b[1;34m{path}\x1b[0m ┘"));
			}

			if let Err(e) = std::env::set_current_dir("..") {
				lines.push(format!("{prefix}{} {e}", opts.locale.err_chdir_parent));
				return (found, lines);
			}
		}
	}
	(found, lines)
}

pub fn renomme(nom: &str, nouveau_nom: &str, indent: &str, opts: &AppOptions) -> (bool, Vec<String>) {
	let mut lines = Vec::new();
	if opts.simulate {
		lines.push(format!("{indent}▶︎ {nom} \x1b[92m\x1b[40m {} \x1b[0m {nouveau_nom}", opts.locale.devient));
		return (false, lines);
	}
	else {
		// renomme ici
		match fs::rename(nom, nouveau_nom) {
			Ok(_) => {
				if opts.verbose {
					lines.push(format!("{indent}▶︎ {nom} \x1b[91m\x1b[40m {} \x1b[0m {nouveau_nom}", opts.locale.devenu));
				}
				return (true, lines);
			}
			Err(e) => {
				lines.push(format!("{indent}!- {} {}: {}", opts.locale.err_renom, nom, e));
				return (false, lines);
			}
		}
	}
}
