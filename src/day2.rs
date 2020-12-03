use std::fs::File;
use std::io::{BufRead, BufReader};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        assert_eq!(day2_part1(), 622);
    }

    #[test]
    fn test_day2() {
        assert_eq!(day2_part2(), 263);
    }
}

pub fn day2_part1() -> i32 {

    let lines = read("data/day2/input");
    
    return lines.iter().filter(|line| is_valid(line)).count() as i32;
}

pub fn day2_part2() -> i32 {

    let lines = read("data/day2/input");
    
    return lines.iter().filter(|line| is_valid2(line)).count() as i32;
}

fn is_valid(line: &str) -> bool {
    
    let (min_occurrences, max_occurrences, letter, password) = line_to_tuple(line).unwrap();

    let num_matches_letter = password.chars().filter(|c| *c == letter).count();

    return num_matches_letter >= min_occurrences && num_matches_letter <= max_occurrences;
}

fn is_valid2(line: &str) -> bool {
    let (first_occurrence, last_occurrence, letter, password) = line_to_tuple(line).unwrap();

    let first_matches = char_equals(password, first_occurrence, letter);
    let last_matches = char_equals(password, last_occurrence, letter);
    return first_matches && !last_matches || !first_matches && last_matches;
}

fn char_equals(password: &str, index: usize, c: char) -> bool {
    if password.len() <= index {
        false
    } else {
        password.chars().nth(index).expect("Must exist") == c
    }
}

fn line_to_tuple(line: &str) -> Option<(usize, usize, char, &str)> {
    let left = line.split(":").nth(0)?;
    let password = line.split(":").nth(1)?;
    let range = left.split(" ").nth(0)?;
    let letter = left.split(":").nth(0)?.split(" ").nth(1)?.chars().nth(0)?;
    
    let first_occurrence = range.split("-").nth(0)?.parse().ok()?;
    let last_occurrence = range.split("-").nth(1)?.parse().ok()?;

    return Some((first_occurrence, last_occurrence, letter, password));
}

fn read(path: &str) -> Vec<String> {
    let br = BufReader::new(File::open(path).expect("No such file"));
    return br.lines()
        .map(|line| line.expect("Expect a valid String"))
        .collect();
}
