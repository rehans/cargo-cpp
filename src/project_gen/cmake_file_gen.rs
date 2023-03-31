use super::directory_gen::DirName;
use std::{collections::HashMap, fs, path::PathBuf};

const CMAKE_LISTS_FILE_CONTENT: &str = include_str!("res/CMakeLists.txt.in");
const CMAKE_LISTS_FILE_NAME: &str = "CMakeLists";

#[derive(Debug)]
pub enum File {
    Root { path: PathBuf, content: String },
}

#[derive(Debug)]
pub struct FileGen {
    out_dir: PathBuf,
    target_name: String,
    domain_name: String,
    top_level_dir_names: Vec<DirName>,
}

impl FileGen {
    pub fn new() -> Self {
        Self {
            out_dir: PathBuf::new(),
            domain_name: "".to_string(),
            target_name: "".to_string(),
            top_level_dir_names: Vec::new(),
        }
    }

    pub fn set_out_dir(mut self, out_dir: &PathBuf) -> Self {
        self.out_dir = out_dir.clone();
        self
    }
    pub fn set_dir_names(mut self, dirs: &Vec<DirName>) -> Self {
        self.top_level_dir_names = dirs.clone();
        self
    }

    pub fn set_domain_name(mut self, domain_name: &String) -> Self {
        self.domain_name = domain_name.clone();
        self
    }

    pub fn set_target_name(mut self, target_name: &String) -> Self {
        self.target_name = target_name.clone();
        self
    }

    pub fn create_file_dry(&mut self) -> File {
        let mut path = self.out_dir.clone();
        path.push(self.target_name.clone());
        path.push(PathBuf::from(CMAKE_LISTS_FILE_NAME));
        path.set_extension("txt");

        let content = self.replace_all_vars(CMAKE_LISTS_FILE_CONTENT.to_string());

        return File::Root { path, content };
    }

    fn create_var_replacements(&self) -> HashMap<&str, String> {
        // @CMAKE_MINIMUM_VERSION@
        // @PROJECT_NAME@
        // @TARGET_NAME@
        // @HEADER_INCLUDE_DIR@
        // @INCLUDE_DIR@
        // @SOURCE_DIR@

        let mut vars = HashMap::from([
            ("@CMAKE_MINIMUM_VERSION@", "3.19.0".to_string()),
            (
                "@PROJECT_NAME@",
                format!("{}-{}", self.domain_name, self.target_name),
            ),
            ("@TARGET_NAME@", self.target_name.clone()),
        ]);

        for el in self.top_level_dir_names.iter() {
            match el {
                DirName::Include { name } => {
                    vars.insert("@INCLUDE_DIR@", name.clone());

                    let mut header_include_dir = PathBuf::from(name);
                    header_include_dir.push(&self.domain_name);
                    header_include_dir.push(&self.target_name);
                    vars.insert(
                        "@HEADER_INCLUDE_DIR@",
                        header_include_dir.display().to_string(),
                    );
                }
                DirName::Source { name } => {
                    vars.insert("@SOURCE_DIR@", name.clone());
                }
                DirName::Test { name } => {
                    vars.insert("@TEST_DIR@", name.clone());
                }
                DirName::External { name } => {
                    vars.insert("@EXTERNAL_DIR@", name.clone());
                }
            }
        }

        vars
    }

    fn replace_all_vars(&self, content: String) -> String {
        let mut tmp_content = content.clone();

        let vars = self.create_var_replacements();
        for var_val in vars.iter() {
            tmp_content = tmp_content.replace(var_val.0, var_val.1);
        }

        tmp_content
    }

    pub fn create_file(&mut self) -> File {
        let file_path = self.create_file_dry();
        match file_path {
            File::Root { path, content } => {
                fs::write(&path, &content).expect("Could not write file {path}!");
                return File::Root { path, content };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test() {
        let dir_names = vec![
            DirName::Include {
                name: "include".to_string(),
            },
            DirName::Source {
                name: "source".to_string(),
            },
            DirName::External {
                name: "external".to_string(),
            },
            DirName::External {
                name: "test".to_string(),
            },
        ];

        let file_gen = FileGen::new()
            .set_out_dir(&PathBuf::from("/c/"))
            .set_dir_names(&dir_names)
            .set_domain_name(&"my_domain".to_string())
            .set_target_name(&"my_target".to_string())
            .create_file_dry();

        println!("{file_gen:#?}");
    }
}
