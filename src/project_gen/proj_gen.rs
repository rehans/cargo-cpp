use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    name: String,
    template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    name: String,
    folders: Option<Vec<Folder>>,
    files: Option<Vec<File>>,
}

#[derive(Debug, Clone)]
pub struct ProjGen {
    vars: HashMap<String, String>,
    templates: HashMap<String, String>,
    out_dir: Option<PathBuf>,
}

impl ProjGen {
    pub fn new(domain_name: String, target_name: String, out_dir: Option<PathBuf>) -> Self {
        let templates = HashMap::from([
            (
                "res/include/header.h.in".to_string(),
                include_str!("res/include/header.h.in").to_string(),
            ),
            (
                "res/source/source.cpp.in".to_string(),
                include_str!("res/source/source.cpp.in").to_string(),
            ),
            (
                "res/CMakeLists.txt.in".to_string(),
                include_str!("res/CMakeLists.txt.in").to_string(),
            ),
        ]);

        let project_name = format!("{domain_name}-{target_name}");
        let vars = HashMap::from([
            ("@DOMAIN_NAME@".to_string(), domain_name.clone()),
            ("@TARGET_NAME@".to_string(), target_name.clone()),
            ("@PROJECT_NAME@".to_string(), project_name.clone()),
            ("@CMAKE_MINIMUM_VERSION@".to_string(), "3.19.0".to_string()),
        ]);

        Self {
            vars,
            templates,
            out_dir,
        }
    }

    pub fn gen(&self) {
        let out_dir = self
            .out_dir
            .clone()
            .unwrap_or(PathBuf::from(std::env::current_dir().unwrap().clone()));

        let folder = self.gen_folder_struct();
        self.create_folder_struct(&out_dir, &folder);
    }

    fn replace_vars(&self, mut string: String) -> String {
        for (key, value) in self.vars.iter() {
            string = string.replace(key, value);
        }

        string
    }

    pub fn gen_folder_struct(&self) -> Folder {
        let mut json_string = include_str!("res/folder_struct.json").to_string();
        json_string = self.replace_vars(json_string);

        serde_json::from_str(&json_string).unwrap()
    }

    fn create_folder(&self, out_dir: &PathBuf, folder: &Folder) -> PathBuf {
        let mut folder_path = out_dir.clone();
        folder_path.push(&folder.name);

        fs::create_dir_all(&folder_path).expect("Could not create directory {path}");
        folder_path
    }

    pub fn create_folder_struct(&self, out_dir: &PathBuf, folder: &Folder) {
        let folder_path = self.create_folder(out_dir, folder);

        if let Some(folders) = &folder.folders {
            for sub_folder in folders.iter() {
                self.create_folder_struct(&folder_path, &sub_folder);
            }
        };

        if let Some(files) = &folder.files {
            for file in files.iter() {
                let mut file_path = folder_path.clone();
                file_path.push(&file.name);

                if let Some(content_file) = &file.template {
                    if let Some(content) = self.templates.get(content_file) {
                        let mut tmp_content = content.clone();
                        tmp_content = self.replace_vars(tmp_content);
                        fs::write(file_path, tmp_content).expect("Could not write file {path}!")
                    }
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let out_dir = Some(PathBuf::from(std::env::current_dir().unwrap().clone()));
        let proj_gen = ProjGen::new("hao".to_string(), "mylib".to_string(), out_dir);
        let proj_struct = proj_gen.gen_folder_struct();
        println!("{proj_struct:#?}");

        proj_gen.gen();
    }
}
