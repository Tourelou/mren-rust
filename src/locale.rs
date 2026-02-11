// locale.rs

use std::env;


const OPTIONS_FR: &str =
r#"[-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]

Renommage multiple selon un certain motif.

Arguments en position:
  <motif regex>     Motif à chercher : Mettre entre guillemets '...'
  <remplacement>    Chaîne de remplacement. Doit obligatoirement suivre le motif.
  [dirname ...]     Répertoire(s) de recherche.

Options:
  -f                  N'agit que sur les fichiers.
  -d                  N'agit que sur les répertoires.
  -r, --recursive     Procède de façon récursive sur les répertoires.
  -i, --include       En mode récursif, inclut le dossier en ligne de commande.
  -I, --ignoreCase    Fait une recherche en ignorant la casse.
  -n, --simulate      Simule les opérations demandées - Fichiers affectés en VERT.
  -v, --verbose       Donne des détails sur le(s) fichier(s) traité(s) - Fichiers affectés en ROUGE.
  -V, --version       Affiche la version.
  -h, --help          Montre ce message d'aide et termine."#;

const OPTIONS_ES: &str =
r#"[-f|-d] [-riInv] <patrón regex> <reemplazo> [dirname ...]

Cambio de nombre múltiple según un patrón.

Argumentos posicionales:
  <patrón regex>    Patrón de búsqueda: Poner entre comillas '...'
  <reemplazo>       Cadena de reemplazo. Debe seguir al patrón.
  [dirname ...]     Directorio(s) de búsqueda.

Opciones:
  -f                  Solo afecta a los archivos.
  -d                  Solo afecta a los directorios.
  -r, --recursive     Recursivo en los directorios.
  -i, --include       En modo recursivo, incluye la carpeta de la línea de comandos.
  -I, --ignoreCase    Ignorar distinción entre mayúsculas y minúsculas.
  -n, --simulate      Simula las operaciones - Archivos afectados en VERDE.
  -v, --verbose       Detalles sobre el/los archivo(s) procesado(s) - Afectados en ROJO.
  -V, --version       Muestra la versión.
  -h, --help          Muestra este mensaje de ayuda y finaliza."#;

const OPTIONS_EN: &str =
r#"[-f|-d] [-riInv] <regex pattern> <replacement> [dirname ...]

Multiple renaming from a pattern.

Positional arguments:
  <regex pattern>   Pattern to search: Put into single quotes '...'
  <replacement>     Replacement string. Must follow the pattern.
  [dirname ...]     Path(s) to search.

Options:
  -f                  Search only for files.
  -d                  Search only for folders.
  -r, --recursive     Recursively works through directories.
  -i, --include       In recursive mode, include the folder passed on the command line.
  -I, --ignoreCase    Search ignoring case.
  -n, --simulate      Simulates the requested operations - Affected files in GREEN.
  -v, --verbose       Gives details of the processed file(s) - Affected files in RED.
  -V, --version       Show version information.
  -h, --help          Show this help message and exit."#;

