mod parse;
mod md_file;
mod vault;
mod linking;
mod options;
mod args;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::md_file::MDFile;
    use crate::options::LinkerOptions;
    use crate::vault::StringTree;


    #[test]
    fn full_vault_test() {
        let vault_path = PathBuf::from("test_vaults/full_vault");
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let mut vault = crate::vault::Vault::new(vault_path, ignore, linker_options);


        vault.unlink_all_files();
        vault.link_all_files();

        let mut data: Vec<(PathBuf, MDFile)> = vault.data.clone().iter().map(|(path, md_file)| {
            (path.clone(), md_file.clone())
        }).collect();
        data.sort_by(|a, b| a.0.cmp(&b.0));

        for (path, md_file) in data.iter() {
            let result = md_file.to_string();
            let expected = fs::read_to_string(md_file.path.clone().unwrap()).unwrap();

            if result != expected {
                println!("Result:\n{}", result);
                println!("Expected:\n{}", expected);
            }
            assert_eq!(result, expected, "Failed on file: {}", md_file.path.clone().unwrap().display());
        }
    }

    // Stability Tests
    #[test]
    fn stability_tests_reference_clean_file1() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file1.md");
        assert!(file_path.is_file());
        let file_contents = fs::read_to_string(file_path.clone()).unwrap();
        let md_file = MDFile::from(file_path.clone()).expect(&format!("Failed to create MDFile from {}", file_path.display()));
        let result = md_file.to_string();
        let expected = file_contents;
        assert_eq!(result, expected, "Failed on file: {}", file_path.display());
    }

    #[test]
    fn stability_tests_reference_clean_file2() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file2.md");
        assert!(file_path.is_file());
        let file_contents = fs::read_to_string(file_path.clone()).unwrap();
        let md_file = MDFile::from(file_path.clone()).expect(&format!("Failed to create MDFile from {}", file_path.display()));
        let result = md_file.to_string();
        let expected = file_contents;
        assert_eq!(result, expected, "Failed on file: {}", file_path.display());
    }

    #[test]
    fn stability_tests_reference_clean_file3() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file3.md");
        assert!(file_path.is_file());
        let file_contents = fs::read_to_string(file_path.clone()).unwrap();
        let md_file = MDFile::from(file_path.clone()).expect(&format!("Failed to create MDFile from {}", file_path.display()));
        let result = md_file.to_string();
        let expected = file_contents;
        assert_eq!(result, expected, "Failed on file: {}", file_path.display());
    }

    #[test]
    fn stability_tests_reference_clean_file4() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: true,
            link_self: true
        };
        let file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file4.md");
        assert!(file_path.is_file());
        let file_contents = fs::read_to_string(file_path.clone()).unwrap();
        let md_file = MDFile::from(file_path.clone()).expect(&format!("Failed to create MDFile from {}", file_path.display()));
        let result = md_file.to_string();
        let expected = file_contents;
        assert_eq!(result, expected, "Failed on file: {}", file_path.display());
    }

    // Linking Tests
    #[test]
    fn linking_tests_no_self_reference_clean_file1() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file1.md");
        let linked_file_path: PathBuf = linked_file_path.canonicalize().unwrap();
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file1.md");
        let clean_file_path: PathBuf = clean_file_path.canonicalize().unwrap();
        let vault_path: PathBuf = PathBuf::from("test_vaults/reference_clean");
        let vault_path: PathBuf = vault_path.canonicalize().unwrap();
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut vault = crate::vault::Vault::new(vault_path, ignore, linker_options);
        vault.link_all_files_no_self();
        let md_file: &MDFile = vault.get_md_file(&clean_file_path).unwrap();
        let result = md_file.to_string();
        let expected = fs::read_to_string(linked_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", clean_file_path.display());
    }

    #[test]
    fn linking_tests_no_self_reference_clean_file2() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file2.md");
        let linked_file_path: PathBuf = linked_file_path.canonicalize().unwrap();
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file2.md");
        let clean_file_path: PathBuf = clean_file_path.canonicalize().unwrap();
        let vault_path: PathBuf = PathBuf::from("test_vaults/reference_clean");
        let vault_path: PathBuf = vault_path.canonicalize().unwrap();
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut vault = crate::vault::Vault::new(vault_path, ignore, linker_options);
        vault.link_all_files_no_self();
        let md_file: &MDFile = vault.get_md_file(&clean_file_path).unwrap();
        let result = md_file.to_string();
        let expected = fs::read_to_string(linked_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", clean_file_path.display());
    }

    #[test]
    fn linking_tests_no_self_reference_clean_file3() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file3.md");
        let linked_file_path: PathBuf = linked_file_path.canonicalize().unwrap();
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file3.md");
        let clean_file_path: PathBuf = clean_file_path.canonicalize().unwrap();
        let vault_path: PathBuf = PathBuf::from("test_vaults/reference_clean");
        let vault_path: PathBuf = vault_path.canonicalize().unwrap();
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut vault = crate::vault::Vault::new(vault_path, ignore, linker_options);
        vault.link_all_files_no_self();
        let md_file: &MDFile = vault.get_md_file(&clean_file_path).unwrap();
        let result = md_file.to_string();
        let expected = fs::read_to_string(linked_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", clean_file_path.display());
    }

    // #[test]
    // fn linking_tests_no_self_reference_clean_file4() {
    //     let ignore: Vec<PathBuf> = vec![];
    //     let linker_options = crate::options::LinkerOptions {
    //         link_share_tag: true,
    //         link_self: true
    //     };
    //     let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file4.md");
    //     let linked_file_path: PathBuf = linked_file_path.canonicalize().unwrap();
    //     let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file4.md");
    //     let clean_file_path: PathBuf = clean_file_path.canonicalize().unwrap();
    //     let vault_path: PathBuf = PathBuf::from("test_vaults/reference_clean");
    //     let vault_path: PathBuf = vault_path.canonicalize().unwrap();
    //     assert!(linked_file_path.is_file());
    //     assert!(clean_file_path.is_file());
    //     let mut vault = crate::vault::Vault::new(vault_path, ignore, linker_options);
    //     vault.link_all_files();
    //     let md_file: &MDFile = vault.get_md_file(&clean_file_path).unwrap();
    //     let result = md_file.to_string();
    //     let expected = fs::read_to_string(linked_file_path.clone()).unwrap();
    //     assert_eq!(result, expected, "Failed on file: {}", clean_file_path.display());
    // }

    // Unlinking Tests
    #[test]
    fn unlinking_tests_no_self_reference_linked_file1() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file1.md");
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file1.md");
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut md_file: MDFile = MDFile::from(linked_file_path.clone()).unwrap();
        md_file.unlink();
        let result = md_file.to_string();
        let expected = fs::read_to_string(clean_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", linked_file_path.display());
    }

    #[test]
    fn unlinking_tests_no_self_reference_linked_file2() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file2.md");
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file2.md");
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut md_file: MDFile = MDFile::from(linked_file_path.clone()).unwrap();
        md_file.unlink();
        let result = md_file.to_string();
        let expected = fs::read_to_string(clean_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", linked_file_path.display());
    }

    #[test]
    fn unlinking_tests_no_self_reference_linked_file3() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: false,
            link_self: false
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file3.md");
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file3.md");
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut md_file: MDFile = MDFile::from(linked_file_path.clone()).unwrap();
        md_file.unlink();
        let result = md_file.to_string();
        let expected = fs::read_to_string(clean_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", linked_file_path.display());
    }

    #[test]
    fn unlinking_tests_no_self_reference_linked_file4() {
        let ignore: Vec<PathBuf> = vec![];
        let linker_options = crate::options::LinkerOptions {
            link_share_tag: true,
            link_self: true
        };
        let linked_file_path: PathBuf = PathBuf::from("test_vaults/reference_linked_noself/file4.md");
        let clean_file_path: PathBuf = PathBuf::from("test_vaults/reference_clean/file4.md");
        assert!(linked_file_path.is_file());
        assert!(clean_file_path.is_file());
        let mut md_file: MDFile = MDFile::from(linked_file_path.clone()).unwrap();
        md_file.unlink();
        let result = md_file.to_string();
        let expected = fs::read_to_string(clean_file_path.clone()).unwrap();
        assert_eq!(result, expected, "Failed on file: {}", linked_file_path.display());
    }

    // StringTree
    #[test]
    fn string_tree_to_string_test() {
        let mut tree: StringTree<PathBuf> = crate::vault::StringTree::new();
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
