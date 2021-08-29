mod cpp_proj_gen;
use std::path::PathBuf;
use structopt::StructOpt;

//-----------------------------------------------------------------------------
fn main() -> std::io::Result<()> {
    let opt = cpp_proj_gen::Opt::from_args();

    cpp_proj_gen::CppProjGen::new(opt)
        .add_include_dir()
        .add_toplevel_dir(PathBuf::from("source"))
        .add_toplevel_dir(PathBuf::from("test"))
        .add_cmake_lists_file()
        .print()
        .create()?;

    Ok(())
}
