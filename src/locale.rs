// locale.rs

use std::env;


const OPTIONS_EN: &str =
r#"    -f,   --find  <regex pattern> : Pattern to search: Put in single cote '...'.
	-s,   --scan  <folder>        : Path to volume/folder to scan.
	-ver, --version               : Program info version.
	-h,   --help                  : Help message then exit."#;

const OPTIONS_FR: &str =
r#"    -f,   --find  <motif regex>   : Motif à chercher: Mettre entre '...'.
	-s,   --scan  <dossier>       : Analyse un dossier.
	-ver, --version               : Affiche la version du programme.
	-h,   --help                  : Affiche ce message d'aide."#;

const OPTIONS_ES: &str =
r#"    -f,   --find     <patrón regex>       : Patrón a buscar: Poner entre '...'
	-s,   --scan     <volumen a escanear> : Ruta del directorio a escanear.
	-ver, --version                       : Proporciona información de la versión del programa.
	-h,   --help                          : Muestra este mensaje de ayuda y finaliza."#;

#[derive(Debug)]
pub struct LangStrings {
	pub options: &'static str,
	pub usage: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	options: OPTIONS_FR,
	usage: "Utilisation",
};

pub const LANG_ES: LangStrings = LangStrings {
	options: OPTIONS_ES,
	usage: "Uso",
};

pub const LANG_EN: LangStrings = LangStrings {
	options: OPTIONS_EN,
	usage: "Usage",
};

pub fn set_lang_vec() -> LangStrings {
	match get_system_lang().as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	}
}

fn get_system_lang() -> String {
	let raw_lang = std::env::var("LC_ALL")
		.or_else(|_| env::var("LANG"))
		.or_else(|_| env::var("LANGUAGE"))
		.unwrap_or_else(|_| "en".to_string()); // Langue par défaut (anglais)

	// Extraire uniquement le code de langue avant le premier '_'
	let lang_code = raw_lang.split('_').next().unwrap_or(&raw_lang);
	lang_code.to_string() // Retourne "fr" au lieu de "fr_CA.UTF-8"
}
