use crate::cli::cmd::{self, Help};
use clap::{arg, Arg, Command};
use log::{debug, info};
use std::ffi::OsString;
pub fn run() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("list", _sub_matches)) => {
            cmd::list();
        }
        Some(("add", sub_matches)) => {
            let types = sub_matches
                .get_many::<String>("TYPE")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            info!("Adding {types:?}");

            let typs: Vec<String> = types.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>();
            cmd::add(typs);
        }
        Some(("show", sub_matches)) => {
            let types = sub_matches
                .get_many::<String>("TYPE")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            debug!("Show {types:?}");

            let typs: Vec<String> = types.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>();
            cmd::show(typs);
        }
        Some(("search", sub_matches)) => {
            let keywords = sub_matches
                .get_one::<String>("TYPE")
                .map(|s| s.as_str())
                .expect("defaulted in clap");
            debug!("Search {keywords:?}");

            cmd::search(keywords);
        }
        Some(("version", _sub_matches)) => {
            cmd::version();
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            debug!("不支持命令 {ext:?} 及参数 {args:?}");
            println!("不支持命令{ext:?}");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

fn cli() -> Command {
    Command::new("gitig")
        .about("gitig - 一款快速生成 gitignore 条目的命令行工具")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .after_help(Help::default())
        .subcommand(
            Command::new("add")
                .about("添加相关 gitignore 条目")
                .arg_required_else_help(true)
                .after_help(Help::add())
                .arg(
                    Arg::new("TYPE")
                        .help("项目类型, 多个类型之间以空格隔开")
                        .required(true)
                        .num_args(1..), // 允许一个或多个参数
                ), // .arg(arg!(<TYPE> "项目类型, 如 Go、Java、Rust 等, 不区分大小写")),
        )
        .subcommand(Command::new("list").about("列出所有支持的类型"))
        .subcommand(
            Command::new("show")
                .about("查看类型默认忽略条目")
                .arg_required_else_help(true)
                .arg(arg!(<TYPE> "项目类型, 如 Go、Java、Rust 等, 不区分大小写")),
        )
        .subcommand(
            Command::new("search")
                .about("搜索项目类型")
                .arg_required_else_help(true)
                .after_help(Help::search())
                .arg(arg!(<TYPE> "以关键字搜索项目类型, 不区分大小写")),
        )
        .subcommand(Command::new("version").short_flag('v').about("查看版本号"))
}
