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
    #[test]
    fn test_day11_part2_example() {
        assert_eq!(day11_part2("data/day11/example1"), 26);
    }
    #[test]
    fn test_day11_part2() {
        assert_eq!(day11_part2("data/day11/input"), 2068);
    }

}

pub fn day11_part1(path: &str) -> usize {

    let mut grid = read_grid(path);

    loop {
        let next_grid = grid.next_1(|g, i, j| g.get_occupied_neighbours(i, j), 4);
        if grid == next_grid {
            break;
        }

        grid = next_grid;
    }
    
    grid.occupied_seats()
}

pub fn day11_part2(path: &str) -> usize {

    let mut grid = read_grid(path);

    loop {
        let next_grid = grid.next_1(|g, i, j| g.get_occupied_neighbours2(i, j), 5);
        if grid == next_grid {
            break;
        }

        grid = next_grid;
    }

    grid.occupied_seats()
}

fn read_grid(path: &str) -> Grid {
    Grid {
        points: read(path)
            .map(|l| l.chars().map(|c| GridPoint::from_char(c)).collect())
            .collect()
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


#[derive(Debug, Eq, PartialEq, Hash)]
struct Grid {
    points: Vec<Vec<GridPoint>>,
}

impl Grid {

    fn get_width(&self) -> usize {
        self.points[0].len()
    }

    fn get_length(&self) -> usize {
        self.points.len()
    }

    fn next_1(&self, occupied_neighbours_f: fn(&Grid, usize, usize) -> usize, tolerance: usize) -> Grid {
        let mut new_points = Vec::<Vec::<GridPoint>>::new();

        let width = self.get_width();
        let length = self.get_length();

        for i in 0..length {
            let mut new_row = Vec::<GridPoint>::new();
            for j in 0..width {
                let point = &self.points[i][j];


                let new_point = match point {
                    GridPoint::Seat(occupied) => {
                        let on = occupied_neighbours_f(self, i, j);
                        if *occupied && on >= tolerance {
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

        let directions = [(1,0), (-1,0), (0,-1), (0,1), (1,1), (1,-1), (-1,-1), (-1,1)];

        directions.iter().map(|d| self.neighbours_in_sight(i, j, *d)).sum()
    }

    fn neighbours_in_sight(&self, i: usize, j: usize, direction: (i32, i32)) -> usize {

        let mut new_coord = (i as i32 + direction.0, j as i32 + direction.1);

        loop {
            if self.coord_in_grid(new_coord.0, new_coord.1) {


                match self.points[new_coord.0 as usize][new_coord.1 as usize] { 
                    GridPoint::Seat(true) => {return 1;},
                    GridPoint::Seat(false) => {return 0;},
                    _ => {} //noop
                } 
                
            } else {
                return 0;
            }

            new_coord = (new_coord.0 as i32 + direction.0, new_coord.1 as i32 + direction.1);
        }
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

    fn occupied_seats(&self) -> usize {
        self.points.iter().flat_map(|row| row.iter()).filter(|p| {
            match p {
                GridPoint::Seat(occupied) => *occupied,
                GridPoint::Floor => false
            }
        }).count()
    }
}
