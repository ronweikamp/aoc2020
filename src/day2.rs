use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2_part1() -> i32 {

    let lines = read("data/day2/input".to_string());
    
    return lines.iter().filter(|line| is_valid(line.to_string())).count() as i32;
}

pub fn day2_part2() -> i32 {

    let lines = read("data/day2/input".to_string());
    
    return lines.iter().filter(|line| is_valid2(line.to_string())).count() as i32;
}

fn is_valid(line: String) -> bool {
    
    let (min_occurrences, max_occurrences, letter, password) = line_to_tuple(line);

    let num_matches_letter = password.chars().filter(|c| *c == letter).count();

    return num_matches_letter >= min_occurrences && num_matches_letter <= max_occurrences;
}

fn is_valid2(line: String) -> bool {
    let (first_occurrence, last_occurrence, letter, password) = line_to_tuple(line);

    let first_matches = char_equals(password.to_string(), first_occurrence, letter);
    let last_matches = char_equals(password.to_string(), last_occurrence, letter);
    return first_matches && !last_matches || !first_matches && last_matches;
}

fn char_equals(password: String, index: usize, c: char) -> bool {
    if password.len() <= index {
        false
    } else {
        password.chars().nth(index).expect("Must exist") == c
    }
}

fn line_to_tuple(line: String) -> (usize, usize, char, String) {
    let left = line.split(":").nth(0).expect("Must exist");
    let password = line.split(":").nth(1).expect("Must exist");
    let range = left.split(" ").nth(0).expect("Must exist");
    let letter = left.split(":").nth(0).expect("Must exist").split(" ").nth(1).expect("Must exist").chars().nth(0).expect("exist");
    
    let first_occurrence = range.split("-").nth(0).expect("Must exist").parse().expect("Must be a number");
    let last_occurrence = range.split("-").nth(1).expect("Must exist").parse().expect("Must be a number");

    return (first_occurrence, last_occurrence, letter, password.to_string());
}

fn read(path: String) -> Vec<String> {
    let br = BufReader::new(File::open(path).expect("No such file"));
    return br.lines()
        .map(|line| line.expect("Expect a valid String"))
        .collect();
}
