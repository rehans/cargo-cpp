use super::generate::DirPath;
use std::{fs, path::PathBuf};

const CPP_HEADER_FILE_CONTENT: &str = include_str!("res/header.h.in");
const CPP_SOURCE_FILE_CONTENT: &str = include_str!("res/source.cpp.in");

#[derive(Debug)]
pub enum File {
    Header { path: PathBuf, content: String },
    Source { path: PathBuf, content: String },
}

#[derive(Debug)]
pub struct FileGen {
    domain_name: String,
    target_name: String,
    dirs: Vec<DirPath>,
}

impl FileGen {
    pub fn new() -> Self {
        FileGen {
            dirs: Vec::new(),
            target_name: String::new(),
            domain_name: String::new(),
        }
    }

    pub fn set_dirs(mut self, dirs: &Vec<DirPath>) -> Self {
        self.dirs = dirs.clone();
        self
    }

    pub fn set_target_name(mut self, name: &String) -> Self {
        self.target_name = name.clone();
        self
    }

    pub fn set_domain_name(mut self, name: &String) -> Self {
        self.domain_name = name.clone();
        self
    }

    pub fn create_files_dry(self) -> Vec<File> {
        //let dirs = &self.dirs;
        let mut files = Vec::new();
        let include_header = self
            .build_header_include(&self.domain_name, &self.target_name)
            .display()
            .to_string();

        for el in self.dirs.iter() {
            match el {
                DirPath::HeaderInclude { path } => {
                    let mut file = path.clone();
                    file.push(&self.target_name);
                    file.set_extension("h");
                    files.push(File::Header {
                        path: file,
                        content: CPP_HEADER_FILE_CONTENT.to_string(),
                    });
                }
                DirPath::Source { path } => {
                    let mut content = CPP_SOURCE_FILE_CONTENT.to_string();

                    content = content.replace("@HEADER_INCLUDE@", &include_header);

                    let mut file = path.clone();
                    file.push(&self.target_name);
                    file.set_extension("cpp");
                    files.push(File::Source {
                        path: file,
                        content,
                    });
                }
                _ => (),
            }
        }

        files
    }

    pub fn create_files(self) -> Vec<File> {
        let files = self.create_files_dry();

        for el in files.iter() {
            let (path, content) = match el {
                File::Header { path, content } => (path, content),
                File::Source { path, content } => (path, content),
            };

            fs::write(path, content).expect("Could not write file {path}!");
        }

        files
    }

    fn build_header_include(&self, domain_name: &String, target_name: &String) -> PathBuf {
        let mut header_include = PathBuf::new();
        header_include.push(&domain_name); // domain
        header_include.push(&target_name); // domain/target
        header_include.push(&target_name); // domain/target/target
        header_include.set_extension("h"); // domain/target/target.h
        header_include
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test() {
        let mut dirs = Vec::new();
        dirs.push(DirPath::HeaderInclude {
            path: PathBuf::from("/c/include/domain/target"),
        });
        dirs.push(DirPath::Source {
            path: PathBuf::from("/c/source"),
        });
        dirs.push(DirPath::Root {
            path: PathBuf::from("/c"),
        });

        let file_gen = FileGen::new()
            .set_dirs(&dirs)
            .set_domain_name(&"hao".to_string())
            .set_target_name(&"hello_world".to_string())
            .create_files_dry();
        println!("{:#?}", file_gen);
    }
}
