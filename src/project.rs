// Copyright(c) 2023 rehans.

mod file;
mod folder;

use chrono::Datelike;
use include_dir::{include_dir, Dir};
use log::info;
use std::path::PathBuf;
use std::{collections::HashMap, fs};
use tera::{Context, Tera};

pub enum PathType {
    File {
        path: PathBuf,
        opt_template_file: Option<String>,
    },
    Folder {
        path: PathBuf,
    },
}

// Compile all files in /templates folder into the binary
static PROJECT_TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");
static PROJECT_STRUCTURE_TEMPLATE: &str = "project_structure.json";

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
                opt_template_file,
            } => {
                if let Some(template_file) = opt_template_file {
                    let content = Self::template_file_content(template_file);
                    let rendered = Tera::one_off(content, &tera_context, true)
                        .expect("Cannot render file {template_file}");

                    fs::write(&path, rendered).expect("Could not write file {path}!");
                    info!("Created: {path:#?}");
                }
            }
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
        config.insert("year", chrono::Utc::now().year().to_string());

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

    fn parse_json_proj_struct(&self) -> folder::Folder {
        let opt_file = PROJECT_TEMPLATES.get_file(PROJECT_STRUCTURE_TEMPLATE);
        match opt_file {
            Some(file) => {
                let opt_path_str = file.path().to_str();
                if let Some(path_str) = opt_path_str {
                    let template_content = Self::template_file_content(&path_str.to_string());
                    let rendered =
                        Tera::one_off(template_content, &self.new_tera_context(), true).unwrap();

                    let p: folder::Folder = serde_json::from_str(&rendered).unwrap();
                    p
                } else {
                    todo!()
                }
            }
            None => todo!(),
        }
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
            .expect(&format!("Cannot find file {path}!"))
            .contents_utf8()
            .expect(&format!("Contents of {path} cannot be read!"));

        let rendered = Tera::one_off(template, &ctx, true).expect("Cannot load file");

        println!("{}", rendered);
    }
}
