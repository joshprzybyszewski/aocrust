use std::collections::VecDeque;

// I think that the max number it can reach is _technically_ something like 10^4,
// but because of the coordinate plane it's probably closer to 100. We can bump
// that if we need to
const MAX_REACHABLE: usize = 100;

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
    fn add_all(&mut self, other: CanReach) -> bool {
        // TODO find a way to make this faster, probably.
        let prev = self.index;
        for i in 0..other.index {
            self.add_nine(other.reaches[i]);
        }
        return prev != self.index;
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

#[inline(always)]
fn check_other_1(
    grid: [[u8; GRID_SIZE]; GRID_SIZE],
    coord: Coord,
    other: Coord,
    can_reach: &mut [[CanReach; GRID_SIZE]; GRID_SIZE],
    pending: &mut VecDeque<Coord>,
) {
    if grid[other.row][other.col] + 1 != grid[coord.row][coord.col] {
        return;
    }
    if !can_reach[other.row][other.col].add_all(can_reach[coord.row][coord.col]) {
        return;
    }
    pending.push_back(other);
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> u64 {
    let (grid, zeros, nines) = build_input(input);

    let mut can_reach: [[CanReach; GRID_SIZE]; GRID_SIZE] = [[CanReach {
        reaches: [0; MAX_REACHABLE],
        index: 0,
    }; GRID_SIZE]; GRID_SIZE];

    let mut queue: VecDeque<Coord> = VecDeque::with_capacity(GRID_SIZE * GRID_SIZE);
    for nine_id in 0..nines.len() {
        let nine = nines[nine_id];
        queue.push_front(nine);
        can_reach[nine.row][nine.col].add_nine(nine_id);
    }

    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();

        // Look up
        if coord.row > 0 {
            check_other_1(grid, coord, coord.up(), &mut can_reach, &mut queue);
        }
        // look right
        if coord.col < GRID_SIZE - 1 {
            check_other_1(grid, coord, coord.right(), &mut can_reach, &mut queue);
        }
        // Look down
        if coord.row < GRID_SIZE - 1 {
            check_other_1(grid, coord, coord.down(), &mut can_reach, &mut queue);
        }
        // look left
        if coord.col > 0 {
            check_other_1(grid, coord, coord.left(), &mut can_reach, &mut queue);
        }
    }

    let mut sum: u64 = 0;
    for coord in zeros {
        sum += can_reach[coord.row][coord.col].index as u64;
    }

    return sum;
}

#[inline(always)]
fn check_other_2(
    grid: [[u8; GRID_SIZE]; GRID_SIZE],
    coord: Coord,
    other: Coord,
    paths_to_nines: &mut [[u64; GRID_SIZE]; GRID_SIZE],
    pending: &mut VecDeque<Coord>,
) {
    if grid[other.row][other.col] + 1 != grid[coord.row][coord.col] {
        return;
    }
    paths_to_nines[other.row][other.col] += paths_to_nines[coord.row][coord.col];
    pending.push_back(other);
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
            check_other_2(grid, coord, coord.up(), &mut paths_to_nines, &mut queue);
        }
        // look right
        if coord.col < GRID_SIZE - 1 {
            check_other_2(grid, coord, coord.right(), &mut paths_to_nines, &mut queue);
        }
        // Look down
        if coord.row < GRID_SIZE - 1 {
            check_other_2(grid, coord, coord.down(), &mut paths_to_nines, &mut queue);
        }
        // look left
        if coord.col > 0 {
            check_other_2(grid, coord, coord.left(), &mut paths_to_nines, &mut queue);
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
