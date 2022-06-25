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

    for (i, _) in buffer.iter_mut().enumerate() {
        if buffer[i] == 32u8 || buffer[i] == 58u8 || buffer[i] == 61u8 || buffer[i] == b'\n' {
            println!("cleaning");
            tokens.push(str::from_utf8(&word).unwrap().to_string());
            word = Vec::new();
            continue;
        }

        let mut j: usize = i + 1;
        if buffer[i] == b'"' {
            while buffer[j] != b'"' {
                println!("inside while: {}", buffer[j] as char);

                word.push(buffer[j]);
                j = j + 1;
            }
            println!("exited");
        }

        println!(
            "i+1 is: {} and j is: {}, buffer is {}",
            i + 1,
            j,
            buffer[i] as char
        );
        if j == i + 1 {
            word.push(buffer[i]);
        }
    }
    tokens
}
