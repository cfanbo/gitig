# gitig

`gitig(nore)` 是一款帮助开发者将项目中常见的不需要版本控制的文件和目录添加到本地 `.gitignore` 文件的工具。它的主要数据来源是 GitHub 上的 https://github.com/github/gitignore 仓库，该仓库提供了一系列有用的 `.gitignore` 模板。



## 功能列表

- `gitig add <TYPE>`：将指定类型 `<TYPE>` 的忽略条目写入本地 `.gitignore` 文件中
- `gitig list`：查看当前支持的所有项目类型
- `gitig search <keywords>`：根据关键字搜索相关类型
- `gitig show <TYPE>`：查看指定项目类型 `<TYPE>` 的忽略清单

> gitig add 命令将直接修改用户当前目录的 `.gitignore` 文件, 并将新的条目添加到文件末尾，并不会修改原来已存在的条目。



## 安装

`gitig` 的预编译二进制文件档案可用于 Windows、macOS 和 Linux。

直接从 https://github.com/cfanbo/gitig/releases 下载相对应的软件包解压即可，以 Linux 为例

```shell
tar zxvf gitig-x86_64-unknown-linux-gnu.tar.gz
mv gitig /usr/local/bin/
chmod +x /usr/local/bin/gitig
```



另外也可以选择在本地进行编译安装，不过由于程序依赖于 [github.com/github/gitignore](github.com/github/gitignore) 项目，需要将其 clone 到本地，并将其与当前项目放在同一个目录，保持两个项目目录同级关系。

```shell
git clone https://github.com/github/gitignore gitignore-repo
git clone https://github.com/cfanbo/gitig.git
cd gitig
cargo build --release # 编译程序
target/release/gitig -v # 查看版本号
```



## 使用

### 添加忽略条目

使用 `gitig add <TYPE>` 命令，将指定类型的常用忽略条目添加到当前工作目录的 `.gitignore` 文件中。

```shell
gitig add Rust
```

上面命令将 rust 项目推荐的 `.gitignore`忽略条目添加到本地 `.gitignore` 文件。

也可以一次指定多个类型，如：

```shell
gitig add python java rust
```

### 查看支持的项目类型

使用 `gitig list` 命令，查看当前支持的所有项目类型。

```shell
gitig list
```

### 搜索相关项目类型

使用 `gitig search <keywords>` 命令，通过关键词查询相关的项目类型。

```shell
gitig search ja
```

### 查看忽略条目清单

使用 `gitig show <TYPE>` 命令，查看指定项目类型的忽略清单。

```shell
gitig show Python
```



## 说明

项目忽略清单配置主要来源于仓库 [https://github.com/github/gitignore](https://github.com/github/gitignore)，不过由于此仓库维护频率过低，导致许多用户提交的新配置迟迟无法 merge, 因此基于此仓库 fork 了一个新仓库 [https://github.com/cfanbo/gitignore](https://github.com/cfanbo/gitignore)，后期将由本人维护，同时定期同步上游变更。



## 贡献

如果你有任何建议或发现任何问题，欢迎提交 [issues](https://github.com/cfanbo/gitig/issues) 或发起 [pull requests](https://github.com/cfanbo/gitig/pulls)。



## 许可证

`gitig` 遵循 MIT 许可证。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

