use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    git();
    data();
}
const SOURCE_DIR: &str = "../gitignore-repo";

fn data() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("file_contents.json");

    // 指定要读取的目录
    let mut file_contents = HashMap::new();

    let src_path = Path::new(SOURCE_DIR);
    if !src_path.exists() {
        panic!("{:?} 依赖 gitignore 仓库未找到!", src_path.to_str());
    }
    // 递归读取目录内容
    read_dir_contents(Path::new(SOURCE_DIR), &mut file_contents);

    // 将内容序列化为 JSON
    let json = serde_json::to_string(&file_contents).unwrap();
    fs::write(&dest_path, json).unwrap();

    println!("cargo:rerun-if-changed={}", SOURCE_DIR);
}

fn read_dir_contents(dir: &Path, contents: &mut HashMap<String, String>) {
    if dir.is_dir() {
        // 排除隐藏目录
        if dir.file_name().unwrap().to_str().unwrap().starts_with(".") {
            return;
        }
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_str().unwrap() != "gitignore" {
                        continue;
                    }
                }

                // 排除隐藏文件，主要是 .gitignore 文件
                let file_name = path.file_stem().unwrap().to_str().unwrap();
                if file_name.starts_with(".") {
                    continue;
                }

                let key = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let content = fs::read_to_string(&path).unwrap();
                contents.insert(key, content);
            } else if path.is_dir() {
                read_dir_contents(&path, contents);
            }
        }
    }
}
fn git() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");

    let git_hash = String::from_utf8(output.stdout).expect("Failed to read git hash");

    let short_hash = git_hash.chars().take(7).collect::<String>();
    println!("cargo:rustc-env=GIT_HASH={}", short_hash);

    // git tag
    // 获取最新的 tag 名称
    let tag_output = Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0"])
        .output();

    let git_tag = match tag_output {
        Ok(output) if output.status.success() => {
            let tag = String::from_utf8(output.stdout)
                .expect("Failed to read git tag")
                .trim()
                .to_string();
            if tag.is_empty() {
                "v0.0.0".to_string()
            } else {
                tag
            }
        }
        _ => "v0.0.0".to_string(),
    };

    println!("cargo:rustc-env=GIT_TAG={}", git_tag.trim());
}
