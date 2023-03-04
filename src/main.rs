mod cpp_proj_gen;
use clap::Parser;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let opt = cpp_proj_gen::Args::parse();

    let progress = |text: String| println!("Created: {text}");

    cpp_proj_gen::CppProjGen::new(opt)
        .add_include_dir(PathBuf::from("include"))
        .add_source_dir(PathBuf::from("source"))
        .add_toplevel_dir(PathBuf::from("test"))
        .gen(Some(progress))?; // or 'None' for no callback!

    Ok(())
}
