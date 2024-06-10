use std::path::PathBuf;
use regex::Regex;
use crate::parse::Node;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Link {
	pub alias: String,
	pub path: PathBuf,
}

impl Link {
	pub fn new(alias: String, path: PathBuf) -> Self {
		Self {
			alias,
			path,
		}
	}
	pub fn get_alias(&self) -> &String {
		&self.alias
	}
	pub fn get_path(&self) -> &PathBuf {
		&self.path
	}
}


pub fn add_link_to_nodes(nodes: Vec<Node>, link: Link) -> Vec<Node> {
	let mut nodes = nodes;
	let mut index = 0;
	let dest = link.get_path();
	let text = link.get_alias();
	let mut links: Vec<(usize, Vec<Node>)> = Vec::new();
	for node in &mut nodes {
		match node {
			Node::BoldItalic(inner_nodes) => {
				*inner_nodes = crate::linking::add_link_to_nodes(inner_nodes.clone(), link.clone());
			}
			Node::Bold(inner_nodes) => {
				*inner_nodes = crate::linking::add_link_to_nodes(inner_nodes.clone(), link.clone());
			}
			Node::Italic(inner_nodes) => {
				*inner_nodes = crate::linking::add_link_to_nodes(inner_nodes.clone(), link.clone());
			}
			Node::String(string) => {
				if string.contains(text) {
					// let string_clone: String = (*string).to_string();
					let dest_str: String = dest.to_str().unwrap().to_string();

					let split_reg = Regex::new(&format!(r"\b{text}\b")).unwrap();
					let split = split_reg.split(&string);
					let mut nodes_after: Vec<Node> = Vec::new();

					for split_text in split {
						if split_text.is_empty() {
							nodes_after.push(Node::FormattedMarkdownLink(dest_str.clone(), text.to_string()));
						} else {
							nodes_after.push(Node::String(split_text.to_string()));
							nodes_after.push(Node::FormattedMarkdownLink(dest_str.clone(), text.to_string()));
						}
					}
					nodes_after.pop();
					links.push((index, nodes_after));
				}
			}
			_ => {}
		}
		index += 1;
	}

	let mut insertions: usize = 0;
	for (index, nodes_vec) in links {
		let len = nodes_vec.len();
		nodes.remove(index + insertions);
		nodes.splice(index + insertions..index + insertions, nodes_vec);
		insertions += len - 1;
	}

	let mut index: usize = 0;
	while index < nodes.len() {
		match &nodes[index] {
			Node::String(string) => {
				if string.is_empty() {
					nodes.remove(index);
				}
				else {
					index += 1;
				}
			}
			_ => {
				index += 1;
			}
		}
	}
	nodes
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Default)]
pub struct LinkerOptions {
	pub link_share_tag: bool,
	pub link_self: bool
}

impl LinkerOptions {
	#[allow(dead_code)]
	pub fn new(link_share_tag: bool, link_self: bool) -> Self {
		Self {
			link_share_tag,
			link_self
		}
	}
}

