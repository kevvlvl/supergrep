# Supergrep

I created this simple grep command to learn the rust language.

This was also my first experience using [zed](https://zed.dev/).

## How to run

Run tests
```shell
cargo test
```

run the app
```shell
cargo run "Del monte" searched-file.txt
```

## TODO

- Verify file exists and is text file
- Read the file in an efficient manner
- Parse for matching patterns
- Output matching patterns by also providing line number and a few characters before and a few characters after (to show where matching patterns were found)
