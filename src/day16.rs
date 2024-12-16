use std::cmp::{min, Ordering};
// use std::ops::{Index, IndexMut};

const GRID_SIZE: usize = 141;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(r: usize, c: usize) -> Self {
        return Coord { row: r, col: c };
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row.cmp(&other.row).then(self.col.cmp(&other.col))
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Position {
    cost: u64,
    coord: Coord,
    direction: Direction,
}

impl Position {
    fn new(coord: Coord) -> Self {
        return Position {
            coord,
            cost: 0,
            direction: Direction::East,
        };
    }

    fn forward(&self) -> Position {
        match self.direction {
            Direction::East => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col + 1),
                    cost: self.cost + 1,
                    direction: self.direction,
                };
            }
            Direction::North => {
                return Position {
                    coord: Coord::new(self.coord.row - 1, self.coord.col),
                    cost: self.cost + 1,
                    direction: self.direction,
                };
            }
            Direction::West => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col - 1),
                    cost: self.cost + 1,
                    direction: self.direction,
                };
            }
            Direction::South => {
                return Position {
                    coord: Coord::new(self.coord.row + 1, self.coord.col),
                    cost: self.cost + 1,
                    direction: self.direction,
                };
            }
        }
    }

    fn left(&self) -> Position {
        match self.direction {
            Direction::East => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::North,
                };
            }
            Direction::North => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::West,
                };
            }
            Direction::West => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::South,
                };
            }
            Direction::South => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::East,
                };
            }
        }
    }

    fn right(&self) -> Position {
        match self.direction {
            Direction::East => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::South,
                };
            }
            Direction::North => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::East,
                };
            }
            Direction::West => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::North,
                };
            }
            Direction::South => {
                return Position {
                    coord: self.coord,
                    cost: self.cost + 1000,
                    direction: Direction::West,
                };
            }
        }
    }
}

struct Finder {
    start: Coord,
    goal: Coord,
    best: [[[u64; 4]; GRID_SIZE]; GRID_SIZE],
}

impl Finder {
    fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let mut start: Option<Coord> = None;
        let mut goal: Option<Coord> = None;
        let mut best: [[[u64; 4]; GRID_SIZE]; GRID_SIZE] =
            [[[0xFF_FF_FF_FF_FF_FF_FF_FF; 4]; GRID_SIZE]; GRID_SIZE];

        let mut i: usize = 0;

        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                match input[i] {
                    b'#' => {
                        best[r][c][0] = 0;
                        best[r][c][1] = 0;
                        best[r][c][2] = 0;
                        best[r][c][3] = 0;
                    }

                    b'.' => {}
                    b'S' => {
                        start = Some(Coord::new(r, c));
                    }
                    b'E' => {
                        goal = Some(Coord::new(r, c));
                    }
                    _ => unreachable!(),
                }
                i += 1;
            }
            i += 1; // input[i] is a newline
        }

        if start.is_none() || goal.is_none() {
            unreachable!();
        }

        return Finder {
            start: start.unwrap(),
            goal: goal.unwrap(),
            best,
        };
    }

    fn find(&mut self) -> u64 {
        let mut queue: Vec<Position> = Vec::with_capacity(GRID_SIZE * GRID_SIZE * 4);
        queue.push(Position::new(self.start));

        let mut target_cost = self.get_best_goal_cost();

        while !queue.is_empty() {
            let pos = queue.pop().unwrap();
            if pos.cost >= target_cost || pos.cost >= self.get_cost(pos) {
                continue;
            }
            println!("Processing {:?}", pos);
            self.set_cost(pos);
            // self.process(pos.forward(), queue);
            queue.push(pos.forward());
            queue.push(pos.left());
            queue.push(pos.right());
            // queue.sort();

            if pos.coord == self.goal {
                // update the target cost
                target_cost = self.get_best_goal_cost();
            }
        }

        return target_cost;
    }

    fn get_best_goal_cost(&self) -> u64 {
        let mut best_cost = min(
            self.best[self.goal.row as usize][self.goal.col as usize][0],
            self.best[self.goal.row as usize][self.goal.col as usize][1],
        );
        for i in 2..4 {
            best_cost = min(
                best_cost,
                self.best[self.goal.row as usize][self.goal.col as usize][i],
            )
        }
        return best_cost;
    }

    fn get_cost(&self, pos: Position) -> u64 {
        match pos.direction {
            Direction::East => self.best[pos.coord.row as usize][pos.coord.col as usize][0],
            Direction::North => self.best[pos.coord.row as usize][pos.coord.col as usize][1],
            Direction::West => self.best[pos.coord.row as usize][pos.coord.col as usize][2],
            Direction::South => self.best[pos.coord.row as usize][pos.coord.col as usize][3],
        }
    }

    fn set_cost(&mut self, pos: Position) {
        match pos.direction {
            Direction::East => {
                self.best[pos.coord.row as usize][pos.coord.col as usize][0] = pos.cost
            }
            Direction::North => {
                self.best[pos.coord.row as usize][pos.coord.col as usize][1] = pos.cost
            }
            Direction::West => {
                self.best[pos.coord.row as usize][pos.coord.col as usize][2] = pos.cost
            }
            Direction::South => {
                self.best[pos.coord.row as usize][pos.coord.col as usize][3] = pos.cost
            }
        }
    }
}

// impl Index<Position> for [[[u64; 4]; GRID_SIZE]; GRID_SIZE] {
//     type Output = u64;

//     fn index(&self, pos: Position) -> &Self::Output {
//         match pos.direction {
//             Direction::East => &self[pos.coord.row as usize][pos.coord.col as usize][0],
//             Direction::North => &self[pos.coord.row as usize][pos.coord.col as usize][1],
//             Direction::West => &self[pos.coord.row as usize][pos.coord.col as usize][2],
//             Direction::South => &self[pos.coord.row as usize][pos.coord.col as usize][3],
//         }
//     }
// }

// impl IndexMut<Position> for [[[u64; 4]; GRID_SIZE]; GRID_SIZE] {
//     fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
//         match pos.direction {
//             Direction::East => &mut self[pos.coord.row as usize][pos.coord.col as usize][0],
//             Direction::North => &mut self[pos.coord.row as usize][pos.coord.col as usize][1],
//             Direction::West => &mut self[pos.coord.row as usize][pos.coord.col as usize][2],
//             Direction::South => &mut self[pos.coord.row as usize][pos.coord.col as usize][3],
//         }
//     }
// }

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.coord.cmp(&other.coord))
            .then(self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u64 {
    // the reindeer games...
    let mut finder = Finder::new(input);
    return finder.find();
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day16.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 0);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0);
    }
}
