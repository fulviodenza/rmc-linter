use std::fs::File;
use std::io::prelude::*;
use std::str;

fn main() {
    open_file().unwrap();
}

fn open_file() -> std::io::Result<()> {
    let file = File::open("assets/file_to_lint.go")?;
    let mut file_copy = file.try_clone()?;

    let mut contents = vec![];
    file_copy.read_to_end(&mut contents)?;

    let tokens = tokenize_go(&mut contents);
    for t in tokens {
        print!("{} ->", t);
    }
    Ok(())
}

fn tokenize_go(buffer: &mut Vec<u8>) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut word: Vec<u8> = Vec::new();
    let mut b: bool = false;

    for (i, _) in buffer.iter().enumerate() {
        match buffer[i] {
            32u8 | 58u8 | 61u8 => {
                tokens.push(str::from_utf8(&word).unwrap().to_string());
                word = Vec::new();
                continue;
            }
            34u8 => {
                if b {
                    if buffer[i] == b'"' {
                        b = false;
                    }
                    println!("continuing");
                    continue;
                }
                let mut k = i + 1;
                b = true;

                while buffer[k] != b'"' {
                    println!("{}", buffer[k]);
                    word.push(buffer[k]);
                    k = k + 1;
                }
                println!("exited");
            }
            _ => word.push(buffer[i]),
        }
    }
    tokens
}
