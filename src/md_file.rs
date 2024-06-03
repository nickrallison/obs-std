use std::fmt::Display;
use std::path::PathBuf;
use crate::parse::AST;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MDFile {
	path: Option<PathBuf>,
	title: Option<String>,
	aliases: Vec<String>,
	ast: AST,
	last_modified: Option<std::time::SystemTime>,
}

#[allow(dead_code)]
impl MDFile {
	pub fn from(path: PathBuf) -> Result<MDFile, String> {
		assert!(path.is_file());
		let file_contents = match std::fs::read_to_string(&path) {
			Ok(contents) => contents,
			Err(_) => return Err(format!("Error reading file: {}", path.display())),
		};
		let mut md_file = MDFile::from_string(file_contents);

		let title: String = path.file_stem().unwrap().to_str().expect(&format!("Can't convert path to &str: {}", path.display())).to_string();
		let last_modified = match std::fs::metadata(&path) {
			Ok(metadata) => metadata.modified().unwrap(),
			Err(_) => return Err(format!("Error getting last modified time from file: {}", path.display())),
		};

		md_file.set_title(Some(title));
		md_file.set_path(Some(path));

		Ok(md_file)
	}

	pub fn from_string(string: String) -> MDFile {

		let ast: AST = AST::new(string);
		let aliases: Vec<String> = ast.get_aliases();

		MDFile {
			path: None,
			title: None,
			aliases,
			ast,
			last_modified: None,
		}
	}

	// Getter Methods

	pub fn get_title(&self) -> Option<&String> {
		self.title.as_ref()
	}

	pub fn get_aliases(&self) -> Vec<&String> {
		self.aliases.iter().collect()
	}

	pub fn get_path(&self) -> Option<&PathBuf> {
		self.path.as_ref()
	}

	pub fn get_lines(&self) -> Vec<&crate::parse::Line> {
		self.ast.get_lines()
	}

	pub fn get_lines_mut(&mut self) -> Vec<&mut crate::parse::Line> {
		self.ast.get_lines_mut()
	}

	// Setter Methods
	pub fn set_title(&mut self, title: Option<String>) {
		self.title = title;
	}
	pub fn set_aliases(&mut self, aliases: Vec<String>) {
		self.aliases = aliases;
	}
	pub fn set_path(&mut self, path: Option<PathBuf>) {
		self.path = path;
	}
	pub fn set_last_modified(&mut self, last_modified: Option<std::time::SystemTime>) {
		self.last_modified = last_modified;
	}
}

impl Display for MDFile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.ast.to_string())
	}
}


