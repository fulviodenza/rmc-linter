use std::fs::File;
use std::io::prelude::*;
use std::str;

const SPACE: u8 = 32u8;
const COLON: u8 = 58u8;
const EQUAL: u8 = 61u8;
const NEWLINE: u8 = b'\n';
const NEWTAB: u8 = b'\t';
const OPEN_BRACE: u8 = 123u8;
const CLOSED_BRACE: u8 = 125u8;

fn main() {
    open_file().unwrap();
}

fn open_file() -> std::io::Result<()> {
    let file = File::open("assets/file_to_lint.go")?;
    let mut file_copy = file.try_clone()?;

    let mut contents = vec![];
    file_copy.read_to_end(&mut contents)?;

    let tokens = tokenize_go(&mut contents);
    println!("{:?}", tokens);
    Ok(())
}

fn tokenize_go(buffer: &mut Vec<u8>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut word: Vec<u8> = Vec::new();
    let mut in_str: bool = false;

    for (i, c) in buffer.iter().enumerate() {
        match c {
            &SPACE | &COLON | &EQUAL | &NEWLINE | &NEWTAB | &OPEN_BRACE | &CLOSED_BRACE
                if { word.len() != 0 } =>
            {
                tokens.push(str::from_utf8(&word).unwrap().to_string());
                word = Vec::new();
                continue;
            }
            b' ' | 58u8 | b'=' | b'\n' | b'\t' | b'{' | b'}' if { word.len() == 0 } => {
                continue;
            }
            34u8 => {
                if in_str {
                    if *c == b'"' {
                        in_str = false;
                    }
                    continue;
                }
                let mut k = i + 1;
                in_str = true;

                while buffer[k] != b'"' {
                    word.push(buffer[k]);
                    k = k + 1;
                }
            }
            _ => {
                if in_str {
                    continue;
                }
                word.push(*c);
            }
        }
    }
    tokens
}
