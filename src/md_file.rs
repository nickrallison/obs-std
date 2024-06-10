use std::fmt::Display;
use std::path::PathBuf;
use crate::linking::Link;
use crate::parse::{AST, Line, Node};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct MDFile {
	pub path: PathBuf,
	pub title: String,
	pub aliases: Vec<String>,
	pub ast: AST,
	pub last_modified: Option<std::time::SystemTime>,
}

#[allow(dead_code)]
impl MDFile {

	// Constructor Methods
	pub fn from(file_path: PathBuf) -> Result<MDFile, String> {
		assert!(file_path.is_file(), "Path is not a file: {}", file_path.display());
		let file_contents = match std::fs::read_to_string(&file_path) {
			Ok(contents) => contents,
			Err(_) => return Err(format!("Error reading file: {}", file_path.display())),
		};

		let title: String = file_path.file_stem().unwrap().to_str().expect(&format!("Can't convert path to &str: {}", file_path.display())).to_string();
		let last_modified = match std::fs::metadata(&file_path) {
			Ok(metadata) => metadata.modified().unwrap(),
			Err(_) => return Err(format!("Error getting last modified time from file: {}", file_path.display())),
		};

		let mut md_file = MDFile::new(file_path, title, file_contents);

		md_file.set_last_modified(Some(last_modified));

		Ok(md_file)
	}

	pub fn new(path: PathBuf, title: String, string: String) -> MDFile {

		let ast: AST = AST::new(string);
		let aliases: Vec<String> = ast.get_aliases();

		MDFile {
			path,
			title,
			aliases,
			ast,
			last_modified: None,
		}
	}


	// Getter Methods

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_aliases(&self) -> Vec<&String> {
		self.aliases.iter().collect()
	}

	pub fn get_path(&self) -> &PathBuf {
		&self.path
	}

	pub fn get_lines(&self) -> Vec<&Line> {
		self.ast.get_lines()
	}

	pub fn get_lines_mut(&mut self) -> Vec<&mut Line> {
		self.ast.get_lines_mut()
	}

	pub fn get_tags(&self) -> Vec<String> {
		self.ast.get_tags()
	}

	// Setter Methods
	pub fn set_title(&mut self, title: String) {
		self.title = title;
	}
	pub fn set_aliases(&mut self, aliases: Vec<String>) {
		self.aliases = aliases;
	}
	pub fn set_path(&mut self, path: PathBuf) {
		self.path = path;
	}
	pub fn set_last_modified(&mut self, last_modified: Option<std::time::SystemTime>) {
		self.last_modified = last_modified;
	}

	// IO Methods
	pub fn link(&mut self, link: Link) {
		let mut lines: Vec<&mut Line> = self.get_lines_mut();
		for line in &mut lines {
			let nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
			match nodes {
				Some(nodes) => {
					*nodes = crate::linking::add_link_to_nodes(nodes.clone(), link.clone());
				}
				None => {}
			}
		}
	}
	pub fn link_noself(&mut self, link: Link) {

		let self_path_basename = self.path.file_stem().unwrap().to_str().unwrap();
		let link_path_basename = link.get_path().file_stem().unwrap().to_str().unwrap();
		if self_path_basename == link_path_basename {
			return;
		}

		let mut lines: Vec<&mut Line> = self.get_lines_mut();
		for line in &mut lines {
			let nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
			match nodes {
				Some(nodes) => {
					*nodes = crate::linking::add_link_to_nodes(nodes.clone(), link.clone());
				}
				None => {}
			}
		}
	}
	pub fn unlink(&mut self) {
		let lines: Vec<&mut Line> = self.get_lines_mut();
		for line in lines {
			if let Some(nodes) = line.get_nodes_mut() {
				*nodes = nodes.clone().iter().map(|node| {
					match node {
						Node::FormattedMarkdownLink(_, text) => {
							Node::String(text.clone())
						}
						_ => node.to_owned()
					}
				}).collect();

				*nodes = nodes.clone().iter().fold(vec![], |mut acc, node| {
					match node {
						Node::String(text) => {
							if let Some(Node::String(last_text)) = acc.clone().last() {
								acc.pop();
								acc.push(Node::String(format!("{}{}", last_text, text)));
							} else {
								acc.push(Node::String(text.clone()));
							}
						}
						_ => {
							acc.push(node.to_owned());
						}
					}
					acc
				});

			}
		}

	}

	pub fn export_to_file(&self, path: &PathBuf) -> Result<(), String> {
		match std::fs::write(&path, self.to_string()) {
			Ok(_) => Ok(()),
			Err(_) => Err(format!("Error writing to file: {}", path.display())),
		}
	}

	pub fn export(&self) -> Result<(), String>  {
		self.export_to_file(self.get_path())
	}
}

impl Display for MDFile {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.ast.to_string())
	}
}