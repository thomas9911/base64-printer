use base64::prelude::*;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::borrow::Cow;
use std::io::{self, IsTerminal, Write};

fn is_ascii_printable(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_graphic())
}

struct Base64Swapper;

impl regex::Replacer for Base64Swapper {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        if let Some(x) = caps.get(0) {
            if let Ok(binary) = BASE64_STANDARD.decode(x.as_str()) {
                if let Ok(string) = std::str::from_utf8(&binary) {
                    if is_ascii_printable(string) {
                        dbg!(string);
                        dst.push_str(string);
                        return
                    }
                }
            }

            dst.push_str(x.as_str());
        }
    }
}

fn replace_base64(haystack: &str) -> Cow<'_, str> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([[:alnum:]]|\/|=|\+)+").unwrap());
    RE.replace_all(haystack, Base64Swapper)
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut buffer_up = false;
    if stdin.is_terminal() {
        print!("> ");
        io::stdout().flush()?;
        buffer_up = true;
}

    let lines = io::stdin().lines();
    let mut out = Vec::new();
    for line in lines {
        let line = line?;
        if line.starts_with("\u{4}") {
            break
        }
        if buffer_up {
            out.push(replace_base64(&line).to_string());
        } else {
            println!("{}", replace_base64(&line));
        }
    }

    for line in out {
        println!("{}", line);
    }

    Ok(())
}
