use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

pub struct StringTree<T> {
	pub end: Option<Vec<T>>,
	pub children: HashMap<String, StringTree<T>>,
}

impl<T: std::fmt::Debug> StringTree<T> {
	pub fn new() -> Self {
		Self {
			end: None,
			children: HashMap::new(),
		}
	}
	pub fn insert(&mut self, keys: Vec<String>, value: T) {
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
	pub fn get(&self, keys: Vec<&String>) -> Option<&Vec<T>> {
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

	pub fn get_best<'a>(&'a self, keys: Vec<&'a String>) -> Option<(&Vec<T>, Vec<&String>)> {

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
		let child = self.children.get(current_key).unwrap();
		let best_child_option = child.get_best(keys);
		let child_end = child.end.as_ref();
		if best_child_option.is_some() {
			let (best_child, mut best_child_keys) = best_child_option.unwrap();
			best_child_keys.insert(0, current_key);
			return Some((best_child, best_child_keys));
		} else if best_child_option.is_none() && child_end.is_some() {
			return Some((child_end.as_ref().expect("End should not be none"), vec![current_key]));
		} else {
			return None;
		}
	}
}

// #### Correct way to format StringTree
//
// ├─ a
// │  ├─ b
// │  │  └─ ["keys1"]
// │  ├─ c
// │  │  ├─ e
// │  │  │  └─ ["keys4"]
// │  │  └─ ["keys2"]
// │  └─ d
// │     └─ ["keys3"]
// └─ e
//    └─ ["keys5"]
impl Display for StringTree<PathBuf> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fn fmt_helper(tree: &StringTree<PathBuf>, tag: Option<&String>) -> String {
			let mut result = String::new();
			match tag {
				Some(tag) => {
					result.push_str(&format!("{}\n", tag));
				}
				None => {}
			}
			let mut keys: Vec<&String> = tree.children.keys().map(|s| s).collect::<Vec<&String>>();
			keys.sort();
			let end: Option<Vec<PathBuf>> = tree.end.clone();
			let mut child_strings: Vec<String> = keys.iter().map(|key| {
				let child = tree.children.get(*key).unwrap();
				fmt_helper(child, Some(key))
			}).collect();
			match end {
				Some(end) => {
					let cwd = std::env::current_dir().unwrap();

					let mut match_result_string: String = String::from("[");
					for value in end {
						let value = match value.strip_prefix(&cwd) {
							Ok(value) => value,
							Err(_) => value.as_path()
						};
						match_result_string.push_str(&format!("{:?}, ", value));
					}
					match_result_string.pop();
					match_result_string.pop();
					match_result_string.push_str("]");
					child_strings.push(match_result_string);
				}
				None => {}
			}
			for (index_outer, child_string) in child_strings.iter().enumerate() {
				for (index_inner, line) in child_string.lines().enumerate() {
					if index_inner == 0 && index_outer == child_strings.iter().count() - 1 {
						result.push_str(&format!("└─ {}\n", line));
					} else if index_inner == 0 && index_outer != child_strings.iter().count() - 1 {
						result.push_str(&format!("├─ {}\n", line));
					} else if index_inner != 0 && index_outer == child_strings.iter().count() - 1  {
						result.push_str(&format!("   {}\n", line));
					} else {
						result.push_str(&format!("│  {}\n", line));
					}
				}
			}
			result
		}
		let result = fmt_helper(self, None);
		write!(f, "{}", result)
	}
}

#[cfg(test)]
mod stringtree_tests {
	use std::path::PathBuf;
	use crate::stringtree::StringTree;

	#[test]
	fn test_stringtree() {
		let mut tree: StringTree<PathBuf> = StringTree::new();
		let keys1: Vec<String> = vec!["a".to_string(), "b".to_string()];
		tree.insert(keys1, PathBuf::from("keys1"));
		let keys2: Vec<String> = vec!["a".to_string(), "c".to_string()];
		tree.insert(keys2, PathBuf::from("keys2"));
		let keys3: Vec<String> = vec!["a".to_string(), "d".to_string()];
		tree.insert(keys3, PathBuf::from("keys3"));
		let keys4: Vec<String> = vec!["a".to_string(), "c".to_string(), "e".to_string()];
		tree.insert(keys4, PathBuf::from("keys4"));
		let keys5: Vec<String> = vec!["e".to_string()];
		tree.insert(keys5, PathBuf::from("keys5"));
		let result = tree.to_string();
		let expected =
			"├─ a
│  ├─ b
│  │  └─ [\"keys1\"]
│  ├─ c
│  │  ├─ e
│  │  │  └─ [\"keys4\"]
│  │  └─ [\"keys2\"]
│  └─ d
│     └─ [\"keys3\"]
└─ e
   └─ [\"keys5\"]
";
		assert_eq!(result, expected);
	}
}