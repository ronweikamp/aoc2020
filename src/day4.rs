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

    #[test]
    fn test_day4_part2() {
        assert_eq!(day4_part2("data/day4/input"), 156);
    }
}

pub fn day4_part1(path: &str) -> usize {
    let map = read_between_lines(path);
    map.map(|line| Passport::new(&line))
        .filter(|p| p.is_valid())
        .count()
}

pub fn day4_part2(path: &str) -> usize {
    let map = read_between_lines(path);
    map.map(|line| Passport::new(&line))
        .filter(|p| p.is_valid2())
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

    fn is_valid2(&self) -> bool {
        

        if self.is_valid() {
            byr_valid(&self.byr.as_ref().unwrap()) &&
                iyr_valid(&self.iyr.as_ref().unwrap()) &&
                eyr_valid(&self.eyr.as_ref().unwrap()) &&
                hgt_valid(&self.hgt.as_ref().unwrap()) &&
                hcl_valid(&self.hcl.as_ref().unwrap()) &&
                ecl_valid(&self.ecl.as_ref().unwrap()) &&
                self.pid.as_ref().unwrap().chars().filter(|c| c.is_digit(10)).count() == 9
       } else {
            false
        }
    }

}

fn byr_valid(text: &str) -> bool {
    x_digits_min_max(text, 4, 1920, 2002)
}

fn iyr_valid(text: &str) -> bool {
    x_digits_min_max(text, 4, 2010, 2020)
}    

fn eyr_valid(text: &str) -> bool {
    x_digits_min_max(text, 4, 2020, 2030)
}

fn hgt_valid(text: &str) -> bool {
    if text.len() < 4 {
        return false;
    }

    match &text[text.len() - 2..] {
        "cm" => x_digits_min_max(&text[..3], 3, 150, 193),
        "in" => x_digits_min_max(&text[..2], 2, 59, 76),
        _ => {
            println!("hgt invalid {}", text);
            false
        } 
    }
}

fn hcl_valid(text: &str) -> bool {
    if text.len() != 7 {
        return false;        
    }

    if &text[..1] != "#" {
        return false;
    }

    text[1..].chars().all(|c| c.is_digit(16))
}

fn ecl_valid(text: &str) -> bool {
    match text {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }
}

fn x_digits_min_max(text: &str, num: usize, min: usize, max: usize) -> bool {
    if text.chars().all(|c| c.is_digit(10)) &&
        text.len() == num &&
        text.parse::<usize>().unwrap() >= min &&
        text.parse::<usize>().unwrap() <= max {
            true
        } else {
            println!("invalid {}", text);
            false
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

