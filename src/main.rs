#[macro_use]
extern crate lazy_static;

mod doc_tester;
mod swc_util;
mod util;

use std::path::PathBuf;
// use std::process::Command;

use structopt::StructOpt;
/// Namespace for deno doc command
/// Doesn't do anything in this case without --test
/// Ikechukwu Eze <iykekings36@gmail.com>
#[derive(StructOpt, Debug)]
#[structopt(name = "deno_doctest")]
pub struct Opt {
    /// Runs test on JSDoc documentation Tests are run on
    /// code snippets in @example block
    #[structopt(short, long)]
    test: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    // let opt = Opt::from_args();
    // let res = util::prepare_doctest(opt.files);
    // let file = util::render_doctest_to_file(res);
    // std::fs::write(".deno_doctest.ts", &file).expect("Couldn't write file");
    // Command::new("deno")
    //     .arg("test")
    //     .arg(".deno_doctest.ts")
    //     .status()?;
    // std::fs::remove_file(".deno_doctest.ts")?;
    util::new_parse();
    Ok(())
}
