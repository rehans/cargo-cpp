mod cpp_proj_gen;
mod file_templates;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    let opt = cpp_proj_gen::Opt::from_args();

    let progress = |text: String| println!("Created: {}", text);

    cpp_proj_gen::CppProjGen::new(opt)
        .add_include_dir(PathBuf::from("include"))
        .add_source_dir(PathBuf::from("source"))
        .add_toplevel_dir(PathBuf::from("test"))
        .gen(Some(progress))?; // or 'None' for no callback!

    Ok(())
}
