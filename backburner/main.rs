// main.rs

mod parse;

use parse::Options;
use std::fs;
use std::path::Path;

const PRG_NAME: &str = "mren";
const VERSION: &str = "2025-07-15";

fn main() {
	let opts = Options::parse_args();

	// Affiche l'aide
	if opts.help == 1 {	// option -h
		println!("usage: mren [-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]");
		return;
	}

	if opts.help == 2 {	// option --help
		println!("usage: mren [-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]\n
Renommage multiple selon un certain motif.\n
Arguments en position:
  <motif regex>     Motif à chercher: Mettre entre guillements '...'
  <remplacement>    Chaîne de remplacement. Doit obligatoirement suivre le motif.
  [dirname ...]     Répertoire(s) de recherche.
\nOptions:
  -f                  N'agit que sur les fichiers.
  -d                  N'agit que sur les répertoires.
  -r,   --recursive   Procède de façon récursive sur les répertoires.
  -i,   --include     En mode récusif, inclu le dossier en ligne de commande.
  -I,   --ignoreCase  Fait une recherche en ignorant la case.
  -n,   --simulate    Simule les opérations demandées - Fichiers affectés en VERT.
  -v,   --verbose     Donne des détails sur le(s) fichier(s) traité(s) - Fichiers affectés en ROUGE.
  -ver, --version     Renommage multiple à partir d'un motif.
  -h,   --help        Montre ce message d'aide et termine.");
		return;
	}

	// Affiche la version
	if opts.version == 1 {	// option -ver
		println!("{PRG_NAME}: version {VERSION}");
		return;
	}

	if opts.version == 2 {	// option --version
		println!("{PRG_NAME}: Renommage multiple selon un certain motif, version {VERSION}");
		return;
	}

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
	let dirs = if opts.directories.is_empty() {
		vec![".".to_string()]
	} else {
		opts.directories.clone()
	};

	// Lance le traitement pour chaque répertoire
	for dir in dirs {
		let path = Path::new(&dir);
		visit_dir(path, pattern, replacement, &opts, opts.include_dir);
	}
}

fn visit_dir(path: &Path, pattern: &str, replacement: &str, opts: &Options, include_self: bool) {
	if include_self {
		handle_entry(path, pattern, replacement, opts);
	}

	if let Ok(entries) = fs::read_dir(path) {
		for entry in entries.filter_map(Result::ok) {
			let p = entry.path();

			if opts.recursive && p.is_dir() {
				visit_dir(&p, pattern, replacement, opts, true);
			}

			handle_entry(&p, pattern, replacement, opts);
		}
	}
}

fn handle_entry(path: &Path, pattern: &str, replacement: &str, opts: &Options) {
	let is_file = path.is_file();
	let is_dir = path.is_dir();

	// Respecte les filtres -f / -d
	if (opts.files_only && !is_file) || (opts.dirs_only && !is_dir) {
		return;
	}

	let file_name = match path.file_name().and_then(|s| s.to_str()) {
		Some(name) => name,
		None => return,
	};

	// Remplacement simple sans regex
	let new_name = if opts.ignore_case {
		replace_ignore_case(file_name, pattern, replacement)
	} else {
		file_name.replace(pattern, replacement)
	};

	if new_name == file_name {
		return;
	}

	let new_path = path.with_file_name(&new_name);

	if opts.simulate {
		println!("{} \x1b[92m\x1b[40m Deviendrait \x1b[0m {}", path.display(), new_path.display());
	} else {
		match fs::rename(path, &new_path) {
			Ok(_) => {
				if opts.verbose {
					println!("{} \x1b[92m\x1b[40m est devenu \x1b[0m {}", path.display(), new_path.display());
				}
			}
			Err(e) => {
				eprintln!("Erreur : Impossible de renommer {:?} → {:?} : {}", path, new_path, e);
			}
		}
	}
}

// Remplacement insensible à la casse
fn replace_ignore_case(source: &str, pattern: &str, replacement: &str) -> String {
	let source_chars: Vec<char> = source.chars().collect();
	let pattern_len = pattern.chars().count();
	let pattern_lower = pattern.to_lowercase();

	let mut result = String::new();
	let mut i = 0;

	while i < source_chars.len() {
		// Vérifie qu'il reste assez de caractères pour comparer
		if i + pattern_len <= source_chars.len() {
			let slice: String = source_chars[i..i + pattern_len].iter().collect();
			if slice.to_lowercase() == pattern_lower {
				result.push_str(replacement);
				i += pattern_len;
				continue;
			}
		}
		result.push(source_chars[i]);
		i += 1;
	}
	result
}
