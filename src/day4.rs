use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1_example() {
        assert_eq!(day4_part1("data/day4/example"), 2);
    }
    #[test]
    fn test_day4_part1() {
        assert_eq!(day4_part1("data/day4/input"), 230);
    }
}

pub fn day4_part1(path: &str) -> usize {
    read_between_lines(path).for_each(|x| println!("{}", x));
    let map = read_between_lines(path);
    map.map(|line| Passport::new(&line))
        .filter(|p| p.is_valid())
        .count()
}

//    byr (Birth Year)
//    iyr (Issue Year)
//    eyr (Expiration Year)
//    hgt (Height)
//    hcl (Hair Color)
//    ecl (Eye Color)
//    pid (Passport ID)
//    cid (Country ID)

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn new(line: &str) -> Passport {
        
        Passport {
            byr: get_val_for_key(line, "byr"),
            iyr: get_val_for_key(line, "iyr"),
            eyr: get_val_for_key(line, "eyr"),
            hgt: get_val_for_key(line, "hgt"),
            hcl: get_val_for_key(line, "hcl"),
            ecl: get_val_for_key(line, "ecl"),
            pid: get_val_for_key(line, "pid"),
            cid: get_val_for_key(line, "cid"),
        }
    }

    fn is_valid(&self) -> bool {
        [&self.byr, &self.iyr, &self.eyr, &self.hgt, &self.hcl, &self.ecl, &self.pid].iter().all(|field| field.is_some())
    }
}

fn get_val_for_key(line: &str, key: &str) -> Option<String> {
    line.split(" ")
        .filter(|pair| pair.split(":").nth(0).unwrap() == key)
        .map(|pair| pair.split(":").nth(1).unwrap().to_string())
        .find(|_| true)
} 

// remove new lines between subsequent non empty lines
fn read_between_lines(path: &str) -> impl Iterator<Item=String> {
    PasspordLineIterator::new(path)
}

// https://stackoverflow.com/questions/47838596/how-do-i-have-a-structs-field-be-an-iterator-over-t-elements/47838741
// https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator-or-any-other-trait
struct PasspordLineIterator {
    wrapped: Box<dyn Iterator<Item = String>>,
}

impl PasspordLineIterator {
    fn new(path: &str) -> PasspordLineIterator {
        PasspordLineIterator {
            wrapped: Box::new(read(path)),
        }
    }
    
}

impl Iterator for PasspordLineIterator {
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

