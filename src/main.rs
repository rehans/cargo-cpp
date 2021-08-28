mod cpp_proj_gen;
use std::path::PathBuf;
use structopt::StructOpt;

//-----------------------------------------------------------------------------
fn main() -> std::io::Result<()> {
    let mut opt = cpp_proj_gen::Opt::from_args();
    if opt.output_dir.as_os_str().is_empty() {
        opt.output_dir = std::env::current_dir().unwrap();
        opt.output_dir.push(&opt.project);
    }

    // Print all options form cli in debug print by using ':?'
    println!("{:?}", opt);

    cpp_proj_gen::CppProjGen::new(opt)
        .add_include_dir()
        .add_toplevel_dir(PathBuf::from("source"))
        .add_toplevel_dir(PathBuf::from("test"))
        .add_cmake_lists_file()
        .print()
        .create()?;

    Ok(())
}
