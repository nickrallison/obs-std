use std::fmt::Display;
use std::path::PathBuf;
use crate::parse::AST;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MDFile {

	path: PathBuf,
	title: String,
	aliases: Vec<String>,
	ast: AST,
	last_modified: std::time::SystemTime,
}

#[allow(dead_code)]
impl MDFile {
	pub(crate) fn new(path: PathBuf) -> MDFile {
		// println!("Path: {:?}", path);
		let title: String = path.file_stem().unwrap().to_str().unwrap().to_string();
		let file_contents: String = std::fs::read_to_string(&path).unwrap();
		let ast: AST = AST::new(file_contents);
		let aliases: Vec<String> = ast.get_aliases();
		let last_modified = std::fs::metadata(&path).expect(&format!("Can't access file or doesn't exit: {}", &path.display())).modified().unwrap();

		MDFile {
			path,
			title,
			aliases,
			ast,
			last_modified,
		}
	}
	pub(crate) fn get_title(&self) -> &String {
		&self.title
	}

	pub(crate) fn get_aliases(&self) -> Vec<&String> {
		self.aliases.iter().collect()
	}

	pub(crate) fn get_path(&self) -> &PathBuf {
		&self.path
	}

	pub(crate) fn get_lines(&self) -> Vec<&crate::parse::Line> {
		self.ast.get_lines()
	}
}

impl Display for MDFile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.ast.to_string())
	}
}


