use colored::*;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn read_file_contents(path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_file_size(path: &Path) -> u64 {
    match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(_) => 0,
    }
}

pub fn get_version() -> String {
    const GIT_HASH: &str = env!("GIT_HASH");
    const GIT_TAG: &str = env!("GIT_TAG");

    let version = format!("gitig {}-{}", GIT_TAG, GIT_HASH);
    version
}

const FILE_CONTENTS: &str = include_str!(concat!(env!("OUT_DIR"), "/file_contents.json"));
pub fn get_json_from_file() -> HashMap<String, String> {
    let contents: HashMap<String, String> = serde_json::from_str(FILE_CONTENTS).unwrap();

    contents
}

pub fn highlight(s: &str, word: &str) -> String {
    s.replace(word, &word.red().bold().to_string())
}

#[cfg(test)]
mod tests_helper {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_file_size_existing_file() {
        let path = PathBuf::from("test.txt");
        // 创建一个测试文件
        fs::write(&path, "Hello, world!").unwrap();

        let size = get_file_size(&path);
        assert_eq!(size, 13);

        // 删除测试文件
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_get_file_size_non_existing_file() {
        let path = PathBuf::from("non_existing.txt");

        let size = get_file_size(&path);
        assert_eq!(size, 0);
    }
}
