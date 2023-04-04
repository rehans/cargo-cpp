# Cargo CPP

Cargo C++ project generator outputs a folder structure:
```shell
cargo-cpp new -t my-project -d my-company
```

Results in a project folder structure like this:
```shell
my-project
├── CMakeLists.txt
├── external
│   └── CMakeLists.txt
├── include
│   └── my-company
│       └── my-project
│           └── my-project.h
├── README.md
├── source
│   ├── main.cpp
│   └── my-project.cpp
└── test
```

Afterwards build and execute the project:
```shell
cd my-project
mkdir build
cd build
cmake ..
cmake --build .
./my-project-app
```

Use ```./cargo-cpp --help``` for more help.

Run with logging:
```shell
RUST_LOG=INFO cargo-cpp new -t my-project -d my-company
```

## Motivation

* Learning Rust ;)
* Having a tool like cargo for C++
* Speeding up my C++ development workflow

## Prerequisite

Install rustup from here: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Clone & Build

```
git clone https://github.com/rehans/cargo-cpp.git
cd cargo-cpp
cargo build
cargo run -- new -t my-project -d my-company
```