use super::generate::DirPath;
use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub enum DirName {
    Include { name: String },
    Source { name: String },
    Test { name: String },
    External { name: String },
}

#[derive(Debug, Clone)]
pub struct DirectoryGen {
    out_dir: PathBuf,
    target_name: String,
    domain_name: String,
    toplevel_dir_names: Vec<DirName>,
}

impl DirectoryGen {
    pub fn new() -> Self {
        Self {
            out_dir: PathBuf::new(),
            target_name: String::new(),
            domain_name: String::new(),
            toplevel_dir_names: Vec::new(),
        }
    }

    pub fn set_out_dir(mut self, out_dir: &PathBuf) -> Self {
        self.out_dir = out_dir.clone();
        self
    }

    pub fn add_toplevel_dir(mut self, dir: &DirName) -> Self {
        self.toplevel_dir_names.push(dir.clone());
        self
    }

    pub fn toplevel_dir_names(&self) -> Vec<DirName> {
        self.toplevel_dir_names.clone()
    }

    pub fn set_target_name(mut self, name: &String) -> Self {
        self.target_name = name.clone();
        self
    }

    pub fn set_domain_name(mut self, name: &String) -> Self {
        self.domain_name = name.clone();
        self
    }

    pub fn create_dirs_dry(&self) -> Vec<DirPath> {
        let mut out_dir = self.out_dir.clone();
        out_dir.push(self.target_name.clone());

        let mut out_paths = Vec::<DirPath>::new();
        out_paths.push(DirPath::Root {
            path: out_dir.clone(),
        });

        for el in self.toplevel_dir_names.iter() {
            match el {
                DirName::Include { name } => {
                    let mut header_include_path = out_dir.clone();
                    header_include_path.push(name.clone());
                    header_include_path.push(self.domain_name.clone());
                    header_include_path.push(self.target_name.clone());
                    out_paths.push(DirPath::HeaderInclude {
                        path: header_include_path,
                    });
                }
                DirName::Source { name } => {
                    let mut source_path = out_dir.clone();
                    source_path.push(name.clone());
                    out_paths.push(DirPath::Source { path: source_path });
                }
                DirName::Test { name } => {
                    let mut test_path = out_dir.clone();
                    test_path.push(name.clone());
                    out_paths.push(DirPath::Test { path: test_path });
                }
                DirName::External { name } => {
                    let mut external_path = out_dir.clone();
                    external_path.push(name.clone());
                    out_paths.push(DirPath::External {
                        path: external_path,
                    });
                }
            }
        }
        out_paths
    }

    pub fn create_dirs(&self) -> Vec<DirPath> {
        let dirs = self.create_dirs_dry();

        for dir in dirs.iter() {
            let path = match dir {
                DirPath::Root { path } => path,
                DirPath::HeaderInclude { path } => path,
                DirPath::Source { path } => path,
                DirPath::Test { path } => path,
                DirPath::External { path } => path,
            };

            fs::create_dir_all(path).expect("Could not create directory {path}");
        }

        dirs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test2() {
        let out_dir = PathBuf::from(std::env::current_dir().unwrap().clone());

        let out_paths = DirectoryGen::new()
            .set_out_dir(&out_dir)
            .add_toplevel_dir(&DirName::Include {
                name: "include".to_string(),
            })
            .add_toplevel_dir(&DirName::Source {
                name: "source".to_string(),
            })
            .add_toplevel_dir(&DirName::Test {
                name: "test".to_string(),
            })
            .add_toplevel_dir(&DirName::External {
                name: "external".to_string(),
            })
            .set_domain_name(&"domain_name".to_string())
            .set_target_name(&"target_name".to_string())
            .create_dirs_dry();

        println!("{out_paths:#?}");
    }
}
