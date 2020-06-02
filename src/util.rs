use regex::Regex;

pub struct DocTest {
  // This removes repetition of imports in a file
  imports: std::collections::HashSet<String>,
  // This contains codes in an @example section with their imports removed
  body: Vec<String>,
}

pub fn extract_jsdoc_examples<T: Into<String>>(input: T) -> Option<DocTest> {
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
            _ => Some(res),
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
