import os
import sys
import re

body_template = """
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
	static ref TEST_SUITE_CLEAN_VAULT_PATH: PathBuf = PathBuf::from("[VAULT_PATH_CLEAN]");
	static ref TEST_SUITE_LINKED_VAULT_PATH: PathBuf = PathBuf::from("[VAULT_PATH_LINKED]");
	static ref TEST_SUITE_REF_CLEAN_VAULT: Vault = Vault::new(TEST_SUITE_CLEAN_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone(), None);
	static ref TEST_SUITE_REF_LINKED_VAULT: Vault = Vault::new(TEST_SUITE_LINKED_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone(), None);
	static ref TEST_SUITE_REF_CLEAN_TO_LINKED_VAULT: Vault = Vault::new(TEST_SUITE_LINKED_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone(), None).link_all_files();
	static ref TEST_SUITE_REF_LINKED_TO_CLEAN_VAULT: Vault = Vault::new(TEST_SUITE_CLEAN_VAULT_PATH.clone(), TEST_SUITE_IGNORE.clone(), TEST_SUITE_LINKER_OPTIONS.clone(), None).unlink_all_files();
}



#[cfg(test)]
mod test_suite {
	use std::fs;
	use std::path::PathBuf;
	use crate::test_suite::{TEST_SUITE_REF_CLEAN_TO_LINKED_VAULT, TEST_SUITE_REF_CLEAN_VAULT, TEST_SUITE_REF_LINKED_TO_CLEAN_VAULT, TEST_SUITE_REF_LINKED_VAULT};

	[TESTS]
}
"""

test_template = """
    #[test]
	fn test_suite_stability_[FILE_TITLE_TL]_clean() {
		let vault_path: PathBuf = PathBuf::from("[VAULT_PATH_CLEAN]");
		let file: PathBuf = PathBuf::from("[FILENAME]");
		let file_path: PathBuf = vault_path.join(file.clone());
		assert!(file_path.is_file());
		let file_contents = fs::read_to_string(file_path.clone()).unwrap();
		let md_file = TEST_SUITE_REF_CLEAN_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path.display()));
		let result = md_file.to_string();
		let expected = file_contents;
		assert_eq!(result, expected, "Failed on file: {}", file_path.display());
	}

	#[test]
	fn test_suite_stability_[FILE_TITLE_TL]_linked() {
		let vault_path: PathBuf = PathBuf::from("[VAULT_PATH_LINKED]");
		let file: PathBuf = PathBuf::from("[FILENAME]");
		let file_path: PathBuf = vault_path.join(file.clone());
		assert!(file_path.is_file());
		let file_contents = fs::read_to_string(file_path.clone()).unwrap();
		let md_file = TEST_SUITE_REF_LINKED_VAULT.get_md_file(&file).expect(&format!("Failed to get MDFile from {}", file_path.display()));
		let result = md_file.to_string();
		let expected = file_contents;
		assert_eq!(result, expected, "Failed on file: {}", file_path.display());
	}

	#[test]
	fn test_suite_[FILE_TITLE_TL]_link() {
		let vault_path_clean: PathBuf = PathBuf::from("[VAULT_PATH_CLEAN]");
		let vault_path_linked: PathBuf = PathBuf::from("[VAULT_PATH_LINKED]");

		let file = PathBuf::from("[FILENAME]");

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
	fn test_suite_[FILE_TITLE_TL]_unlink() {

		let vault_path_clean: PathBuf = PathBuf::from("[VAULT_PATH_CLEAN]");
		let vault_path_linked: PathBuf = PathBuf::from("[VAULT_PATH_LINKED]");

		let file = PathBuf::from("[FILENAME]");

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
"""

if __name__ == '__main__':
    import sys

    if len(sys.argv) != 4:
        print('Usage: {} <vault_path_linked> <vault_path_unlinked> <output_test_file>'.format(sys.argv[0]))
        sys.exit(1)

    vault_path_linked = sys.argv[1]
    vault_path_unlinked = sys.argv[2]
    output_test_file = sys.argv[3]

    tests = []
    for root, _, files in os.walk(vault_path_linked):
        for file in files:
            if file.endswith('.md'):

                file_title_lower = (file.removesuffix('.md').lower().replace(' ', '_').replace('-', '_'))
                file_title_lower = ''.join([c if c.isalnum() or c == '_' else '_' for c in file_title_lower])
                file_title_lower = re.sub(r'_+', '_', file_title_lower)
                file_title_lower = file_title_lower.strip('_')

                file_path = os.path.join(root, file)
                file_path_lower = os.path.join(root, file_title_lower + '.md')
                if os.path.normpath(file_path_lower) != os.path.normpath(file_path):
                    print(f'renaming {file_path.strip()} to {file_path_lower}')
                    os.rename(file_path, file_path_lower)

        for root, _, files in os.walk(vault_path_unlinked):
            for file in files:
                if file.endswith('.md'):

                    file_title_lower = (file.removesuffix('.md').lower().replace(' ', '_').replace('-', '_'))
                    file_title_lower = ''.join([c if c.isalnum() or c == '_' else '_' for c in file_title_lower])
                    file_title_lower = re.sub(r'_+', '_', file_title_lower)
                    file_title_lower = file_title_lower.strip('_')

                    file_path = os.path.join(root, file)
                    file_path_lower = os.path.join(root, file_title_lower + '.md')
                    if os.path.normpath(file_path_lower) != os.path.normpath(file_path):
                        print(f'renaming {file_path} to {file_path_lower}')
                        os.rename(file_path, file_path_lower)


                tests.append(test_template.replace('[FILENAME]', file_title_lower + '.md').replace('[FILE_TITLE_TL]', file_title_lower))

    body = (body_template
            .replace('[TESTS]', '\n'.join(tests)))
    body = (body
            .replace('[VAULT_PATH_CLEAN]', vault_path_unlinked)
            .replace('[VAULT_PATH_LINKED]', vault_path_linked))

    with open(output_test_file, 'w') as f:
        f.write(body)
    # print(f'{body}')
