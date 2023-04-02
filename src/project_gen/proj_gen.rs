use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjFile {
    name: String,
    template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjFolder {
    name: String,
    folders: Option<Vec<ProjFolder>>,
    files: Option<Vec<ProjFile>>,
}

#[derive(Debug, Clone)]
pub struct ProjGen {
    vars: HashMap<String, String>,
    templates: HashMap<String, String>,
    out_dir: Option<PathBuf>,
}

impl ProjGen {
    pub fn new(domain_name: String, target_name: String, out_dir: Option<PathBuf>) -> Self {
        Self {
            vars: Self::create_vars(&domain_name.make_conform(), &target_name.make_conform()),
            templates: Self::create_templates(),
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

    fn gen_folder_struct(&self) -> ProjFolder {
        let mut json_string = include_str!("res/folder_struct.json").to_string();
        json_string = self.replace_vars(json_string);

        serde_json::from_str(&json_string).unwrap()
    }

    fn create_folder(&self, out_dir: &PathBuf, folder: &ProjFolder) -> PathBuf {
        let mut folder_path = out_dir.clone();
        folder_path.push(&folder.name);

        fs::create_dir_all(&folder_path).expect("Could not create directory {path}");
        folder_path
    }

    fn create_folder_struct(&self, out_dir: &PathBuf, folder: &ProjFolder) {
        let folder_path = self.create_folder(out_dir, folder);

        // folders
        if let Some(folders) = &folder.folders {
            self.create_folders_at(folders, &folder_path);
        };

        // files
        if let Some(files) = &folder.files {
            self.create_files_at(files, &folder_path);
        };
    }

    fn create_files_at(&self, files: &Vec<ProjFile>, folder_path: &PathBuf) {
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
    }

    fn create_folders_at(&self, folders: &Vec<ProjFolder>, folder_path: &PathBuf) {
        for sub_folder in folders.iter() {
            self.create_folder_struct(folder_path, &sub_folder);
        }
    }

    fn create_vars(domain_name: &String, target_name: &String) -> HashMap<String, String> {
        let project_name = format!("{domain_name}-{target_name}");
        HashMap::from([
            ("@DOMAIN_NAME@".to_string(), domain_name.clone()),
            ("@TARGET_NAME@".to_string(), target_name.clone()),
            ("@PROJECT_NAME@".to_string(), project_name.clone()),
            ("@CMAKE_MINIMUM_VERSION@".to_string(), "3.19.0".to_string()),
        ])
    }

    fn create_templates() -> HashMap<String, String> {
        HashMap::from([
            (
                "include/print.h".to_string(),
                include_str!("res/include/print.h.in").to_string(),
            ),
            (
                "source/print_hello_world.cpp".to_string(),
                include_str!("res/source/print_hello_world.cpp.in").to_string(),
            ),
            (
                "CMakeLists.txt".to_string(),
                include_str!("res/CMakeLists.txt.in").to_string(),
            ),
            (
                "README.md".to_string(),
                include_str!("res/README.md.in").to_string(),
            ),
            (
                "external/CMakeLists.txt".to_string(),
                include_str!("res/external/CMakeLists.txt.in").to_string(),
            ),
            (
                "source/main.cpp".to_string(),
                include_str!("res/source/main.cpp.in").to_string(),
            ),
        ])
    }
}

trait Conform {
    fn make_conform(&self) -> Self;
}

impl Conform for String {
    fn make_conform(&self) -> String {
        // Replace all whitespaces and make lower case
        let mut conform_name = self.replace(char::is_whitespace, "_");
        conform_name = conform_name.to_lowercase();
        conform_name
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
