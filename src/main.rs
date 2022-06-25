use std::fs::File;
use std::io::prelude::*;
use std::str;

mod error;

const SPACE: u8 = 32u8;
const COLON: u8 = 58u8;
const EQUAL: u8 = 61u8;
const NEWLINE: u8 = b'\n';
const NEWTAB: u8 = b'\t';
const OPEN_BRACE: u8 = 123u8;
const CLOSED_BRACE: u8 = 125u8;
const QUOTES: u8 = 34u8;

fn main() {
    open_file().unwrap();
}

fn open_file() -> std::io::Result<()> {
    let mut file = File::open("assets/file_to_lint.go")?;

    let mut contents = vec![];
    file.read_to_end(&mut contents)?;

    let tokens = tokenize_go(&contents);
    println!("{:?}", tokens);
    Ok(())
}

fn tokenize_go(buffer: &[u8]) -> Result<Vec<String>, error::MyError> {
    let mut tokens: Vec<String> = Vec::new();
    let mut word = Vec::new();
    let mut in_str = false;

    for (i, &c) in buffer.iter().enumerate() {
        match c {
            SPACE | COLON | EQUAL | NEWLINE | NEWTAB | OPEN_BRACE | CLOSED_BRACE
                if { !word.is_empty() } =>
            {
                let from_utf8 = str::from_utf8(&word);
                let from_utf8 = match from_utf8 {
                    Ok(w) => w,
                    Err(_) => panic!("Error parsing utf8 char"),
                };

                tokens.push(from_utf8.to_string());
                word.clear();
            }
            SPACE | COLON | EQUAL | NEWLINE | NEWTAB | OPEN_BRACE | CLOSED_BRACE
                if { !word.is_empty() } =>
            {
                continue;
            }
            QUOTES => {
                if in_str {
                    in_str = false;
                    continue;
                }
                in_str = true;
                word.extend(buffer[i + 1..].iter().copied().take_while(|&c| c != b'"'));
            }
            _ if in_str => {}
            _ => word.push(c),
        }
    }
    Ok(tokens)
}
