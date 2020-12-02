use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2_part1() -> i32 {

    let lines = read("src/day2/input".to_string());
    
    return lines.iter().filter(|line| is_valid(line.to_string())).count() as i32;
}

fn is_valid(line: String) -> bool {
    let v = line.split(":").collect::<Vec<_>>();
    let range = line.split(":").nth(0).expect("Must exist").split(" ").nth(0).expect("Must exist");
    let letter = line.split(":").nth(0).expect("Must exist").split(" ").nth(1).expect("Must exist");
    
    let min_occurrences = range.split("-").nth(0).expect("Must exist").parse().expect("Must be a number");
    let max_occurrences = range.split("-").nth(1).expect("Must exist").parse().expect("Must be a number");

    let password = v[1];

    let num_matches_letter = password.chars().filter(|c| *c == letter.chars().nth(0).expect("should be char")).count();

    return num_matches_letter >= min_occurrences && num_matches_letter <= max_occurrences;
}

fn read(path: String) -> Vec<String> {
    let br = BufReader::new(File::open(path).expect("No such file"));
    return br.lines()
        .map(|line| line.expect("Expect a valid String"))
        .collect();
}
