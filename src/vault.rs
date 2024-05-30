use std::collections::HashMap;
use std::path::PathBuf;
use rand::Rng;
use walkdir::WalkDir;
use crate::md_file::MDFile;
use rayon::prelude::*;
use crate::parse::Line;

pub struct Vault {
	path: PathBuf,
	data: HashMap<PathBuf, MDFile>,
	alias_tree: StringTree<PathBuf>
}

impl Vault {
	pub fn new(path: PathBuf) -> Self {

		let entries: Vec<PathBuf> = WalkDir::new(&path).into_iter().filter_map(|entry| {
			let entry = entry.unwrap();
			let path = entry.path().to_path_buf();
			println!("Path: {:?}", path);
			if path.is_dir() {
				return None;
			}
			Some(path)
		}).collect();


		let data: HashMap<PathBuf, MDFile> = entries.into_par_iter().map(|path| {
			let md_file = MDFile::new(path.to_path_buf());
			(path, md_file)
		}).collect();

		let mut alias_tree: StringTree<PathBuf> = StringTree::new();
		for md_file in data.values() {
			for alias in md_file.get_aliases() {
				let alias = alias.to_lowercase();
				let path = md_file.get_path().clone();
				let keys: Vec<String> = alias.split_whitespace().map(|s| s.to_string()).collect();
				alias_tree.insert(keys, path.clone());
			}
		}

		Self { path, data, alias_tree }
	}

	pub fn get_links(&self) -> Vec<(&String, &PathBuf)> {
		let mut links: Vec<(&String, &PathBuf)> = Vec::new();
		for md_file in self.data.values() {
			links.push((md_file.get_title(), md_file.get_path()));
			for alias in md_file.get_aliases() {
				links.push((alias, md_file.get_path()));
			}
		}
		links
	}

	pub(crate) fn search_for_alias(&self, alias: &str) -> Option<&Vec<PathBuf>> {
		let keys: Vec<String> = alias.split_whitespace().map(|s| s.to_lowercase().to_string()).collect();
		let mut result = self.alias_tree.get_best(keys);
		if result.is_none() {
			return None;
		}
		let (path, _) = result.unwrap();
		Some(path)
	}

	pub(crate) fn get_outgoing_links(&self, mdfile: &MDFile) -> Vec<&PathBuf> {

		todo!()

	}

	pub(crate) fn random_md_file(&self) -> &MDFile {
		let mut rng = rand::thread_rng();
		let index = rng.gen_range(0..self.data.len());
		let md_file = self.data.values().nth(index).unwrap();
		md_file
	}
}

pub(crate) struct StringTree<T> {
	end: Option<Vec<T>>,
	children: HashMap<String, StringTree<T>>,
	// parent: Option<Arc<StringTree<T>>>
}

impl<T: std::fmt::Debug> StringTree<T> {
	pub(crate) fn new() -> Self {
		Self {
			end: None,
			children: HashMap::new(),
			// parent: None
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

	pub(crate) fn get(&self, keys: Vec<String>) -> Option<&Vec<T>> {
		// if len of keys is 0, return end
		if keys.len() == 0 {
			return self.end.as_ref();
		}

		let mut keys = keys;
		let current_key = keys.remove(0);

		// if the next key is not in the tree, return None
		if !self.children.contains_key(&current_key) {
			return None;
		}

		// Recurse on the next key
		self.children.get(&current_key).unwrap().get(keys)
	}

	pub(crate) fn get_best(&self, keys: Vec<String>) -> Option<(&Vec<T>, Vec<String>)> {
		// if len of keys is 0, or no children, return end
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
		if !self.children.contains_key(&current_key) {
			return None;
		}

		// Recurse on the next key
		let best_child_option = self.children.get(&current_key).unwrap().get_best(keys);
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
