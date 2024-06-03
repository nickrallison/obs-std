#![allow(dead_code)]
use crate::parse::Node;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use rand::Rng;
use walkdir::WalkDir;
use crate::md_file::{Link, MDFile};
use crate::parse::Line;
use crate::linking::add_link_to_nodes;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use crate::options::LinkerOptions;


pub struct Vault {
	// Absolute path to the vault
	path: PathBuf,

	// Paths are relative to the vault
	data: HashMap<PathBuf, MDFile>,

	// Paths are relative to the vault
	alias_tree: StringTree<PathBuf>,

	// ignore directories
	ignore: Vec<PathBuf>,

	// link options
	options: LinkerOptions
}

impl Vault {
	#[cfg(not(feature = "parallel"))]
	pub fn new(vault_path: PathBuf, ignore: Vec<PathBuf>, options: LinkerOptions) -> Self {
		// if path is relative, convert to absolute
		let vault_path = match vault_path.is_absolute() {
			true => vault_path,
			false => std::env::current_dir().unwrap().join(vault_path)
		};

		// entries should be relative to the vault
		let ignore: Vec<PathBuf> = ignore.into_iter().map(|path| {
			let path = match path.is_absolute() {
				true => path,
				false => std::env::current_dir().unwrap().join(path)
			};
			path
		}).collect();

		// entries should be relative to the vault
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

		let mut alias_tree: crate::vault::StringTree<PathBuf> = crate::vault::StringTree::new();
		for md_file in data.values() {
			let mut aliases = md_file.get_aliases();
			match md_file.get_title() {
				Some(title) => {
					aliases.push(title);
				}
				None => {}
			}
			for alias in aliases {
				let alias = alias.to_lowercase();
				let path = md_file.get_path().clone();
				let keys: Vec<String> = alias.split_whitespace().map(|s| s.to_string()).collect();
				match path {
					Some(path) => {
						alias_tree.insert(keys, path.clone());
					}
					None => {
						panic!("Path is None for alias: {}", alias);
					}
				}
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

	#[cfg(feature = "parallel")]
	pub fn new(vault_path: PathBuf, ignore: Vec<PathBuf>, options: LinkerOptions) -> Self {
		// if path is relative, convert to absolute
		let vault_path = match vault_path.is_absolute() {
			true => vault_path,
			false => std::env::current_dir().unwrap().join(vault_path)
		};

		// entries should be relative to the vault
		let ignore: Vec<PathBuf> = ignore.into_iter().map(|path| {
			let path = match path.is_absolute() {
				true => path,
				false => std::env::current_dir().unwrap().join(path)
			};
			path
		}).collect();

		// entries should be relative to the vault
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

		let data: HashMap<PathBuf, MDFile> = entries.into_par_iter().map(|path| {
			let md_file_path = vault_path.join(&path);
			let md_file = MDFile::from(md_file_path.to_path_buf()).expect(format!("Should be able to create MDFile: {}", path.display()).as_str());
			(path, md_file)
		}).collect();

		let mut alias_tree: StringTree<PathBuf> = StringTree::new();
		for md_file in data.values() {
			let mut aliases = md_file.get_aliases();
			match md_file.get_title() {
				Some(title) => {
					aliases.push(title);
				}
				None => {}
			}
			for alias in aliases {
				let alias = alias.to_lowercase();
				let path = md_file.get_path().clone();
				let keys: Vec<String> = alias.split_whitespace().map(|s| s.to_string()).collect();
				match path {
					Some(path) => {
						alias_tree.insert(keys, path.clone());
					}
					None => {
						panic!("Path is None for alias: {}", alias);
					}
				}
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

	#[cfg(not(feature = "parallel"))]
	pub fn get_links(&self) -> Vec<Link> {
		let links: Vec<Link> = self.data.iter().map(|(path, md_file)| {
			let title = md_file.get_title();
			let mut links = match title {
				Some(title) => {
					let link = Link::new(title.to_string(), path.to_owned());
					vec![link]
				}
				None => {
					vec![]
				}
			};
			for alias in md_file.get_aliases() {
				let link = Link::new(alias.to_string(), path.to_owned());
				links.push(link);
			}
			links
		}).flatten().collect();
		links
	}

	#[cfg(feature = "parallel")]
	pub fn get_links(&self) -> Vec<Link> {
		let links: Vec<Link> = self.data.par_iter().map(|(path, md_file)| {
			let title = md_file.get_title();
			let mut links = match title {
				Some(title) => {
					let link = Link::new(title.to_string(), path.to_owned());
					vec![link]
				}
				None => {
					vec![]
				}
			};
			for alias in md_file.get_aliases() {
				let link = Link::new(alias.to_string(), path.to_owned());
				links.push(link);
			}
			links
		}).flatten().collect();
		links
	}

	#[cfg(not(feature = "parallel"))]
	pub(crate) fn get_outgoing_links(&self, mdfile: &MDFile) -> Vec<Link> {
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
				}
				else {
					let (paths, fragment) = result.unwrap();
					if paths.len() > 1 {
						for path in paths {
							println!("Fragment Path: {:?}", path);
						}
						panic!("Fragment with multiple paths: {:?}, ", fragment);
					}
					for path in paths {
						let fragment_len = fragment.len();
						let original_fragment: Vec<String> = original_whitespace_strings[index..index+fragment_len].to_vec().iter().map(|s| s.to_string()).collect();
						let original_fragment: String = original_fragment.join(" ");
						let stripped_path = path.strip_prefix(&self.path).unwrap();
						let link: Link = Link::new(original_fragment, PathBuf::from(stripped_path));
						outgoing_links.push(link);
					}
					index += fragment.len();
				}
			}
		}

		// options filter link to self
		let outgoing_links: Vec<Link> = match self.options.link_self {
			true => outgoing_links,
			false => {
				outgoing_links.into_iter().filter(|link| {
					link.get_path() != mdfile.get_path().unwrap()
				}).collect()
			}
		};

		// options filter tag share
		let outgoing_links: Vec<Link> = match self.options.link_share_tag {
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


		return outgoing_links;
	}

	#[cfg(feature = "parallel")]
	pub(crate) fn get_outgoing_links(&self, mdfile: &MDFile) -> Vec<Link> {
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
				}
				else {
					let (paths, fragment) = result.unwrap();
					if paths.len() > 1 {
						for path in paths {
							println!("Fragment Path: {:?}", path);
						}
						panic!("Fragment with multiple paths: {:?}, ", fragment);
					}
					for path in paths {
						let fragment_len = fragment.len();
						let original_fragment: Vec<String> = original_whitespace_strings[index..index+fragment_len].to_vec().par_iter().map(|s| s.to_string()).collect();
						let original_fragment: String = original_fragment.join(" ");
						let stripped_path = path.strip_prefix(&self.path).unwrap();
						let link: Link = Link::new(original_fragment, PathBuf::from(stripped_path));
						outgoing_links.push(link);
					}
					index += fragment.len();
				}
			}
		}

		// options filter link to self
		let outgoing_links: Vec<Link> = match self.options.link_self {
			true => outgoing_links,
			false => {
				outgoing_links.into_par_iter().filter(|link| {
					link.get_path() != mdfile.get_path().unwrap()
				}).collect()
			}
		};

		// options filter tag share
		let outgoing_links: Vec<Link> = match self.options.link_share_tag {
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

		return outgoing_links;
	}

	// #[cfg(not(feature = "parallel"))]
	// pub(crate) fn link_file_from_path(&mut self, path: &Path) {
	// 	let outgoing_links: Vec<(PathBuf, String)> = self.get_outgoing_links(self.get_md_file(path).unwrap());
	// 	let mut md_file  = self.get_md_file_mut(path).unwrap();
	// 	for (outgoing_path, fragment) in outgoing_links {
	// 		// let outgoing_md_file = self.get_md_file(&outgoing_path).unwrap();
	// 		// md_file.add_outgoing_link(outgoing_md_file.get_path(), fragment);
	// 		let mut lines: Vec<&mut Line> = md_file.get_lines_mut();
	//
	// 		for line in &mut lines {
	// 			let mut nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
	// 			match nodes {
	// 				Some(nodes) => {
	// 					*nodes = add_link_to_nodes(nodes.clone(), &fragment, &outgoing_path);
	// 				}
	// 				None => {}
	// 			}
	// 		}
	// 	}
	//
	// }
	//
	// #[cfg(feature = "parallel")]
	// pub(crate) fn link_file_from_path(&mut self, path: &Path) {
	// 	let outgoing_links: Vec<(PathBuf, String)> = self.get_outgoing_links(self.get_md_file(path).unwrap());
	// 	let md_file  = self.get_md_file_mut(path).unwrap();
	// 	for (outgoing_path, fragment) in outgoing_links {
	// 		let mut lines: Vec<&mut Line> = md_file.get_lines_mut();
	//
	// 		for line in &mut lines {
	// 			let nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
	// 			match nodes {
	// 				Some(nodes) => {
	// 					*nodes = add_link_to_nodes(nodes.clone(), &fragment, &outgoing_path);
	// 				}
	// 				None => {}
	// 			}
	// 		}
	// 	}
	//
	// }

	// pub(crate) fn link_file_with_links(md_file: &mut MDFile, outgoing_links: Vec<(PathBuf, String)>) {
	// 	for (outgoing_path, fragment) in outgoing_links {
	// 		// let outgoing_md_file = self.get_md_file(&outgoing_path).unwrap();
	// 		// md_file.add_outgoing_link(outgoing_md_file.get_path(), fragment);
	// 		let mut lines: Vec<&mut Line> = md_file.get_lines_mut();
	//
	// 		for line in &mut lines {
	// 			let nodes: Option<&mut Vec<Node>> = line.get_nodes_mut();
	// 			match nodes {
	// 				Some(nodes) => {
	// 					*nodes = add_link_to_nodes(nodes.clone(), &fragment, &outgoing_path);
	// 				}
	// 				None => {}
	// 			}
	// 		}
	// 	}
	//
	// }

	pub(crate) fn link_file(&mut self, md_file: &mut MDFile) {
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

	#[cfg(not(feature = "parallel"))]
	pub fn link_all_files(&mut self) {
		let outgoing_links: HashMap<PathBuf, Vec<Link>> = self.data.iter().map(|(path, md_file)| {
			let outgoing_links = self.get_outgoing_links(md_file);
			(path.clone(), outgoing_links)
		}).collect();

		let _ = self.data.iter_mut().map(|(path, md_file)| {
			let outgoing_links = outgoing_links.get(path).unwrap();

			for link in outgoing_links {
				md_file.link_noself(link.clone());
			}
		}).collect::<Vec<()>>();
	}

	#[cfg(feature = "parallel")]
	pub fn link_all_files(&mut self) {
		let outgoing_links: HashMap<PathBuf, Vec<Link>> = self.data.par_iter().map(|(path, md_file)| {
			let outgoing_links = self.get_outgoing_links(md_file);
			(path.clone(), outgoing_links)
		}).collect();

		let _ = self.data.par_iter_mut().map(|(path, md_file)| {
			let outgoing_links = outgoing_links.get(path).unwrap();

			for link in outgoing_links {
				md_file.link_noself(link.clone());
			}
		}).collect::<Vec<()>>();
	}

	#[cfg(not(feature = "parallel"))]
	pub fn link_all_files_no_self(&mut self) {
		let outgoing_links: HashMap<PathBuf, Vec<Link>> = self.data.iter().map(|(path, md_file)| {
			let outgoing_links = self.get_outgoing_links(md_file);
			(path.clone(), outgoing_links)
		}).collect();

		let _ = self.data.iter_mut().map(|(path, md_file)| {
			let outgoing_links = outgoing_links.get(path).unwrap();
			for link in outgoing_links {
				md_file.link_noself(link.clone());
			}
		}).collect::<Vec<()>>();
	}

	#[cfg(feature = "parallel")]
	pub fn link_all_files_no_self(&mut self) {
		let outgoing_links: HashMap<PathBuf, Vec<Link>> = self.data.par_iter().map(|(path, md_file)| {
			let outgoing_links = self.get_outgoing_links(md_file);
			(path.clone(), outgoing_links)
		}).collect();

		let _ = self.data.par_iter_mut().map(|(path, md_file)| {
			let outgoing_links = outgoing_links.get(path).unwrap();
			for link in outgoing_links {
				md_file.link_noself(link.clone());
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

	pub fn unlink_all_files(&mut self) {
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

}

pub(crate) struct StringTree<T> {
	end: Option<Vec<T>>,
	children: HashMap<String, StringTree<T>>,
}

impl<T: std::fmt::Debug> StringTree<T> {
	pub(crate) fn new() -> Self {
		Self {
			end: None,
			children: HashMap::new(),
		}
	}
	pub(crate) fn insert(&mut self, keys: Vec<String>, value: T) {
		// if len of keys is 0, set end to value
		if keys.len() == 0 {
			// if end is None, set end to value
			if self.end.is_none() {
				self.end = Some(vec![value]);
			} else {
				// if end is Some, push value to end
				self.end.as_mut().unwrap().push(value);
			}
			return;
		}

		let mut keys = keys;
		let current_key = keys.remove(0);

		// If the next key is not in the tree, create it
		if !self.children.contains_key(&current_key) {
			self.children.insert(current_key.clone(), StringTree::new());
		}

		// Recurse on the next key
		self.children.get_mut(&current_key).unwrap().insert(keys, value);
		return;
	}

	#[allow(dead_code)]
	pub(crate) fn get(&self, keys: Vec<&String>) -> Option<&Vec<T>> {
		// if len of keys is 0, return end
		if keys.len() == 0 {
			return self.end.as_ref();
		}

		let mut keys = keys;
		let current_key = keys.remove(0);

		// if the next key is not in the tree, return None
		if !self.children.contains_key(current_key) {
			return None;
		}

		// Recurse on the next key
		self.children.get(current_key).unwrap().get(keys)
	}

	pub(crate) fn get_best<'a>(&'a self, keys: Vec<&'a String>) -> Option<(&Vec<T>, Vec<&String>)> {
		if keys.len() == 0 || self.children.len() == 0 {
			if self.end.is_none() {
				return None;
			} else {
				return Some((self.end.as_ref().expect("End should not be none"), vec![]));
			}
		}

		let mut keys = keys;
		let current_key = keys.remove(0);

		// if the next key is not in the tree, return None
		if !self.children.contains_key(current_key) {
			return None;
		}

		// Recurse on the next key
		let best_child_option = self.children.get(current_key).unwrap().get_best(keys);
		if best_child_option.is_some() {
			let (best_child, mut best_child_keys) = best_child_option.unwrap();
			best_child_keys.insert(0, current_key);
			return Some((best_child, best_child_keys));
		} else if best_child_option.is_none() && self.end.is_some() {
			return Some((self.end.as_ref().expect("End should not be none"), vec![current_key]));
		} else {
			return None;
		}
	}
}
