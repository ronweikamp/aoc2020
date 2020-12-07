use std::collections::HashSet;
use crate::utils::read;
use crate::utils::EmptyLineIterator;
use itertools::Itertools;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1_example() {
        assert_eq!(day6_part1("data/day6/example"), 11);
    }

    #[test]
    fn test_day6_part1() {
        assert_eq!(day6_part1("data/day6/input"), 6532);
    }

    #[test]
    fn test_day6_part2_example() {
        assert_eq!(day6_part2("data/day6/example"), 6);
    }

    #[test]
    fn test_day6_part2() {
        assert_eq!(day6_part2("data/day6/input"), 3427);
    }

}

pub fn day6_part1(path: &str) -> usize {
    EmptyLineIterator::new(path).map(|l| l.chars().unique().count() - 1).sum()
}

pub fn day6_part2(path: &str) -> usize {
    EmptyLineIterator::new(path)
        .map(|l| {
            let mut set = HashSet::new();
            // init set
            l.split(' ').nth(0).unwrap().chars().for_each(|c| {
                set.insert(c);
            });

            for answers_person in l.split(' ').filter(|ap| ap.len() > 0) {
                let answer_set_person = answers_person.chars().collect::<HashSet<char>>();
                set = set.intersection(&answer_set_person).map(|c| *c).collect();
            }

            set
        })
        .map(|s| s.len()).sum()
}

