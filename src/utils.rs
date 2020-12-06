use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read(path: &str) -> impl Iterator<Item=String> {
    let br = BufReader::new(File::open(path).expect("No such file"));
    br.lines().map(|line| line.expect("Expect a valid String"))
}
