# gitig

![gitig](./docs/image/logo.png)

`gitig(nore)` 是一款帮助开发者将项目中常见的不需要版本控制的文件和目录添加到本地 `.gitignore` 文件的工具。它的主要数据来源自 https://github.com/github/gitignore 仓库，该仓库提供了一系列有用的 `.gitignore` 模板。



## 功能列表

- `gitig add <TYPE>`：将指定类型 `<TYPE>` 的忽略条目写入本地 `.gitignore` 文件中
- `gitig local <FILE>` 手动添加忽略项
- `gitig list`：查看当前支持的所有项目类型
- `gitig search <keywords>`：根据关键字搜索相关类型
- `gitig show <TYPE>`：查看指定项目类型 `<TYPE>` 的忽略清单

> gitig add 命令将直接修改用户当前目录的 `.gitignore` 文件, 并将新的条目添加到文件末尾，并不会修改原来已存在的条目。



## 安装

### 手动下载安装

下载并解压安装包，将 `gitig` 可执行文件移动到 `/usr/local/bin/` 目录或将其添加到 `$PATH` 环境变量。

下载地址:

- [MacOS x86_64](https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-latest-x86_64-apple-darwin.tar.gz)
- [MacOS aarch64](https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-latest-aarch64-apple-darwin.tar.gz)
- [Linux x86_64](https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-latest-x86_64-unknown-linux-gnu.tar.gz)
- [Linux aarch64](https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-latest-aarch64-unknown-linux-gnu.tar.gz)
- [Windows x86_64](https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-latest-x86_64-pc-windows-msvc.zip)
- [Windows aarch64](https://githubfiles.oss-cn-shanghai.aliyuncs.com/gitig/gitig-latest-aarch64-pc-windows-msvc.zip)

所有发布版本请[点击这里](https://github.com/cfanbo/gitig/releases)

### shell 脚本安装

您可以在macOS终端或Linux shell提示符中粘贴以下命令并执行，注意权限问题。

```shell
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/cfanbo/gitig/HEAD/install.sh)"
```

如果您使用的是 `zsh`的话，命令行前面更改为 `/bin/zsh`即可。

### 编译安装

如果您是Rust开发者，也可以选择在本地进行编译安装。不过由于程序依赖于 [github.com/github/gitignore](github.com/github/gitignore) 项目，需要将其 `clone` 到本地，并将其与当前项目放在同一个目录，保持两个项目目录同级关系。

```shell
git clone https://github.com/github/gitignore gitignore-repo
git clone https://github.com/cfanbo/gitig.git
cd gitig
cargo install --path . # 将自动安装到 ~/.cargo/bin/ 目录
gitig -v # 查看版本号
```



## 使用

### 添加忽略条目

使用 `gitig add <TYPE>` 命令，将指定类型的常用忽略条目添加到当前工作目录的 `.gitignore` 文件中。

```shell
$ gitig add Rust
```

上面命令将 rust 项目推荐的 `.gitignore`忽略条目添加到本地 `.gitignore` 文件。

也可以一次指定多个类型，如：

```shell
$ gitig add python java rust
```

您也可以手动添加本地文件或目录项

```shell
$ gitig local .zed/ .vscode/ .output/
```



### 查看支持的项目类型

使用 `gitig list` 命令，查看当前支持的所有项目类型。

```shell
$ gitig list
```

### 搜索相关项目类型

使用 `gitig search <keywords>` 命令，通过关键词查询相关的项目类型。

```shell
$ gitig search ja
$ gitig search visual
```

### 查看忽略条目清单

使用 `gitig show <TYPE>` 命令，查看指定项目类型的忽略清单。

```shell
$ gitig show Python
```



## 说明

项目忽略清单配置主要来源于仓库 [https://github.com/github/gitignore](https://github.com/github/gitignore)，不过由于此仓库维护频率过低，导致许多用户提交的新配置迟迟无法 merge, 因此基于此仓库 fork 了一个新仓库 [https://github.com/cfanbo/gitignore](https://github.com/cfanbo/gitignore)，后期将由本人维护，同时定期同步上游变更。



## 贡献

如果你有任何建议或发现任何问题，欢迎提交 [issues](https://github.com/cfanbo/gitig/issues) 或发起 [pull requests](https://github.com/cfanbo/gitig/pulls)。



## 许可证

`gitig` 遵循 MIT 许可证。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

