#![allow(dead_code)]
use crate::parse::Node;
use std::collections::HashMap;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use rand::Rng;
use walkdir::WalkDir;
use crate::md_file::MDFile;
use crate::parse::Line;
use crate::linking::{add_link_to_nodes, Link, LinkerOptions};
use crate::stringtree::StringTree;

#[cfg(feature = "parallel")]
use rayon::prelude::*;


pub struct Vault {
	// Absolute path to the full_vault
	pub path: PathBuf,

	// Paths are relative to the full_vault
	pub data: HashMap<PathBuf, MDFile>,

	// Paths are relative to the full_vault
	pub alias_tree: StringTree<PathBuf>,

	// ignore directories
	pub ignore: Vec<PathBuf>,

	// link options
	pub options: LinkerOptions
}

impl Vault {
	pub fn new(vault_path: PathBuf, ignore: Vec<PathBuf>, options: LinkerOptions) -> Self {
		// if path is relative, convert to absolute
		let vault_path = match vault_path.is_absolute() {
			true => vault_path,
			false => std::env::current_dir().unwrap().join(vault_path)
		};

		// entries should be relative to the full_vault
		let ignore: Vec<PathBuf> = ignore.into_iter().map(|path| {
			let path = match path.is_absolute() {
				true => path,
				false => std::env::current_dir().unwrap().join(path)
			};
			path
		}).collect();

		// entries should be relative to the full_vault
		let entries: Vec<PathBuf> = WalkDir::new(&vault_path).into_iter().filter_map(|entry| {
			let entry = entry.unwrap();
			let path = entry.path().to_path_buf();
			if path.is_dir() {
				return None;
			}
			for ignore_path in &ignore {
				if path.starts_with(ignore_path) {
					return None;
				}
			}
			let path = path.strip_prefix(&vault_path).unwrap().to_path_buf();
			Some(path)
		}).collect();

		let data: HashMap<PathBuf, MDFile> = entries.into_iter().map(|path| {
			let md_file_path = vault_path.join(&path);
			let md_file = MDFile::from(md_file_path.to_path_buf()).expect(format!("Should be able to create MDFile: {}", path.display()).as_str());
			(path, md_file)
		}).collect();

		let mut alias_tree: crate::stringtree::StringTree<PathBuf> = crate::stringtree::StringTree::new();
		for md_file in data.values() {
			let mut aliases = md_file.get_aliases();
			aliases.push(md_file.get_title());

			for alias in aliases {
				let alias = alias.to_lowercase();
				let path = md_file.get_path().clone();
				let keys: Vec<String> = alias.split_whitespace().map(|s| s.to_string()).collect();
				alias_tree.insert(keys, path.clone());
			}
		}
		Self {
			path: vault_path,
			data,
			alias_tree,
			ignore,
			options
		}
	}

	pub fn get_links(&self) -> Vec<Link> {
		let links: Vec<Link> = self.data.iter().map(|(path, md_file)| {
			let title = md_file.get_title();
			let mut links = vec![Link::new(title.to_string(), path.to_owned())];
			for alias in md_file.get_aliases() {
				let link = Link::new(alias.to_string(), path.to_owned());
				links.push(link);
			}
			links
		}).flatten().collect();
		links
	}
	pub fn get_outgoing_links(&self, mdfile: &MDFile) -> Vec<Link> {
		let mut outgoing_links: Vec<Link> = Vec::new();
		let lines: Vec<&Line> = mdfile.get_lines();
		for line in lines {
			let original_strings: Vec<&String> = line.iterate_strings();
			let mut original_whitespace_strings: Vec<&str> = Vec::new();
			for string in &original_strings {
				let whitespace_strings: Vec<&str> = string.split_whitespace().collect();
				for whitespace_string in &whitespace_strings {
					let stripped = match whitespace_string.strip_suffix(",") {
						Some(s) => s,
						None => whitespace_string
					};
					let stripped = match stripped.strip_suffix(".") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix(":") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix(";") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix("!") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix("?") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix(")") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_prefix("(") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_prefix("[") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix("]") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_prefix("{") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix("}") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix("\"") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_prefix("\"") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_suffix("'") {
						Some(s) => s,
						None => stripped
					};
					let stripped = match stripped.strip_prefix("'") {
						Some(s) => s,
						None => stripped
					};
					original_whitespace_strings.push(stripped);
				}
				// original_whitespace_strings.extend(whitespace_strings);
			}

			let lowercase_strings: Vec<String> = original_whitespace_strings.iter().map(|s| s.to_lowercase()).collect();


			let len = original_whitespace_strings.len();
			let mut index = 0;
			while index < len {
				let keys: &[String] = &lowercase_strings[index..];
				let keys: Vec<&String> = keys.iter().collect();

				let result = self.alias_tree.get_best(keys);
				if result.is_none() {
					index += 1;
				} else {
					let (paths, fragment) = result.unwrap();
					if paths.len() > 1 {
						for path in paths {
							println!("Fragment Path: {:?}", path);
						}
						panic!("Fragment with multiple paths: {:?}, ", fragment);
					}
					for path in paths {
						let stripped_path = path.strip_prefix(&self.path).unwrap();
						if stripped_path == mdfile.get_path().strip_prefix(&self.path).unwrap() {
							match self.options.link_self {
								true => {
									let fragment_len = fragment.len();
									let original_fragment: Vec<String> = original_whitespace_strings[index..index + fragment_len].to_vec().iter().map(|s| s.to_string()).collect();
									let original_fragment: String = original_fragment.join(" ");
									let link: Link = Link::new(original_fragment, PathBuf::from(stripped_path));
									outgoing_links.push(link);
									index += fragment.len();
								}
								false => {
									index += 1;
								}
							}
						} else {
							let fragment_len = fragment.len();
							let original_fragment: Vec<String> = original_whitespace_strings[index..index + fragment_len].to_vec().iter().map(|s| s.to_string()).collect();
							let original_fragment: String = original_fragment.join(" ");
							let stripped_path = path.strip_prefix(&self.path).unwrap();
							let link: Link = Link::new(original_fragment, PathBuf::from(stripped_path));
							outgoing_links.push(link);
							index += fragment.len();
						}
					}
				}
			}
		}

		// let basename = mdfile.get_path().unwrap().file_name().unwrap().to_str().unwrap();
		// if basename == "AVL Tree.md" {
		// 	println!("Outgoing Links: {:?}", outgoing_links);
		// }

		// options filter link to self
		let outgoing_links: Vec<Link> = match self.options.link_self {
			true => outgoing_links,
			false => {
				outgoing_links.into_iter().filter(|link| {
					link.get_path() != mdfile.get_path()
				}).collect()
			}
		};

		// options filter tag share
		let mut outgoing_links: Vec<Link> = match self.options.link_share_tag {
			false => outgoing_links,
			true => {
				outgoing_links.into_iter().filter(|link| {
					let path = link.get_path();
					let link_md_file = self.get_md_file(path).unwrap();
					let link_tags = link_md_file.get_tags();
					let self_tags = mdfile.get_tags();
					let intersection: Vec<&String> = link_tags.iter().filter(|tag| self_tags.contains(tag)).collect();
					intersection.len() != 0
				}).collect()
			}
		};

		outgoing_links.dedup();

		return outgoing_links;
	}

