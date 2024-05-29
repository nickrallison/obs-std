mod parse;
mod md_file;
mod traits;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn stability_test() {
        let filelist = fs::read_dir("vault").unwrap();

        for file in filelist {

            let file_path = file.unwrap().path();
            println!("Testing file: {:?}", file_path.clone());
            let file_contents = fs::read_to_string(&file_path).unwrap();
            let md_file = md_file::MDFile::new(file_path.clone());
            let result = md_file.to_string();
            let expected = file_contents;
            assert_eq!(result, expected, "Failed on file: {:?}", file_path);
        }
        println!("All stability tests passed!")
    }

    // #[test]
    // fn stability_test_2() {
    //     let file_path: PathBuf = PathBuf::from("vault/A-Star Search.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }

    // #[test]
    // fn stability_test_3() {
    //     let file_path: PathBuf = PathBuf::from("vault/AC Circuits.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    //
    // #[test]
    // fn stability_test_4() {
    //     let file_path: PathBuf = PathBuf::from("vault/Access Clipboard History.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    //
    // #[test]
    // fn stability_test_5() {
    //     let file_path: PathBuf = PathBuf::from("vault/Address Resolution Protocol.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_6() {
    //     let file_path: PathBuf = PathBuf::from("vault/Adjacency Matrix.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_7() {
    //     let file_path: PathBuf = PathBuf::from("vault/Akra-Bazzi Theorem.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    //
    // #[test]
    // fn stability_test_8() {
    //     let file_path: PathBuf = PathBuf::from("vault/Alan Turing.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     println!("{:?}", file_contents);
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_9() {
    //     let file_path: PathBuf = PathBuf::from("vault/Algorithm Specifications.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_10() {
    //     let file_path: PathBuf = PathBuf::from("vault/Aliasing.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_11() {
    //     let file_path: PathBuf = PathBuf::from("vault/Application Binary Interface.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_12() {
    //     let file_path: PathBuf = PathBuf::from("vault/Associativity.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_13() {
    //     let file_path: PathBuf = PathBuf::from("vault/Powerset.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }
    // #[test]
    // fn stability_test_14() {
    //     let file_path: PathBuf = PathBuf::from("vault/Resonance Peak.md");
    //     let file_contents = fs::read_to_string(&file_path).unwrap();
    //     let md_file = md_file::MDFile::new(file_path);
    //     let result = md_file.to_string();
    //     let expected = file_contents;
    //     assert_eq!(result, expected);
    // }


}
