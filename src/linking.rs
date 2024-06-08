use std::path::PathBuf;
use regex::Regex;
use crate::md_file::Link;
use crate::parse::Node;

pub(crate) fn add_link_to_nodes(nodes: Vec<Node>, link: Link) -> Vec<Node> {
	let mut nodes = nodes;
	let mut index = 0;
	let dest = link.get_path();
	let text = link.get_alias();
	let mut links: Vec<(usize, Vec<Node>)> = Vec::new();
	for node in nodes.iter_mut() {
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
					let string_clone: String = string.to_string();
					let dest_str: String = dest.to_str().unwrap().to_string();
					// if text == "binary search tree" {
					// 	println!("{}: {}", text, string_clone);
					// }

					// Case Insensitive
					// let split_reg = Regex::new(&format!(r"(?i)\b{}\b", text)).unwrap();
					let split_reg = Regex::new(&format!(r"\b{}\b", text)).unwrap();
					let split = split_reg.split(&string_clone);
					// let string_clone_2: String = string.to_string();
					// let split_2 = split_reg.split(&string_clone_2);
					// let parts_temp: Vec<&str> = split_2.collect();
					// if text == "binary search tree" {
					// 	println!("Split: {:?}", parts_temp);
					// }
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
				} else {
					// do nothing
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