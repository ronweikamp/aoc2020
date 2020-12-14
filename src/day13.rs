use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_part1_example() {
        assert_eq!(day13_part1("data/day13/example1"), 295);
    }

    #[test]
    fn test_day13_part1() {
        assert_eq!(day13_part1("data/day13/input"), 2095);
    }
    #[test]
    fn test_day13_part2_example() {
        assert_eq!(day13_part2("data/day13/example1"), 1068781);
    }
    #[test]
    fn test_day13_part2_example2() {
        assert_eq!(day13_part2("data/day13/example2"), 3417);
    }
    #[test]
    fn test_day13_part2_example5() {
        assert_eq!(day13_part2("data/day13/example5"), 1261476);
    }
    #[test]
    fn test_day13_part2_example6() {
        assert_eq!(day13_part2("data/day13/example6"), 1202161486);
    }
    
}

pub fn day13_part1(path: &str) -> usize {

    let (earliest, busses) = read_input(path);

    let t = busses.iter().map(|b| (b - earliest % b, b)).min_by(|t1, t2| t1.0.cmp(&t2.0)).unwrap();
    t.0 * t.1
}

pub fn day13_part2(path: &str) -> usize {

    let numbers = read_input2(path);

    let base = numbers.iter().nth(0).unwrap();
    let mut multiplier: usize = 1;

    loop {
        if numbers.iter().enumerate()
            .filter(|(_, n)| *n > &(0 as usize))
            .all(|(i, n)| (base * multiplier + i) % n == 0) {
            return multiplier * base;
        } else {
            multiplier = multiplier + 1;
        }
    }


}

fn read_input2(path: &str) -> Vec<usize> {


    let numbers: Vec<usize> = read(path).nth(1).unwrap().split(',')
        .map(|l| if l == "x" {
            0
        } else {
            l.parse().unwrap()
        }).collect();

    numbers
}

fn read_input(path: &str) -> (usize, Vec<usize>) {

    let earliest = read(path).nth(0).unwrap().parse().unwrap();

    let numbers: Vec<usize> = read(path).nth(1).unwrap().split(',')
        .filter(|l| *l != "x")
        .map(|l| l.parse().unwrap()).collect();

    (earliest, numbers)
}