	pub fn link_file(&mut self, md_file: &mut MDFile) {
		let outgoing_links: Vec<Link> = self.get_outgoing_links(md_file);
		// for (outgoing_path, fragment) in outgoing_links {
		for link in outgoing_links {

			let mut lines: Vec<&mut Line> = md_file.get_lines_mut();

			for line in &mut lines {
				let nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
				match nodes {
					Some(nodes) => {
						*nodes = add_link_to_nodes(nodes.clone(), link.clone());
					}
					None => {}
				}
			}
		}

	}


	pub fn link_all_files(self) -> Self {
		let mut self_owned = self;
		self_owned.link_all_files_mut_ref();
		self_owned
	}

	#[cfg(not(feature = "parallel"))]
	pub fn link_all_files_mut_ref(&mut self) {
		let outgoing_links: HashMap<PathBuf, Vec<Link>> = self.data.iter().map(|(path, md_file)| {
			let outgoing_links = self.get_outgoing_links(md_file);
			(path.clone(), outgoing_links)
		}).collect();

		let _ = self.data.iter_mut().map(|(path, md_file)| {
			let outgoing_links_local = outgoing_links.get(path).unwrap();

			for link in outgoing_links_local {
				md_file.link(link.clone());
			}
		}).collect::<Vec<()>>();
	}

	#[cfg(feature = "parallel")]
	pub fn link_all_files_ref_mut(&mut self) {
		let outgoing_links: HashMap<PathBuf, Vec<Link>> = self.data.par_iter().map(|(path, md_file)| {
			let outgoing_links = self.get_outgoing_links(md_file);
			(path.clone(), outgoing_links)
		}).collect();



		let _ = self.data.par_iter_mut().map(|(path, md_file)| {
			let outgoing_links_local = outgoing_links.get(path).unwrap();

			for link in outgoing_links_local {
				md_file.link(link.clone());
			}
		}).collect::<Vec<()>>();

	}

	pub fn get_md_file(&self, path: &Path) -> Result<&MDFile, String> {
		let vault_path = self.path.clone();

		// if path is absolute, strip prefix
		let path = match path.is_absolute() {
			true => {
				match path.strip_prefix(&vault_path) {
					Ok(path) => path,
					Err(_) => return Err(format!("Path: {:?} does not start with Vault Path: {:?}", path, vault_path))
				}
			},
			false => path
		};

		let data = self.data.get(path);
		match data {
			Some(md_file) => {
				return Ok(md_file)
			}
			None => {
				return Err(format!("No MDFile found for path: {}", path.display()))
			}
		}
	}

	pub fn get_md_file_mut(&mut self, path: &Path) -> Option<&mut MDFile> {
		self.data.get_mut(path)
	}

	pub fn random_md_file(&self) -> &MDFile {
		let mut rng = rand::thread_rng();
		let index = rng.gen_range(0..self.data.len());
		let md_file = self.data.values().nth(index).unwrap();
		md_file
	}

	pub fn unlink_md_file(&mut self, path: &Path) -> Result<(), String> {
		let md_file = self.get_md_file_mut(path);
		match md_file {
			Some(md_file) => {
				md_file.unlink();
				return Ok(())
			}
			None => {
				return Err(format!("No MDFile found for path: {}", path.display()))
			}
		}
	}

	pub fn get_path(&self) -> &PathBuf {
		&self.path
	}

	pub fn unlink_all_files(self) -> Self {
		let mut self_owned = self;
		self_owned.unlink_all_files_mut_ref();
		self_owned
	}

	pub fn unlink_all_files_mut_ref(&mut self) {
		let _ = self.data.iter_mut().map(|(_, md_file)| {
			md_file.unlink();
		}).collect::<Vec<()>>();
	}





	pub fn export(&self, vault_path: &Path) {
		let mut vault_path = vault_path.to_path_buf();
		std::fs::create_dir_all(vault_path.clone()).unwrap();
		for (path, md_file) in self.data.iter() {
			let output_path = vault_path.join(path);
			let string = md_file.to_string();
			std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
			std::fs::write(output_path, string).unwrap();
		}
	}

	pub fn alias_tree_string(&self) -> String {
		format!("{}", self.alias_tree)
	}

}

