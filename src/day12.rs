use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1_example() {
        assert_eq!(day12_part1("data/day12/example1"), 25);
    }

    #[test]
    fn test_day12_part1() {
        assert_eq!(day12_part1("data/day12/input"), 1294);
    }


}

pub fn day12_part1(path: &str) -> i32 {

    let actions = read_actions(path);
    let mut ship = Ship {
        orientation: Orientation::E,
        coordinate: (0,0)
    };

    for a in actions {
        ship = ship.action(a);
    }
    
    println!("{:?}", ship);
    ship.coordinate.0.abs() + ship.coordinate.1.abs()
}

fn read_actions(path: &str) -> Vec<Action> {
    read(path).map(|l| Action::from_line(&l)).collect() 
}

#[derive(Debug)]
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl Action {
    fn from_line(line: &str) -> Action {
        let a = line.chars().nth(0).unwrap();
        let steps: i32 = line[1..].parse().unwrap();

        match a {
            'N' => Action::N(steps),
            'S' => Action::S(steps),
            'E' => Action::E(steps),
            'W' => Action::W(steps),
            'L' => Action::L(steps),
            'R' => Action::R(steps),
            'F' => Action::F(steps),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    N,
    E,
    S,
    W,
}

impl Orientation {

    fn to_number(&self) -> i32 {
        match self {
            Orientation::N => 0,
            Orientation::E => 1,
            Orientation::S => 2,
            Orientation::W => 3
        }
    }

    fn from_number(&self, n: i32) -> Orientation {

        let n = (n + 4 + self.to_number()) % 4;

        match n {
            0 => Orientation::N,
            1 => Orientation::E,
            2 => Orientation::S,
            3 => Orientation::W,
            _ => unreachable!()
        }
    }

    fn from_degrees(&self, a: Action) -> Orientation {

        match a {
            Action::L(degrees) => {
                let steps = (degrees % 360) / 90 as i32;
                self.from_number(-steps)
            }
            Action::R(degrees) => {
                let steps = (degrees % 360) / 90 as i32;
                self.from_number(steps)
            }
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Ship {
    orientation: Orientation,
    coordinate: (i32, i32),
}

impl Ship {
    fn action(&self, action: Action) -> Ship {
        match action {
            Action::N(steps) => Ship {
                orientation: self.orientation,
                coordinate: (self.coordinate.0 + steps, self.coordinate.1),
            },
            Action::E(steps) => Ship {
                orientation: self.orientation,
                coordinate: (self.coordinate.0, self.coordinate.1 + steps),
            },
            Action::S(steps) => Ship {
                orientation: self.orientation,
                coordinate: (self.coordinate.0 - steps, self.coordinate.1),
            },
            Action::W(steps) => Ship {
                orientation: self.orientation,
                coordinate: (self.coordinate.0, self.coordinate.1 - steps),
            },
            Action::L(_) | Action::R(_) => Ship {
                orientation: self.orientation.from_degrees(action),
                coordinate: self.coordinate,
            },
            Action::F(steps) => Ship {
                orientation: self.orientation,
                coordinate: match self.orientation {
                    Orientation::N => (self.coordinate.0 + steps, self.coordinate.1),
                    Orientation::E => (self.coordinate.0, self.coordinate.1 + steps),
                    Orientation::S => (self.coordinate.0 - steps, self.coordinate.1),
                    Orientation::W => (self.coordinate.0, self.coordinate.1 - steps)
                }
            }
            
        }
    }
}
