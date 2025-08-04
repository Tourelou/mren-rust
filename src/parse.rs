
use std::env;

use crate::locale::LangStrings;
use crate::locale::LANG_EN;

#[derive(Debug)]
pub struct Options {
	pub files_only: bool,
	pub dirs_only: bool,
	pub recursive: bool,
	pub include_dir: bool,
	pub ignore_case: bool,
	pub simulate: bool,
	pub verbose: bool,
	pub locale: LangStrings,
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
			locale: LANG_EN,
			pattern: None,
			replacement: None,
			directories: Vec::new(),
		};

		if args.len() == 0 {
			eprintln!("Arguments nécessaire manquant !!\n-----");
			help_longue(prg_name, 1);
		}

		for arg in args.iter() {
			if arg == "-ver" {
				version_courte(prg_name, version);
			}
			else if arg.starts_with("--") {
				match arg.as_str() {
					"--recursive" => opts.recursive = true,
					"--include" => opts.include_dir = true,
					"--ignoreCase" => opts.ignore_case = true,
					"--simulate" => opts.simulate = true,
					"--verbose" => opts.verbose = true,
					"--version" => version_longue(prg_name, version), // version standard
					"--help" => help_longue(prg_name, 0),	// forme longue
					_ => {
						eprintln!("Erreur : option longue invalide '{}'", arg);
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
						'h' => help_courte(prg_name, 0),		// forme courte
						// pas de version dans combinaison courte
						_ => {
							eprintln!("Erreur : option invalide dans combinaison '-{}'", ch);
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
					"-h" => help_courte(prg_name, 0),		// forme courte
					_ => {
						eprintln!("Erreur : option invalide '{}'", arg);
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

fn help_courte(prg_name: &str, ecode: i32) {			// Affiche l'aide -h
	println!("usage: {prg_name} [-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]");
	std::process::exit(ecode);
}

fn help_longue(prg_name: &str, ecode: i32) {
	println!("usage: {prg_name} [-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]\n
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
	std::process::exit(ecode);
}

fn version_courte(prg_name: &str, version: &str) {
	println!("{prg_name}: version {version}");
	std::process::exit(0);
}

fn version_longue(prg_name: &str, version: &str) {
	println!("{prg_name}: Renommage multiple selon un certain motif, version {version}");
	std::process::exit(0);
}

