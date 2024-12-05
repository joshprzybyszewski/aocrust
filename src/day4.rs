const GRID_SIZE: usize = 140;

const X_SET: u8 = 1 << 0;
const M_SET: u8 = 1 << 1;
const A_SET: u8 = 1 << 2;
const S_SET: u8 = 1 << 3;
const ALL_SET: u8 = X_SET | M_SET | A_SET | S_SET;

const right: usize = 0;
const ur: usize = 1;
const up: usize = 2;
const ul: usize = 3;
const left: usize = 4;
const dl: usize = 5;
const down: usize = 6;
const dr: usize = 7;

#[derive(Copy, Clone)]
struct CoordP1 {
    dirs: [u8; 8],
}

fn p1set_x(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    grid[r][c].dirs[right] |= X_SET;
    grid[r][c].dirs[ur] |= X_SET;
    grid[r][c].dirs[up] |= X_SET;
    grid[r][c].dirs[ul] |= X_SET;
    grid[r][c].dirs[left] |= X_SET;
    grid[r][c].dirs[dl] |= X_SET;
    grid[r][c].dirs[down] |= X_SET;
    grid[r][c].dirs[dr] |= X_SET;
}

fn p1set_m(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    if c > 0 {
        grid[r][c - 1].dirs[right] |= M_SET;
        if r > 0 {
            grid[r - 1][c - 1].dirs[dr] |= M_SET;
        }
        if r < GRID_SIZE - 1 {
            grid[r + 1][c - 1].dirs[ur] |= M_SET;
        }
    }
    if c < GRID_SIZE - 1 {
        grid[r][c + 1].dirs[left] |= M_SET;
        if r > 0 {
            grid[r - 1][c + 1].dirs[dl] |= M_SET;
        }
        if r < GRID_SIZE - 1 {
            grid[r + 1][c + 1].dirs[ul] |= M_SET;
        }
    }
    if r > 0 {
        grid[r - 1][c].dirs[down] |= M_SET;
    }
    if r < GRID_SIZE - 1 {
        grid[r + 1][c].dirs[up] |= M_SET;
    }
}

fn p1set_a(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    if c > 1 {
        grid[r][c - 2].dirs[right] |= A_SET;
        if r > 1 {
            grid[r - 2][c - 2].dirs[dr] |= A_SET;
        }
        if r < GRID_SIZE - 2 {
            grid[r + 2][c - 2].dirs[ur] |= A_SET;
        }
    }
    if c < GRID_SIZE - 2 {
        grid[r][c + 2].dirs[left] |= A_SET;
        if r > 1 {
            grid[r - 2][c + 2].dirs[dl] |= A_SET;
        }
        if r < GRID_SIZE - 2 {
            grid[r + 2][c + 2].dirs[ul] |= A_SET;
        }
    }
    if r > 1 {
        grid[r - 2][c].dirs[down] |= A_SET;
    }
    if r < GRID_SIZE - 2 {
        grid[r + 2][c].dirs[up] |= A_SET;
    }
}

fn p1set_s(grid: &mut [[CoordP1; GRID_SIZE]; GRID_SIZE], r: usize, c: usize) {
    if c > 2 {
        grid[r][c - 3].dirs[right] |= S_SET;
        if r > 2 {
            grid[r - 3][c - 3].dirs[dr] |= S_SET;
        }
        if r < GRID_SIZE - 3 {
            grid[r + 3][c - 3].dirs[ur] |= S_SET;
        }
    }
    if c < GRID_SIZE - 3 {
        grid[r][c + 3].dirs[left] |= S_SET;
        if r > 2 {
            grid[r - 3][c + 3].dirs[dl] |= S_SET;
        }
        if r < GRID_SIZE - 3 {
            grid[r + 3][c + 3].dirs[ul] |= S_SET;
        }
    }
    if r > 2 {
        grid[r - 3][c].dirs[down] |= S_SET;
    }
    if r < GRID_SIZE - 3 {
        grid[r + 3][c].dirs[up] |= S_SET;
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let input = input.as_bytes();
    let mut grid: [[CoordP1; GRID_SIZE]; GRID_SIZE] =
        [[CoordP1 { dirs: [0; 8] }; GRID_SIZE]; GRID_SIZE];

    let mut i: usize = 0;

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'X' => p1set_x(&mut grid, r, c),
                b'M' => p1set_m(&mut grid, r, c),
                b'A' => p1set_a(&mut grid, r, c),
                b'S' => p1set_s(&mut grid, r, c),
                _ => unreachable!(),
            }
            i += 1;
        }
        i += 1; // input[i] is a newline
    }

    let mut total = 0;
    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            for dir in grid[r][c].dirs {
                if dir == ALL_SET {
                    total += 1;
                }
            }
        }
    }

    return total;
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_input() -> String {
        let input_path = "input/2024/day4.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_minimal() {
        assert_eq!(part1("mul(2,3)"), 6);
        assert_eq!(part1("mul(2,3mul(4,5)"), 20);
        assert_eq!(part1("mul(2,3)mul(4,5)mul(2,3"), 26);
        assert_eq!(part1("mul(2,3)add(4,5)mul( 2,3)"), 6);
    }

    #[test]
    fn part2_minimal() {
        assert_eq!(part2("mul(2,3)"), 6);
        assert_eq!(part1("mul(2,3mul(4,5)"), 20);
        assert_eq!(part2("mul(2,3)mul(4,5)mul(2,3"), 26);
        assert_eq!(part1("mul(2,3)add(4,5)mul( 2,3)"), 6);
        assert_eq!(part2("mul(2,3)don't()mul(4,5)do()mul(6,7)do"), 48);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 153469856)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 77055967)
    }
}
