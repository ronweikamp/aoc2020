use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1_example() {
        assert_eq!(day8_part1("data/day8/example"), 4);
    }

}

pub fn day8_part1(path: &str) -> usize {
    read_code(path).iter().for_each(|cl| println!("{:?}", cl));
    3
}

fn read_code(path: &str) -> Vec<CodeLine> {
    read(path).map(|l| line_to_code(&l)).collect()
}

fn line_to_code(line: &str) -> CodeLine {
    let op_code = line.split(' ').nth(0).unwrap();
    let arg: i32 = line.split(' ').nth(1).unwrap().parse().unwrap();

    let op = match op_code {
        "nop" => Operation::NOP,
        "acc" => Operation::ACC,
        "jmp" => Operation::JMP,
        _ => Operation::NOP,
    };

    CodeLine {
        instruction: Instruction {
            op: op,
            arg: arg,
        },
        visited: false,
    }
}

#[derive(Debug)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

#[derive(Debug)]
struct CodeLine {
    instruction: Instruction,
    //number: usize,
    visited: bool,
}
