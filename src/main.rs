// #[macro_use]
// extern crate lazy_static;

// #[macro_use]
// extern crate serde_json;

mod cli;
use std::path::PathBuf;
use structopt::StructOpt;
/// Namespace for deno doc command
/// Doesn't do anything in this case without --test
/// Ikechukwu Eze <iykekings36@gmail.com>
#[derive(StructOpt, Debug)]
#[structopt(name = "denodoc")]
pub struct Opt {
    /// Runs test on JSDoc documentation Tests are run on
    /// code snippets in @example block
    #[structopt(short, long)]
    test: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    // let cwd = std::path::PathBuf::from("js_test");
    // let res = util::prepare_doctest(cwd);
    // let file = util::render_doctest_to_file(res, true, true, None);
    // // write to file for manual test
    // std::fs::write(".deno_doctest.ts", &file).expect("Couldn't write file");
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
