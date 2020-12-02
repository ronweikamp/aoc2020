use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1_part1() -> i32 {
    println!("{:?}", read());

    for num1 in read().iter() {
        for num2 in read().iter() {
            if num1 + num2 == 2020 {
                println!("num1: {} num2: {}", num1, num2);
                return num1*num2
            }
        }
    }

    return -1
}

pub fn day1_part2() -> i32 {
    println!("{:?}", read());

    for num1 in read().iter() {
        for num2 in read().iter() {
            for num3 in read().iter() {
                if num1 + num2 + num3 == 2020 {
                    println!("num1: {} num2: {}, num3: {}", num1, num2, num3);
                    return num1*num2*num3
                }
            }
        }
    }

    return -1
}

fn read() -> Vec<i32> {
    let br = BufReader::new(File::open("data/day1/input").expect("No such file"));
    return br.lines()
        .map(|line| line.expect("sadf").trim().parse().expect("cannot parse line"))
        .collect();
}

