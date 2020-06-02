use regex::Regex;
use walkdir::WalkDir;

use std::path::{Path, PathBuf};

pub struct DocTest {
    // This removes repetition of imports in a file
    imports: std::collections::HashSet<String>,
    // This contains codes in an @example section with their imports removed
    body: Vec<String>,
    // might need this
    path: PathBuf,
}

pub fn extract_jsdoc_examples<T: Into<String>>(input: T, path: PathBuf) -> Option<DocTest> {
    lazy_static! {
      static ref JS_DOC_PATTERN: Regex =
        Regex::new(r"/\*\*\s*\n([^\*]|\*[^/])*\*/").unwrap();
      // IMPORT_PATTERN doesn't match dynamic imports
      static ref IMPORT_PATTERN: Regex =
        Regex::new(r"import[^(].*\n").unwrap();
      static ref EXAMPLE_PATTERN: Regex = Regex::new(r"@example\s*\n(?:\s*\*\s*\n*)*```\w*\n([^\*]|\*\s*[^/])*```\n").unwrap();
      static ref TICKS_OR_IMPORT: Regex = Regex::new(r"(?:import[^(].*)|(?:```\w*)").unwrap();
    }

    let mut docs = DocTest {
        imports: std::collections::HashSet::new(),
        body: vec![],
        path,
    };
    JS_DOC_PATTERN
        .captures_iter(&input.into())
        .filter_map(|caps| caps.get(0).map(|m| m.as_str()))
        .filter_map(|section| {
            EXAMPLE_PATTERN
                .captures(section)
                .and_then(|res| res.get(0).map(|m| m.as_str()))
        })
        .for_each(|example_section| {
            // imports
            IMPORT_PATTERN
                .captures_iter(example_section)
                .filter_map(|caps| caps.get(0).map(|m| m.as_str()))
                .for_each(|import| {
                    docs.imports.insert(import.to_string());
                });

            let body = TICKS_OR_IMPORT
                .replace_all(example_section, "\n")
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
                .collect::<Vec<String>>()
                .join("\n");
            docs.body.push(body);
        });

    if docs.body.len() > 0 {
        Some(docs)
    } else {
        None
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
// TODO(iykekings) provide an adaptaton for doctest -> currently using an inversion of this but not sufficient
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

pub fn prepare_doctest(path: PathBuf) -> Vec<Option<DocTest>> {
    let dirs = files_in_subtree(path, |p| !is_supported(p));

    dirs.iter()
        .map(|dir| {
            let content = std::fs::read_to_string(&dir).expect("Error reading test files");
            extract_jsdoc_examples(content, dir.to_path_buf())
        })
        .collect::<Vec<_>>()
}

pub fn render_doctest_to_file(
    doctests: Vec<Option<DocTest>>,
    fail_fast: bool,
    quiet: bool,
    filter: Option<String>,
) -> String {
    use std::collections::HashSet;

    let mut test_file = "".to_string();

    let all_imports: HashSet<_> = doctests
        .iter()
        .filter_map(|opt_doctest| {
            opt_doctest
                .as_ref()
                .and_then(|opt| Some(opt.imports.clone()))
        })
        .flatten()
        .collect();

    // verify if this line is really line
    test_file.push_str(&all_imports.into_iter().collect::<String>());

    let all_test_section = doctests
        .iter()
        .filter_map(|opt_doctest| {
            opt_doctest
                .as_ref()
                .and_then(|opt| Some(opt.body.clone().into_iter()))
        })
        .flatten()
        .enumerate()
        .map(|(i, test)| format!("Deno.test(\"doctest {}\", () => {{\n{}\n}});\n", i, test))
        .collect::<Vec<_>>()
        .join("\n");

    test_file.push_str(&all_test_section);

    let options = if let Some(filter) = filter {
        json!({ "failFast": fail_fast, "reportToConsole": !quiet, "disableLog": quiet, "filter": filter })
    } else {
        json!({ "failFast": fail_fast, "reportToConsole": !quiet, "disableLog": quiet })
    };

    let run_tests_cmd = format!(
        "// @ts-ignore\nDeno[Deno.internal].runTests({});\n",
        options
    );

    test_file.push_str(&run_tests_cmd);

    test_file
}
