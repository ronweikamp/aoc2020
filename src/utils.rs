use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read(path: &str) -> impl Iterator<Item=String> {
    let br = BufReader::new(File::open(path).expect("No such file"));
    br.lines().map(|line| line.expect("Expect a valid String"))
}


pub struct EmptyLineIterator {
    wrapped: Box<dyn Iterator<Item = String>>,
}

impl EmptyLineIterator {
    pub fn new(path: &str) -> EmptyLineIterator {
        EmptyLineIterator {
            wrapped: Box::new(read(path)),
        }
    }
    
}

impl Iterator for EmptyLineIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x = String::new();
        
        while let Some(line) = self.wrapped.next() {
            if line.len() > 0 {
                x.push_str(" ");
                x.push_str(&line);
            } else {
                return Some(x);
            }
        }

        // last line
        if x.len() > 0 {
            return Some(x);
        }

        return None;
    }
}

