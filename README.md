# My Mini GREP

My implementation of the Rust Lang Book's minigrep program.

## How to Run

1. Open the project's directory on your terminal.
2. Run `cargo run <search_string> <file_path>`, where `<search_string>` is the term you want to search for in the content of the file specified by `<file_path>`.
3. To run a case sensitive search, simply set an environment variable called `CASE_SENSITIVE` with any value assigned to it, as in the example below:

```
CASE_SENSITIVE=1 cargo run the /tests/fixtures/poem.txt
```