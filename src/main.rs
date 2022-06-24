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

    for (i, _) in buffer.iter().enumerate() {
        if buffer[i] == 32u8 || buffer[i] == 58u8 || buffer[i] == 61u8 {
            tokens.push(str::from_utf8(&word).unwrap().to_string());
            word = Vec::new();
            continue;
        }

        let mut j: usize = 0;
        if buffer[i] == 34u8 {
            while buffer[i + j + 1] != 34u8 {
                println!("inside while: {}", buffer[i + j] as char);

                word.push(buffer[i + j]);
                j = j + 1;
            }
            println!("exited")
        }

        word.push(buffer[i]);
    }
    tokens
}
