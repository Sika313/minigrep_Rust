# minigrep_Rust

This is a demo project that uses a query string to search the poem.txt file. It takes two
command line arguments, the first is the query string, the second is poem.txt.
E.g. cargo run to poem.txt
In the example, we are searching for the string 'to' in poem.txt.

To make the search case insensitive, set an environment variable 'CASE_INSENSITIVE' to 1.
E.g. CASE_INSENSITIVE=1 cargo run to poem.txt
