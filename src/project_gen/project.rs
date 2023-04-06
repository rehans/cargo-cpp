use include_dir::{include_dir, Dir};
use log::info;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use tera::{Context, Tera};

// Compile all files in /templates folder into the binary
static PROJECT_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
static PROJECT_STRUCTURE_TEMPLATE: &str = "project_structure.json";

enum PathType {
    File {
        path: PathBuf,
        opt_template: Option<String>,
    },
    Folder {
        path: PathBuf,
    },
}

#[derive(Debug, Clone)]
pub struct Project {
    out_dir: Option<PathBuf>,
    domain_name: String,
    target_name: String,
}

impl Project {
    pub fn new(domain_name: String, target_name: String, out_dir: Option<PathBuf>) -> Self {
        Self {
            out_dir,
            domain_name: domain_name.conform(),
            target_name: target_name.conform(),
        }
    }

    pub fn gen(&self) {
        let out_dir = self
            .out_dir
            .clone()
            .unwrap_or(PathBuf::from(std::env::current_dir().unwrap().clone()));

        let tera_context = self.new_tera_context();

        let folder = self.parse_json_proj_struct();

        folder.create_recursively_at(&out_dir, &|path_type| match path_type {
            PathType::File {
                path,
                opt_template: opt_template_file,
            } => match opt_template_file {
                Some(template_file) => {
                    let template_content = PROJECT_TEMPLATES
                        .get_file(template_file)
                        .unwrap()
                        .contents_utf8()
                        .unwrap();

                    let rendered = Tera::one_off(template_content, &tera_context, true)
                        .expect("Cannot render file {template_file}");

                    fs::write(&path, rendered).expect("Could not write file {path}!");
                    info!("Created: {path:#?}");
                }
                None => (),
            },
            PathType::Folder { path } => {
                fs::create_dir_all(&path).expect("Could not create directory {path}");
                info!("Created: {path:#?}");
            }
        });
    }

    fn new_tera_context(&self) -> Context {
        let domain_name = self.domain_name.clone();
        let target_name = self.target_name.clone();
        let project_name = format!("{}-{}", domain_name.clone(), target_name.clone());

        let mut config = HashMap::new();
        config.insert("target_name", target_name);
        config.insert("domain_name", domain_name);
        config.insert("project_name", project_name);
        config.insert("cmake_minimum_version", "3.19.0".to_string());

        let mut context = Context::new();
        context.insert("with_test_app", &true);
        context.insert("config", &config);
        context
    }

    fn parse_json_proj_struct(&self) -> ProjectFolder {
        let opt_file = PROJECT_TEMPLATES.get_file(PROJECT_STRUCTURE_TEMPLATE);
        match opt_file {
            Some(file) => {
                let opt_path_str = file.path().to_str();
                if let Some(path_str) = opt_path_str {
                    let template_content = PROJECT_TEMPLATES
                        .get_file(path_str)
                        .unwrap()
                        .contents_utf8()
                        .unwrap();

                    let rendered = Tera::one_off(template_content, &self.new_tera_context(), true)
                        .expect("Cannot render file {path_str}");

                    return serde_json::from_str(&rendered).unwrap();
                } else {
                    todo!()
                }
            }
            None => todo!(),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectFile {
    name: String,
    template: Option<String>,
}

impl ProjectFile {
    fn create_at<F>(&self, out_dir: &PathBuf, fn_create: &F) -> PathBuf
    where
        F: Fn(&PathType),
    {
        let mut path = out_dir.clone();
        path.push(&self.name);

        let path_type = PathType::File {
            path: path.clone(),
            opt_template: self.template.clone(),
        };
        fn_create(&path_type);

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
    fn create_at<F>(&self, out_dir: &PathBuf, fn_create: &F) -> PathBuf
    where
        F: Fn(&PathType),
    {
        let mut path = out_dir.clone();
        path.push(&self.name);
        fn_create(&PathType::Folder { path: path.clone() });
        fs::create_dir_all(&path).expect("Could not create directory {path}");
        path
    }

    fn create_recursively_at<F>(&self, out_dir: &PathBuf, f: &F) -> PathBuf
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

trait StringExt {
    fn conform(&self) -> Self;
}

impl StringExt for String {
    fn conform(&self) -> String {
        self.replace(char::is_whitespace, "_").to_lowercase()
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

    #[test]
    fn test_jinja2() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let mut context = Context::new();
        context.insert("target_name", &"mylib");
        context.insert("domain_name", &"hao");
        context.insert("project_name", &"hao-mylib");
        context.insert("with_test_app", &true);
        context.insert("cmake_minimum_version", &"3.19.0");

        let mut out_dir = PathBuf::from(std::env::current_dir().unwrap().clone());

        match tera.render("CMakeLists.txt.in", &context) {
            Ok(s) => {
                out_dir.push("CMakeLists.txt");
                fs::write(out_dir, s).expect("Could not write file {path}!");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }

    #[test]
    fn test3() {
        let mut config = HashMap::new();
        config.insert("target_name", "mylib");
        config.insert("domain_name", "hao");
        config.insert("project_name", "hao-mylib");
        config.insert("cmake_minimum_version", "3.19.0");

        let mut ctx = Context::new();
        ctx.insert("config", &config);
        ctx.insert("with_test_app", &true);

        let path = "project_structure.json";
        let template = PROJECT_TEMPLATES
            .get_file(path)
            .unwrap()
            .contents_utf8()
            .unwrap();

        let rendered = Tera::one_off(template, &ctx, true).expect("Cannot load file");

        println!("{}", rendered);
    }
}
