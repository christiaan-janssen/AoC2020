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