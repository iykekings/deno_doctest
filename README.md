# deno_doctest
 proposal on how deno doctest will work
 
 .deno_doctest.ts is sample file generated that will plugged in to deno runtime
 for the tests to be run.
 
 - Currently: It can
  - Extract JsDoc comments
  - Extract @example section from it
  - Extract caption
  - Extract the code in the example section
  - Extract import statements and remove duplicate imports across tests
  
- Would love(util.rs::extract_jsdoc_examples): If it can
  - Extract line number where the JSDocs were picked up
  
- Wouldn't like: If it
  - May take 2x the current execution time to get the line number
  - Then it will be unnecessary - since caption might just suffice
 
