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
}

#[allow(dead_code)]
impl MDFile {
	pub(crate) fn new(path: PathBuf) -> MDFile {
		let title: String = path.file_stem().unwrap().to_str().unwrap().to_string();
		let file_contents: String = std::fs::read_to_string(&path).unwrap();
		let ast: AST = AST::new(file_contents);
		let aliases: Vec<String> = ast.get_aliases();


		MDFile {
			path,
			title,
			aliases,
			ast,

		}
	}

	// pub(crate) fn lines(&self) -> Vec<String> {
	// 	self.ast.lines()
	// }

	// pub(crate) fn num_lines (&self) -> usize {
	// 	self.ast.num_lines()
	// }
}

impl Display for MDFile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.ast.to_string())
	}
}

// impl Verbose for MDFile {
// 	fn verbose(&self) -> String {
// 		format!("Path:\n\t{}\nTitle:\n\t{}\nAliases:\n\t{:?}\nAST:\n{}", self.path.display(), self.title, self.aliases, add_indentation("\t", &self.ast.verbose()))
// 	}
// }

