use std::path::PathBuf;
use lazy_static::lazy_static;
use crate::linking::LinkerOptions;
use crate::vault::Vault;

lazy_static! {
	static ref TEST_SUITE_IGNORE: Vec<PathBuf> = vec![];
	static ref TEST_SUITE_LINKER_OPTIONS: LinkerOptions = LinkerOptions {
		link_share_tag: false,
		link_self: false
	};
	static ref TEST_SUITE_CLEAN_VAULT_PATH: PathBuf = PathBuf::from("test_vaults/reference_clean");
	static ref TEST_SUITE_LINKED_VAULT_PATH: PathBuf = PathBuf::from("test_vaults/reference_linked_noself");
	static ref TEST_SUITE_REF_CLEAN_VAULT: Vault = Vault::new(TEST_SUITE_CLEAN_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone());
	static ref TEST_SUITE_REF_LINKED_VAULT: Vault = Vault::new(TEST_SUITE_LINKED_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone());
	static ref TEST_SUITE_REF_CLEAN_TO_LINKED_VAULT: Vault = Vault::new(TEST_SUITE_LINKED_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone()).link_all_files();
	static ref TEST_SUITE_REF_LINKED_TO_CLEAN_VAULT: Vault = Vault::new(TEST_SUITE_CLEAN_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone()).unlink_all_files();
}



#[cfg(test)]
mod test_suite {
	use std::fs;
	use std::path::PathBuf;
	use crate::md_file::MDFile;
	use crate::linking::LinkerOptions;
	use crate::test_suite::{TEST_SUITE_REF_CLEAN_TO_LINKED_VAULT, TEST_SUITE_REF_CLEAN_VAULT, TEST_SUITE_REF_LINKED_TO_CLEAN_VAULT, TEST_SUITE_REF_LINKED_VAULT};

	#[test]
	fn test_suite_stability_clean() {
		let vault_path: PathBuf = PathBuf::from("test_vaults/reference_clean");
		let file: PathBuf = PathBuf::from("file1.md");
		let file_path: PathBuf = vault_path.join(file.clone());
		assert!(file_path.is_file());
		let file_contents = fs::read_to_string(file_path.clone()).unwrap();
		let md_file = TEST_SUITE_REF_CLEAN_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path.display()));
		let result = md_file.to_string();
		let expected = file_contents;
		assert_eq!(result, expected, "Failed on file: {}", file_path.display());
	}

	#[test]
	fn test_suite_stability_linked() {
		let vault_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself");
		let file: PathBuf = PathBuf::from("file1.md");
		let file_path: PathBuf = vault_path.join(file.clone());
		assert!(file_path.is_file());
		let file_contents = fs::read_to_string(file_path.clone()).unwrap();
		let md_file = TEST_SUITE_REF_LINKED_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path.display()));
		let result = md_file.to_string();
		let expected = file_contents;
		assert_eq!(result, expected, "Failed on file: {}", file_path.display());
	}

	#[test]
	fn test_suite_link() {
		let vault_path_clean: PathBuf = PathBuf::from("test_vaults/reference_clean");
		let vault_path_linked: PathBuf = PathBuf::from("test_vaults/reference_linked_noself");

		let file = PathBuf::from("file1.md");

		let file_path_clean: PathBuf = vault_path_clean.join(file.clone());
		let file_path_linked: PathBuf = vault_path_linked.join(file.clone());

		assert!(file_path_clean.is_file());
		assert!(file_path_linked.is_file());

		let md_file_clean_to_linked = TEST_SUITE_REF_CLEAN_TO_LINKED_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path_clean.display()));
		let md_file_linked = TEST_SUITE_REF_LINKED_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path_linked.display()));

		let result = md_file_clean_to_linked.to_string();
		let expected = md_file_linked.to_string();
		assert_eq!(result, expected, "Failed on file: {}", file_path_clean.display());
	}

	#[test]
	fn test_suite_unlink() {

		let vault_path_clean: PathBuf = PathBuf::from("test_vaults/reference_clean");
		let vault_path_linked: PathBuf = PathBuf::from("test_vaults/reference_linked_noself");

		let file = PathBuf::from("file1.md");

		let file_path_clean: PathBuf = vault_path_clean.join(file.clone());
		let file_path_linked: PathBuf = vault_path_linked.join(file.clone());

		assert!(file_path_clean.is_file());
		assert!(file_path_linked.is_file());

		let md_file_clean = TEST_SUITE_REF_CLEAN_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path_clean.display()));
		let md_file_linked_to_clean = TEST_SUITE_REF_LINKED_TO_CLEAN_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path_clean.display()));

		let result = md_file_linked_to_clean.to_string();
		let expected = md_file_clean.to_string();
		assert_eq!(result, expected, "Failed on file: {}", file_path_clean.display());
	}
}