# deno_doctest

proposal on how deno doctest will work

.deno_doctest.ts is sample file generated that will be plugged in to deno runtime
for the tests to be run.

- Currently: It can
- Extract JsDoc comments
- Extract @example section from it
- Extract caption
- Extract the code in the @example section within opening and closing three backticks
- Extract import statements and remove duplicate imports across tests
- Extract line number of the @example section
- Support multiple examples in one block
- doc can be ignored from testing, by adding ignore to the opening three backticks (eg. ```ignore)
- doc can be skipped from testing, by adding text to the opening three backticks (eg. ```text)
- If doc contains `await` keyword, generated test will be automatically wrapped with async fn

## To use

- Build

  `cargo build --release`

- Run

  `./target/release/deno_doctest --test <File|Dir>`
