use crate::util::helper;
use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::io::Write;
use std::io::{self};
use std::path::Path;

pub struct Core {
    files: HashMap<String, String>,
    db: HashSet<String>,
    // ignore_dirs: HashSet<String>,
}

impl Core {
    pub fn new() -> Self {
        let mut ins = Core {
            files: HashMap::new(),
            db: HashSet::new(),
        };

        ins.init_files();

        ins.parse_gitignore_file();

        ins
    }

    // 更新客户端 .gitignore 条目
    pub fn update_gitignore(&self, files: Vec<String>) -> io::Result<Vec<String>> {
        let mut lines = self.fetch_new_records(files);
        if lines.len() > 0 {
            // 检查原文件内容是否为空
            let mut file_is_empty = false;
            let fpath = Path::new(".gitignore");
            let file_content = helper::read_file_contents(&fpath).unwrap_or_else(|_| {
                file_is_empty = true;
                String::new()
            });
            if !file_is_empty && file_content.len() == 0 {
                file_is_empty = true;
            }

            // 追加忽略项
            let mut file = fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(".gitignore")?;

            if !file_is_empty {
                file.write_all("\n".as_bytes())?;
            }
            lines.sort();
            file.write_all(lines.join("\n").as_bytes())?;
        }
        Ok(lines)
    }

    pub fn get_lines_by_files(&self, files: Vec<String>) -> Vec<String> {
        let mut lines = HashSet::new();
        for file in files {
            match self.files.get(&file.to_lowercase()) {
                Some(contents) => {
                    contents.lines().for_each(|line| {
                        if !line.trim().is_empty() && !line.trim().starts_with('#') {
                            lines.insert(line.to_string());
                        }
                    });
                }
                None => {}
            };
        }

        let lines = lines.into_iter().collect::<Vec<_>>();

        lines
    }

    // 解析当前目录存在的 .gitignore, 如果文件存在的话
    fn parse_gitignore_file(&mut self) {
        let fpath = Path::new(".gitignore");
        if !fpath.exists() {
            return;
        }

        match helper::read_file_contents(&fpath) {
            Ok(contents) => {
                contents.lines().for_each(|line| {
                    if !line.trim().is_empty() && !line.trim().starts_with('#') {
                        self.db.insert(line.to_string());
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to read file .gitignore: {}", e);
            }
        }
    }

    pub fn get_files(&self) -> Vec<String> {
        let mut vec: Vec<String> = self.files.keys().cloned().collect();
        vec.sort();
        vec
    }

    pub fn search(&self, keywords: &str) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        for key in self.files.keys() {
            if key.contains(keywords) {
                result.push(key.clone());
            }
        }

        result
    }

    // 对比本地 .gitignore 文件内容，计算要添加的忽略项
    fn fetch_new_records(&self, files: Vec<String>) -> Vec<String> {
        let mut new_records = Vec::new();
        let lines = self.get_lines_by_files(files);
        for record in lines {
            if !self.db.contains(&record) {
                new_records.push(record.clone());
            }
        }

        new_records
    }

    fn init_files(&mut self) {
        let contents: HashMap<String, String> = helper::get_json_from_file();
        for (path, content) in contents.iter() {
            let file_name = path.rsplit_once('.').map_or(
                path.to_string(),             // 如果没有找到 '.', 返回原始文件名
                |(last, _)| last.to_string(), // 如果找到 '.', 返回分割后的第一部分
            );

            self.files
                .insert(file_name.to_lowercase(), content.to_string());
        }
    }
}
