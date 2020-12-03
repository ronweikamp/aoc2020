use std::fs::File;
use std::io::{BufRead, BufReader};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1_example() {
        assert_eq!(day3_part1("data/day3/example"), 7);
    }
    #[test]
    fn test_day3_part1() {
        assert_eq!(day3_part1("data/day3/input"), 252);
    }
    #[test]
    fn test_day3_part2_example() {
        assert_eq!(day3_part2("data/day3/example"), 336);
    }
    #[test]
    fn test_day3_part2() {
        assert_eq!(day3_part2("data/day3/input"), 2608962048);
    }
}

pub fn day3_part2(path: &str) -> usize {
    let map = &read_map(path);
    let start = &Coord {
        x : 0,
        y: 0,
    };
    trip(map, start, 0, 1, 1) * 
        trip(map, start, 0, 3, 1) * 
        trip(map, start, 0, 5, 1) *
        trip(map, start, 0, 7, 1) *
        trip(map, start, 0, 1, 2)
}

pub fn day3_part1(path: &str) -> usize {
    let map = &read_map(path);
    let start = Coord {
        x : 0,
        y: 0,
    };
    trip(map, &start, 0, 3, 1)
}

fn trip(map: &Map, coord: &Coord, count_trees: usize, h_move: usize, v_move: usize) -> usize {

    if coord.y == map.len() - 1 {
        count_trees
    } else {
        let next_coord = coord.step(map[0].len(), h_move, v_move);
        match map[next_coord.y][next_coord.x] {
            GridType::Square => trip(map, &next_coord, count_trees, h_move, v_move),
            GridType::Tree => trip(map, &next_coord, count_trees + 1, h_move, v_move)
        }
    }
}


type Map = Vec<Vec<GridType>>;

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn step(&self, map_width: usize, h_move: usize, v_move: usize) -> Coord {
        Coord {
            x: (self.x + h_move) % map_width,
            y: self.y + v_move,
        }
    }
}

#[derive(Debug)]
enum GridType {
    Square,
    Tree,
}

fn read_map(path: &str) -> Vec<Vec<GridType>> {
    read(path).map(|line| line.chars().map(char_to_gridtype).collect()).collect()
}

fn char_to_gridtype(c: char) -> GridType {
    match c {
        '.' => GridType::Square,
        '#' => GridType::Tree,
        _ => unreachable!("Invalid char '{}'", c)
    }
} 

fn read(path: &str) -> impl Iterator<Item=String> {
    let br = BufReader::new(File::open(path).expect("No such file"));
    br.lines().map(|line| line.expect("Expect a valid String"))
}