pub struct LangStrings {
	pub err_opt_format: &'static str,
	pub err_opt_inv: &'static str,
	pub err_mutuel: &'static str,
	pub err_regex_manque: &'static str,
	pub err_rempl_manque: &'static str,
	pub err_dir_invalide: &'static str,
	pub err_chdir: &'static str,
	pub err_chdir_parent: &'static str,
	pub process_dir: &'static str,
	pub ren_src_dir: &'static str,
	pub nouveau_path: &'static str,
	pub no_match: &'static str,
	pub devient: &'static str,
	pub devenu: &'static str,
	pub err_renom: &'static str,
	pub err_trop_recurs: &'static str,
	pub err_read_dir: &'static str,
	pub err_entry: &'static str,
	pub usage: &'static str,
	pub options: &'static str,
	pub ver: &'static str,
	pub ver_desc: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	err_opt_format: "Erreur : Ligne de commande mal formatée",
	err_opt_inv: "Erreur : L'option '{1}' n'est pas reconnue.",
	err_mutuel: "-f et -d sont mutuellement exclusif. C'est un ou c'est l'autre.",
	err_regex_manque: "Erreur : motif regex manquant.",
	err_rempl_manque: "Erreur : chaîne de remplacement manquante.",
	err_dir_invalide: "n'est pas un répertoire valide.",
	err_chdir: "Erreur changement de répertoire vers",
	err_chdir_parent: "Erreur en revenant au dossier parent:",
	process_dir: "Traitement du répertoire",
	ren_src_dir: "- - - - -\nRenommage du répertoire source",
	nouveau_path: "Nouveau chemin absolu :",
	no_match: "Pas de correspondance dans ce dossier",
	devient: "==> Deviendrait ==>",
	devenu: "==> est devenu ==>",
	err_renom: "Erreur lors du renommage de",
	err_trop_recurs: "--- Trop de récursion (>100) ---",
	err_read_dir: "Erreur de lecture du répertoire:",
	err_entry: "Entrée illisible:",
	usage: "[-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]",
	options: OPTIONS_FR,
	ver: ": version",
	ver_desc: ": Renommage multiple selon un certain motif, version",
};

pub const LANG_ES: LangStrings = LangStrings {
	err_opt_format: "Error: Línea de comandos mal formada.",
	err_opt_inv: "Error: Opción '{1}' desconocida.",
	err_mutuel: "-f y -d son mutuamente excluyentes. Es una o la otra.",
	err_regex_manque: "Error: Falta el patrón de expresión regular.",
	err_rempl_manque: "Error: Falta la cadena de reemplazo.",
	err_dir_invalide: "no es un directorio válido.",
	err_chdir: "Error al cambiar el directorio a",
	err_chdir_parent: "Error al regresar a la carpeta principal:",
	process_dir: "Procesando el directorio",
	ren_src_dir: "- - - - -\nCambiar el nombre del directorio de origen",
	nouveau_path: "Nueva ruta absoluta :",
	no_match: "No hay coincidencias en esta carpeta",
	devient: "==> Se convertiría en ==>",
	devenu: "==> se convirtió en ==>",
	err_renom: "Error al renombrar",
	err_trop_recurs: "--- Demasiada recursión (>100) ---",
	err_read_dir: "Error al leer el directorio:",
	err_entry: "Entrada ilegible:",
	usage: "[-f|-d] [-riInv] <patrón de expresión regular> <reemplazo> [nombredirectorio ...]",
	options: OPTIONS_ES,
	ver: ": versión",
	ver_desc: ": Cambio de nombre múltiple basado en un patrón determinado, versión",
};

pub const LANG_EN: LangStrings = LangStrings {
	err_opt_format: "Error: Malformed command line.",
	err_opt_inv: "Error: Unknown option '{1}'.",
	err_mutuel: "-f and -d are mutually exclusive. It's either one or the other.",
	err_regex_manque: "Error : Missing regex pattern.",
	err_rempl_manque: "Error : Missing replacement string.",
	err_dir_invalide: "is not a valid directory.",
	err_chdir: "Error changing directory to",
	err_chdir_parent: "Error returning to parent folder:",
	process_dir: "Processing directory",
	ren_src_dir: "- - - - -\nRenaming the source directory",
	nouveau_path: "New absolute path :",
	no_match: "No matches in this folder",
	devient: "==> Would become ==>",
	devenu: "==> became ==>",
	err_renom: "Error renaming",
	err_trop_recurs: "--- Too much recursion (>100) ---",
	err_read_dir: "Error reading directory:",
	err_entry: "Illegible entry:",
	usage: "[-f|-d] [-riInv] <regex pattern> <replacement> [dirname ...]",
	options: OPTIONS_EN,
	ver: ": version",
	ver_desc: ": Multiple renaming based on a certain pattern, version",
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
