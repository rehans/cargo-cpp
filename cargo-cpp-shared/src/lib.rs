// Copyright(c) 2023 rehans.

pub mod cpp_new;
enum RootDir {
    External,
    Include,
    Source,
    Test,
}

impl RootDir {
    fn to_string(&self) -> &'static str {
        match self {
            RootDir::External => "external",
            RootDir::Include => "include",
            RootDir::Source => "source",
            RootDir::Test => "test",
        }
    }
}
