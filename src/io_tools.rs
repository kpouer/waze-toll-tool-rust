use std::fs::File;
use std::{fs, io};
use std::io::BufRead;
use std::path::Path;

pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) fn tokenize_string(line: &str, separator: &str) -> Vec<String> {
    let tokens = line.split(separator);
    let tokens = tokens.map(|token| token.to_string()).collect::<Vec<String>>();
    tokens
}

pub(crate) fn read_lines_tokens<P>(filename: P) -> io::Result<Vec<Vec<String>>> where P: AsRef<Path>, {
    if let Ok(lines) = read_lines(filename) {
        let mut lines_tokens: Vec<Vec<String>> = Vec::new();
        for line in lines {
            let line = line.unwrap();
            let line = line.trim();
            let tokens = tokenize_string(&line, "\t");
            lines_tokens.push(tokens);
        }
        Ok(lines_tokens)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Error reading file"))
    }
}

pub(crate) fn is_dir(path: &String) -> bool {
    let metadata = fs::metadata(path);
    if metadata.is_err() {
        return false;
    }
    let metadata = metadata.unwrap();
    metadata.is_dir()
}