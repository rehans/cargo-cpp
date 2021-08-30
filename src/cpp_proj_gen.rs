/*
For 'unwrap' /sa https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html
For 'iter' and 'collect' /sa  https://doc.rust-lang.org/std/path/struct.PathBuf.html#examples
For '?' /sa https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
*/

use std::{collections::HashMap, fs, path::PathBuf};

use structopt::StructOpt;

// Constants
const CMAKELISTS_FILE_STR: &str = "
    cmake_minimum_required(VERSION @CMAKE_MINIMUM_VERSION@)

    project(@CMAKE_PROJECT_NAME@)

    add_library(@CMAKE_TARGET_NAME@ STATIC
        # @INCLUDE_DIR@/@INCLUDE_DOMAIN_DIR@/@CMAKE_TARGET_NAME@.h
        # @SOURCE_DIR@/@CMAKE_TARGET_NAME@.cpp
    )

    target_include_directories(@CMAKE_TARGET_NAME@
        PUBLIC
            ${CMAKE_CURRENT_LIST_DIR}/@INCLUDE_DIR@
        PRIVATE
            ${CMAKE_CURRENT_LIST_DIR}/@SOURCE_DIR@
    )
";

const CMAKELISTS_FILENAME: &str = "CMakeLists.txt";

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
    #[structopt(short, long, parse(from_os_str))]
    output_dir: Option<PathBuf>,
}

// CppProjGen
pub struct CppProjGen {
    directories: Vec<PathBuf>,
    cmake_lists_file: PathBuf,
    cmake_vars: HashMap<String, String>,
    opt: Opt,
    out_dir: PathBuf,
}

impl CppProjGen {
    pub fn new(opt: Opt) -> CppProjGen {
        let mut vars = HashMap::new();
        vars.insert(
            String::from("@CMAKE_MINIMUM_VERSION@"),
            String::from(&opt.cmake_version),
        );

        vars.insert(
            String::from("@CMAKE_TARGET_NAME@"),
            String::from(&opt.target_name),
        );

        vars.insert(
            String::from("@CMAKE_PROJECT_NAME@"),
            build_cmake_project_name(&opt, "-"),
        );

        vars.insert(
            String::from("@INCLUDE_DOMAIN_DIR@"),
            build_cmake_project_name(&opt, "/"),
        );

        CppProjGen {
            directories: Vec::new(),
            cmake_lists_file: PathBuf::from(CMAKELISTS_FILENAME),
            cmake_vars: vars,
            out_dir: build_out_dir(&opt),
            opt: opt,
        }
    }

    pub fn add_include_dir(mut self, dir: PathBuf) -> CppProjGen {
        self.cmake_vars.insert(
            String::from("@INCLUDE_DIR@"),
            String::from(dir.to_str().unwrap()),
        );

        let local_include_dir: PathBuf = build_cmake_local_include_dir(&self.opt, dir);

        self.add_toplevel_dir(local_include_dir)
    }

    pub fn add_source_dir(mut self, dir: PathBuf) -> CppProjGen {
        self.cmake_vars.insert(
            String::from("@SOURCE_DIR@"),
            String::from(dir.to_str().unwrap()),
        );

        self.add_toplevel_dir(dir)
    }

    pub fn add_toplevel_dir(mut self, dir: PathBuf) -> CppProjGen {
        self.directories.push(dir);

        self
    }

    pub fn create(&self) -> std::io::Result<()> {
        let contents = replace_cmake_vars(&self.cmake_vars);
        let paths = self.build_paths();
        create_all_paths(paths, contents)?;

        Ok(())
    }

    pub fn build_paths(&self) -> Vec<PathBuf> {
        let mut paths = Vec::new();

        for dir in &self.directories {
            paths.push(make_absolute_path(&self.out_dir, dir));
        }

        paths.push(make_absolute_path(&self.out_dir, &self.cmake_lists_file));

        paths
    }
}

fn build_cmake_local_include_dir(opt: &Opt, dir: PathBuf) -> PathBuf {
    let result_dir: PathBuf = if opt.name_space.is_none() {
        // e.g. include/target-name
        [dir, PathBuf::from(&opt.target_name)].iter().collect()
    } else {
        // e.g. include/name-space/target-name
        [
            dir,
            PathBuf::from(opt.name_space.as_ref().unwrap()),
            PathBuf::from(&opt.target_name),
        ]
        .iter()
        .collect()
    };

    result_dir
}

