use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1_example() {
        assert_eq!(day10_part1("data/day10/example1"), 7*5);
    }

    #[test]
    fn test_day10_part1_example2() {
        assert_eq!(day10_part1("data/day10/example2"), 22*10);
    }

    #[test]
    fn test_day10_part1() {
        assert_eq!(day10_part1("data/day10/input"), 2312);
    }

    #[test]
    fn test_day10_part2_example() {
        assert_eq!(day10_part2("data/day10/example1"), 8);
    }

    #[test]
    fn test_day10_part2_example2() {
        assert_eq!(day10_part2("data/day10/example2"), 19208);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(day10_part2("data/day10/input"), 12089663946752);
    }

}

pub fn day10_part1(path: &str) -> usize {

    let mut numbers = read_numbers(path); 
    numbers.sort();
    //println!("{:?}", numbers);
    let mut one_differences = 0;
    let mut three_differences = 0;

    let mut prev_jolt_out = 0;
    for jo in numbers {
        //println!("{}", jo);

        match jo - prev_jolt_out {
            1 => {
                one_differences += 1;
            },
            3 => {
                three_differences += 1;
            },
            _ => { println!("diff {}", jo - prev_jolt_out)},
        }
        prev_jolt_out = jo;
    }

    // since your adapter is 3 jolts higher
    three_differences += 1;

    one_differences * three_differences
}

pub fn day10_part2(path: &str) -> usize {

    let mut jolts  = read_numbers(path);
    jolts.push(0);
    jolts.sort();
    jolts.reverse();

    let max = jolts.iter().max().unwrap();

    let mut nodes: Vec<Node> = (0..*max + 4).map(|_| Node {count: 0}).collect();

    nodes[*max].count = 1;

    for j in &jolts[1..] {
        nodes[*j].count = nodes[j+1].count + nodes[j+2].count + nodes[j+3].count;
    }
    
    nodes[0].count
}

fn count_paths(jolt: usize, adapter_set: &HashSet<&usize>, max_adapter: usize) -> usize {

    //println!("{}", jolt);
    if jolt == max_adapter {
        1
    } else if !adapter_set.contains(&jolt) && jolt != 0 {
        0
    } else {
        count_paths(jolt + 1, adapter_set, max_adapter) +
            count_paths(jolt + 2, adapter_set, max_adapter) +
            count_paths(jolt + 3, adapter_set, max_adapter)
    }
}

fn read_numbers(path: &str) -> Vec<usize> {
    read(path).map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Node {
    count : usize
}

