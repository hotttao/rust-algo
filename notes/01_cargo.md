# 使用 Cargo 初始化一个 Rust 项目

## 1. rust 安装
```shell
# 安装
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# 更新
rustup update

# 卸载 
rustup self uninstall

# 验证
$ rustc  --version
rustc 1.65.0 (897e37553 2022-11-02)

# 离线文档
rustup doc
# 在本地构建一份有关所有依赖的文档，并自动地在浏览器中将文档打开来供你查阅
cargo doc --open
```

## 1. Cargo
### 1.1 Cargo 基础

```shell
# 项目初始化
# 创建一个二进制单元
$ cargo new rust_algo
# 创建一个库单元
$ cargo new --lib restaurant

# 编译项目
$ cd rust_algo
$ cargo build

$ tree rust_algo/ -L 3
rust_algo/
├── Cargo.lock            # 这个文件记录了当前项目所有依赖库的具体版本号
├── Cargo.toml            # cargo 配置文件
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        ├── examples
        ├── incremental
        ├── rust_algo       # 编译生成的可执行文件
        └── rust_algo.d

# 运行程序
# 方法一:
$ ./target/debug/rust_algo 
Hello, world!

# 编译 + 运行
$ cargo run

# 检查代码是否可以通过编译
$ cargo check

# release 模式构建
# cargo build --release在优化模式下构建并生成可执行程序。它生成的可执行文件会被放置在target/release目录下
# 这种模式会以更长的编译时间为代价来优化代码，从而使代码拥有更好的运行时性能。
$ cargo build --release
```

### 1.1 Cargo.toml
Cargo使用TOML（Tom's Obvious, Minimal Language）作为标准的配置格式

```toml
[package]
name = "rust_algo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

Cargo 的以来管理:
1. `[dependencies]` 区域被用来声明项目中需要用到的全部依赖包及其版本号。
2. Cargo会按照标准的语义化版本系统（Semantic Versioning，SemVer）来理解所有的版本号。这里的数字0.8.5实际上是^0.8.5的一个简写，它表示“任何与0.8.5版本公共API相兼容的版本”。
3. Cargo可以从注册表（registry）中获取所有可用库的最新版本信息，而这些信息通常是从crates.io上复制过来的。在Rust的生态中，crates.io是人们用于分享各种各样开源Rust项目的网站。

### 1.2 Cargo.lock
类似 gomod.lock。当你第一次构建项目时，Cargo会依次遍历我们声明的依赖及其对应的语义化版本，找到符合要求的具体版本号，并将它们写入Cargo.lock文件中。随后再次构建项目时，Cargo就会优先检索Cargo.lock，假如文件中存在已经指明具体版本的依赖库，那么它就会跳过计算版本号的过程，并直接使用文件中指明的版本。这使得我们拥有了一个自动化的、可重现的构建系统。

要升级某个依赖包时，Cargo提供了一个专用命令：update，它会强制Cargo忽略Cargo.lock文件，并重新计算出所有依赖包中符合Cargo.toml声明的最新版本，并覆盖 Cargo.lock。`cargo update` 只会基于语义化版本的规则，对依赖进行升级，如果需要不兼容升级，必须手动修改 Cargo.toml 文件。