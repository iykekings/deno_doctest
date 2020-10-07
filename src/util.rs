use regex::Regex;
use walkdir::WalkDir;

use jsdoc::{parse, Input};
use std::ffi::OsStr;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub struct DocTest {
    // This removes repetition of imports in a file
    imports: std::collections::HashSet<String>,
    // This contains codes in an @example section with their imports stripped
    bodies: Vec<DocTestBody>,
}

struct DocTestBody {
    caption: String,
    line_number: usize,
    path: String,
    value: String,
    ignore: bool,
    is_async: bool,
}
lazy_static! {
  static ref JS_DOC_PATTERN: Regex =
    Regex::new(r"/\*\*\s*\n([^\*]|\*[^/])*\*/").unwrap();
  // IMPORT_PATTERN doesn't match dynamic imports by design
  static ref IMPORT_PATTERN: Regex =
    Regex::new(r"import[^(].*\n").unwrap();
  static ref EXAMPLE_PATTERN: Regex = Regex::new(r"@example\s*(?:<\w+>.*</\w+>)*\n(?:\s*\*\s*\n*)*```").unwrap();
  static ref TEST_TAG_PATTERN: Regex = Regex::new(r"@example\s*(?:<\w+>.*</\w+>)*\n(?:\s*\*\s*\n*)*```(\w+)").unwrap();
  static ref AWAIT_PATTERN: Regex = Regex::new(r"\Wawait\s").unwrap();
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

pub fn extract_jsdoc_examples(input: String, p: PathBuf) -> Option<DocTest> {
    let mut import_set = HashSet::new();
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

            let caption = get_caption_from_example(&example_section);
            let line_number = &input[0..offset].lines().count();
            let code_block = get_code_from_example(&example_section);
            let is_async = AWAIT_PATTERN.find(&example_section).is_some();

            let cwd =
                std::env::current_dir().expect("expected: process has a current working directory");
            let path = p
                .to_str()
                .map(|x| x.replace(cwd.to_str().unwrap_or(""), ""));
            Some(DocTestBody {
                caption,
                line_number: *line_number,
                path: path.unwrap_or_else(|| "".to_string()),
                value: code_block,
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

pub fn render_doctest_to_file(doctests: Vec<DocTest>) -> String {
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
                "Deno.test({{\n\tname: \"{} - {} (line {})\",\n\tignore: {},\n\t{}fn() {{\n{}\n}}\n}});\n",
                test.path,
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

    // Should be uncommented when in Deno
    // let options = if let Some(filter) = filter {
    //     json!({ "failFast": fail_fast, "reportToConsole": !quiet, "disableLog": quiet, "isDoctest": true, "filter": filter })
    // } else {
    //     json!({ "failFast": fail_fast, "reportToConsole": !quiet, "disableLog": quiet, "isDoctest": true })
    // };
    // let run_tests_cmd = format!(
    //     "\n// @ts-ignore\nDeno[Deno.internal].runTests({});\n",
    //     options
    // );
    // test_file.push_str(&run_tests_cmd);
    test_file
}

fn get_caption_from_example(ex: &str) -> String {
    lazy_static! {
        static ref CAPTION_PATTERN: Regex = Regex::new(r"<caption>([\s\w\W]+)</caption>").unwrap();
    }
    CAPTION_PATTERN
        .captures(ex)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .unwrap_or("")
        .to_string()
}

fn get_code_from_example(ex: &str) -> String {
    lazy_static! {
        static ref TICKS_OR_IMPORT_PATTERN: Regex =
            Regex::new(r"(?:import[^(].*)|(?:```\w*)").unwrap();
    }
    TICKS_OR_IMPORT_PATTERN
        .replace_all(ex, "\n")
        .lines()
        .skip(1)
        .filter_map(|line| {
            let res = if line.trim_start().starts_with('*') {
                line.replacen("*", "", 1).trim_start().to_string()
            } else {
                line.trim_start().to_string()
            };
            match res.len() {
                0 => None,
                _ => Some(format!("  {}", res)),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// Using jsdoc 0.1.0
pub fn new_parse() {
    use swc_common::BytePos;

    let file = std::fs::read_to_string("./js_test/linkedlist.ts").expect("couldn't read file");
    let _test = r#"/**
    * 
    * @param list - LinkedList<T>
    * @example 
    * <caption>Linkedlists.compareWith</caption>
    * import { LinkedList } from './js_test/linkedlist.ts'
    * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
    * const firstList = new LinkedList<number>();
    * const secondList = new LinkedList<number>();
    * for (let data of testArr) {
    *   firstList.insertNode(data);
    *   secondList.insertNode(data);
    * }
    * const result = firstList.compareWith(secondList);
    * assert(result);
    * @returns boolean
    */
     compareWith(list: LinkedList<T>): boolean {
       let current1 = this.head;
       let current2 = list.head;
       while (current1 && current2) {
         if (current1.data !== current2.data) return false;
         if (current1.next && !current2.next && !current1.next && current2.next) {
           return false;
         }
         current1 = current1.next;
         current2 = current2.next;
       }
       return true;
     }"#;

    let input = Input::new(BytePos(0), BytePos((file.len() - 1) as u32), &file);
    let parsed = jsdoc::parse(input).expect("Failed");
    eprintln!("{:#?}", parsed.1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_jsdoc() {
        let test = r#"/**
    * 
    * @param list - LinkedList<T>
    * @example <caption>Linkedlists.compareWith</caption>
    * ```ts
    * import { LinkedList } from './js_test/linkedlist.ts'
    * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
    * const firstList = new LinkedList<number>();
    * const secondList = new LinkedList<number>();
    * for (let data of testArr) {
    *   firstList.insertNode(data);
    *   secondList.insertNode(data);
    * }
    * const result = firstList.compareWith(secondList);
    * assert(result);
    * ```
    * @returns boolean
    */
     compareWith(list: LinkedList<T>): boolean {
       let current1 = this.head;
       let current2 = list.head;
       while (current1 && current2) {
         if (current1.data !== current2.data) return false;
         if (current1.next && !current2.next && !current1.next && current2.next) {
           return false;
         }
         current1 = current1.next;
         current2 = current2.next;
       }
       return true;
     }"#;
        let res = extract_jsdoc_examples(test.to_string(), PathBuf::from("user"));
        assert!(res.is_some());

        let doctest = res.unwrap();
        assert_eq!(1, doctest.imports.len());
        assert_eq!(doctest.bodies.len(), 1);
        let body = &doctest.bodies[0];
        assert!(!body.is_async);
        assert!(!body.ignore);
        assert_eq!(body.caption, "Linkedlists.compareWith".to_string());
        assert_eq!(body.line_number, 5);
        assert_eq!(
            body.value,
            vec![
                "  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];",
                "  const firstList = new LinkedList<number>();",
                "  const secondList = new LinkedList<number>();",
                "  for (let data of testArr) {",
                "  firstList.insertNode(data);",
                "  secondList.insertNode(data);",
                "  }",
                "  const result = firstList.compareWith(secondList);",
                "  assert(result);"
            ]
            .join("\n")
        )
    }

    #[test]
    fn test_multiple_examples() {
        let test = r#"  /**
    * 
    * @param fn - (data: T, index: number) => T
    * @example <caption>Linkedlist.map</caption>
    * ```ts
    * import { LinkedList } from './js_test/linkedlist.ts'
    * const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];
    * const testList = new LinkedList<number>();
    * for (let data of testArr) {
    *  testList.insertNode(data);
    * }
    * testList.map((c: number) => c ** 2);
    * testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
    * ```
    * 
    * @example <caption>Linkedlist.map 2</caption>
    * ```ignore
    * import { LinkedList } from './js_test/linkedlist.ts'
    * const testArr = [1, 2, 3, 4, 5];
    * const testList = new LinkedList<number>();
    * for (let data of testArr) {
    *  testList.insertNode(data);
    * }
    * testList.map((c: number) => c ** 2);
    * testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));
    * ```
    */"#;
        let res = extract_jsdoc_examples(test.to_string(), PathBuf::from("user"));
        assert!(res.is_some());

        let doctest = res.unwrap();
        // imports are deduped
        assert_eq!(1, doctest.imports.len());
        assert_eq!(doctest.bodies.len(), 2);
        let body1 = &doctest.bodies[0];
        let body2 = &doctest.bodies[1];
        assert!(!body1.is_async);
        assert!(!body2.is_async);
        assert!(!body1.ignore);
        assert!(body2.ignore);
        assert_eq!(body1.caption, "Linkedlist.map".to_string());
        assert_eq!(body2.caption, "Linkedlist.map 2".to_string());
        assert_eq!(body1.line_number, 5);
        assert_eq!(body2.line_number, 17);
        assert_eq!(
            body2.value,
            vec![
                "  const testArr = [1, 2, 3, 4, 5];",
                "  const testList = new LinkedList<number>();",
                "  for (let data of testArr) {",
                "  testList.insertNode(data);",
                "  }",
                "  testList.map((c: number) => c ** 2);",
                "  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));"
            ]
            .join("\n")
        );
        assert_eq!(
            body1.value,
            vec![
                "  const testArr = [1, 2, 3, 4, 5, 6, 78, 9, 0, 65];",
                "  const testList = new LinkedList<number>();",
                "  for (let data of testArr) {",
                "  testList.insertNode(data);",
                "  }",
                "  testList.map((c: number) => c ** 2);",
                "  testList.forEach((c: number, i: number) => assertEquals(c, testArr[i] ** 2));",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_code_without_jsdoc() {
        let test = r#"class Node<T> {
      constructor(public data: T, public next?: Node<T>) {}
    
      swap(other: Node<T>) {
        let temp = this.data;
        this.data = other.data;
        other.data = temp;
      }
    }"#;
        let res = extract_jsdoc_examples(test.to_string(), PathBuf::from("user"));
        assert!(res.is_none());
    }

    #[test]
    fn test_async_detection() {
        let test = r#"  /**
    * @example
    * ```ts
    * const response = await fetch("https://deno.land");
    * const body = await response.text();
    * assert(body.length > 0);
    * ```
    */"#;

        let res = extract_jsdoc_examples(test.to_string(), PathBuf::from("user"));
        assert!(res.is_some());
        let doctest = res.unwrap();
        let body = &doctest.bodies[0];
        assert!(body.is_async);
    }

    #[test]
    fn test_text_tag() {
        let test = r#"  /**
    * @example
    * ```text
    * const response = await fetch("https://deno.land");
    * const body = await response.text();
    * assert(body.length > 0);
    * ```
    */"#;

        let res = extract_jsdoc_examples(test.to_string(), PathBuf::from("user"));
        assert!(res.is_none());
    }

    #[test]
    fn test_jump_example_without_backticks() {
        let test = r#"  /**
    * @example
    * const response = await fetch("https://deno.land");
    * const body = await response.text();
    * assert(body.length > 0);
    */"#;

        let res = extract_jsdoc_examples(test.to_string(), PathBuf::from("user"));
        assert!(res.is_none());
    }
}
