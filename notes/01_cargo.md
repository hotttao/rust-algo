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
```

## 1. Cargo

```shell
# 项目初始化
$ cargo new rust_algo

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
