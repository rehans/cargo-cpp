# C++ Project Generator

The C++ project generator outputs a folder structure like this:

```
my-project
+-- include/my-company/my-project/
+-- source/
+-- test/
+-- CMakeLists.txt
```

Use ```./cpp-proj-gen --help``` for more help.

## Motivation

* Learning Rust ;)
* Speed up my C++ development workflow

## Prerequisite

Install rustup from here: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Clone & Build

```
git clone https://github.com/rehans/cpp-proj-gen.git
cd cpp-proj-gen
cargo build
```