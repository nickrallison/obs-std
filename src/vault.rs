#![allow(dead_code)]
use crate::parse::Node;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use itertools::Itertools;
use normpath::PathExt;
// use normpath::PathExt;
use rand::Rng;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize, Serializer};
use crate::md_file::MDFile;
use crate::parse::Line;
use crate::linking::{add_link_to_nodes, Link, LinkerOptions};
use crate::stringtree::StringTree;

// #[cfg(feature = "parallel")]
// use rayon::prelude::*;


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default, Debug)]
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
	pub options: LinkerOptions,

	// db path
	pub db_path: Option<PathBuf>
}

impl Vault {
	pub fn new(vault_path: PathBuf, ignore: Vec<PathBuf>, options: LinkerOptions, db_path: Option<PathBuf>) -> Self {

		// if db_path is relative, convert to absolute
		let db_path: Option<PathBuf> = match db_path {
			Some(db_path) => {
				match db_path.is_absolute() {
					true => Some(db_path),
					false => Some(std::env::current_dir().unwrap().join(db_path))
				}
			}
			None => None
		};

		// if db_path contains a valid file, deserialize it into a vault
		if !db_path.is_none() && db_path.as_ref().unwrap().exists() {
			let vault = Self::from_json(db_path.as_ref().unwrap());
			match vault {
				Ok(mut vault) => {
					let vault = vault.update();
					return vault;
				}
				Err(e) => {
					println!("Error deserializing db_path: {db_path:?}\nerror: {e}");
				}
			}
		}

		// if vault_path is relative, convert to absolute
		let vault_path = match vault_path.is_absolute() {
			true => vault_path.clone(),
			false => std::env::current_dir().unwrap().join(vault_path)
		};
		let vault_path = PathBuf::from(vault_path.normalize().expect("Failed to normalize vault_path"));

		// entries should be relative to the full_vault
		let ignore: Vec<PathBuf> = ignore.into_iter().map(|path| {
			match path.is_absolute() {
				true => path,
				false => std::env::current_dir().unwrap().join(path)
			}
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
			let path: PathBuf = path.strip_prefix(&vault_path).unwrap().to_path_buf();
			Some(path)
		}).collect();

		let data: HashMap<PathBuf, MDFile> = entries.into_iter().map(|path| {
			let md_file_path = vault_path.join(&path);
			let md_file = MDFile::from(md_file_path);
			(path, md_file)
		}).collect();

		let mut alias_tree: StringTree<PathBuf> = StringTree::new();
		for md_file in data.values() {
			let mut aliases = md_file.get_aliases();
			aliases.push(md_file.get_title());

			for alias in aliases {
				let alias = alias.to_lowercase();
				let path = md_file.get_path().clone();
				let keys: Vec<String> = alias.split_whitespace().map(std::string::ToString::to_string).collect();
				alias_tree.insert(keys, path.clone());
			}
		}
		Self {
			path: vault_path,
			data,
			alias_tree,
			ignore,
			options,
			db_path
		}
	}

	pub fn from_json(path_to_json: &Path) -> Result<Self, Box<dyn std::error::Error>> {
		let vault: Self = serde_json::from_str(&std::fs::read_to_string(path_to_json)?)?;
		Ok(vault)
	}

	pub fn to_json(self, path_to_json: &Path) -> Result<(), Box<dyn std::error::Error>> {
		let vault = self;
		std::fs::write(path_to_json, serde_json::to_string_pretty(&vault)?).unwrap();
		Ok(())
	}

