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

}

pub fn day6_part1(path: &str) -> usize {
    EmptyLineIterator::new(path).map(|l| l.chars().unique().count() - 1).sum()
}

