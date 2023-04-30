// Copyright(c) 2023 rehans.

use chrono::Datelike;
use include_dir::{include_dir, Dir};
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{collections::HashMap, fs};
use tera::{Context, Tera};

use crate::RootDir;

pub enum PathKind {
    File {
        path: PathBuf,
        template_file: Option<String>,
    },
    Folder {
        path: PathBuf,
    },
}

// Compile all files in /templates folder into the binary
static PROJECT_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
static PROJECT_STRUCTURE_TEMPLATE: &str = "project_structure.json";

#[derive(Debug, Clone)]
pub struct NewOptions {
    out_dir: Option<PathBuf>,
    domain_name: String,
    target_name: String,
}

impl NewOptions {
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
            PathKind::File {
                path,
                template_file,
            } => {
                if let Some(template_file) = template_file {
                    let content = Self::template_file_content(template_file);
                    let rendered = Tera::one_off(content, &tera_context, true)
                        .expect("Cannot render file {template_file}");

                    if !path.exists() {
                        fs::write(&path, rendered).expect("Could not write file {path}!");
                        info!("Created: {path:#?}");
                    } else {
                        info!("Exists: {path:#?}");
                    }
                }
            }
            PathKind::Folder { path } => {
                if !path.exists() {
                    fs::create_dir_all(&path).expect("Could not create directory {path}");
                    info!("Created: {path:#?}");
                } else {
                    info!("Exists: {path:#?}");
                }
            }
        });
    }

    /**
     * Returns the new tera context of this [`Project`].
     */
    fn new_tera_context(&self) -> Context {
        let domain_name = self.domain_name.clone();
        let target_name = self.target_name.clone();
        let project_name = format!("{}-{}", domain_name.clone(), target_name.clone());

        let mut config = HashMap::new();
        config.insert("target_name", target_name);
        config.insert("domain_name", domain_name);
        config.insert("project_name", project_name);
        config.insert("cmake_minimum_version", "3.19.0".to_string());
        config.insert("year", chrono::Utc::now().year().to_string());
        config.insert(
            "external_dir_name",
            RootDir::External.to_string().to_string(),
        );
        config.insert("include_dir_name", RootDir::Include.to_string().to_string());
        config.insert("source_dir_name", RootDir::Source.to_string().to_string());
        config.insert("test_dir_name", RootDir::Test.to_string().to_string());

        let mut context = Context::new();
        context.insert("with_test_app", &true);
        context.insert("config", &config);
        context
    }

    fn template_file_content(template_file: &String) -> &'static str {
        PROJECT_TEMPLATES
            .get_file(template_file)
            .expect(&format!("Cannot find template for {template_file}"))
            .contents_utf8()
            .expect(&format!("Cannot read utf8 string from {template_file}"))
    }

    fn parse_json_proj_struct(&self) -> Folder {
        let file = PROJECT_TEMPLATES.get_file(PROJECT_STRUCTURE_TEMPLATE);
        match file {
            Some(file) => {
                let path_str = file.path().to_str();
                if let Some(path_str) = path_str {
                    let template_content = Self::template_file_content(&path_str.to_string());
                    let rendered =
                        Tera::one_off(template_content, &self.new_tera_context(), true).unwrap();

                    let p: Folder = serde_json::from_str(&rendered).unwrap();
                    p
                } else {
                    todo!()
                }
            }
            None => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Folder {
    name: String,
    folders: Option<Vec<Folder>>,
    files: Option<Vec<File>>,
}

impl Folder {
    fn create_at<F>(&self, out_dir: &PathBuf, fn_create: &F) -> PathBuf
    where
        F: Fn(&PathKind),
    {
        let path = out_dir.join(&self.name);
        fn_create(&PathKind::Folder { path: path.clone() });

        path
    }

    pub fn create_recursively_at<F>(&self, out_dir: &PathBuf, f: &F) -> PathBuf
    where
        F: Fn(&PathKind),
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
            files.iter().for_each(|file| {
                file.create_at(&path, f);
            });
        };

        path
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    name: String,
    template: Option<String>,
}

impl File {
    pub fn create_at<F>(&self, out_dir: &PathBuf, fn_create: &F) -> PathBuf
    where
        F: Fn(&PathKind),
    {
        let path = out_dir.join(&self.name);
        let path_type = PathKind::File {
            path: path.clone(),
            template_file: self.template.clone(),
        };
        fn_create(&path_type);

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
        let new_options = NewOptions::new("hao".to_string(), "mylib".to_string(), out_dir);
        let proj_tree = new_options.parse_json_proj_struct();
        println!("{proj_tree:#?}");

        new_options.gen();
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
            .expect(&format!("Cannot find file {path}!"))
            .contents_utf8()
            .expect(&format!("Contents of {path} cannot be read!"));

        let rendered = Tera::one_off(template, &ctx, true).expect("Cannot load file");

        println!("{}", rendered);
    }
}
