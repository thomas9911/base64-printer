# Base64 Printer

Takes in text with base64 encoded strings and prints them out. For instance in
kubernetes configs.

## Usage

```sh
cat test.yaml | cargo run --release

cat test.yaml | base64-printer

cargo run --release < test.yaml
```

Or

```sh
cargo run --release

base64-printer
```

starts and you can paste or type in the text to be decoded.

## Out of scope

multiline base64 strings. For instance:

```sh
cat src/main.rs | base64 | cargo run --release
```

Will just return the base64 of the file still.
