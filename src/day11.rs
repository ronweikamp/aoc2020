use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1_example() {
        assert_eq!(day11_part1("data/day11/example1"), 37);
    }
    #[test]
    fn test_day11_part1() {
        assert_eq!(day11_part1("data/day11/input"), 2324);
    }

}

pub fn day11_part1(path: &str) -> usize {

    let mut grid = read_grid(path);

    //println!("{:?}", grid);
    //println!("{:?}", grid == grid);
    //println!("{:?}", grid == grid.next());
    //println!("{:?}", grid.next().next().next().next().next().next());
    //println!("{:?}", grid.next().next().next().next().next().next() == grid.next().next().next().next().next().next().next());
    //println!("{:?}", grid.next().next().next().next().next().next().next());
    //println!("{:?}", grid.next());
    //println!("{:?}", grid.next().next());

    loop {
        let next_grid = grid.next();
        if grid == next_grid {
            break;
        }

        grid = next_grid;
    }

    grid.points.iter().flat_map(|row| row.iter()).filter(|p| {
        match p {
            GridPoint::Seat(occupied) => *occupied,
            GridPoint::Floor => false
        }
    }).count()
}

fn read_grid(path: &str) -> Grid {
    let mut points = Vec::<Vec::<GridPoint>>::new();

    for line in read(path) {
        let mut row = Vec::<GridPoint>::new();

        for c in line.chars() {
            row.push(GridPoint::from_char(c));
        }
        points.push(row);
    }

    Grid {
        points: points
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GridPoint {
    Seat(bool),
    Floor,
}

impl GridPoint {
    pub fn from_char(c: char) -> GridPoint {
        match c {
            '.' => GridPoint::Floor,
            'L' => GridPoint::Seat(false),
            '#' => GridPoint::Seat(true),
            _ => unreachable!()
        }
    }
}

//type Grid = Vec<Vec<GridPoint>>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Grid {
    points: Vec<Vec<GridPoint>>,
}

impl Grid {
    fn copy(&self) -> Grid {
        Grid {
            points: self.points.clone(),
        }
    }

    fn get_width(&self) -> usize {
        self.points[0].len()
    }

    fn get_length(&self) -> usize {
        self.points.len()
    }

    fn next(&self) -> Grid {
        let mut new_points = Vec::<Vec::<GridPoint>>::new();

        let width = self.get_width();
        let length = self.get_length();

        for i in 0..length {
            let mut new_row = Vec::<GridPoint>::new();
            for j in 0..width {
                let point = &self.points[i][j];

                //println!("({}, {}) on {} ", i, j, self.get_occupied_neighbours(i, j));

                let new_point = match point {
                    GridPoint::Seat(occupied) => {
                        let on = self.get_occupied_neighbours(i, j);
                        if *occupied && on >= 4 {
                            GridPoint::Seat(false)
                        } else if *occupied {
                            GridPoint::Seat(true)
                        } else {
                            if on == 0 {
                                GridPoint::Seat(true)
                            } else {
                                GridPoint::Seat(false)
                            }
                        }
                    },
                    GridPoint::Floor => GridPoint::Floor
                };

                new_row.push(new_point);
            }
            new_points.push(new_row);
        }


        Grid {
            points: new_points,
        }
    }

    fn get_occupied_neighbours(&self, i: usize, j: usize) -> usize {
        self.get_neighbours(i, j).iter()
            .filter(|n| match n {
                GridPoint::Seat(occupied) => *occupied,
                _ => false,
            }).count()
    }

    fn get_occupied_neighbours2(&self, i: usize, j: usize) -> usize {
        self.get_neighbours(i, j).iter()
            .filter(|n| match n {
                GridPoint::Seat(occupied) => *occupied,
                _ => false,
            }).count()
    }


    fn get_neighbours(&self, m: usize, n: usize) -> Vec<&GridPoint> {

        let i = m as isize;
        let j = n as isize;

        let potential_neighbours = [(i+1,j), (i-1,j), 
                                (i, j+1), (i, j-1), 
                                (i-1, j-1), (i+1, j+1), (i-1, j+1), (i+1, j-1)];

        let mut neighbours = Vec::<&GridPoint>::new();

        for (k,l) in potential_neighbours.iter() {
            if self.coord_in_grid(*k as i32, *l as i32) {
                neighbours.push(&self.points[*k as usize][*l as usize]);
            }
        }

        neighbours
    }

    fn coord_in_grid(&self, i: i32, j: i32) -> bool {
        let width = self.points[0].len() as i32;
        let length = self.points.len() as i32;

        !(i < 0 || i >= length || j < 0 || j >= width)
    }

    fn count_occupied(&self) -> usize {
        self.points.iter().flat_map(|row| row.iter()).count()
    }
}
