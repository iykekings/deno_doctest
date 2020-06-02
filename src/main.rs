#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_json;

mod util;

fn main() {
    let cwd = std::path::PathBuf::from("js_test");
    let res = util::prepare_doctest(cwd);
    let file = util::render_doctest_to_file(res, true, true, None);
    // write to file for manual test
    std::fs::write(".deno_doctest.ts", &file).expect("Couldn't write file");
}
