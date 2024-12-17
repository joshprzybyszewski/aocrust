use std::cmp::min;

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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

impl Direction {
    fn index(self) -> usize {
        match self {
            Direction::East => return 0,
            Direction::North => return 1,
            Direction::West => return 2,
            Direction::South => return 3,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Position {
    cost: u64,
    coord: Coord,
    direction: Direction,

    id: usize,
    prev_ids: Vec<usize>,
}

impl Position {
    fn new(id: usize, coord: Coord) -> Self {
        return Position {
            coord,
            cost: 0,
            direction: Direction::East,
            id: id,
            prev_ids: Vec::new(),
        };
    }

    fn forward(&self, id: usize) -> Position {
        match self.direction {
            Direction::East => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col + 1),
                    cost: self.cost + 1,
                    direction: self.direction,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::North => {
                return Position {
                    coord: Coord::new(self.coord.row - 1, self.coord.col),
                    cost: self.cost + 1,
                    direction: self.direction,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::West => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col - 1),
                    cost: self.cost + 1,
                    direction: self.direction,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::South => {
                return Position {
                    coord: Coord::new(self.coord.row + 1, self.coord.col),
                    cost: self.cost + 1,
                    direction: self.direction,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
        }
    }

    fn left(&self, id: usize) -> Position {
        match self.direction {
            Direction::East => {
                return Position {
                    coord: Coord::new(self.coord.row - 1, self.coord.col),
                    cost: self.cost + 1001,
                    direction: Direction::North,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::North => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col - 1),
                    cost: self.cost + 1001,
                    direction: Direction::West,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::West => {
                return Position {
                    coord: Coord::new(self.coord.row + 1, self.coord.col),
                    cost: self.cost + 1001,
                    direction: Direction::South,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::South => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col + 1),
                    cost: self.cost + 1001,
                    direction: Direction::East,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
        }
    }

    fn right(&self, id: usize) -> Position {
        match self.direction {
            Direction::East => {
                return Position {
                    coord: Coord::new(self.coord.row + 1, self.coord.col),
                    cost: self.cost + 1001,
                    direction: Direction::South,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::North => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col + 1),
                    cost: self.cost + 1001,
                    direction: Direction::East,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::West => {
                return Position {
                    coord: Coord::new(self.coord.row - 1, self.coord.col),
                    cost: self.cost + 1001,
                    direction: Direction::North,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
            Direction::South => {
                return Position {
                    coord: Coord::new(self.coord.row, self.coord.col - 1),
                    cost: self.cost + 1001,
                    direction: Direction::West,
                    id,
                    prev_ids: Vec::from([self.id]),
                };
            }
        }
    }
}

struct Finder {
    start: Coord,
    goal: Coord,
    // the best cost. A cache of using the best_index.
    best: [[[u64; 4]; GRID_SIZE]; GRID_SIZE],
    // the index into self.fifo to find the Position with the best cost.
    best_index: [[[usize; 4]; GRID_SIZE]; GRID_SIZE],

    fifo: Vec<Position>,
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
            best_index: [[[0; 4]; GRID_SIZE]; GRID_SIZE],
            fifo: Vec::new(),
        };
    }

    fn find<const PART1: bool>(&mut self) -> u64 {
        // zero'th element represents invalid for backwards lookup.
        self.fifo.push(Position::new(0, Coord::new(0, 0)));
        self.fifo.push(Position::new(1, self.start));
        let mut i: usize = 1;

        while i < self.fifo.len() {
            if self.check_cost(i) {
                i += 1;
                continue;
            }

            // TODO push forward until hitting a wall.
            self.fifo.push(self.fifo[i].forward(self.fifo.len()));
            if self.choose_left(i) {
                self.fifo.push(self.fifo[i].left(self.fifo.len()));
                self.fifo.push(self.fifo[i].right(self.fifo.len()));
            } else {
                self.fifo.push(self.fifo[i].right(self.fifo.len()));
                self.fifo.push(self.fifo[i].left(self.fifo.len()));
            }
            i += 1;
        }
        if PART1 {
            return self.get_best_goal_cost();
        }

        return self.get_best_paths_length();
    }

    fn choose_left(&self, id: usize) -> bool {
        let pos = &self.fifo[id];

        match pos.direction {
            Direction::East => {
                return pos.coord.row > self.goal.row;
            }
            Direction::North => {
                return pos.coord.col > self.goal.col;
            }
            Direction::West => {
                return pos.coord.row < self.goal.row;
            }
            Direction::South => {
                return pos.coord.col < self.goal.col;
            }
        }
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

    fn check_cost(&mut self, id: usize) -> bool {
        let pos = &self.fifo[id];
        if self.best[pos.coord.row][pos.coord.col][pos.direction.index()] < pos.cost {
            return true;
        }
        if self.best[pos.coord.row][pos.coord.col][pos.direction.index()] == pos.cost {
            let current_best_id =
                self.best_index[pos.coord.row][pos.coord.col][pos.direction.index()];
            self.fifo[current_best_id].prev_ids.push(id);
            return true;
        }
        self.best[pos.coord.row][pos.coord.col][pos.direction.index()] = pos.cost;
        self.best_index[pos.coord.row][pos.coord.col][pos.direction.index()] = id;
        return false;
    }

    fn get_best_paths_length(&self) -> u64 {
        let mut seen: [[bool; GRID_SIZE]; GRID_SIZE] = [[false; GRID_SIZE]; GRID_SIZE];
        let best_goal_cost = self.get_best_goal_cost();
        for dir_i in 0..4 {
            if best_goal_cost < self.best[self.goal.row][self.goal.col][dir_i] {
                continue;
            }
            self.get_best_paths_length_from_id(
                &mut seen,
                self.best_index[self.goal.row][self.goal.col][dir_i],
            );
        }

        let mut output = 0;
        for row in seen {
            for val in row {
                if val {
                    output += 1;
                }
            }
        }
        return output;
    }

    fn get_best_paths_length_from_id(&self, seen: &mut [[bool; GRID_SIZE]; GRID_SIZE], id: usize) {
        // if id == 0 {
        //     // shouldn't happen!
        //     unreachable!();
        // }
        let pos = &self.fifo[id];
        seen[pos.coord.row][pos.coord.col] = true;
        // if id == 1 {
        //     if pos.prev_ids.len() > 0 {
        //         // sanity check
        //         unreachable!();
        //     }
        // }

        for i in 0..pos.prev_ids.len() {
            self.get_best_paths_length_from_id(seen, pos.prev_ids[i]);
        }
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> u64 {
    // the reindeer games...
    let mut finder = Finder::new(input);
    return finder.find::<true>();
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> u64 {
    let mut finder = Finder::new(input);
    return finder.find::<false>();
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
        assert_eq!(part1(&get_input()), 147628);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 670);
    }
}
