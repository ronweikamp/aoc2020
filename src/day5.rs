use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1_example() {
        assert_eq!(get_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_day5_part1() {

        let max_seat_id = read("data/day5/input").map(|code| get_seat_id(&code)).max().unwrap();
        assert_eq!(max_seat_id, 818);
    }


}

pub fn get_seat_id(code: &str) -> usize {
    bin_space_partition(&code[..7], 1, 128) * 8 + bin_space_partition(&code[7..], 1, 8)
}

fn bin_space_partition(code: &str, lower: i32, upper: i32) -> usize {

    let half = Half::new(code.chars().nth(0).unwrap());

    println!("code: {}, half: {:?}, lower: {}, upper: {}", code, half, lower, upper);

    if code.len() == 1 {
        match half {
            Half::Upper => (upper - 1) as usize,
            Half::Lower => (lower - 1) as usize,
        }
    } else {
        match half {
            Half::Upper => bin_space_partition(&code[1..], 
                lower + 2_i32.pow((code.len() - 1) as u32),
                upper),
            Half::Lower => bin_space_partition(&code[1..], 
                lower,
                upper - 2_i32.pow((code.len() - 1) as u32)),
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

