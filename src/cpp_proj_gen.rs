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

use structopt::StructOpt;

// Constants
const CMAKELISTS_FILE_STR: &str = "
    cmake_minimum_required(VERSION @CMAKE_VERSION@)

    project(@CMAKE_PROJECT_NAME@)

    add_library(@CMAKE_TARGET_NAME@ STATIC
        # include/@INCLUDE_DIR@/@CMAKE_TARGET_NAME@.h
        # source/@CMAKE_TARGET_NAME@.cpp
    )

    target_include_directories(@CMAKE_TARGET_NAME@
        PUBLIC include
        PRIVATE source
    )
";

const CMAKELISTS_FILENAME: &str = "CMakeLists.txt";
const INCLUDE_DIR_NAME: &str = "include";

// Options
#[derive(Debug, StructOpt)]
#[structopt(name = "cpp-proj-gen", about = "C++ project generator.")]
pub struct Opt {
    // Project name
    #[structopt(short, long, help = "e.g. company name")]
    name_space: Option<String>,

    // Target name
    #[structopt(short, long, default_value = "my-target")]
    target_name: String,

    // CMake version
    #[structopt(short, long, default_value = "3.15.0")]
    cmake_version: String,

    // Output directory
    #[structopt(short, long, parse(from_os_str), default_value = "")]
    output_dir: PathBuf,
}

// CppProjGen
pub struct CppProjGen {
    folders: Vec<PathBuf>,
    cmake_lists_file: PathBuf,
    opt: Opt,
    out_dir: PathBuf,
}

impl CppProjGen {
    pub fn new(opt: Opt) -> CppProjGen {
        CppProjGen {
            folders: Vec::new(),
            cmake_lists_file: PathBuf::from("CMakeLists.txt"),
            out_dir: make_out_dir(&opt),
            opt: opt,
        }
    }

    pub fn add_include_dir(self) -> CppProjGen {
        let local_include_dir: PathBuf = self.get_cmake_local_include_dir();

        self.add_toplevel_dir(local_include_dir)
    }

    pub fn add_toplevel_dir(mut self, dir: PathBuf) -> CppProjGen {
        let toplevel_dir: PathBuf = [&self.out_dir, &dir].iter().collect();
        self.folders.push(toplevel_dir);

        self
    }

    pub fn add_cmake_lists_file(mut self) -> CppProjGen {
        let cmake_lists_file_path: PathBuf = [&self.out_dir, &PathBuf::from(CMAKELISTS_FILENAME)]
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
        // Print all options form cli in debug print by using ':?'
        println!("CLI options: {:?}", &self.opt);

        for item in &self.folders {
            println!("Created: {:?}", item);
        }

        println!("Created: {:?}", &self.cmake_lists_file);

        println!("{}", self.replace_cmake_content_file_variables());

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
        let cmake_lists_file_content = self.replace_cmake_content_file_variables();

        // Create CMakeLists.txt
        File::create(&self.cmake_lists_file)?.write_all(cmake_lists_file_content.as_bytes())?;

        Ok(())
    }

    fn replace_cmake_content_file_variables(&self) -> String {
        let local_include_dir = self.get_cmake_local_include_dir();
        let project_name = self.get_cmake_project_name();
        let cmake_lists_file_content = String::from(CMAKELISTS_FILE_STR)
            .replace("@INCLUDE_DIR@", &local_include_dir.display().to_string())
            .replace("@CMAKE_TARGET_NAME@", &self.opt.target_name)
            .replace("@CMAKE_PROJECT_NAME@", &project_name)
            .replace("@CMAKE_VERSION@", &self.opt.cmake_version);

        cmake_lists_file_content
    }

    fn get_cmake_project_name(&self) -> String {
        let project_name = if self.opt.name_space.is_none() {
            String::from(&self.opt.target_name)
        } else {
            let tmp = format!(
                "{}-{}",
                self.opt.name_space.as_ref().unwrap(),
                &self.opt.target_name
            );
            tmp
        };

        project_name
    }

    fn get_cmake_local_include_dir(&self) -> PathBuf {
        let local_include_dir: PathBuf = if self.opt.name_space.is_none() {
            // include/target-name
            [
                String::from(INCLUDE_DIR_NAME),
                String::from(&self.opt.target_name),
            ]
            .iter()
            .collect()
        } else {
            // include/name-space/target-name
            [
                String::from(INCLUDE_DIR_NAME),
                String::from(self.opt.name_space.as_ref().unwrap()),
                String::from(&self.opt.target_name),
            ]
            .iter()
            .collect()
        };

        local_include_dir
    }
}

fn make_out_dir(opt: &Opt) -> PathBuf {
    let is_out_dir_empty = opt.output_dir.as_os_str().is_empty();
    let out_dir_parent = if is_out_dir_empty {
        std::env::current_dir().unwrap()
    } else {
        opt.output_dir.clone()
    };

    let tmp_out_dir: PathBuf = [out_dir_parent, PathBuf::from(&opt.target_name)]
        .iter()
        .collect();

    tmp_out_dir
}
