use std::collections::VecDeque;

const GRID_SIZE: usize = 57;
// const GRID_SIZE: usize = 8;
const DIGIT_SPACE_IN_GRID: usize = GRID_SIZE * GRID_SIZE / 10 + 8;

#[derive(Copy, Clone)]
struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
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

#[inline(always)]
fn convert_byte(a: u8) -> u8 {
    return (a - b'0') as u8;
}

#[derive(Copy, Clone)]
struct CanReach {
    // handles at most 8 * 64 = 512 squares that are the number 9 in a puzzle input.
    reaches: [u64; 8],
}

impl CanReach {
    #[inline(always)]
    fn add_nine(&mut self, nine_id: usize) {
        // same as nine_id / 64
        // let i = nine_id >> 6;
        let i = nine_id / 64;
        // same as 1 << (nine_id % 64)
        // let b: u64 = 1 << (nine_id & 0x3F);
        let b: u64 = 1 << (nine_id % 64);
        self.reaches[i] |= b;
    }

    #[inline(always)]
    fn add_all(&mut self, other: CanReach) {
        for i in 0..self.reaches.len() {
            self.reaches[i] |= other.reaches[i];
        }
    }

    #[inline(always)]
    fn num_reaches(&self) -> u64 {
        let mut total = 0;
        let mut b: u64 = 1;
        for _ in 0..64 {
            for i in 0..self.reaches.len() {
                if self.reaches[i] & b == b {
                    total += 1;
                }
            }
            b <<= 1;
        }
        return total;
    }
}

#[inline(always)]
fn build_input(
    input: &str,
) -> (
    [[u8; GRID_SIZE]; GRID_SIZE],
    Vec<Coord>, // zeros
    Vec<Coord>, // nines
) {
    let input = input.as_bytes();
    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];

    let mut i: usize = 0;

    let mut zeros: Vec<Coord> = Vec::with_capacity(DIGIT_SPACE_IN_GRID);
    let mut nines: Vec<Coord> = Vec::with_capacity(DIGIT_SPACE_IN_GRID);

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            grid[r][c] = convert_byte(input[i]);
            i += 1;

            if grid[r][c] == 9 {
                nines.push(Coord { row: r, col: c });
            }
            if grid[r][c] == 0 {
                zeros.push(Coord { row: r, col: c });
            }
        }
        i += 1; // input[i] is a newline
    }

    return (grid, zeros, nines);
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let (grid, zeros, nines) = build_input(input);

    let mut can_reach: [[CanReach; GRID_SIZE]; GRID_SIZE] =
        [[CanReach { reaches: [0; 8] }; GRID_SIZE]; GRID_SIZE];

    let mut seen: [u64; GRID_SIZE] = [0; GRID_SIZE];
    let mut queue: VecDeque<Coord> = VecDeque::with_capacity(GRID_SIZE * GRID_SIZE);
    for nine_id in 0..nines.len() {
        let nine = nines[nine_id];
        queue.push_front(nine);
        can_reach[nine.row][nine.col].add_nine(nine_id);
    }

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();
        if seen[coord.row] & 1 << coord.col != 0 {
            continue;
        }
        seen[coord.row] |= 1 << coord.col;

        // Look up
        if coord.row > 0 {
            let other = coord.up();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                can_reach[other.row][other.col].add_all(can_reach[coord.row][coord.col]);
                queue.push_back(other);
            }
        }
        // look right
        if coord.col < GRID_SIZE - 1 {
            let other = coord.right();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                can_reach[other.row][other.col].add_all(can_reach[coord.row][coord.col]);
                queue.push_back(other);
            }
        }
        // Look down
        if coord.row < GRID_SIZE - 1 {
            let other = coord.down();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                can_reach[other.row][other.col].add_all(can_reach[coord.row][coord.col]);
                queue.push_back(other);
            }
        }
        // look left
        if coord.col > 0 {
            let other = coord.left();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                can_reach[other.row][other.col].add_all(can_reach[coord.row][coord.col]);
                queue.push_back(other);
            }
        }
    }

    let mut sum: u64 = 0;
    for coord in zeros {
        sum += can_reach[coord.row][coord.col].num_reaches();
    }

    return sum;
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u64 {
    let (grid, zeros, nines) = build_input(input);

    let mut seen: [u64; GRID_SIZE] = [0; GRID_SIZE];
    let mut paths_to_nines: [[u64; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut queue: VecDeque<Coord> = VecDeque::with_capacity(GRID_SIZE * GRID_SIZE);
    for nine in nines {
        queue.push_front(nine);
        paths_to_nines[nine.row][nine.col] = 1;
    }

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();
        if seen[coord.row] & 1 << coord.col != 0 {
            continue;
        }
        seen[coord.row] |= 1 << coord.col;

        // Look up
        if coord.row > 0 {
            let other = coord.up();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                paths_to_nines[other.row][other.col] += paths_to_nines[coord.row][coord.col];
                queue.push_back(other);
            }
        }
        // look right
        if coord.col < GRID_SIZE - 1 {
            let other = coord.right();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                paths_to_nines[other.row][other.col] += paths_to_nines[coord.row][coord.col];
                queue.push_back(other);
            }
        }
        // Look down
        if coord.row < GRID_SIZE - 1 {
            let other = coord.down();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                paths_to_nines[other.row][other.col] += paths_to_nines[coord.row][coord.col];
                queue.push_back(other);
            }
        }
        // look left
        if coord.col > 0 {
            let other = coord.left();
            if grid[other.row][other.col] + 1 == grid[coord.row][coord.col] {
                paths_to_nines[other.row][other.col] += paths_to_nines[coord.row][coord.col];
                queue.push_back(other);
            }
        }
    }

    let mut sum: u64 = 0;
    for coord in zeros {
        sum += paths_to_nines[coord.row][coord.col];
    }

    return sum;
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
        assert_eq!(part2(&get_example_input()), 81);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 798)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 1816)
    }
}
