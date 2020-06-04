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

## Goals
