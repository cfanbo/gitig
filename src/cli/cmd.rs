use crate::core::core;
use crate::util::helper;
use log::debug;
use std::collections::HashSet;

pub struct Help;

impl Help {
    pub fn default() -> String {
        "Examples:

    # 将 rust 和 java 项目推荐的 gitignore 条目添加到本地 .gitignore 文件
        gitig add rust
        gitig add java

    # 查看 rust 项目推荐的 gitignore 项清单
        gitig show rust

    # 查看当前支持的所有项目类型清单
        gitig list

    # 搜索支持的项目类型
        gitig search ja

    "
        .to_string()
    }

    pub fn add() -> String {
        "Examples:

    # 将 rust 项目推荐的 gitignore 条目添加到本地 .gitignore 文件
        gitig add rust

    # 一次性添加多类型项目
        gitig add rust java

    "
        .to_string()
    }

    pub fn file() -> String {
        "Examples:

    # 将本地文件添加到 .gitignore 文件
        gitig file .DS_Store
        gitig file .zed/ .vscode/ .output/

    "
        .to_string()
    }

    pub fn search() -> String {
        "Examples:

    # 搜索支持的项目类型, 完整匹配
        gitig search java

    # 搜索支持的项目类型, 关键字搜索
        gitig search ja

    "
        .to_string()
    }
}

pub fn search(keywords: &str) {
    let core = core::Core::new();
    let mut sorted_items = core.search(keywords);
    if sorted_items.len() == 0 {
        println!("没有找到任何匹配的忽略条目！");
        return;
    }

    sorted_items.sort();
    for (index, item) in sorted_items.iter().enumerate() {
        println!("{}: {}", index + 1, helper::highlight(item, keywords));
    }
}

pub fn add(files: Vec<String>) {
    let core = core::Core::new();
    match core.update_gitignore(files) {
        Ok(items) => {
            for item in &items {
                println!("{}", item);
            }
            println!("\n本次成功更新 {} 个忽略条目\n", items.len());
        }
        Err(e) => {
            debug!("Failed to update .gitignore: {e}");
        }
    }
}

pub fn local(items: Vec<String>) {
    let mut core = core::Core::new();
    let items = remove_duplicates(items);
    match core.append(items) {
        Ok(items) => {
            for item in &items {
                println!("{}", item);
            }
            println!("\n本次成功更新 {} 个忽略条目\n", items.len());
        }
        Err(e) => {
            debug!("Failed to update .gitignore: {e}");
        }
    };
}

pub fn show(files: Vec<String>) {
    let core = core::Core::new();
    let mut sorted_items = core.get_lines_by_files(files);
    if sorted_items.len() == 0 {
        println!("没有找到任何匹配的忽略条目！请通过 gitig search <TYPE> 确认是否支持");
        return;
    }

    sorted_items.sort();
    for (index, item) in sorted_items.iter().enumerate() {
        println!("{}: {}", index + 1, item);
    }
}

pub fn list() {
    let core = core::Core::new();
    let mut sorted_items = core.get_files();
    sorted_items.sort();
    for (index, item) in sorted_items.iter().enumerate() {
        println!("{}: {}", index + 1, item);
    }
}

pub fn version() {
    println!("{}", helper::get_version());
}

pub fn remove_duplicates(vec: Vec<String>) -> Vec<String> {
    let mut set = HashSet::new();
    let mut unique_vec = Vec::new();

    for item in vec {
        // 插入到 HashSet 中，如果成功插入，则是新元素
        if set.insert(item.clone()) {
            unique_vec.push(item);
        }
    }

    unique_vec
}
