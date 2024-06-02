use std::fmt::Display;
use std::io::Error;
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
	pub(crate) fn from(path: PathBuf) -> Result<MDFile, String> {
		let title: String = path.file_stem().unwrap().to_str().unwrap().to_string();
		let file_contents_result = std::fs::read_to_string(&path);
		let file_contents = match file_contents_result {
			Ok(contents) => contents,
			Err(_) => return Err(format!("Error reading file: {}", path.display())),
		};
		let ast: AST = AST::new(file_contents);
		let aliases: Vec<String> = ast.get_aliases();
		let last_modified_result = std::fs::metadata(&path).expect(&format!("Can't access file or doesn't exit: {}", &path.display())).modified();
		let last_modified = match last_modified_result {
			Ok(time) => time,
			Err(_) => return Err(format!("Error getting last modified time from file: {}", path.display())),
		};

		Ok(MDFile {
			path,
			title,
			aliases,
			ast,
			last_modified,
		})
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

	pub(crate) fn get_lines_mut(&mut self) -> Vec<&mut crate::parse::Line> {
		self.ast.get_lines_mut()
	}
}

impl Display for MDFile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.ast.to_string())
	}
}


