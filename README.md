# My Mini GREP

My implementation of the Rust Lang Book's minigrep program.

## How to Run

1. Open the project's directory on your terminal.
2. Run ˋcargo run <search_string> <file_path>ˋ, where ˋ<search_string>ˋ is the term you want to search for in the content of the file specified by ˋ<file_path>ˋ.
3. To run a case sensitive search, simply set an environment variable called CASE_SENSITIVE with any value assigned to it, as in the example below:

ˋˋˋsh
CASE_SENSITIVE=1 cargo run the /tests/fixtures/poem.txt
ˋˋˋ