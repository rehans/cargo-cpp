use crate::project_gen::cmake_file_gen;

use super::cpp_file_gen::FileGen;
use super::directory_gen::{DirName, DirectoryGen};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DirPath {
    Root { path: PathBuf },
    HeaderInclude { path: PathBuf },
    Source { path: PathBuf },
    Test { path: PathBuf },
    External { path: PathBuf },
}

pub fn gen_project(domain_name: String, target_name: String, out_dir: PathBuf) {
    let dir_gen = DirectoryGen::new()
        .add_toplevel_dir(&DirName::Include {
            name: "include".to_string(),
        })
        .add_toplevel_dir(&DirName::Source {
            name: "source".to_string(),
        })
        .add_toplevel_dir(&DirName::Test {
            name: "test".to_string(),
        })
        .add_toplevel_dir(&DirName::External {
            name: "external".to_string(),
        })
        .set_domain_name(&domain_name)
        .set_target_name(&target_name)
        .set_out_dir(&out_dir);

    let dirs = dir_gen.create_dirs();

    println!("{:#?}", dirs);

    let files = FileGen::new()
        .set_dirs(&dirs)
        .set_domain_name(&domain_name)
        .set_target_name(&target_name)
        .create_files();

    println!("{:#?}", files);

    let cmake_file = cmake_file_gen::FileGen::new()
        .set_out_dir(&out_dir)
        .set_domain_name(&domain_name)
        .set_target_name(&target_name)
        .set_dir_names(&dir_gen.toplevel_dir_names())
        .create_file();

    println!("{:#?}", cmake_file);
}
