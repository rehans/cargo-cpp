use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Project {
    vars: HashMap<String, String>,
    templates: HashMap<String, String>,
    out_dir: Option<PathBuf>,
}

impl Project {
    pub fn new(domain_name: String, target_name: String, out_dir: Option<PathBuf>) -> Self {
        Self {
            vars: Self::create_vars(&domain_name.conform(), &target_name.conform()),
            templates: Self::create_templates(),
            out_dir,
        }
    }

    pub fn gen(&self) {
        let out_dir = self
            .out_dir
            .clone()
            .unwrap_or(PathBuf::from(std::env::current_dir().unwrap().clone()));

        let folder = self.parse_json_proj_struct();

        folder.create_recursively_at(&out_dir, &|content_file| -> Option<String> {
            let opt_content = self.templates.get(content_file);
            match opt_content {
                Some(content) => Some(content.clone().replace_vars(&self.vars)),
                None => None,
            }
        });
    }

    fn parse_json_proj_struct(&self) -> ProjectFolder {
        let mut json_string = include_str!("res/folder_struct.json").to_string();
        json_string = json_string.replace_vars(&self.vars);

        serde_json::from_str(&json_string).unwrap()
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
        let templates = [
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
        ];
        HashMap::from(templates)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectFile {
    name: String,
    template: Option<String>,
}

impl ProjectFile {
    fn create_at<F>(&self, out_dir: &PathBuf, f: &F) -> PathBuf
    where
        F: Fn(&String) -> Option<String>,
    {
        let mut path = out_dir.clone();
        path.push(&self.name);

        if let Some(content_file) = &self.template {
            if let Some(content) = f(content_file) {
                fs::write(&path, content).expect("Could not write file {path}!")
            }
        }

        path
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ProjectFolder {
    name: String,
    folders: Option<Vec<ProjectFolder>>,
    files: Option<Vec<ProjectFile>>,
}

impl ProjectFolder {
    fn create_at(&self, out_dir: &PathBuf) -> PathBuf {
        let mut path = out_dir.clone();
        path.push(&self.name);

        fs::create_dir_all(&path).expect("Could not create directory {path}");
        path
    }

    fn create_recursively_at<F>(&self, out_dir: &PathBuf, f: &F) -> PathBuf
    where
        F: Fn(&String) -> Option<String>,
    {
        let path = self.create_at(out_dir);

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

trait StringExt {
    fn conform(&self) -> Self;
    fn replace_vars(self, vars: &HashMap<String, String>) -> Self;
}

impl StringExt for String {
    fn conform(&self) -> String {
        self.replace(char::is_whitespace, "_").to_lowercase()
    }

    fn replace_vars(self, vars: &HashMap<String, String>) -> String {
        let mut string = self.clone();
        for (key, value) in vars.iter() {
            string = string.replace(key, value);
        }

        string
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let out_dir = Some(PathBuf::from(std::env::current_dir().unwrap().clone()));
        let proj_gen = Project::new("hao".to_string(), "mylib".to_string(), out_dir);
        let proj_struct = proj_gen.parse_json_proj_struct();
        println!("{proj_struct:#?}");

        proj_gen.gen();
    }
}
