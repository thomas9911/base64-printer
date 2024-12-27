# Base64 Printer

Takes in text with base64 encoded strings and prints them out. For instance in
kubernetes configs.

## Installation

```sh
cargo install --git https://github.com/thomas9911/base64-printer.git
```

Now you can use it by calling `base64-printer` in your terminal.

Or

```sh
git clone https://github.com/thomas9911/base64-printer.git && cd ./base64-printer
```

Then you need to do `cargo run --release` ect

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
