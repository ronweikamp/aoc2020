use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1_example() {
        assert_eq!(day5_part1("FBFBBFF"), 44);
    }
}

pub fn day5_part1(code: &str) -> usize {
    bin_space_partition(code, 1, 128, 'B', 'F')
}

fn bin_space_partition(code: &str, lower: i32, upper: i32, lower_char: char, upper_char: char) -> usize {

    let half = Half::new(code.chars().nth(0).unwrap());

    if code.len() == 1 {
        match half {
            Half::Upper => (upper - 1) as usize,
            Half::Lower => (lower - 1) as usize,
        }
    } else {
        match half {
            Half::Upper => bin_space_partition(&code[1..], 
                lower + 2_i32.pow((code.len() - 1) as u32),
                upper, lower_char, upper_char),
            Half::Lower => bin_space_partition(&code[1..], 
                lower,
                upper - 2_i32.pow((code.len() - 1) as u32), lower_char, upper_char),
        }
    }
}

#[derive(Debug)]
enum Half {
    Upper,
    Lower,
}

impl Half {
    fn new(c: char) -> Half {
        match c {
            'B' | 'R' => Half::Upper,
            'F' | 'L' => Half::Lower,
            _ => unreachable!()
        }
    }
}

