use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1_example() {
        assert_eq!(day9_part1("data/day9/example", 5), 127);
    }

    #[test]
    fn test_day9_part1() {
        assert_eq!(day9_part1("data/day9/input", 25), 105950735);
    }
    
    #[test]
    fn test_day9_part2() {
        assert_eq!(day9_part2("data/day9/input", 25), 13826915);
    }

    #[test]
    fn test_day9_part2_example() {
        assert_eq!(day9_part2("data/day9/example", 5), 62);
    }
}

pub fn day9_part1(path: &str, preamble: usize) -> i64 {
    let numbers = read_numbers(path);

    for i in preamble..numbers.len() {
        let slice = &numbers[i-preamble..i];

        if !numbers_to_pairs(slice).iter().any(|(x,y)| x+y == numbers[i] as i64) {
            return numbers[i];
        }
    }
    unreachable!();
}

pub fn day9_part2(path: &str, preamble: usize) -> i64 {
    
    let invalid_number = day9_part1(path, preamble);
    println!("invalid {}", invalid_number);
    let numbers = read_numbers(path);

    for i in 0..numbers.len() {
        for j in 2..(numbers.len() - i) {

            let s = numbers.iter().skip(i).take(j).sum::<i64>();

            if s == invalid_number {
                let min = numbers.iter().skip(i).take(j).min().unwrap(); 
                let max = numbers.iter().skip(i).take(j).max().unwrap(); 
                println!("{} {}", min, max);
                return  (min + max) as i64;
            } else if s > invalid_number {
                break;
            }
        }
    }
    unreachable!();
}

fn numbers_to_pairs(numbers: &[i64]) -> Vec<(i64, i64)> {
    (0..numbers.len()).flat_map(|i| (i+1..numbers.len()).map(move |j| (numbers[i], numbers[j]))).collect()
}

fn read_numbers(path: &str) -> Vec<i64> {
    read(path).map(|s| s.parse().unwrap()).collect()
}

