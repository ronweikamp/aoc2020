use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1_example() {
        assert_eq!(day8_part1("data/day8/example"), 5);
    }

    #[test]
    fn test_day8_part1() {
        assert_eq!(day8_part1("data/day8/input"), 1744);
    }

    #[test]
    fn test_day8_part2_example() {
        assert_eq!(day8_part2("data/day8/example"), 8);
    }

    #[test]
    fn test_day8_part2() {
        assert_eq!(day8_part2("data/day8/input"), 1174);
    }

}

pub fn day8_part1(path: &str) -> usize {
    read_code(path).iter().for_each(|cl| println!("{:?}", cl));
    let mut code = read_code(path);
    start(&mut code).err().unwrap()
}

pub fn day8_part2(path: &str) -> usize {
    let code = read_code(path);

    for i in 0..code.len() {
        let mut code_copy = read_code(path);
        code_copy[i].try_repair();
        match start(&mut code_copy) {
            Ok(acc) => {
                println!("Repair at {} was succesful!, acc {}", i, acc);
                return acc;
            },
            Err(acc) => {
                println!("repair of {} was not succesful, acc {}", i, acc);
            }
        }
    }

    1
}

fn read_code(path: &str) -> Vec<CodeLine> {
    read(path).map(|l| line_to_code(&l)).collect()
}

fn start(code: &mut Vec<CodeLine>) -> Result<usize, usize> {
    
    let mut line_number = 0;
    let mut acc = 0;

    loop {
        let result = execute_instr(acc, line_number, code);
        match result {
            Ok(r) => { 
                println!("line nr {} was ok, acc {}", line_number, acc);
                line_number = r.1;
                acc = r.0;
                if line_number == code.len() {
                    println!("terminating!");
                    return Ok(acc);
                }
            }
            Err(err_line) => {
                println!("Error at nr {}", err_line);
                return Err(acc);
            }
        }
    }
}

fn execute_instr(acc: usize, line_number: usize, mut code: &mut Vec<CodeLine>) -> Result<(usize, usize, &Vec<CodeLine>), usize> {

    let codeline = &mut code[line_number];

    match codeline.visited {
        true => Err(line_number),
        false => Ok({
            codeline.visited();
            match codeline.instruction.op {
                Operation::NOP => (acc, line_number + 1, code),
                Operation::ACC => ((acc as i32 + codeline.instruction.arg) as usize, line_number + 1, code),
                Operation::JMP => (acc, (line_number as i32 + codeline.instruction.arg) as usize, code)
            }
        })
    }
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

impl CodeLine {
    fn visited(&mut self) {
        self.visited = true;
    }

    fn try_repair(&mut self) {
        match self.instruction.op {
            Operation::JMP => {
                self.instruction = Instruction { 
                    op: Operation::NOP,
                    arg: self.instruction.arg,
                };
            },
            Operation::NOP => {
                self.instruction = Instruction {
                    op: Operation::JMP,
                    arg: self.instruction.arg,
                };
            },
            _ => {}// noop
        }
    }
}
