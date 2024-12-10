// How many nines we expect in the input. mine has 257
const MAX_NINES: usize = 512;
// I think that the max number it can reach is _technically_ something like 10^4,
// but because of the coordinate plane it's probably closer to 100. We can bump
// that if we need to
const MAX_REACHABLE: usize = 100;

const GRID_SIZE: usize = 57;
// const GRID_SIZE: usize = 8;

#[derive(Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

#[inline(always)]
fn convert_byte(a: u8) -> u8 {
    return (a - b'0') as u8;
}

#[derive(Copy, Clone)]
struct CanReach {
    reaches: [usize; MAX_REACHABLE],
    index: usize,
}

impl CanReach {
    #[inline(always)]
    fn add_nine(&mut self, nine_id: usize) {
        for i in 0..self.index {
            if self.reaches[i] == nine_id {
                return;
            }
        }
        self.reaches[self.index] = nine_id;
        self.index += 1;
    }

    #[inline(always)]
    fn add_all(&mut self, other: CanReach) {
        // TODO find a way to make this faster, probably.
        for i in 0..other.index {
            self.add_nine(other.reaches[i]);
        }
    }
}

#[inline(always)]
fn build_input(
    input: &str,
) -> (
    [[u8; GRID_SIZE]; GRID_SIZE],
    Vec<Coord>, // zeros
    Vec<Coord>, // pending
    [[CanReach; GRID_SIZE]; GRID_SIZE],
) {
    let input = input.as_bytes();
    // let mut nines: [Coord; MAX_NINES] = [Coord { row: 0, col: 0 }; MAX_NINES];
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut can_reach: [[CanReach; GRID_SIZE]; GRID_SIZE] = [[CanReach {
        reaches: [0; MAX_REACHABLE],
        index: 0,
    }; GRID_SIZE]; GRID_SIZE];

    let mut i: usize = 0;
    let mut nines_id: usize = 1;

    let mut zeros: Vec<Coord> = Vec::with_capacity(512);
    let mut pending: Vec<Coord> = Vec::with_capacity(GRID_SIZE * GRID_SIZE);

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            grid[r][c] = convert_byte(input[i]);
            i += 1;

            if grid[r][c] == 9 {
                let coord = Coord { row: r, col: c };
                pending.push(coord);
                // nines[nines_id] = coord;
                can_reach[r][c].add_nine(nines_id);
                nines_id += 1;
            }
            if grid[r][c] == 0 {
                let coord = Coord { row: r, col: c };
                zeros.push(coord);
            }
        }
        i += 1; // input[i] is a newline
    }

    return (grid, zeros, pending, can_reach);
}

#[inline(always)]
fn check_other(
    grid: [[u8; GRID_SIZE]; GRID_SIZE],
    coord: Coord,
    other: Coord,
    can_reach: &mut [[CanReach; GRID_SIZE]; GRID_SIZE],
    pending: &mut Vec<Coord>,
) {
    if grid[other.row][other.col] + 1 != grid[coord.row][coord.col] {
        return;
    }
    can_reach[other.row][other.col].add_all(can_reach[coord.row][coord.col]);
    pending.push(other);
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let (grid, zeros, mut pending, mut can_reach) = build_input(input);

    while !pending.is_empty() {
        let coord = pending.pop().unwrap();

        // TODO look up, right, down and left.

        // Look up
        if coord.row > 0 {
            let other = Coord {
                row: coord.row - 1,
                col: coord.col,
            };
            check_other(grid, coord, other, &mut can_reach, &mut pending);
        }
        // look right
        if coord.col < GRID_SIZE - 1 {
            let other = Coord {
                row: coord.row,
                col: coord.col + 1,
            };
            check_other(grid, coord, other, &mut can_reach, &mut pending);
        }
        // Look down
        if coord.row < GRID_SIZE - 1 {
            let other = Coord {
                row: coord.row + 1,
                col: coord.col,
            };
            check_other(grid, coord, other, &mut can_reach, &mut pending);
        }
        // look left
        if coord.col > 0 {
            let other = Coord {
                row: coord.row,
                col: coord.col - 1,
            };
            check_other(grid, coord, other, &mut can_reach, &mut pending);
        }
    }

    let mut sum: u64 = 0;
    for coord in zeros {
        sum += can_reach[coord.row][coord.col].index as u64;
    }

    return sum;
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day10.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day10.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 0);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 798)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 0)
    }
}
