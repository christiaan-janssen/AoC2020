use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::fs::File;

/// Read a file line by line into a Vec<i32>
pub fn read_lines_to_i32(path: &str) -> Result<Vec<i32>, std::io::Error> {
    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut v = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let n = line.trim().parse::<i32>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        v.push(n)
    }
    Ok(v)
}

/// Takes a path to a file and returns a Vec<String>. Keep in mind:
/// No error handling is done!
pub fn lines_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("cant read file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}