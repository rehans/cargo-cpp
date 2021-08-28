/*
For 'unwrap' /sa https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
For 'iter' and 'collect' /sa  https://doc.rust-lang.org/std/path/struct.PathBuf.html#examples
For '?' /sa https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
*/

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

// Constants
const CMAKELISTS_FILE_STR: &str = "
    cmake_minimum_required(VERSION @CMAKE_VERSION@)

    project(@COMPANY_NAME@-@PROJECT_NAME@)

    add_library(@PROJECT_NAME@-lib STATIC
        # include/@COMPANY_NAME@/@PROJECT_NAME@/@PROJECT_NAME@.h
        # source/@PROJECT_NAME@.cpp
    )
";

const CMAKELISTS_FILENAME: &str = "CMakeLists.txt";
const INCLUDE_DIR_NAME: &str = "include";

// Options
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cpp-proj-gen", about = "C++ project generator.")]
pub struct Opt {
    // Company name
    #[structopt(short, long, default_value = "my-company")]
    company: String,

    // Project name
    #[structopt(short, long, default_value = "my-project")]
    pub project: String,

    // CMake version
    #[structopt(short = "m", long = "cmake", default_value = "3.15.0")]
    cmake_version: String,

    // Output directory
    #[structopt(short, long, parse(from_os_str), default_value = "")]
    pub output_dir: PathBuf,
}

// CppProjGen
pub struct CppProjGen {
    folders: Vec<PathBuf>,
    cmake_lists_file: PathBuf,
    opt: Opt,
}

impl CppProjGen {
    pub fn new(opt: Opt) -> CppProjGen {
        CppProjGen {
            folders: Vec::new(),
            cmake_lists_file: PathBuf::from("CMakeLists.txt"),
            opt: opt,
        }
    }

    pub fn add_include_dir(self) -> CppProjGen {
        let local_include_dir: PathBuf = [INCLUDE_DIR_NAME, &self.opt.company, &self.opt.project]
            .iter()
            .collect();

        self.add_toplevel_dir(local_include_dir)
    }

    pub fn add_toplevel_dir(mut self, dir: PathBuf) -> CppProjGen {
        self._add_toplevel_dir(dir);

        self
    }

    pub fn add_cmake_lists_file(mut self) -> CppProjGen {
        let cmake_lists_file_path: PathBuf =
            [&self.opt.output_dir, &PathBuf::from(CMAKELISTS_FILENAME)]
                .iter()
                .collect();

        self.cmake_lists_file = cmake_lists_file_path;

        self
    }

    pub fn create(&self) -> std::io::Result<()> {
        self.create_folders()?;
        self.create_cmake_lists_file()?;

        Ok(())
    }

    pub fn print(self) -> CppProjGen {
        for item in &self.folders {
            println!("Created: {:?}", item);
        }

        println!("Created: {:?}", &self.cmake_lists_file);

        self
    }

    // private
    fn create_folders(&self) -> std::io::Result<()> {
        for item in &self.folders {
            fs::create_dir_all(item)?;
        }

        Ok(())
    }

    fn create_cmake_lists_file(&self) -> std::io::Result<()> {
        // Replace all variables
        let cmake_lists_file_content = String::from(CMAKELISTS_FILE_STR)
            .replace("@PROJECT_NAME@", &self.opt.project)
            .replace("@COMPANY_NAME@", &self.opt.company)
            .replace("@CMAKE_VERSION@", &self.opt.cmake_version);

        // Create CMakeLists.txt
        File::create(&self.cmake_lists_file)?.write_all(cmake_lists_file_content.as_bytes())?;

        Ok(())
    }

    fn _add_toplevel_dir(&mut self, dir: PathBuf) {
        let toplevel_dir: PathBuf = [&self.opt.output_dir, &dir].iter().collect();

        self.folders.push(toplevel_dir);
    }
}