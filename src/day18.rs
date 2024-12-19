use std::collections::VecDeque;
use std::{thread, time};

struct CorruptionProvider<'a> {
    input: &'a [u8],
    i: usize,
}

impl<'a> CorruptionProvider<'a> {
    #[inline(always)]
    fn new(input: &'a str) -> Self {
        return CorruptionProvider {
            input: input.as_bytes(),
            i: 0,
        };
    }

    #[inline(always)]
    fn next(&mut self) -> Coord {
        let mut r = (self.input[self.i] - b'0') as usize;
        self.i += 1;
        while self.input[self.i] != b',' {
            r *= 10;
            r += (self.input[self.i] - b'0') as usize;
            self.i += 1;
        }
        self.i += 1;

        let mut c = (self.input[self.i] - b'0') as usize;
        self.i += 1;
        while self.input[self.i] != b'\n' {
            c *= 10;
            c += (self.input[self.i] - b'0') as usize;
            self.i += 1;
        }
        self.i += 1;

        return Coord::new(r, c);
    }
}

const MAX_GRID_SIZE: usize = 73;
const MAX_GRID_VALUE: u64 = (MAX_GRID_SIZE * MAX_GRID_SIZE * 2) as u64;

#[derive(Copy, Clone, Debug)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    fn new(r: usize, c: usize) -> Self {
        return Coord { row: r, col: c };
    }

    #[inline(always)]
    fn to_one_index(&self) -> Coord {
        return Coord {
            row: self.row + 1,
            col: self.col + 1,
        };
    }

    fn to_string(&self) -> String {
        let mut array: [u8; 32] = [b','; 32];
        let i: usize;

        if self.row >= 10 {
            array[0] = b'0' + (self.row / 10) as u8;
            array[1] = b'0' + (self.row % 10) as u8;
            array[2] = b',';
            i = 3;
        } else {
            array[0] = b'0' + self.row as u8;
            array[1] = b',';
            i = 2;
        }
        if self.col >= 10 {
            array[i] = b'0' + (self.col / 10) as u8;
            array[i + 1] = b'0' + (self.col % 10) as u8;
            return String::from_utf8_lossy(&array[0..i + 2]).to_string();
        }
        array[i] = b'0' + self.col as u8;
        return String::from_utf8_lossy(&array[0..i + 1]).to_string();
    }

    #[inline(always)]
    fn up(&self) -> Coord {
        return Coord {
            row: self.row - 1,
            col: self.col,
        };
    }

    #[inline(always)]
    fn right(&self) -> Coord {
        return Coord {
            row: self.row,
            col: self.col + 1,
        };
    }

    #[inline(always)]
    fn down(&self) -> Coord {
        return Coord {
            row: self.row + 1,
            col: self.col,
        };
    }

    #[inline(always)]
    fn left(&self) -> Coord {
        return Coord {
            row: self.row,
            col: self.col - 1,
        };
    }
}

#[derive(Copy, Clone, Debug)]
struct Step {
    coord: Coord,
    cost: u64, // TODO u16
}

impl Step {
    fn new(coord: Coord, cost: u64) -> Self {
        return Step { coord, cost };
    }

    #[inline(always)]
    fn up(&self) -> Step {
        return Step {
            coord: self.coord.up(),
            cost: self.cost + 1,
        };
    }

    #[inline(always)]
    fn right(&self) -> Step {
        return Step {
            coord: self.coord.right(),
            cost: self.cost + 1,
        };
    }

    #[inline(always)]
    fn down(&self) -> Step {
        return Step {
            coord: self.coord.down(),
            cost: self.cost + 1,
        };
    }

