const GRID_SIZE: usize = 130;
// const GRID_SIZE: usize = 10;

const BLOCK: u8 = 1 << 0;
const VISITED: u8 = 1 << 1;

const UP: u8 = 1 << 2;
const RIGHT: u8 = 1 << 3;
const DOWN: u8 = 1 << 4;
const LEFT: u8 = 1 << 5;

struct Guard {
    r: usize,
    c: usize,
    dir: u8,
}

fn march(grid: &mut [[u8; GRID_SIZE]; GRID_SIZE], guard: &mut Guard) -> usize {
    //
    let mut seen: usize = 1;
    loop {
        match guard.dir {
            UP => {
                if guard.r == 0 {
                    return seen + 1;
                }
                if grid[guard.r - 1][guard.c] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.r -= 1;
                }
            }
            RIGHT => {
                if guard.c == GRID_SIZE - 1 {
                    return seen + 1;
                }
                if grid[guard.r][guard.c + 1] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.c += 1;
                }
            }
            DOWN => {
                if guard.r == GRID_SIZE - 1 {
                    return seen + 1;
                }
                if grid[guard.r + 1][guard.c] == BLOCK {
                    guard.dir <<= 1;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.r += 1;
                }
            }
            LEFT => {
                if guard.c == 0 {
                    return seen + 1;
                }
                if grid[guard.r][guard.c - 1] == BLOCK {
                    guard.dir = UP;
                } else {
                    if grid[guard.r][guard.c] & VISITED != VISITED {
                        seen += 1;
                    }
                    grid[guard.r][guard.c] |= VISITED;
                    guard.c -= 1;
                }
            }
            _ => unreachable!(),
        }
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut i: usize = 0;

    let mut grid: [[u8; GRID_SIZE]; GRID_SIZE] = [[0; GRID_SIZE]; GRID_SIZE];
    let mut guard: Guard = Guard {
        r: 0,
        c: 0,
        dir: UP,
    };

    for r in 0..GRID_SIZE {
        for c in 0..GRID_SIZE {
            match input[i] {
                b'.' => grid[r][c] = 0,
                b'#' => grid[r][c] = BLOCK,
                b'^' => {
                    grid[r][c] = VISITED | UP;
                    guard.r = r;
                    guard.c = c;
                }
                _ => unreachable!(),
            }
            i += 1;
        }
        i += 1; // input[i] is a newline
    }

    // 4888 is too low
    return march(&mut grid, &mut guard);
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    return 0;
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs;

    fn get_example_input() -> String {
        let input_path = "input/2024/examples/day6.txt";
        fs::read_to_string(input_path).unwrap()
    }

    fn get_input() -> String {
        let input_path = "input/2024/day6.txt";
        fs::read_to_string(input_path).unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_example_input()), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_example_input()), 0);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(part1(&get_input()), 4890)
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(part2(&get_input()), 9999)
    }
}
