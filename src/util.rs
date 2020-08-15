use regex::Regex;
use walkdir::WalkDir;

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub struct DocTest {
    // This removes repetition of imports in a file
    imports: std::collections::HashSet<String>,
    // This contains codes in an @example section with their imports stripped
    bodies: Vec<DocTestBody>,
}

struct DocTestBody {
    caption: String,
    line_number: usize,
    path: PathBuf,
    value: String,
    ignore: bool,
    is_async: bool,
}

fn extract_jsdoc_examples(input: String, p: PathBuf) -> Option<DocTest> {
    lazy_static! {
      static ref JS_DOC_PATTERN: Regex =
        Regex::new(r"/\*\*\s*\n([^\*]|\*[^/])*\*/").unwrap();
      // IMPORT_PATTERN doesn't match dynamic imports
      static ref IMPORT_PATTERN: Regex =
        Regex::new(r"import[^(].*\n").unwrap();
      static ref EXAMPLE_PATTERN: Regex = Regex::new(r"@example\s*(?:<\w+>.*</\w+>)*\n(?:\s*\*\s*\n*)*```").unwrap();
      static ref TICKS_OR_IMPORT: Regex = Regex::new(r"(?:import[^(].*)|(?:```\w*)").unwrap();
      static ref CAPTION_PATTERN: Regex = Regex::new(r"<caption>([\s\w\W]+)</caption>").unwrap();
      static ref TEST_TAG_PATTERN: Regex = Regex::new(r"@example\s*(?:<\w+>.*</\w+>)*\n(?:\s*\*\s*\n*)*```(\w+)").unwrap();
      static ref AWAIT_PATTERN: Regex = Regex::new(r"\Wawait\s").unwrap();
    }

    let mut import_set = std::collections::HashSet::new();

    let test_bodies = JS_DOC_PATTERN
        .captures_iter(&input)
        .filter_map(|caps| caps.get(0).map(|c| (c.start(), c.as_str())))
        .flat_map(|(offset, section)| {
            EXAMPLE_PATTERN.find_iter(section).filter_map(move |cap| {
                section[cap.end()..].find("```").map(|i| {
                    (
                        offset + cap.end(),
                        section[cap.start()..i + cap.end()].to_string(),
                    )
                })
            })
        })
        .filter_map(|(offset, example_section)| {
            let test_tag = TEST_TAG_PATTERN
                .captures(&example_section)
                .and_then(|m| m.get(1).map(|c| c.as_str()));

            if test_tag == Some("text") {
                return None;
            }

            IMPORT_PATTERN
                .captures_iter(&example_section)
                .filter_map(|caps| caps.get(0).map(|m| m.as_str()))
                .for_each(|import| {
                    import_set.insert(import.to_string());
                });

            let caption = CAPTION_PATTERN
                .captures(&example_section)
                .and_then(|cap| cap.get(1).map(|m| m.as_str()))
                .unwrap_or("");

            let line_number = &input[0..offset].lines().count();

            let body = TICKS_OR_IMPORT
                .replace_all(&example_section, "\n")
                .lines()
                .skip(1)
                .filter_map(|line| {
                    let res = match line.trim_start().starts_with("*") {
                        true => line.replacen("*", "", 1).trim_start().to_string(),
                        false => line.trim_start().to_string(),
                    };
                    match res.len() {
                        0 => None,
                        _ => Some(format!("  {}", res)),
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            let is_async = match AWAIT_PATTERN.find(&example_section) {
                Some(_) => true,
                _ => false,
            };
            Some(DocTestBody {
                caption: caption.to_owned(),
                line_number: line_number.clone(),
                path: p.clone(),
                value: body,
                ignore: test_tag == Some("ignore"),
                is_async,
            })
        })
        .collect::<Vec<_>>();

    match test_bodies.len() {
        0 => None,
        _ => Some(DocTest {
            imports: import_set,
            bodies: test_bodies,
        }),
    }
}

// from deno -> cli::fs
pub fn files_in_subtree<F>(root: PathBuf, filter: F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool,
{
    assert!(root.is_dir());

    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_owned())
        .filter(|p| if p.is_dir() { false } else { filter(&p) })
        .collect()
}

// from deno -> cli::test_runner
pub fn is_supported(p: &Path) -> bool {
    use std::path::Component;
    if let Some(Component::Normal(basename_os_str)) = p.components().next_back() {
        let basename = basename_os_str.to_string_lossy();
        basename.ends_with("_test.ts")
            || basename.ends_with("_test.tsx")
            || basename.ends_with("_test.js")
            || basename.ends_with("_test.jsx")
            || basename.ends_with(".test.ts")
            || basename.ends_with(".test.tsx")
            || basename.ends_with(".test.js")
            || basename.ends_with(".test.jsx")
            || basename == "test.ts"
            || basename == "test.tsx"
            || basename == "test.js"
            || basename == "test.jsx"
    } else {
        false
    }
}

pub fn prepare_doctest(paths: Vec<PathBuf>) -> Vec<DocTest> {
    let mut prepared = vec![];

    for p in paths {
        if p.is_dir() {
            let test_files = files_in_subtree(p, |p| {
                let valid_ext = ["ts", "tsx", "js", "jsx"];
                p.extension()
                    .and_then(OsStr::to_str)
                    .map(|ext| valid_ext.contains(&ext) && !is_supported(p))
                    .unwrap_or(false)
            });
            prepared.extend(test_files);
        } else {
            prepared.push(p);
        }
    }

    prepared
        .iter()
        .filter_map(|dir| {
            let content = std::fs::read_to_string(&dir).expect("Error reading test files");
            extract_jsdoc_examples(content, dir.to_owned())
        })
        .collect::<Vec<_>>()
}

pub fn render_doctest_to_file(
    doctests: Vec<DocTest>,
    // fail_fast: bool,
    // quiet: bool,
    // filter: Option<String>,
) -> String {
    let mut test_file = "".to_string();

    let default_import = "import { 
        equal,
        assert,
        assertEquals,
        assertNotEquals,
        assertStrictEquals,
        assertStringContains,
        assertArrayContains,
        assertMatch,
        fail,
        assertThrows,
        unimplemented,
        unreachable
     } from \"https://deno.land/std@0.65.0/testing/asserts.ts\";\n";

    test_file.push_str(default_import);

    let all_imports: String = doctests
        .iter()
        .map(|doctest| doctest.imports.clone())
        .flatten()
        .collect();

    test_file.push_str(&all_imports);
    test_file.push_str("\n");

    let all_test_section = doctests
        .into_iter()
        .map(|doctest| doctest.bodies.into_iter())
        .flatten()
        .map(|test| {
            let async_str = if test.is_async {"async "} else {""};
            format!(
                "Deno.test({{\n\tname: \"{} -> {} (line {})\",\n\tignore: {},\n\t{}fn() {{\n{}\n}}\n}});\n",
                test.path.display(),
                test.caption,
                test.line_number,
                test.ignore,
                async_str,
                test.value
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    test_file.push_str(&all_test_section);

    // let options = if let Some(filter) = filter {
    //     json!({ "failFast": fail_fast, "reportToConsole": !quiet, "disableLog": quiet, "filter": filter })
    // } else {
    //     json!({ "failFast": fail_fast, "reportToConsole": !quiet, "disableLog": quiet })
    // };

    // let run_tests_cmd = format!(
    //     "\n// @ts-ignore\nDeno[Deno.internal].runTests({});\n",
    //     options
    // );

    // test_file.push_str(&run_tests_cmd);

    test_file
}
