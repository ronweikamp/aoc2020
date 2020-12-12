use crate::utils::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1_example() {
        assert_eq!(day12_part1("data/day12/example1"), 25);
    }
    
    #[test]
    fn test_day12_part2_example() {
        assert_eq!(day12_part2("data/day12/example1"), 286);
    }
    #[test]
    fn test_day12_part2_example2() {
        assert_eq!(day12_part2("data/day12/example2"), 10);
    }

    #[test]
    fn test_day12_part1() {
        assert_eq!(day12_part1("data/day12/input"), 1294);
    }
    
    #[test]
    fn test_day12_part2() {
        assert_eq!(day12_part2("data/day12/input"), 20592);
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

pub fn day12_part2(path: &str) -> i32 {

    let actions = read_actions(path);
    let mut ship = Ship2 {
        waypoint: Waypoint {
            y: (Orientation::N, 1),
            x: (Orientation::E, 10)
        },
        coordinate: (0,0)
    };

    for a in actions {
        println!("{:?}", a);
        ship = ship.action(a);

        println!("{:?}", ship);
    }
    
    //println!("{:?}", ship);
    ship.coordinate.0.abs() + ship.coordinate.1.abs()
}

fn read_actions(path: &str) -> Vec<Action> {
    read(path).map(|l| Action::from_line(&l)).collect() 
}

#[derive(Debug, Clone, Copy)]
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
                let steps = match degrees {
                    90 => 1,
                    180 => 2,
                    270 => 3,
                    _ => unreachable!()
                };
                
                self.from_number(-steps)
            }
            Action::R(degrees) => {
                let steps = match degrees {
                    90 => 1,
                    180 => 2,
                    270 => 3,
                    _ => unreachable!()
                };
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

#[derive(Debug, Clone, Copy)]
struct Waypoint {
    y: (Orientation, i32),
    x: (Orientation, i32)
}

impl Waypoint {

    fn do_action(&self, action: Action) -> Waypoint {
        match action {
            Action::N(value) => Waypoint {
                y: (self.y.0, match self.y.0 {
                    Orientation::N => self.y.1 + value,
                    _ => self.y.1 - value
                }),
                x: self.x
            },
            Action::E(value) => Waypoint {
                y: self.y,
                x: (self.x.0, match self.x.0 {
                    Orientation::E => self.x.1 + value,
                    Orientation::W => self.x.1 - value,
                    _ => unreachable!()
                }),
            },
            Action::S(value) => Waypoint {
                y: (self.y.0, match self.y.0 {
                    Orientation::N => self.y.1 - value,
                    _ => self.y.1 + value
                }),
                x: self.x
            },
            Action::W(value) => Waypoint {
                y: self.y,
                x: (self.x.0, match self.x.0 {
                    Orientation::E => self.x.1 - value,
                    Orientation::W => self.x.1 + value,
                    _ => unreachable!()
                }),
            },
            Action::L(_) | Action::R(_) => {
                let ((y_or, y_n), (x_or, x_n)) = match self.y.0.from_degrees(action) {
                    Orientation::N | Orientation::S => ((self.y.0.from_degrees(action), self.y.1), (self.x.0.from_degrees(action), self.x.1)),
                    _ => ((self.x.0.from_degrees(action), self.x.1), (self.y.0.from_degrees(action), self.y.1))
                };
                
                Waypoint {
                    y: (y_or, y_n),
                    x: (x_or, x_n)
                }
            },
            Action::F(_) => unreachable!()
        }
    }

    fn action(&self, action: Action) -> Waypoint {
        let new_waypoint = self.do_action(action);
        
        // flip orientation
        if new_waypoint.y.1 < 0 {
            match new_waypoint.y.0 {
                Orientation::N => Waypoint {
                    y: (Orientation::S, -new_waypoint.y.1),
                    x: new_waypoint.x
                },
                Orientation::S => Waypoint {
                    y: (Orientation::N, -new_waypoint.y.1),
                    x: new_waypoint.x
                },
                _ => unreachable!()
            }
        } else if new_waypoint.x.1 < 0 {
            match new_waypoint.x.0 {
                Orientation::E => Waypoint {
                    y: new_waypoint.y,
                    x: (Orientation::W, -new_waypoint.x.1)
                },
                Orientation::W => Waypoint {
                    y: new_waypoint.y,
                    x: (Orientation::E, -new_waypoint.x.1)
                },
                _ => unreachable!()
            }
        } else {
            new_waypoint
        }
    }


    fn x_direction(&self) -> i32 {
        match self.x.0 {
            Orientation::E => self.x.1,
            Orientation::W => -self.x.1,
            _ => 0
        }
    }

    fn y_direction(&self) -> i32 {
        match self.y.0 {
            Orientation::N => self.y.1,
            Orientation::S => -self.y.1,
            _ => 0
        }
    }

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

#[derive(Debug)]
struct Ship2 {
    waypoint: Waypoint,
    coordinate: (i32, i32)
}

impl Ship2 {

    fn action(&self, action: Action) -> Ship2 {

        match action {
            Action::N(_) | Action::E(_) | Action::S(_) | Action::W(_) | Action::L(_) | Action::R(_) => Ship2 {
                waypoint: self.waypoint.action(action),
                coordinate: self.coordinate
            },
            Action::F(steps) => Ship2 {
                waypoint: self.waypoint,
                coordinate: (self.coordinate.0 + steps * self.waypoint.y_direction(), 
                    self.coordinate.1 + steps * self.waypoint.x_direction())
            }
            
        }
    }
}
