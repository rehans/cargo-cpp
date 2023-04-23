// Copyright(c) 2023 rehans.

use crate::project::file;
use crate::project::PathType;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    name: String,
    folders: Option<Vec<Folder>>,
    files: Option<Vec<file::File>>,
}

impl Folder {
    fn create_at<F>(&self, out_dir: &PathBuf, fn_create: &F) -> PathBuf
    where
        F: Fn(&PathType),
    {
        let path = out_dir.join(&self.name);
        fn_create(&PathType::Folder { path: path.clone() });

        path
    }

    pub fn create_recursively_at<F>(&self, out_dir: &PathBuf, f: &F) -> PathBuf
    where
        F: Fn(&PathType),
    {
        let path = self.create_at(out_dir, f);

        // folders
        if let Some(folders) = &self.folders {
            for sub_folder in folders.iter() {
                sub_folder.create_recursively_at(&path, f);
            }
        };

        // files
        if let Some(files) = &self.files {
            for file in files.iter() {
                file.create_at(&path, f);
            }
        };

        path
    }
}