    #[inline(always)]
    fn left(&self) -> Step {
        return Step {
            coord: self.coord.left(),
            cost: self.cost + 1,
        };
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
    let mut cheapest: [[u64; MAX_GRID_SIZE]; MAX_GRID_SIZE] =
        [[MAX_GRID_VALUE; MAX_GRID_SIZE]; MAX_GRID_SIZE];

    for x in 0..MAX_GRID_SIZE {
        cheapest[x][0] = 0;
        cheapest[x][MAX_GRID_SIZE - 1] = 0;
        cheapest[0][x] = 0;
        cheapest[MAX_GRID_SIZE - 1][x] = 0;
    }

    let mut provider = CorruptionProvider::new(input);
    for _ in 0..1024 {
        let corruption = provider.next().to_one_index();
        cheapest[corruption.row][corruption.col] = 0;
    }

    let mut queue: VecDeque<Step> = VecDeque::with_capacity(4 * MAX_GRID_SIZE * MAX_GRID_SIZE);
    queue.push_front(Step::new(Coord::new(1, 1), 0));

    while !queue.is_empty() {
        let step = queue.pop_front().unwrap();
        if cheapest[step.coord.row][step.coord.col] <= step.cost {
            // println!("not cheapest {:?}", step);
            continue;
        }
        // println!("Checking {:?}", step);

        cheapest[step.coord.row][step.coord.col] = step.cost;

        // Look up
        let next = step.up();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
        // look right
        let next = step.right();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
        // Look down
        let next = step.down();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
        // look left
        let next = step.left();
        if cheapest[next.coord.row][next.coord.col] > next.cost {
            queue.push_back(next);
        }
    }

    return cheapest[MAX_GRID_SIZE - 2][MAX_GRID_SIZE - 2];
}

const UNKNOWN: u8 = 1 << 0;
const LEFT: u8 = 1 << 1;
const RIGHT: u8 = 1 << 2;
const BOTH: u8 = LEFT | RIGHT;

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let mut side: [[u8; MAX_GRID_SIZE]; MAX_GRID_SIZE] = [[0; MAX_GRID_SIZE]; MAX_GRID_SIZE];

    for x in 1..MAX_GRID_SIZE - 1 {
        side[x][0] = LEFT;
        side[MAX_GRID_SIZE - 1][x] = LEFT;

        side[0][x] = RIGHT;
        side[x][MAX_GRID_SIZE - 1] = RIGHT;
    }

    let mut provider = CorruptionProvider::new(input);

    loop {
        // zero-index'd, but needs to be one-indexed.
        let corruption = provider.next();

        let nearby = side[corruption.row][corruption.col]
            | side[corruption.row][corruption.col + 1]
            | side[corruption.row][corruption.col + 2]
            | side[corruption.row + 1][corruption.col]
            | side[corruption.row + 1][corruption.col + 2]
            | side[corruption.row + 2][corruption.col]
            | side[corruption.row + 2][corruption.col + 1]
            | side[corruption.row + 2][corruption.col + 2];
        let value = nearby & BOTH;
        if value == BOTH {
            return corruption.to_string();
        }

        side[corruption.row + 1][corruption.col + 1] = UNKNOWN;
        if value != 0 {
            infect_nearby(&mut side, value, corruption.to_one_index());
        }
        // print_sides(&side);
    }
}

fn infect_nearby(side: &mut [[u8; MAX_GRID_SIZE]; MAX_GRID_SIZE], value: u8, coord: Coord) {
    if side[coord.row][coord.col] != UNKNOWN {
        return;
    }

    side[coord.row][coord.col] = value;
    infect_nearby(side, value, coord.up().left());
    infect_nearby(side, value, coord.up());
    infect_nearby(side, value, coord.up().right());
    infect_nearby(side, value, coord.down().left());
    infect_nearby(side, value, coord.down());
    infect_nearby(side, value, coord.down().right());
    infect_nearby(side, value, coord.left());
    infect_nearby(side, value, coord.right());
}

fn print_sides(side: &[[u8; MAX_GRID_SIZE]; MAX_GRID_SIZE]) {
    //
    println!("-------------------------");
    for row in side {
        for col in row {
            if *col & BOTH == BOTH {
                print!("B");
            } else if *col & LEFT == LEFT {
                print!("L");
            } else if *col & RIGHT == RIGHT {
                print!("R");
            } else if *col & UNKNOWN == UNKNOWN {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day18.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 294)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), "31,22")
    }
}