fn build_out_dir(opt: &Opt) -> PathBuf {
    let parent = match &opt.output_dir {
        Some(p) => p.clone(),
        None => PathBuf::from(std::env::current_dir().unwrap().clone()),
    };

    let out_dir: PathBuf = [parent, PathBuf::from(&opt.target_name)].iter().collect();

    out_dir
}

fn make_absolute_path(out_dir: &PathBuf, dir: &PathBuf) -> PathBuf {
    [out_dir, dir].iter().collect()
}

fn replace_cmake_vars(cmake_vars: &HashMap<String, String>) -> String {
    let mut content = String::from(CMAKELISTS_FILE_STR);

    for (var, value) in cmake_vars {
        content = content.replace(var, value);
    }

    content
}

fn build_cmake_project_name(opt: &Opt, delimiter: &str) -> String {
    let project_name = if opt.name_space.is_none() {
        String::from(&opt.target_name)
    } else {
        let tmp = format!(
            "{}{}{}",
            opt.name_space.as_ref().unwrap(),
            delimiter,
            &opt.target_name
        );
        tmp
    };

    project_name
}

fn create_all_paths(paths: Vec<PathBuf>, contents: String) -> std::io::Result<()> {
    for path in paths {
        // TODO: How to distinguish between file and dir?
        if path.ends_with("CMakeLists.txt") {
            fs::write(path, &contents)?;
        } else {
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_opt() -> Opt {
        let opt = Opt {
            name_space: Some(String::from("nmspc")),
            target_name: String::from("tgtnm"),
            cmake_version: String::from("1.23.4"),
            output_dir: Some(PathBuf::from("test_out_dir")),
        };

        opt
    }

    #[test]
    fn test_path_vec_len() {
        let opt = create_test_opt();

        let cpp_proj_gen = CppProjGen::new(opt)
            .add_include_dir(PathBuf::from("include"))
            .add_toplevel_dir(PathBuf::from("test"))
            .add_source_dir(PathBuf::from("source"));

        let paths = cpp_proj_gen.build_paths();
        assert_eq!(paths.len(), 4);
    }

    #[test]
    fn test_path_vec_items() {
        let opt = create_test_opt();

        let cpp_proj_gen = CppProjGen::new(opt)
            .add_include_dir(PathBuf::from("include"))
            .add_toplevel_dir(PathBuf::from("test"))
            .add_source_dir(PathBuf::from("source"));

        let paths = cpp_proj_gen.build_paths();

        assert_eq!(
            paths.contains(&PathBuf::from("test_out_dir/tgtnm/include/nmspc/tgtnm")),
            true
        );

        assert_eq!(
            paths.contains(&PathBuf::from("test_out_dir/tgtnm/test")),
            true
        );

        assert_eq!(
            paths.contains(&PathBuf::from("test_out_dir/tgtnm/source")),
            true
        );

        assert_eq!(
            paths.contains(&PathBuf::from("test_out_dir/tgtnm/CMakeLists.txt")),
            true
        );

        println!("{:#?}", paths);
    }

    #[test]
    fn test_cmake_vars() {
        let opt = create_test_opt();

        let cpp_proj_gen = CppProjGen::new(opt)
            .add_include_dir(PathBuf::from("include"))
            .add_toplevel_dir(PathBuf::from("test"))
            .add_source_dir(PathBuf::from("source"));

        println!("{:#?}", cpp_proj_gen.cmake_vars);

        let result = replace_cmake_vars(&cpp_proj_gen.cmake_vars);
        println!("{}", result);
    }

    #[test]
    fn test_include_dir_without_namespace() {
        let opt = Opt {
            name_space: None,
            target_name: String::from("tgtnm"),
            cmake_version: String::from("1.23.4"),
            output_dir: Some(PathBuf::from("test_out_dir")),
        };

        let cpp_proj_gen = CppProjGen::new(opt)
            .add_include_dir(PathBuf::from("include"))
            .add_toplevel_dir(PathBuf::from("test"))
            .add_source_dir(PathBuf::from("source"));

        let paths = cpp_proj_gen.build_paths();
        // println!("{:#?}", paths);

        assert_eq!(
            paths.contains(&PathBuf::from("test_out_dir/tgtnm/include/tgtnm")),
            true
        );

        // let result = replace_cmake_vars(&cpp_proj_gen.cmake_vars);
        // println!("{}", result);
    }
}
