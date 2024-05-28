mod parse;
mod md_file;
mod traits;

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn filelist(vault_path: &str) -> Vec<PathBuf> {
    let vault = Path::new(vault_path);
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(vault) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path().strip_prefix(vault).unwrap();
            files.push(path.to_path_buf());
        }
    }
    files
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::traits::Verbose;
    use super::*;

    #[test]
    fn stability_test_1() {
        let file_path: PathBuf = PathBuf::from("tests/file1.md");
        let file_contents = fs::read_to_string(&file_path).unwrap();
        let md_file = md_file::MDFile::new(file_path);
        let result = md_file.to_string();
        let expected = file_contents;
        assert_eq!(result, expected);
    }

    #[test]
    fn stability_test_2() {
        let file_path: PathBuf = PathBuf::from("tests/file2.md");
        let file_contents = fs::read_to_string(&file_path).unwrap();
        let md_file = md_file::MDFile::new(file_path);
        println!("{}", md_file.verbose());
        let result = md_file.to_string();
        let expected = file_contents;
        assert_eq!(result, expected);
    }

    #[test]
    fn newline_in_lines_1() {
        let file_path: PathBuf = PathBuf::from("tests/file1.md");
        let md_file = md_file::MDFile::new(file_path);
        let lines = md_file.lines();
        for line in lines {
            assert!(!line.contains("\n"));
        }
    }
    #[test]
    fn newline_in_lines_2() {
        let file_path: PathBuf = PathBuf::from("tests/file2.md");
        let md_file = md_file::MDFile::new(file_path);
        let lines = md_file.lines();
        for line in lines {
            assert!(!line.contains("\n"));
        }
    }
}