	pub fn get_links(&self) -> Vec<Link> {
		let links: Vec<Link> = self.data.iter().flat_map(|(path, md_file)| {
			let title = md_file.get_title();
			let mut links = vec![Link::new(title.to_string(), path.to_owned())];
			for alias in md_file.get_aliases() {
				let link = Link::new(alias.to_string(), path.to_owned());
				links.push(link);
			}
			links
		}).collect();
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
							println!("Fragment Path: {path:?}");
						}
						panic!("Fragment with multiple paths: {fragment:?}, ");
					}
					for path in paths {
						let stripped_path = path.strip_prefix(&self.path).unwrap();
						if stripped_path == mdfile.get_path().strip_prefix(&self.path).unwrap() {
							match self.options.link_self {
								true => {
									let fragment_len = fragment.len();
									let original_fragment: Vec<String> = original_whitespace_strings[index..index + fragment_len].to_vec().iter().map(|s| (*s).to_string()).collect();
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
							let original_fragment: Vec<String> = original_whitespace_strings[index..index + fragment_len].to_vec().iter().map(|s| (*s).to_string()).collect();
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
					!intersection.is_empty()
				}).collect()
			}
		};

		outgoing_links.dedup();

		outgoing_links
	}

	pub fn link_file(&mut self, md_file: &mut MDFile) {
		let outgoing_links: Vec<Link> = self.get_outgoing_links(md_file);
		// for (outgoing_path, fragment) in outgoing_links {
		for link in outgoing_links {

			let mut lines: Vec<&mut Line> = md_file.get_lines_mut();

			for line in &mut lines {
				let nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
				if let Some(nodes) = nodes {
    						*nodes = add_link_to_nodes(nodes.clone(), link.clone());
    					}
			}
		}

	}


	pub fn link_all_files(self) -> Self {
		let mut self_owned = self;
		self_owned.link_all_files_mut_ref();
		self_owned
	}

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

	pub fn get_md_file(&self, path: &Path) -> Result<&MDFile, String> {
		let vault_path = self.path.clone();

		// if path is absolute, strip prefix
		let path = match path.is_absolute() {
			true => {
				match path.strip_prefix(&vault_path) {
					Ok(path) => path,
					Err(_) => return Err(format!("Path: {path:?} does not start with Vault Path: {vault_path:?}"))
				}
			},
			false => path
		};

		let data = self.data.get(path);
		match data {
			Some(md_file) => {
				Ok(md_file)
			}
			None => {
				Err(format!("No MDFile found for path: {}", path.display()))
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
				Ok(())
			}
			None => {
				Err(format!("No MDFile found for path: {}", path.display()))
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
		let vault_path = vault_path.to_path_buf();
		std::fs::create_dir_all(vault_path.clone()).unwrap();
		for (path, md_file) in &self.data {
			let output_path = vault_path.join(path);
			let string = md_file.to_string();
			std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
			std::fs::write(output_path, string).unwrap();
		}
	}

	pub fn update(self) -> Self {
		let mut vault = self;
		let mut removals: Vec<PathBuf> = Vec::new();
		for (file, md_file) in &mut vault.data {
			let path = vault.path.join(file);
			// if path does not exist as a file, remove it from the vault
			if !path.exists() {
				removals.push(file.clone());
				continue;
			}
			md_file.update();
		}
		for file in removals {
			vault.data.remove(&file);
		}
		vault
	}

	pub fn alias_tree_string(&self) -> String {
		format!("{}", self.alias_tree)
	}

}

// #[cfg(test)]
// mod vault_tests {
// 	use super::*;
// 	use std::path::PathBuf;
// 	use crate::md_file::MDFile;
// 	use crate::linking::LinkerOptions;
//
// 	#[test]
// 	fn test_deserialize_vault() {
// 		let vault_path = PathBuf::from("test_vaults/reference_clean");
// 		let ignore = vec![];
// 		let options = LinkerOptions::default();
// 		let db_path = None;
// 		let vault = Vault::new(vault_path, ignore, options, db_path);
// 		let vault_json = serde_json::to_string_pretty(&vault).unwrap();
// 		let mut vault_deserialized: Vault = serde_json::from_str(&vault_json).unwrap();
// 		let vault_deserialized = vault_deserialized.update();
// 		if vault != vault_deserialized {
// 			for (path, md_file) in &vault.data {
// 				let md_file_deserialized = vault_deserialized.get_md_file(path).unwrap();
// 				if md_file != md_file_deserialized {
// 					println!("MDFiles are not equal: {}", path.display());
// 				}
// 			}
// 			panic!("Vaults are not equal")
// 		}
// 		let vault_deserialized_json = serde_json::to_string_pretty(&vault_deserialized).unwrap();
//
//
// 	}
// }
//
