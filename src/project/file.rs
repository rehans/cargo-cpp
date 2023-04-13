// Copyright(c) 2023 rehans.

use super::PathType;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    name: String,
    template: Option<String>,
}

impl File {
    pub fn create_at<F>(&self, out_dir: &PathBuf, fn_create: &F) -> PathBuf
    where
        F: Fn(&PathType),
    {
        let path = out_dir.join(&self.name);
        let path_type = PathType::File {
            path: path.clone(),
            opt_template_file: self.template.clone(),
        };
        fn_create(&path_type);

        path
    }
}
