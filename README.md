# Cargo CPP

The C++ project generator outputs a folder structure like this:

```                                                             ✔ 
my-project
├── CMakeLists.txt
├── external
├── include
│   └── my-company
│       └── my-project
│           └── my-project.h
├── README.md
├── source
│   └── my-project.cpp
└── test
```

Use ```./cargo-cpp --help``` for more help.

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
```