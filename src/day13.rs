use std::cmp::max;
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
    #[test]
    fn test_day13_part2() {
        assert_eq!(day13_part2_input_with_crt().unwrap(), 598411311431841);
    }
    
}


pub fn day13_part1(path: &str) -> usize {

    let (earliest, busses) = read_input(path);

    let t = busses.iter().map(|b| (b - earliest % b, b)).min_by(|t1, t2| t1.0.cmp(&t2.0)).unwrap();
    t.0 * t.1
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}
 
fn day13_part2_input_with_crt() -> Option<i64> {
    let input = [(54, 13), (58, 17), (91, 19), (64, 23), (43, 29), (35, 37), (0, 41), (72, 419), (41, 557)];

    let modulii: Vec<i64> = input.iter().map(|(_, p)| *p).collect();
    let residues: Vec<i64> = input.iter().map(|(o, p)| (p-o) % p ).collect();
    //let modulii = [3,5];
    //let residues = [0,4];
 
    chinese_remainder(&residues, &modulii)
}

pub fn day13_part2(path: &str) -> usize {

    let numbers = read_input2(path);
    let mut num_enumeration: Vec<(usize, usize)> = read_input2(path).iter().enumerate()
        .filter(|(_, n)| *n > &(0 as usize))
        .map(|(a,b)| (a, *b))
        .collect();
    num_enumeration.sort_by(|t1, t2| t1.1.cmp(&t2.1));

    let base = numbers.iter().nth(0).unwrap();
    let (offset, max_p) = num_enumeration.iter().max_by(|t1, t2| t1.1.cmp(&t2.1)).unwrap();
    print!("{:?}", num_enumeration);
    println!("max {} {}", offset, max_p);
    let mut multiplier: usize = 1;

    let mut skip: usize = 0;

    let mut iteration : usize = 0;

    loop {
        let target =multiplier * max_p - offset;

        if num_enumeration.iter().all(|(i, n)| (target + i) % n == 0) {
            println!("mult {}", multiplier);
            println!("skip {}", skip);
            return target;
        } else {
            multiplier = multiplier + 1;
        }

        if iteration % 1000000000 == 0 {
            println!("iter, target {}, {}", iteration, target);
        }
        iteration = iteration + 1;
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


